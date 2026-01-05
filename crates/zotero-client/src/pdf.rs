//! PDF text extraction with position information using MuPDF.
//!
//! This module provides functionality to extract text from PDFs along with
//! position information (bounding boxes/quads) that can be used to create
//! highlight annotations in Zotero.
//!
//! Uses the same underlying MuPDF library as PyMuPDF, ensuring identical
//! text extraction and positioning behavior.
//!
//! Requires the `pdf` feature to be enabled.

use crate::error::{Result, ZoteroClientError};
use crate::types::{OutlineItem, PdfOutline, TextFragment};

use mupdf::{Document, Outline, Quad, TextPageOptions};
use std::path::Path;

/// A quad (4-point polygon) representing text position.
/// This is more accurate than rectangles for rotated or skewed text.
#[derive(Debug, Clone, PartialEq)]
pub struct TextQuad {
    /// Upper-left point
    pub ul: (f32, f32),
    /// Upper-right point
    pub ur: (f32, f32),
    /// Lower-left point
    pub ll: (f32, f32),
    /// Lower-right point
    pub lr: (f32, f32),
}

impl TextQuad {
    /// Convert quad to a bounding rectangle [x1, y1, x2, y2].
    /// Note: This loses precision for rotated text.
    pub fn to_rect(&self) -> [f64; 4] {
        let x1 = self.ul.0.min(self.ll.0) as f64;
        let y1 = self.ul.1.min(self.ur.1) as f64;
        let x2 = self.ur.0.max(self.lr.0) as f64;
        let y2 = self.ll.1.max(self.lr.1) as f64;
        [x1, y1, x2, y2]
    }
}

impl From<Quad> for TextQuad {
    fn from(q: Quad) -> Self {
        Self {
            ul: (q.ul.x, q.ul.y),
            ur: (q.ur.x, q.ur.y),
            ll: (q.ll.x, q.ll.y),
            lr: (q.lr.x, q.lr.y),
        }
    }
}

/// Search result with quads (like PyMuPDF's search_for).
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The search text that was found
    pub text: String,
    /// Page number (0-based)
    pub page: u32,
    /// Quads representing the exact position of the found text
    pub quads: Vec<TextQuad>,
}

impl SearchResult {
    /// Convert quads to rectangles for Zotero annotations.
    pub fn to_rects(&self) -> Vec<Vec<f64>> {
        self.quads.iter().map(|q| q.to_rect().to_vec()).collect()
    }
}

/// Search for text on a page and return quads (like PyMuPDF's `page.search_for()`).
///
/// This is the recommended method for creating highlights as it returns
/// the exact positions MuPDF calculates, matching PyMuPDF behavior.
///
/// **Note**: The returned quads are in MuPDF's coordinate system (origin top-left).
/// Use `search_for_rects()` instead for Zotero-compatible coordinates.
///
/// # Arguments
///
/// * `path` - Path to the PDF file
/// * `page_num` - Zero-based page number
/// * `search_text` - Text to search for
/// * `hit_max` - Maximum number of hits to return (default: 16 if 0)
///
/// # Example
///
/// ```no_run
/// use zotero_client::pdf::search_text;
///
/// let results = search_text("/path/to/file.pdf", 0, "important text", 100)?;
/// for result in results {
///     println!("Found '{}' with {} quads", result.text, result.quads.len());
/// }
/// # Ok::<(), zotero_client::error::ZoteroClientError>(())
/// ```
pub fn search_text<P: AsRef<Path>>(
    path: P,
    page_num: usize,
    search_text: &str,
    hit_max: u32,
) -> Result<Vec<SearchResult>> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    let page = doc
        .load_page(page_num as i32)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to load page {}: {}", page_num, e)))?;

    let quads = page.search(search_text, hit_max).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to search page {}: {}", page_num, e))
    })?;

    if quads.is_empty() {
        return Ok(vec![]);
    }

    Ok(vec![SearchResult {
        text: search_text.to_string(),
        page: page_num as u32,
        quads: quads.into_iter().map(TextQuad::from).collect(),
    }])
}

