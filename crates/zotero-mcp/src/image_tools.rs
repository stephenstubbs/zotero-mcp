//! Image extraction tool implementations for the Zotero MCP server.

use rmcp::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};
use zotero_client::{
    image::{
        detect_figures, render_page_to_file, render_region_to_file, FigureRegion, ImageFormat,
    },
    ZoteroClient,
};

use super::tools::ToolError;

/// Output format for rendered images.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormatParam {
    /// PNG format (lossless, larger size) - default
    #[default]
    Png,
    /// JPEG format (lossy, smaller size)
    Jpeg,
}

impl From<ImageFormatParam> for ImageFormat {
    fn from(param: ImageFormatParam) -> Self {
        match param {
            ImageFormatParam::Png => ImageFormat::Png,
            ImageFormatParam::Jpeg => ImageFormat::Jpeg,
        }
    }
}

/// Result of rendering a PDF page as an image (saved to file).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResult {
    /// Absolute path to the saved image file
    pub file_path: String,
    /// MIME type (e.g., "image/png")
    pub mime_type: String,
}

/// A detected figure on a PDF page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureInfo {
    /// Zero-based index of this figure on the page
    pub index: usize,
    /// Bounding box [x1, y1, x2, y2] in PDF coordinates
    pub rect: [f64; 4],
    /// Estimated figure type
    pub figure_type: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Width of the figure region
    pub width: f64,
    /// Height of the figure region
    pub height: f64,
}

impl From<FigureRegion> for FigureInfo {
    fn from(region: FigureRegion) -> Self {
        Self {
            index: region.index,
            rect: region.rect,
            figure_type: region.figure_type.description().to_string(),
            confidence: region.confidence,
            width: region.width(),
            height: region.height(),
        }
    }
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

/// Extract a PDF page as an image, saving it to a temporary file.
///
/// Returns the file path instead of base64 data, allowing the MCP client
/// to access the image directly without embedding massive data in responses.
pub async fn extract_page_image(
    client: &ZoteroClient,
    attachment_key: &str,
    page: u32,
    rect: Option<[f64; 4]>,
    dpi: Option<u32>,
    format: ImageFormatParam,
) -> Result<ImageResult, ToolError> {
    let pdf_path = find_pdf_path(client, attachment_key).await?;
    let dpi = dpi.unwrap_or(150); // Can use higher DPI now since we're not base64 encoding
    let page_index = page.saturating_sub(1) as usize;
    let img_format: ImageFormat = format.into();

    // Generate a unique temp file path
    let extension = match img_format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpg",
    };
    let temp_file = std::env::temp_dir().join(format!(
        "zotero-page-{}-{}-{}.{}",
        attachment_key,
        page,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        extension
    ));

    let file_path = match rect {
        Some(r) => render_region_to_file(&pdf_path, page_index, r, dpi, img_format, &temp_file)
            .map_err(|e| ToolError::PdfError(e.to_string()))?,
        None => render_page_to_file(&pdf_path, page_index, dpi, img_format, &temp_file)
            .map_err(|e| ToolError::PdfError(e.to_string()))?,
    };

    Ok(ImageResult {
        file_path,
        mime_type: img_format.mime_type().to_string(),
    })
}

/// List detected figures on a PDF page.
///
/// Uses heuristics to detect figure regions (images, charts, diagrams)
/// on the specified page.
pub async fn list_figures(
    client: &ZoteroClient,
    attachment_key: &str,
    page: u32,
) -> Result<Vec<FigureInfo>, ToolError> {
    let path = find_pdf_path(client, attachment_key).await?;
    let page_index = page.saturating_sub(1) as usize;

    let figures = detect_figures(&path, page_index)
        .map_err(|e| ToolError::PdfError(e.to_string()))?;

    Ok(figures.into_iter().map(FigureInfo::from).collect())
}

/// Extract a specific figure as an image, saving it to a temporary file.
///
/// First detects figures on the page, then renders the specified figure
/// by index and saves it to a file.
pub async fn get_figure(
    client: &ZoteroClient,
    attachment_key: &str,
    page: u32,
    figure_index: usize,
    format: ImageFormatParam,
    include_context: bool,
) -> Result<ImageResult, ToolError> {
    let pdf_path = find_pdf_path(client, attachment_key).await?;
    let page_index = page.saturating_sub(1) as usize;

    // Detect figures first
    let figures = detect_figures(&pdf_path, page_index)
        .map_err(|e| ToolError::PdfError(e.to_string()))?;

    // Find the requested figure
    let figure = figures
        .iter()
        .find(|f| f.index == figure_index)
        .ok_or_else(|| {
            ToolError::PdfError(format!(
                "Figure {} not found. Page has {} detected figures.",
                figure_index,
                figures.len()
            ))
        })?;

    // Add padding for context if requested
    let rect = if include_context {
        let padding = 10.0;
        [
            (figure.rect[0] - padding).max(0.0),
            (figure.rect[1] - padding).max(0.0),
            figure.rect[2] + padding,
            figure.rect[3] + padding,
        ]
    } else {
        figure.rect
    };

    let img_format: ImageFormat = format.into();
    let extension = match img_format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpg",
    };

    // Generate temp file path
    let temp_file = std::env::temp_dir().join(format!(
        "zotero-figure-{}-p{}-f{}-{}.{}",
        attachment_key,
        page,
        figure_index,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        extension
    ));

    // Render the figure region at 150 DPI (high quality for figures)
    let file_path = render_region_to_file(&pdf_path, page_index, rect, 150, img_format, &temp_file)
        .map_err(|e| ToolError::PdfError(e.to_string()))?;

    Ok(ImageResult {
        file_path,
        mime_type: img_format.mime_type().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_format_param_conversion() {
        let png: ImageFormat = ImageFormatParam::Png.into();
        assert_eq!(png, ImageFormat::Png);

        let jpeg: ImageFormat = ImageFormatParam::Jpeg.into();
        assert_eq!(jpeg, ImageFormat::Jpeg);
    }

    #[test]
    fn test_image_format_param_default() {
        assert!(matches!(ImageFormatParam::default(), ImageFormatParam::Png));
    }
}
