//! Tool implementations for the Zotero MCP server.

use rmcp::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};
use zotero_client::{
    pdf::{
        extract_text, get_page_count, get_pdf_outline, resolve_sections_to_pages, search_for_rects,
    },
    types::{CreateAnnotationRequest, CreateAreaAnnotationRequest, HighlightColor, PdfOutline},
    ZoteroClient,
};

/// Semantic color parameter for MCP tools.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum HighlightColorParam {
    /// Blue - Section 1 / Primary organization
    Section1,
    /// Purple - Section 2 / Secondary organization
    Section2,
    /// Magenta - Section 3 / Tertiary organization
    Section3,
    /// Green - Positive point / Agreement
    Positive,
    /// Grey - Point detail / Context
    Detail,
    /// Red - Negative point / Criticism
    Negative,
    /// Orange - Code / Technical content
    Code,
}

impl From<HighlightColorParam> for HighlightColor {
    fn from(param: HighlightColorParam) -> Self {
        match param {
            HighlightColorParam::Section1 => HighlightColor::Section1,
            HighlightColorParam::Section2 => HighlightColor::Section2,
            HighlightColorParam::Section3 => HighlightColor::Section3,
            HighlightColorParam::Positive => HighlightColor::Positive,
            HighlightColorParam::Detail => HighlightColor::Detail,
            HighlightColorParam::Negative => HighlightColor::Negative,
            HighlightColorParam::Code => HighlightColor::Code,
        }
    }
}

/// Error type for tool operations.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Item not found for citekey: {0}")]
    ItemNotFound(String),

    #[error("PDF attachment not found for key: {0}")]
    PdfNotFound(String),

    #[error("PDF file not found at path: {0}")]
    FileNotFound(String),

    #[error("Invalid page range: {0}")]
    InvalidPageRange(String),

    #[error("Page {0} is out of range (PDF has {1} pages)")]
    PageOutOfRange(u32, usize),

    #[error("Text not found on page {0}: {1}")]
    TextNotFound(u32, String),

    #[error("Section not found: {0}")]
    SectionNotFound(String),

    #[error("PDF has no outline. Please use page numbers instead.")]
    NoOutline,

    #[error("Zotero client error: {0}")]
    ClientError(#[from] zotero_client::ZoteroClientError),

    #[error("PDF error: {0}")]
    PdfError(String),
}

/// Look up a Zotero item by its BetterBibTeX citation key.
pub async fn lookup_item(client: &ZoteroClient, citekey: &str) -> Result<String, ToolError> {
    // Search for the item using the citation key
    let item = client
        .find_by_citation_key(citekey, 1000)
        .await?
        .ok_or_else(|| ToolError::ItemNotFound(citekey.to_string()))?;

    // Get PDF attachments
    let pdfs = client.get_pdf_attachments(&item.key).await?;

    // Format the result
    let pdf_info: Vec<String> = pdfs
        .iter()
        .map(|pdf| {
            format!(
                "  - Key: {}, Title: {}, Path: {}",
                pdf.key,
                pdf.title.as_deref().unwrap_or("(no title)"),
                pdf.path.as_deref().unwrap_or("(no path)")
            )
        })
        .collect();

    let result = format!(
        "Found item:\n\
         Key: {}\n\
         Title: {}\n\
         Type: {}\n\
         Date: {}\n\
         \n\
         PDF Attachments ({}):\n\
         {}",
        item.key,
        item.title.as_deref().unwrap_or("(no title)"),
        item.item_type,
        item.date.as_deref().unwrap_or("(no date)"),
        pdfs.len(),
        if pdf_info.is_empty() {
            "  (none)".to_string()
        } else {
            pdf_info.join("\n")
        }
    );

    Ok(result)
}