/// Extract text fragments with position information from a PDF page.
///
/// Returns text organized by blocks and lines, with character-level
/// position information available.
///
/// # Arguments
///
/// * `path` - Path to the PDF file
/// * `page_num` - Zero-based page number to extract from
///
/// # Coordinate System
///
/// The returned rectangles are in PDF/Zotero coordinate system (origin bottom-left),
/// in the format [x1, y1, x2, y2] where y1 < y2.
pub fn extract_text_with_positions<P: AsRef<Path>>(
    path: P,
    page_num: usize,
) -> Result<Vec<TextFragment>> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    let page = doc
        .load_page(page_num as i32)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to load page {}: {}", page_num, e)))?;

    // Get page height for coordinate transformation
    let page_bounds = page
        .bounds()
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get page bounds: {}", e)))?;
    let page_height = (page_bounds.y1 - page_bounds.y0) as f64;

    // Use default options for text extraction
    let text_page = page.to_text_page(TextPageOptions::empty()).map_err(|e| {
        ZoteroClientError::Pdf(format!(
            "Failed to extract text from page {}: {}",
            page_num, e
        ))
    })?;

    let mut fragments = Vec::new();

    // Iterate through blocks -> lines -> chars (like PyMuPDF's structure)
    for block in text_page.blocks() {
        for line in block.lines() {
            let line_bounds = line.bounds();

            // Collect characters in this line
            let mut line_text = String::new();
            let mut line_quads: Vec<Quad> = Vec::new();

            for ch in line.chars() {
                if let Some(c) = ch.char() {
                    line_text.push(c);
                    line_quads.push(ch.quad());
                }
            }

            if !line_text.is_empty() {
                // Compute merged bounding box from all character quads (in MuPDF coords)
                let (x1, top_y, x2, bottom_y) = if !line_quads.is_empty() {
                    let mut x1 = f32::MAX;
                    let mut top_y = f32::MAX;
                    let mut x2 = f32::MIN;
                    let mut bottom_y = f32::MIN;

                    for q in &line_quads {
                        x1 = x1.min(q.ul.x).min(q.ll.x);
                        top_y = top_y.min(q.ul.y).min(q.ur.y);
                        x2 = x2.max(q.ur.x).max(q.lr.x);
                        bottom_y = bottom_y.max(q.ll.y).max(q.lr.y);
                    }

                    (x1 as f64, top_y as f64, x2 as f64, bottom_y as f64)
                } else {
                    (
                        line_bounds.x0 as f64,
                        line_bounds.y0 as f64,
                        line_bounds.x1 as f64,
                        line_bounds.y1 as f64,
                    )
                };

                // Transform to PDF/Zotero coords (origin bottom-left)
                let new_y1 = page_height - bottom_y;
                let new_y2 = page_height - top_y;

                fragments.push(TextFragment {
                    text: line_text,
                    page: page_num as u32,
                    rect: [x1, new_y1, x2, new_y2],
                });
            }
        }
    }

    Ok(fragments)
}

/// Find text fragments containing a specific substring.
///
/// This searches through extracted text fragments. For more accurate
/// positioning, use `search_text()` instead which uses MuPDF's native search.
///
/// # Arguments
///
/// * `path` - Path to the PDF file
/// * `page_num` - Zero-based page number to search
/// * `search_text` - Text to search for (case-insensitive)
pub fn find_text_positions<P: AsRef<Path>>(
    path: P,
    page_num: usize,
    needle: &str,
) -> Result<Vec<TextFragment>> {
    let fragments = extract_text_with_positions(path, page_num)?;
    let search_lower = needle.to_lowercase();

    Ok(fragments
        .into_iter()
        .filter(|f| f.text.to_lowercase().contains(&search_lower))
        .collect())
}

