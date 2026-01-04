//! Zotero MCP CLI
//!
//! Command-line interface for the Zotero MCP server.

use anyhow::Result;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use zotero_mcp::{stdio, ServiceExt, ZoteroMcpServer};

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
