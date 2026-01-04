//! # Zotero Client Library
//!
//! A Rust client library for interacting with the Zotero MCP plugin API.
//!
//! This library allows you to:
//! - Search and list items in your Zotero library
//! - Retrieve item details and attachments
//! - Create annotations (highlights, notes) on PDF attachments
//! - Extract text from PDFs with position information (optional `pdf` feature)
//!
//! ## Quick Start
//!
//! ```no_run
//! use zotero_client::{ZoteroClient, types::CreateAnnotationRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client (connects to localhost:23119 by default)
//!     let client = ZoteroClient::new();
//!     
//!     // Check if Zotero is running
//!     let ping = client.ping().await?;
//!     println!("Connected to Zotero {}", ping.zotero_version.unwrap_or_default());
//!     
//!     // Search for items
//!     let items = client.search_items("machine learning", 10).await?;
//!     
//!     // Get PDF attachments for the first item
//!     if let Some(item) = items.first() {
//!         let pdfs = client.get_pdf_attachments(&item.key).await?;
//!         
//!         // Create a highlight annotation
//!         if let Some(pdf) = pdfs.first() {
//!             let request = CreateAnnotationRequest::highlight(
//!                 &pdf.key,
//!                 "Important text",
//!                 0, // page index
//!                 vec![[100.0, 200.0, 300.0, 220.0].to_vec()],
//!             );
//!             
//!             let result = client.create_annotation(request).await?;
//!             println!("Created annotation: {:?}", result.annotation);
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## PDF Text Extraction
//!
//! Enable the `pdf` feature to extract text with position information:
//!
//! ```toml
//! [dependencies]
//! zotero-client = { version = "0.1", features = ["pdf"] }
//! ```
//!
//! ```ignore
//! use zotero_client::pdf::extract_text_with_positions;
//!
//! let fragments = extract_text_with_positions("/path/to/file.pdf", 0)?;
//! for frag in fragments {
//!     println!("'{}' at {:?}", frag.text, frag.rect);
//! }
//! ```
//!
//! ## Features
//!
//! - `pdf` - Enable PDF text extraction with position information (uses `pdf_oxide`)
//! - `integration` - Enable integration tests that require a running Zotero instance

pub mod client;
pub mod error;
pub mod types;

#[cfg(feature = "pdf")]
pub mod pdf;

#[cfg(test)]
mod tests;

pub use client::ZoteroClient;
pub use error::{Result, ZoteroClientError};
