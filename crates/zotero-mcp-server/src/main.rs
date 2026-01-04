//! Zotero MCP Server
//!
//! An MCP (Model Context Protocol) server that exposes Zotero operations as tools
//! for AI assistants. Enables critical reading workflows with PDF text extraction
//! and annotation creation.
//!
//! ## Usage
//!
//! ```bash
//! # Run with default Zotero URL (http://localhost:23119/mcp)
//! zotero-mcp-server
//!
//! # Run with custom Zotero URL
//! ZOTERO_URL=http://192.168.1.100:23119/mcp zotero-mcp-server
//! ```
//!
//! ## Available Tools
//!
//! - `zotero_lookup` - Find a Zotero item by BetterBibTeX citation key
//! - `zotero_read_pdf_pages` - Extract text from PDF pages
//! - `zotero_create_highlight` - Create a text highlight annotation
//! - `zotero_create_area_annotation` - Create an area/image annotation

mod server;
mod tools;

use anyhow::Result;
use rmcp::{transport::io::stdio, ServiceExt};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use server::ZoteroMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr (stdout is used for MCP communication)
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Starting Zotero MCP Server");

    // Get Zotero URL from environment or use default
    let zotero_url =
        std::env::var("ZOTERO_URL").unwrap_or_else(|_| "http://localhost:23119/mcp".to_string());
    tracing::info!("Connecting to Zotero at {}", zotero_url);

    // Create the server
    let server = ZoteroMcpServer::new(&zotero_url);

    // Serve over stdio
    let (stdin, stdout) = stdio();
    let service = server.serve((stdin, stdout)).await?;

    tracing::info!("Server initialized, waiting for requests...");

    // Wait for the service to complete
    service.waiting().await?;

    tracing::info!("Server shutting down");
    Ok(())
}
