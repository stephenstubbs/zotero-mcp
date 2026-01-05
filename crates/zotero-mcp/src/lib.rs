//! Zotero MCP Server Library
//!
//! An MCP (Model Context Protocol) server library that exposes Zotero operations as tools
//! for AI assistants. Enables critical reading workflows with PDF text extraction,
//! annotation creation, and image extraction for vision AI analysis.
//!
//! ## Available Tools
//!
//! - `zotero_lookup` - Find a Zotero item by BetterBibTeX citation key
//! - `zotero_get_pdf_outline` - Get PDF table of contents/bookmarks
//! - `zotero_read_pdf_pages` - Extract text from PDF pages
//! - `zotero_create_highlight` - Create a text highlight annotation
//! - `zotero_create_area_annotation` - Create an area/image annotation
//! - `zotero_extract_page_image` - Render a PDF page or region as an image
//! - `zotero_list_figures` - Detect figure regions on a PDF page
//! - `zotero_get_figure` - Extract a specific figure as an image
//!
//! ## Example
//!
//! ```rust,no_run
//! use zotero_mcp::ZoteroMcpServer;
//! use rmcp::{transport::io::stdio, ServiceExt};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let server = ZoteroMcpServer::new("http://localhost:23119/mcp");
//!     let (stdin, stdout) = stdio();
//!     let service = server.serve((stdin, stdout)).await?;
//!     service.waiting().await?;
//!     Ok(())
//! }
//! ```

mod image_tools;
mod server;
mod tools;

pub use image_tools::ImageFormatParam;
pub use server::ZoteroMcpServer;
pub use tools::HighlightColorParam;

// Re-export rmcp types for convenience
pub use rmcp::{transport::io::stdio, ServiceExt};