/// Search for text using MuPDF's native search and return rectangles.
///
/// This is the **recommended method** for creating Zotero highlights
/// as it matches PyMuPDF's `page.search_for()` behavior exactly.
///
/// The coordinates are transformed from MuPDF's coordinate system (origin top-left)
/// to PDF/Zotero's coordinate system (origin bottom-left).
///
/// # Arguments
///
/// * `path` - Path to the PDF file
/// * `page_num` - Zero-based page number
/// * `needle` - Text to search for
///
/// # Returns
///
/// A vector of rectangles [x1, y1, x2, y2] suitable for Zotero annotations,
/// where y1 < y2 and origin is at bottom-left of the page.
pub fn search_for_rects<P: AsRef<Path>>(
    path: P,
    page_num: usize,
    needle: &str,
) -> Result<Vec<[f64; 4]>> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    let page = doc
        .load_page(page_num as i32)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to load page {}: {}", page_num, e)))?;

    // Get page bounds to get the page height for coordinate transformation
    let page_bounds = page
        .bounds()
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get page bounds: {}", e)))?;
    let page_height = page_bounds.y1 - page_bounds.y0;

    let quads = page.search(needle, 500).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to search page {}: {}", page_num, e))
    })?;

    // Transform coordinates from MuPDF (origin top-left) to PDF/Zotero (origin bottom-left)
    // MuPDF quad: ul=top-left, ur=top-right, ll=bottom-left, lr=bottom-right
    // For each quad, we need to:
    // - Keep x coordinates as-is
    // - Transform y: new_y = page_height - old_y
    let rects: Vec<[f64; 4]> = quads
        .into_iter()
        .map(|q| {
            // Get bounding box from quad (in MuPDF coords, origin top-left)
            let x1 = q.ul.x.min(q.ll.x) as f64;
            let x2 = q.ur.x.max(q.lr.x) as f64;
            // In MuPDF coords: ul.y and ur.y are the top (smaller y), ll.y and lr.y are bottom (larger y)
            let top_y = q.ul.y.min(q.ur.y) as f64;
            let bottom_y = q.ll.y.max(q.lr.y) as f64;

            // Transform to PDF coords (origin bottom-left)
            // new_y1 (bottom of rect in PDF coords) = page_height - bottom_y (which was larger in MuPDF)
            // new_y2 (top of rect in PDF coords) = page_height - top_y (which was smaller in MuPDF)
            let new_y1 = (page_height as f64) - bottom_y;
            let new_y2 = (page_height as f64) - top_y;

            [x1, new_y1, x2, new_y2]
        })
        .collect();

    Ok(rects)
}

/// Get the total number of pages in a PDF.
///
/// # Arguments
///
/// * `path` - Path to the PDF file
pub fn get_page_count<P: AsRef<Path>>(path: P) -> Result<usize> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    doc.page_count()
        .map(|c| c as usize)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get page count: {}", e)))
}

/// Extract all text from a PDF page (without position info).
///
/// This is a simpler method when you just need the text content.
///
/// # Arguments
///
/// * `path` - Path to the PDF file
/// * `page_num` - Zero-based page number
pub fn extract_text<P: AsRef<Path>>(path: P, page_num: usize) -> Result<String> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    let page = doc
        .load_page(page_num as i32)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to load page {}: {}", page_num, e)))?;

    let text_page = page.to_text_page(TextPageOptions::empty()).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to create text page {}: {}", page_num, e))
    })?;

    text_page.to_text().map_err(|e| {
        ZoteroClientError::Pdf(format!(
            "Failed to extract text from page {}: {}",
            page_num, e
        ))
    })
}

/// Convert mupdf Outline to our OutlineItem type.
fn convert_outline(outline: &Outline) -> OutlineItem {
    OutlineItem {
        title: outline.title.clone(),
        page: outline.page,
        children: outline.down.iter().map(convert_outline).collect(),
    }
}

/// Get the PDF outline (table of contents/bookmarks).
///
/// Returns a structured representation of the PDF's outline if one exists.
/// The outline contains section titles and their starting page numbers.
///
/// # Arguments
///
/// * `path` - Path to the PDF file
///
/// # Example
///
/// ```no_run
/// use zotero_client::pdf::get_pdf_outline;
///
/// let outline = get_pdf_outline("/path/to/file.pdf")?;
/// if outline.has_outline {
///     for item in &outline.items {
///         println!("{} starts at page {:?}", item.title, item.page);
///     }
/// }
/// # Ok::<(), zotero_client::error::ZoteroClientError>(())
/// ```
pub fn get_pdf_outline<P: AsRef<Path>>(path: P) -> Result<PdfOutline> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    let total_pages = doc
        .page_count()
        .map(|c| c as usize)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get page count: {}", e)))?;

    let outlines = doc
        .outlines()
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get outlines: {}", e)))?;

    let has_outline = !outlines.is_empty();
    let items: Vec<OutlineItem> = outlines.iter().map(convert_outline).collect();

    Ok(PdfOutline {
        has_outline,
        total_pages,
        items,
    })
}

/// Find an outline item by title (case-insensitive, partial match supported).
///
/// Returns a tuple of (matching item, end page) where end page is determined
/// by the next sibling's start page or the parent's end page.
fn find_outline_item<'a>(
    items: &'a [OutlineItem],
    title: &str,
    parent_end_page: usize,
) -> Option<(&'a OutlineItem, usize)> {
    let title_lower = title.to_lowercase();

    for (i, item) in items.iter().enumerate() {
        let item_title_lower = item.title.to_lowercase();

        // Calculate this item's end page (next sibling or parent's end)
        let item_end_page = if i + 1 < items.len() {
            items[i + 1]
                .page
                .map(|p| p as usize)
                .unwrap_or(parent_end_page)
        } else {
            parent_end_page
        };

        // Check for exact match or partial match
        if item_title_lower == title_lower || item_title_lower.contains(&title_lower) {
            return Some((item, item_end_page));
        }

        // Recursively search children, passing this item's end as their parent's end
        if let Some(result) = find_outline_item(&item.children, title, item_end_page) {
            return Some(result);
        }
    }

    None
}