/// Parse a page range string into a list of page numbers.
fn parse_page_range(pages: &str, total_pages: usize) -> Result<Vec<usize>, ToolError> {
    let pages = pages.trim().to_lowercase();

    if pages == "all" {
        return Ok((0..total_pages).collect());
    }

    let mut result = Vec::new();

    for part in pages.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() != 2 {
                return Err(ToolError::InvalidPageRange(format!(
                    "Invalid range: {}",
                    part
                )));
            }
            let start: usize = bounds[0].parse().map_err(|_| {
                ToolError::InvalidPageRange(format!("Invalid number: {}", bounds[0]))
            })?;
            let end: usize = bounds[1].parse().map_err(|_| {
                ToolError::InvalidPageRange(format!("Invalid number: {}", bounds[1]))
            })?;

            if start == 0 || end == 0 {
                return Err(ToolError::InvalidPageRange(
                    "Page numbers are 1-based".to_string(),
                ));
            }
            if start > end {
                return Err(ToolError::InvalidPageRange(format!(
                    "Start ({}) > end ({})",
                    start, end
                )));
            }

            for p in start..=end {
                if p > total_pages {
                    return Err(ToolError::PageOutOfRange(p as u32, total_pages));
                }
                result.push(p - 1); // Convert to 0-based
            }
        } else {
            let p: usize = part.parse().map_err(|_| {
                ToolError::InvalidPageRange(format!("Invalid page number: {}", part))
            })?;
            if p == 0 {
                return Err(ToolError::InvalidPageRange(
                    "Page numbers are 1-based".to_string(),
                ));
            }
            if p > total_pages {
                return Err(ToolError::PageOutOfRange(p as u32, total_pages));
            }
            result.push(p - 1); // Convert to 0-based
        }
    }

    Ok(result)
}

/// Find the PDF file path for an attachment key.
async fn find_pdf_path(client: &ZoteroClient, attachment_key: &str) -> Result<String, ToolError> {
    let items = client.list_items(500).await?;

    for parent_item in &items {
        let pdfs = client.get_pdf_attachments(&parent_item.key).await?;
        for pdf in pdfs {
            if pdf.key == attachment_key {
                if let Some(path) = pdf.path {
                    if std::path::Path::new(&path).exists() {
                        return Ok(path);
                    } else {
                        return Err(ToolError::FileNotFound(path));
                    }
                }
            }
        }
    }

    Err(ToolError::PdfNotFound(attachment_key.to_string()))
}

/// Get the PDF outline (table of contents/bookmarks).
pub async fn get_outline(
    client: &ZoteroClient,
    attachment_key: &str,
) -> Result<PdfOutline, ToolError> {
    let path = find_pdf_path(client, attachment_key).await?;
    get_pdf_outline(&path).map_err(|e| ToolError::PdfError(e.to_string()))
}

/// Read text from specific pages of a PDF attachment.
/// Supports either page ranges or section names (if PDF has outline).
pub async fn read_pdf_pages(
    client: &ZoteroClient,
    attachment_key: &str,
    pages: Option<&str>,
    section: Option<&str>,
) -> Result<String, ToolError> {
    let path = find_pdf_path(client, attachment_key).await?;

    // Get total page count
    let total_pages = get_page_count(&path).map_err(|e| ToolError::PdfError(e.to_string()))?;

    // Determine which pages to read
    let page_nums = match (pages, section) {
        (Some(p), None) => {
            // Use page range
            parse_page_range(p, total_pages)?
        }
        (None, Some(s)) => {
            // Use section names
            let outline = get_pdf_outline(&path).map_err(|e| ToolError::PdfError(e.to_string()))?;
            if !outline.has_outline {
                return Err(ToolError::NoOutline);
            }
            resolve_sections_to_pages(&outline, s)
                .map_err(|e| ToolError::SectionNotFound(e.to_string()))?
        }
        (Some(p), Some(_s)) => {
            // Both provided - pages takes precedence
            parse_page_range(p, total_pages)?
        }
        (None, None) => {
            // Neither provided - error
            return Err(ToolError::InvalidPageRange(
                "Either 'pages' or 'section' parameter is required".to_string(),
            ));
        }
    };

    // Extract text from each page
    let mut result = String::new();
    for (i, &page_num) in page_nums.iter().enumerate() {
        if i > 0 {
            result.push_str("\n\n");
        }
        result.push_str(&format!("--- Page {} ---\n\n", page_num + 1));

        let text = extract_text(&path, page_num).map_err(|e| ToolError::PdfError(e.to_string()))?;
        result.push_str(&text);
    }

    Ok(result)
}

