//! Figure detection using heuristics.
//!
//! This module provides functionality to detect figure regions in PDF pages
//! using whitespace analysis and content density heuristics.

use crate::error::{Result, ZoteroClientError};
use mupdf::{Document, TextPageOptions};
use std::path::Path;

/// Type of detected figure region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FigureType {
    /// Likely an image or photograph
    Image,
    /// Likely a chart or graph
    Chart,
    /// Likely a diagram or schematic
    Diagram,
    /// Unknown figure type
    Unknown,
}

impl FigureType {
    /// Get a human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Chart => "chart",
            Self::Diagram => "diagram",
            Self::Unknown => "figure",
        }
    }
}

/// A detected figure region on a PDF page.
#[derive(Debug, Clone)]
pub struct FigureRegion {
    /// Zero-based index of this figure on the page
    pub index: usize,
    /// Bounding box [x1, y1, x2, y2] in PDF coordinates (origin bottom-left)
    pub rect: [f64; 4],
    /// Estimated figure type based on heuristics
    pub figure_type: FigureType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
}

impl FigureRegion {
    /// Get the width of the figure region.
    pub fn width(&self) -> f64 {
        self.rect[2] - self.rect[0]
    }

    /// Get the height of the figure region.
    pub fn height(&self) -> f64 {
        self.rect[3] - self.rect[1]
    }

    /// Get the aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f64 {
        if self.height() > 0.0 {
            self.width() / self.height()
        } else {
            1.0
        }
    }
}

/// Detect figure regions on a PDF page using heuristics.
///
/// This function analyzes the page content to identify regions that likely
/// contain figures, charts, diagrams, or images. It uses:
///
/// 1. Whitespace analysis to find non-text regions
/// 2. Content density to estimate figure boundaries
/// 3. Aspect ratio and size filtering
///
/// # Arguments
///
/// * `path` - Path to the PDF file
/// * `page_num` - Zero-based page number
///
/// # Returns
///
/// A vector of `FigureRegion` structures, sorted by confidence score.
///
/// # Limitations
///
/// - Works best on standard academic paper layouts
/// - May miss small inline figures
/// - May incorrectly identify dense text blocks as figures
/// - Does not detect figures spanning multiple columns
///
/// # Example
///
/// ```no_run
/// use zotero_client::image::detect_figures;
///
/// let figures = detect_figures("/path/to/file.pdf", 0)?;
/// for fig in figures {
///     println!("Figure {} at {:?}: {}", fig.index, fig.rect, fig.figure_type.description());
/// }
/// # Ok::<(), zotero_client::error::ZoteroClientError>(())
/// ```
pub fn detect_figures<P: AsRef<Path>>(path: P, page_num: usize) -> Result<Vec<FigureRegion>> {
    let path = path.as_ref();

    let doc = Document::open(path).map_err(|e| {
        ZoteroClientError::Pdf(format!("Failed to open PDF '{}': {}", path.display(), e))
    })?;

    let page = doc
        .load_page(page_num as i32)
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to load page {}: {}", page_num, e)))?;

    // Get page dimensions
    let page_bounds = page
        .bounds()
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get page bounds: {}", e)))?;
    let page_width = (page_bounds.x1 - page_bounds.x0) as f64;
    let page_height = (page_bounds.y1 - page_bounds.y0) as f64;

    // Get text content to identify text regions
    let text_page = page
        .to_text_page(TextPageOptions::empty())
        .map_err(|e| ZoteroClientError::Pdf(format!("Failed to get text page: {}", e)))?;

    // Collect text block bounding boxes (in MuPDF coordinates, origin top-left)
    let mut text_regions: Vec<[f64; 4]> = Vec::new();
    for block in text_page.blocks() {
        let bounds = block.bounds();
        text_regions.push([
            bounds.x0 as f64,
            bounds.y0 as f64,
            bounds.x1 as f64,
            bounds.y1 as f64,
        ]);
    }

    // Find gaps between text regions that might contain figures
    let figure_candidates = find_figure_gaps(&text_regions, page_width, page_height);

    // Filter and score candidates
    let mut figures: Vec<FigureRegion> = figure_candidates
        .into_iter()
        .enumerate()
        .filter_map(|(index, rect)| {
            let width = rect[2] - rect[0];
            let height = rect[3] - rect[1];

            // Filter out regions that are too small or too narrow
            let min_dimension = 50.0; // Minimum dimension in points
            let min_area = 5000.0; // Minimum area in square points

            if width < min_dimension || height < min_dimension {
                return None;
            }
            if width * height < min_area {
                return None;
            }

            // Filter out regions that span almost the full page (likely headers/footers)
            if width > page_width * 0.95 && height < page_height * 0.15 {
                return None;
            }

            // Calculate confidence based on region properties
            let confidence = calculate_confidence(width, height, page_width, page_height);

            if confidence < 0.3 {
                return None;
            }

            // Estimate figure type based on aspect ratio
            let aspect_ratio = width / height;
            let figure_type = estimate_figure_type(aspect_ratio, width, height);

            // Convert MuPDF coordinates (origin top-left) to PDF coordinates (origin bottom-left)
            let pdf_y1 = page_height - rect[3];
            let pdf_y2 = page_height - rect[1];

            Some(FigureRegion {
                index,
                rect: [rect[0], pdf_y1, rect[2], pdf_y2],
                figure_type,
                confidence,
            })
        })
        .collect();

    // Sort by confidence (highest first)
    figures.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    // Re-index after sorting
    for (i, fig) in figures.iter_mut().enumerate() {
        fig.index = i;
    }

    Ok(figures)
}