/// Resolve a section name to a page range using the PDF outline.
///
/// The section name is matched case-insensitively and supports partial matches.
/// Returns (start_page, end_page) where both are 0-based page indices.
///
/// # Arguments
///
/// * `outline` - The PDF outline structure
/// * `section` - The section name to search for
///
/// # Returns
///
/// A tuple of (start_page, end_page) where end_page is exclusive.
/// Returns an error if the section is not found.
///
/// # Example
///
/// ```no_run
/// use zotero_client::pdf::{get_pdf_outline, resolve_section_to_pages};
///
/// let outline = get_pdf_outline("/path/to/file.pdf")?;
/// let (start, end) = resolve_section_to_pages(&outline, "Introduction")?;
/// println!("Introduction spans pages {} to {}", start + 1, end);
/// # Ok::<(), zotero_client::error::ZoteroClientError>(())
/// ```
pub fn resolve_section_to_pages(outline: &PdfOutline, section: &str) -> Result<(usize, usize)> {
    if !outline.has_outline {
        return Err(ZoteroClientError::Pdf(
            "PDF has no outline. Use page numbers instead.".to_string(),
        ));
    }

    match find_outline_item(&outline.items, section, outline.total_pages) {
        Some((item, end_page)) => {
            let start_page = item.page.map(|p| p as usize).unwrap_or(0);
            Ok((start_page, end_page))
        }
        None => {
            // Build a list of available sections for the error message
            let available: Vec<String> = collect_section_names(&outline.items);
            Err(ZoteroClientError::Pdf(format!(
                "Section '{}' not found in outline. Available sections: {}",
                section,
                available.join(", ")
            )))
        }
    }
}

/// Collect all section names from the outline (flattened).
fn collect_section_names(items: &[OutlineItem]) -> Vec<String> {
    let mut names = Vec::new();
    for item in items {
        names.push(item.title.clone());
        names.extend(collect_section_names(&item.children));
    }
    names
}

