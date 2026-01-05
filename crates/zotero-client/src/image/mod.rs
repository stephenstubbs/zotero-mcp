//! PDF image extraction for vision AI workflows.
//!
//! This module provides functionality to render PDF pages as images that can be
//! analyzed by vision AI models (Claude, GPT-4V, etc.). It supports:
//!
//! - Rendering full pages at configurable DPI
//! - Rendering specific rectangular regions
//! - Extracting embedded images from PDFs
//! - Detecting figure regions using heuristics
//!
//! Images are returned as base64-encoded strings suitable for MCP tool responses.
//!
//! Requires the `image` feature to be enabled.
//!
//! # Example
//!
//! ```no_run
//! use zotero_client::image::{render_page, ImageFormat};
//!
//! // Render page 1 at 150 DPI as PNG
//! let result = render_page("/path/to/file.pdf", 0, 150, ImageFormat::Png)?;
//! println!("Base64 image: {} bytes", result.data.len());
//! println!("MIME type: {}", result.mime_type);
//! # Ok::<(), zotero_client::error::ZoteroClientError>(())
//! ```

mod detection;
mod render;

pub use detection::{detect_figures, FigureRegion, FigureType};
pub use render::{
    extract_embedded_images, render_page, render_page_to_file, render_region,
    render_region_to_file, EmbeddedImage, ImageFormat, ImageOutput,
};

#[cfg(test)]
mod tests;
