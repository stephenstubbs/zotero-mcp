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

use crate::image_tools::{extract_page_image, get_figure, list_figures, ImageFormatParam};
use crate::tools::{
    create_area_annotation, create_highlight, get_outline, lookup_item, read_pdf_pages,
    HighlightColorParam,
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
pub struct GetPdfOutlineParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadPdfPagesParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// Page range (e.g., "1-5", "1,3,5", "all"). Either pages or section is required.
    #[serde(default)]
    pub pages: Option<String>,
    /// Section name(s) from PDF outline (e.g., "Introduction", "Introduction,Methods").
    /// Requires PDF to have an outline/bookmarks. Either pages or section is required.
    #[serde(default)]
    pub section: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExtractPageImageParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// 1-based page number
    pub page: u32,
    /// Optional region [x1, y1, x2, y2] in PDF coordinates. If omitted, renders full page.
    #[serde(default)]
    pub rect: Option<[f64; 4]>,
    /// Resolution in DPI (default: 150). Higher DPI = larger file size and better quality.
    #[serde(default)]
    pub dpi: Option<u32>,
    /// Output format: "png" (default) or "jpeg". JPEG is ~80% smaller.
    #[serde(default)]
    pub format: Option<ImageFormatParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListFiguresParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// 1-based page number
    pub page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFigureParams {
    /// Zotero attachment key for the PDF
    pub attachment_key: String,
    /// 1-based page number
    pub page: u32,
    /// Figure index from zotero_list_figures
    pub figure_index: usize,
    /// Output format: "png" (default) or "jpeg"
    #[serde(default)]
    pub format: Option<ImageFormatParam>,
    /// Include padding around the figure for context
    #[serde(default)]
    pub include_context: Option<bool>,
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

    /// Get the PDF outline (table of contents/bookmarks).
    ///
    /// Returns the document's outline structure with section titles and page numbers.
    /// Use this to discover available sections before reading by section name.
    #[tool(
        name = "zotero_get_pdf_outline",
        description = "Get PDF outline (table of contents/bookmarks). Returns section titles and page numbers. Use to discover sections before reading."
    )]
    async fn zotero_get_pdf_outline(
        &self,
        Parameters(params): Parameters<GetPdfOutlineParams>,
    ) -> Result<CallToolResult, McpError> {
        match get_outline(&self.client, &params.attachment_key).await {
            Ok(outline) => {
                // Format the outline for display
                let json = serde_json::to_string_pretty(&outline)
                    .unwrap_or_else(|_| "Failed to serialize outline".to_string());
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Extract text from specific pages of a PDF attachment.
    ///
    /// Supports page ranges (e.g., "1-5"), comma-separated pages (e.g., "1,3,5"),
    /// "all" for the entire document, or section names from the PDF outline.
    #[tool(
        name = "zotero_read_pdf_pages",
        description = "Extract text from specific pages of a PDF attachment. Use page ranges like '1-5', '1,3,5', or 'all'. Alternatively, use 'section' parameter with section names from the PDF outline."
    )]
    async fn zotero_read_pdf_pages(
        &self,
        Parameters(params): Parameters<ReadPdfPagesParams>,
    ) -> Result<CallToolResult, McpError> {
        match read_pdf_pages(
            &self.client,
            &params.attachment_key,
            params.pages.as_deref(),
            params.section.as_deref(),
        )
        .await
        {
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

    /// Render a PDF page or region as an image for vision AI analysis.
    ///
    /// Returns a base64-encoded image (PNG or JPEG) that can be analyzed by vision models.
    #[tool(
        name = "zotero_extract_page_image",
        description = "Render a PDF page or region as an image file for vision analysis. Returns file path to saved image. Use rect parameter to render specific regions only."
    )]
    async fn zotero_extract_page_image(
        &self,
        Parameters(params): Parameters<ExtractPageImageParams>,
    ) -> Result<CallToolResult, McpError> {
        match extract_page_image(
            &self.client,
            &params.attachment_key,
            params.page,
            params.rect,
            params.dpi,
            params.format.unwrap_or_default(),
        )
        .await
        {
            Ok(result) => {
                let json = serde_json::to_string_pretty(&result)
                    .unwrap_or_else(|_| "Failed to serialize result".to_string());
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Detect and list figure regions on a PDF page.
    ///
    /// Uses heuristics to find figures, charts, diagrams, and images.
    /// Returns bounding boxes and estimated types for each figure.
    #[tool(
        name = "zotero_list_figures",
        description = "Detect and list figure regions on a PDF page. Returns bounding boxes and estimated types (image/chart/diagram)."
    )]
    async fn zotero_list_figures(
        &self,
        Parameters(params): Parameters<ListFiguresParams>,
    ) -> Result<CallToolResult, McpError> {
        match list_figures(&self.client, &params.attachment_key, params.page).await {
            Ok(figures) => {
                if figures.is_empty() {
                    Ok(CallToolResult::success(vec![Content::text(
                        "No figures detected on this page.",
                    )]))
                } else {
                    let json = serde_json::to_string_pretty(&figures)
                        .unwrap_or_else(|_| "Failed to serialize figures".to_string());
                    Ok(CallToolResult::success(vec![Content::text(json)]))
                }
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Extract a specific figure as an image file.
    ///
    /// First use `zotero_list_figures` to find figures on a page,
    /// then use this tool to extract a figure by its index. Returns file path.
    #[tool(
        name = "zotero_get_figure",
        description = "Extract a detected figure as an image file. Use after zotero_list_figures to get figure indices. Returns file path."
    )]
    async fn zotero_get_figure(
        &self,
        Parameters(params): Parameters<GetFigureParams>,
    ) -> Result<CallToolResult, McpError> {
        match get_figure(
            &self.client,
            &params.attachment_key,
            params.page,
            params.figure_index,
            params.format.unwrap_or_default(),
            params.include_context.unwrap_or(false),
        )
        .await
        {
            Ok(result) => {
                let json = serde_json::to_string_pretty(&result)
                    .unwrap_or_else(|_| "Failed to serialize result".to_string());
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
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
                "Zotero MCP Server for AI-assisted critical reading with vision support. \
                Use zotero_lookup to find items by citation key, \
                zotero_get_pdf_outline to discover document sections, \
                zotero_read_pdf_pages to extract text (by page or section), \
                zotero_create_highlight/zotero_create_area_annotation to annotate, \
                zotero_extract_page_image to render pages for vision analysis, and \
                zotero_list_figures/zotero_get_figure to detect and extract figures."
                    .to_string(),
            ),
        }
    }
}