/// Resolve multiple section names to a combined page range.
///
/// Section names can be comma-separated. Returns all pages covered by any of the sections.
///
/// # Arguments
///
/// * `outline` - The PDF outline structure
/// * `sections` - Comma-separated section names
///
/// # Returns
///
/// A sorted, deduplicated list of 0-based page indices.
pub fn resolve_sections_to_pages(outline: &PdfOutline, sections: &str) -> Result<Vec<usize>> {
    let mut all_pages = Vec::new();

    for section in sections.split(',') {
        let section = section.trim();
        if section.is_empty() {
            continue;
        }

        let (start, end) = resolve_section_to_pages(outline, section)?;
        for page in start..end {
            if !all_pages.contains(&page) {
                all_pages.push(page);
            }
        }
    }

    all_pages.sort_unstable();
    Ok(all_pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_quad_to_rect() {
        let quad = TextQuad {
            ul: (10.0, 20.0),
            ur: (50.0, 20.0),
            ll: (10.0, 35.0),
            lr: (50.0, 35.0),
        };

        let rect = quad.to_rect();
        assert_eq!(rect[0], 10.0); // x1
        assert_eq!(rect[1], 20.0); // y1
        assert_eq!(rect[2], 50.0); // x2
        assert_eq!(rect[3], 35.0); // y2
    }

    #[test]
    fn test_text_fragment_creation() {
        let frag = TextFragment {
            text: "Hello".to_string(),
            page: 0,
            rect: [10.0, 20.0, 50.0, 35.0],
        };

        assert_eq!(frag.text, "Hello");
        assert_eq!(frag.page, 0);
        assert_eq!(frag.rect[0], 10.0); // x1
        assert_eq!(frag.rect[1], 20.0); // y1
        assert_eq!(frag.rect[2], 50.0); // x2
        assert_eq!(frag.rect[3], 35.0); // y2
    }

    #[test]
    fn test_outline_item_creation() {
        let item = OutlineItem {
            title: "Introduction".to_string(),
            page: Some(0),
            children: vec![OutlineItem {
                title: "Background".to_string(),
                page: Some(2),
                children: vec![],
            }],
        };

        assert_eq!(item.title, "Introduction");
        assert_eq!(item.page, Some(0));
        assert_eq!(item.children.len(), 1);
        assert_eq!(item.children[0].title, "Background");
    }

    #[test]
    fn test_resolve_section_exact_match() {
        let outline = PdfOutline {
            has_outline: true,
            total_pages: 20,
            items: vec![
                OutlineItem {
                    title: "Introduction".to_string(),
                    page: Some(0),
                    children: vec![],
                },
                OutlineItem {
                    title: "Methods".to_string(),
                    page: Some(5),
                    children: vec![],
                },
                OutlineItem {
                    title: "Results".to_string(),
                    page: Some(10),
                    children: vec![],
                },
            ],
        };

        let (start, end) = resolve_section_to_pages(&outline, "Methods").unwrap();
        assert_eq!(start, 5);
        assert_eq!(end, 10); // Next section starts at 10
    }

    #[test]
    fn test_resolve_section_case_insensitive() {
        let outline = PdfOutline {
            has_outline: true,
            total_pages: 20,
            items: vec![OutlineItem {
                title: "Introduction".to_string(),
                page: Some(0),
                children: vec![],
            }],
        };

        let (start, end) = resolve_section_to_pages(&outline, "introduction").unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 20); // Last section goes to end
    }

    #[test]
    fn test_resolve_section_partial_match() {
        let outline = PdfOutline {
            has_outline: true,
            total_pages: 20,
            items: vec![OutlineItem {
                title: "1. Introduction and Background".to_string(),
                page: Some(0),
                children: vec![],
            }],
        };

        let (start, end) = resolve_section_to_pages(&outline, "Introduction").unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 20);
    }

    #[test]
    fn test_resolve_section_not_found() {
        let outline = PdfOutline {
            has_outline: true,
            total_pages: 20,
            items: vec![OutlineItem {
                title: "Introduction".to_string(),
                page: Some(0),
                children: vec![],
            }],
        };

        let result = resolve_section_to_pages(&outline, "Nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("not found"));
        assert!(err.contains("Introduction")); // Available sections listed
    }

    #[test]
    fn test_resolve_section_no_outline() {
        let outline = PdfOutline {
            has_outline: false,
            total_pages: 20,
            items: vec![],
        };

        let result = resolve_section_to_pages(&outline, "Introduction");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("no outline"));
    }

    #[test]
    fn test_resolve_sections_multiple() {
        let outline = PdfOutline {
            has_outline: true,
            total_pages: 20,
            items: vec![
                OutlineItem {
                    title: "Introduction".to_string(),
                    page: Some(0),
                    children: vec![],
                },
                OutlineItem {
                    title: "Methods".to_string(),
                    page: Some(5),
                    children: vec![],
                },
                OutlineItem {
                    title: "Results".to_string(),
                    page: Some(10),
                    children: vec![],
                },
            ],
        };

        let pages = resolve_sections_to_pages(&outline, "Introduction,Results").unwrap();
        // Introduction: 0-4, Results: 10-19
        assert!(pages.contains(&0));
        assert!(pages.contains(&4));
        assert!(pages.contains(&10));
        assert!(pages.contains(&19));
        // Methods pages should not be included
        assert!(!pages.contains(&6));
    }

    #[test]
    fn test_resolve_nested_section() {
        let outline = PdfOutline {
            has_outline: true,
            total_pages: 20,
            items: vec![OutlineItem {
                title: "Methods".to_string(),
                page: Some(5),
                children: vec![
                    OutlineItem {
                        title: "Data Collection".to_string(),
                        page: Some(6),
                        children: vec![],
                    },
                    OutlineItem {
                        title: "Analysis".to_string(),
                        page: Some(8),
                        children: vec![],
                    },
                ],
            }],
        };

        // Find nested section - sibling "Analysis" starts at 8, so Data Collection ends at 8
        let (start, end) = resolve_section_to_pages(&outline, "Data Collection").unwrap();
        assert_eq!(start, 6);
        assert_eq!(end, 8); // Next sibling (Analysis) starts at 8
    }

    #[test]
    fn test_collect_section_names() {
        let items = vec![
            OutlineItem {
                title: "Introduction".to_string(),
                page: Some(0),
                children: vec![],
            },
            OutlineItem {
                title: "Methods".to_string(),
                page: Some(5),
                children: vec![OutlineItem {
                    title: "Data Collection".to_string(),
                    page: Some(6),
                    children: vec![],
                }],
            },
        ];

        let names = collect_section_names(&items);
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"Introduction".to_string()));
        assert!(names.contains(&"Methods".to_string()));
        assert!(names.contains(&"Data Collection".to_string()));
    }
}