/// Find gaps between text regions that might contain figures.
fn find_figure_gaps(text_regions: &[[f64; 4]], page_width: f64, page_height: f64) -> Vec<[f64; 4]> {
    if text_regions.is_empty() {
        // No text - the entire page might be an image
        return vec![[0.0, 0.0, page_width, page_height]];
    }

    let mut candidates = Vec::new();

    // Sort text regions by vertical position
    let mut sorted_regions = text_regions.to_vec();
    sorted_regions.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());

    // Find vertical gaps between text blocks
    let mut last_bottom = 0.0;
    for region in &sorted_regions {
        let gap_height = region[1] - last_bottom;

        // If there's a significant gap, it might contain a figure
        if gap_height > 50.0 {
            candidates.push([0.0, last_bottom, page_width, region[1]]);
        }

        last_bottom = last_bottom.max(region[3]);
    }

    // Check for gap at the bottom of the page
    let gap_height = page_height - last_bottom;
    if gap_height > 50.0 {
        candidates.push([0.0, last_bottom, page_width, page_height]);
    }

    // Also look for horizontal gaps within vertical bands
    // This helps detect figures in multi-column layouts
    for region in &sorted_regions {
        // Check for wide gaps to the left or right of text blocks
        let left_gap = region[0];
        let right_gap = page_width - region[2];

        // If there's a significant gap and the text is narrow,
        // there might be a figure beside it
        let text_width = region[2] - region[0];
        let text_height = region[3] - region[1];

        if left_gap > page_width * 0.3 && text_width < page_width * 0.5 && text_height > 100.0 {
            candidates.push([0.0, region[1], left_gap - 10.0, region[3]]);
        }

        if right_gap > page_width * 0.3 && text_width < page_width * 0.5 && text_height > 100.0 {
            candidates.push([region[2] + 10.0, region[1], page_width, region[3]]);
        }
    }

    // Merge overlapping candidates
    merge_overlapping_regions(candidates)
}

/// Merge overlapping rectangular regions.
fn merge_overlapping_regions(regions: Vec<[f64; 4]>) -> Vec<[f64; 4]> {
    if regions.is_empty() {
        return vec![];
    }

    let mut merged = vec![regions[0]];

    for region in regions.into_iter().skip(1) {
        let mut was_merged = false;

        for existing in &mut merged {
            if regions_overlap(existing, &region) {
                // Merge by taking the bounding box
                existing[0] = existing[0].min(region[0]);
                existing[1] = existing[1].min(region[1]);
                existing[2] = existing[2].max(region[2]);
                existing[3] = existing[3].max(region[3]);
                was_merged = true;
                break;
            }
        }

        if !was_merged {
            merged.push(region);
        }
    }

    merged
}

/// Check if two rectangles overlap.
fn regions_overlap(a: &[f64; 4], b: &[f64; 4]) -> bool {
    !(a[2] < b[0] || b[2] < a[0] || a[3] < b[1] || b[3] < a[1])
}

/// Calculate confidence score for a potential figure region.
fn calculate_confidence(width: f64, height: f64, page_width: f64, page_height: f64) -> f32 {
    let mut confidence: f32 = 0.5;

    // Prefer regions that are a reasonable size relative to the page
    let area_ratio = (width * height) / (page_width * page_height);
    if (0.05..=0.5).contains(&area_ratio) {
        confidence += 0.2;
    }

    // Prefer regions with reasonable aspect ratios
    let aspect_ratio = width / height;
    if (0.5..=2.0).contains(&aspect_ratio) {
        confidence += 0.2;
    }

    // Prefer regions that are somewhat centered horizontally
    let center_x = width / 2.0;
    let page_center = page_width / 2.0;
    let center_offset = (center_x - page_center).abs() / page_center;
    if center_offset < 0.3 {
        confidence += 0.1;
    }

    confidence.min(1.0)
}

/// Estimate the figure type based on region properties.
fn estimate_figure_type(aspect_ratio: f64, width: f64, height: f64) -> FigureType {
    // Square-ish regions are often charts or diagrams
    if (0.8..=1.2).contains(&aspect_ratio) {
        return FigureType::Chart;
    }

    // Wide regions are often charts or diagrams
    if aspect_ratio > 1.5 {
        return FigureType::Chart;
    }

    // Tall regions might be diagrams
    if aspect_ratio < 0.7 {
        return FigureType::Diagram;
    }

    // Large regions are more likely to be images
    if width > 300.0 && height > 300.0 {
        return FigureType::Image;
    }

    FigureType::Unknown
}
