//! MCP Server implementation for Zotero.

use std::sync::Arc;

use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars,
    schemars::JsonSchema,
    tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler,
};
use serde::{Deserialize, Serialize};
use zotero_client::ZoteroClient;

use crate::tools::{
    create_area_annotation, create_highlight, lookup_item, read_pdf_pages, HighlightColorParam,
};

/// MCP Server for Zotero integration.
#[derive(Clone)]
pub struct ZoteroMcpServer {
    client: Arc<ZoteroClient>,
    tool_router: ToolRouter<Self>,
}

// Tool parameter schemas
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LookupParams {
    /// BetterBibTeX citation key (e.g., "smithMachineLearning2023")
    pub citekey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadPdfPagesParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// Page range (e.g., "1-5", "1,3,5", "all")
    pub pages: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateHighlightParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// Text to highlight (must match PDF content exactly)
    pub text: String,
    /// 1-based page number
    pub page: u32,
    /// Semantic color for the highlight
    pub color: HighlightColorParam,
    /// Optional comment
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateAreaAnnotationParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// 1-based page number
    pub page: u32,
    /// Bounding box [x1, y1, x2, y2] in PDF coordinates
    pub rect: [f64; 4],
    /// Semantic color for the annotation
    pub color: HighlightColorParam,
    /// Optional comment
    #[serde(default)]
    pub comment: Option<String>,
}

#[tool_router]
impl ZoteroMcpServer {
    /// Create a new Zotero MCP server.
    pub fn new(zotero_url: &str) -> Self {
        Self {
            client: Arc::new(ZoteroClient::with_base_url(zotero_url)),
            tool_router: Self::tool_router(),
        }
    }

    /// Find a Zotero item by its BetterBibTeX citation key.
    ///
    /// Returns the item metadata including key, title, and PDF attachment keys.
    #[tool(
        name = "zotero_lookup",
        description = "Find a Zotero item by its BetterBibTeX citation key. Returns item metadata including PDF attachment keys."
    )]
    async fn zotero_lookup(
        &self,
        Parameters(params): Parameters<LookupParams>,
    ) -> Result<CallToolResult, McpError> {
        match lookup_item(&self.client, &params.citekey).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Extract text from specific pages of a PDF attachment.
    ///
    /// Supports page ranges (e.g., "1-5"), comma-separated pages (e.g., "1,3,5"),
    /// or "all" for the entire document.
    #[tool(
        name = "zotero_read_pdf_pages",
        description = "Extract text from specific pages of a PDF attachment. Use page ranges like '1-5', '1,3,5', or 'all'."
    )]
    async fn zotero_read_pdf_pages(
        &self,
        Parameters(params): Parameters<ReadPdfPagesParams>,
    ) -> Result<CallToolResult, McpError> {
        match read_pdf_pages(&self.client, &params.attachment_key, &params.pages).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Create a text highlight annotation on a PDF.
    ///
    /// The text must match the PDF content exactly. Uses semantic colors:
    /// - section1 (blue), section2 (purple), section3 (magenta) for organization
    /// - positive (green), detail (grey), negative (red) for assessment
    /// - code (orange) for technical content
    #[tool(
        name = "zotero_create_highlight",
        description = "Create a text highlight annotation. Text must match PDF content exactly. Colors: section1/2/3, positive, detail, negative, code."
    )]
    async fn zotero_create_highlight(
        &self,
        Parameters(params): Parameters<CreateHighlightParams>,
    ) -> Result<CallToolResult, McpError> {
        match create_highlight(
            &self.client,
            &params.attachment_key,
            &params.text,
            params.page,
            params.color,
            params.comment.as_deref(),
        )
        .await
        {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Create an area/image annotation on a PDF for figures, diagrams, etc.
    ///
    /// Specify the rectangular region using PDF coordinates [x1, y1, x2, y2].
    #[tool(
        name = "zotero_create_area_annotation",
        description = "Create an area annotation for figures/diagrams. Specify rect as [x1, y1, x2, y2] in PDF coordinates."
    )]
    async fn zotero_create_area_annotation(
        &self,
        Parameters(params): Parameters<CreateAreaAnnotationParams>,
    ) -> Result<CallToolResult, McpError> {
        match create_area_annotation(
            &self.client,
            &params.attachment_key,
            params.page,
            params.rect,
            params.color,
            params.comment.as_deref(),
        )
        .await
        {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}

#[tool_handler]
impl ServerHandler for ZoteroMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Zotero MCP Server for AI-assisted critical reading. \
                Use zotero_lookup to find items by citation key, \
                zotero_read_pdf_pages to extract text, and \
                zotero_create_highlight/zotero_create_area_annotation to annotate."
                    .to_string(),
            ),
        }
    }
}