/// Create a text highlight annotation.
pub async fn create_highlight(
    client: &ZoteroClient,
    attachment_key: &str,
    text: &str,
    page: u32,
    color: HighlightColorParam,
    comment: Option<&str>,
) -> Result<String, ToolError> {
    // Find the PDF file path
    let items = client.list_items(500).await?;
    let mut pdf_path: Option<String> = None;

    for parent_item in &items {
        let pdfs = client.get_pdf_attachments(&parent_item.key).await?;
        for pdf in pdfs {
            if pdf.key == attachment_key {
                pdf_path = pdf.path;
                break;
            }
        }
        if pdf_path.is_some() {
            break;
        }
    }

    let path = pdf_path.ok_or_else(|| ToolError::PdfNotFound(attachment_key.to_string()))?;

    // Verify the file exists
    if !std::path::Path::new(&path).exists() {
        return Err(ToolError::FileNotFound(path));
    }

    // Convert 1-based page to 0-based index
    let page_index = page.saturating_sub(1) as usize;

    // Search for the text to get coordinates
    let rects = search_for_rects(&path, page_index, text)
        .map_err(|e| ToolError::PdfError(e.to_string()))?;

    if rects.is_empty() {
        return Err(ToolError::TextNotFound(page, text.to_string()));
    }

    // Convert to the format expected by Zotero
    let rects_vec: Vec<Vec<f64>> = rects.iter().map(|r| r.to_vec()).collect();

    // Create the annotation request
    let semantic_color: HighlightColor = color.into();
    let mut request =
        CreateAnnotationRequest::highlight(attachment_key, text, page_index as u32, rects_vec)
            .with_semantic_color(semantic_color);

    if let Some(c) = comment {
        request = request.with_comment(c);
    }

    // Create the annotation
    let response = client.create_annotation(request).await?;

    if response.success {
        let ann_key = response
            .annotation
            .and_then(|a| a.key)
            .unwrap_or_else(|| "(unknown)".to_string());
        Ok(format!(
            "Created highlight annotation: {}\n\
             Text: {}\n\
             Page: {}\n\
             Color: {} ({})",
            ann_key,
            text,
            page,
            semantic_color.to_hex(),
            semantic_color.description()
        ))
    } else {
        Err(ToolError::PdfError(
            response
                .error
                .unwrap_or_else(|| "Unknown error".to_string()),
        ))
    }
}

/// Create an area/image annotation.
pub async fn create_area_annotation(
    client: &ZoteroClient,
    attachment_key: &str,
    page: u32,
    rect: [f64; 4],
    color: HighlightColorParam,
    comment: Option<&str>,
) -> Result<String, ToolError> {
    // Convert 1-based page to 0-based index
    let page_index = page.saturating_sub(1);

    // Create the annotation request
    let semantic_color: HighlightColor = color.into();
    let mut request = CreateAreaAnnotationRequest::new(attachment_key, page_index, rect)
        .with_semantic_color(semantic_color);

    if let Some(c) = comment {
        request = request.with_comment(c);
    }

    // Create the annotation
    let response = client.create_area_annotation(request).await?;

    if response.success {
        let ann_key = response
            .annotation
            .and_then(|a| a.key)
            .unwrap_or_else(|| "(unknown)".to_string());
        Ok(format!(
            "Created area annotation: {}\n\
             Page: {}\n\
             Rect: [{:.1}, {:.1}, {:.1}, {:.1}]\n\
             Color: {} ({})\n\
             Comment: {}",
            ann_key,
            page,
            rect[0],
            rect[1],
            rect[2],
            rect[3],
            semantic_color.to_hex(),
            semantic_color.description(),
            comment.unwrap_or("(none)")
        ))
    } else {
        Err(ToolError::PdfError(
            response
                .error
                .unwrap_or_else(|| "Unknown error".to_string()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_page_range_single() {
        assert_eq!(parse_page_range("1", 10).unwrap(), vec![0]);
        assert_eq!(parse_page_range("5", 10).unwrap(), vec![4]);
    }

    #[test]
    fn test_parse_page_range_range() {
        assert_eq!(parse_page_range("1-3", 10).unwrap(), vec![0, 1, 2]);
        assert_eq!(parse_page_range("2-4", 10).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_page_range_comma() {
        assert_eq!(parse_page_range("1,3,5", 10).unwrap(), vec![0, 2, 4]);
    }

    #[test]
    fn test_parse_page_range_all() {
        assert_eq!(parse_page_range("all", 5).unwrap(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_parse_page_range_out_of_bounds() {
        assert!(parse_page_range("15", 10).is_err());
        assert!(parse_page_range("1-15", 10).is_err());
    }

    #[test]
    fn test_parse_page_range_zero_page() {
        assert!(parse_page_range("0", 10).is_err());
    }

    #[test]
    fn test_highlight_color_param_conversion() {
        let param = HighlightColorParam::Positive;
        let color: HighlightColor = param.into();
        assert_eq!(color, HighlightColor::Positive);
    }
}
