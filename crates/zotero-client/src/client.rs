//! HTTP client for the Zotero MCP plugin API.

use reqwest::Client;

use crate::error::{Result, ZoteroClientError};
use crate::types::{
    ChildrenResponse, CreateAnnotationRequest, CreateAnnotationResponse,
    CreateAreaAnnotationRequest, ItemsResponse, PingResponse, SearchResponse, ZoteroAttachment,
    ZoteroItem,
};

/// Default base URL for the Zotero MCP plugin.
pub const DEFAULT_BASE_URL: &str = "http://localhost:23119/mcp";

/// Client for interacting with the Zotero MCP plugin API.
///
/// # Example
///
/// ```no_run
/// use zotero_client::ZoteroClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = ZoteroClient::new();
///     
///     // Check if Zotero is running
///     let ping = client.ping().await?;
///     println!("Connected to Zotero {}", ping.zotero_version.unwrap_or_default());
///     
///     // Search for items
///     let items = client.search_items("machine learning", 10).await?;
///     for item in items {
///         println!("{}: {}", item.key, item.title.unwrap_or_default());
///     }
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ZoteroClient {
    client: Client,
    base_url: String,
}

impl Default for ZoteroClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoteroClient {
    /// Create a new client with the default base URL (localhost:23119).
    pub fn new() -> Self {
        Self::with_base_url(DEFAULT_BASE_URL)
    }

    /// Create a new client with a custom base URL.
    ///
    /// # Example
    ///
    /// ```
    /// use zotero_client::ZoteroClient;
    ///
    /// let client = ZoteroClient::with_base_url("http://192.168.1.100:23119/mcp");
    /// ```
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Check if the Zotero MCP plugin is active.
    ///
    /// # Errors
    ///
    /// Returns an error if Zotero is not running or the plugin is not installed.
    pub async fn ping(&self) -> Result<PingResponse> {
        let url = format!("{}/ping", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.json().await?)
    }

    /// Search for items matching a query.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string (matches title, authors, etc.)
    /// * `limit` - Maximum number of results to return
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use zotero_client::ZoteroClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ZoteroClient::new();
    /// let items = client.search_items("neural networks", 25).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_items(&self, query: &str, limit: u32) -> Result<Vec<ZoteroItem>> {
        let url = format!("{}/search", self.base_url);
        let body = serde_json::json!({
            "query": query,
            "limit": limit
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let search_response: SearchResponse = response.json().await?;
        Ok(search_response.results)
    }

    /// List items from the library.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return
    pub async fn list_items(&self, limit: u32) -> Result<Vec<ZoteroItem>> {
        let url = format!("{}/items", self.base_url);
        let body = serde_json::json!({
            "limit": limit
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let items_response: ItemsResponse = response.json().await?;
        Ok(items_response.items)
    }

    /// Get a specific item by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The unique item key
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if the item doesn't exist.
    pub async fn get_item(&self, key: &str) -> Result<ZoteroItem> {
        let url = format!("{}/item", self.base_url);
        let body = serde_json::json!({
            "key": key
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if response.status().as_u16() == 404 {
            return Err(ZoteroClientError::NotFound {
                key: key.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.json().await?)
    }

    /// Get child items (attachments, notes, annotations) for an item.
    ///
    /// # Arguments
    ///
    /// * `key` - The parent item key
    ///
    /// # Returns
    ///
    /// A JSON value containing the children. Use `parse_children` to extract
    /// typed attachments and annotations.
    pub async fn get_children(&self, key: &str) -> Result<ChildrenResponse> {
        let url = format!("{}/children", self.base_url);
        let body = serde_json::json!({
            "key": key
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if response.status().as_u16() == 404 {
            return Err(ZoteroClientError::NotFound {
                key: key.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.json().await?)
    }

    /// Get PDF attachments for an item.
    ///
    /// This is a convenience method that filters children to only PDF attachments.
    pub async fn get_pdf_attachments(&self, key: &str) -> Result<Vec<ZoteroAttachment>> {
        let children = self.get_children(key).await?;
        let mut pdfs = Vec::new();

        for child in children.children {
            if let Ok(attachment) = serde_json::from_value::<ZoteroAttachment>(child) {
                if attachment.content_type.as_deref() == Some("application/pdf") {
                    pdfs.push(attachment);
                }
            }
        }

        Ok(pdfs)
    }

    /// Create an annotation on a PDF attachment.
    ///
    /// # Arguments
    ///
    /// * `request` - The annotation creation request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use zotero_client::{ZoteroClient, types::CreateAnnotationRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ZoteroClient::new();
    ///
    /// let request = CreateAnnotationRequest::highlight(
    ///     "PDF_KEY",
    ///     "Important text to highlight",
    ///     0, // page index
    ///     vec![[100.0, 200.0, 300.0, 220.0].to_vec()],
    /// )
    /// .with_comment("This is important!")
    /// .with_color("#ff6666");
    ///
    /// let result = client.create_annotation(request).await?;
    /// if result.success {
    ///     println!("Created annotation: {:?}", result.annotation);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_annotation(
        &self,
        request: CreateAnnotationRequest,
    ) -> Result<CreateAnnotationResponse> {
        let url = format!("{}/annotations", self.base_url);

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().as_u16() == 404 {
            return Err(ZoteroClientError::NotFound {
                key: request.parent_item_key,
            });
        }

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.json().await?)
    }

    /// Create an area/image annotation on a PDF attachment.
    ///
    /// Area annotations are used for selecting regions like figures, diagrams,
    /// or images. They use `annotationType: "image"` and don't require text content.
    ///
    /// # Arguments
    ///
    /// * `request` - The area annotation creation request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use zotero_client::{ZoteroClient, types::{CreateAreaAnnotationRequest, HighlightColor}};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ZoteroClient::new();
    ///
    /// let request = CreateAreaAnnotationRequest::new(
    ///     "PDF_KEY",
    ///     0, // page index
    ///     [100.0, 200.0, 300.0, 400.0], // rect [x1, y1, x2, y2]
    /// )
    /// .with_comment("Figure 1: System architecture")
    /// .with_semantic_color(HighlightColor::Section1);
    ///
    /// let result = client.create_area_annotation(request).await?;
    /// if result.success {
    ///     println!("Created area annotation: {:?}", result.annotation);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_area_annotation(
        &self,
        request: CreateAreaAnnotationRequest,
    ) -> Result<CreateAnnotationResponse> {
        let url = format!("{}/annotations", self.base_url);

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().as_u16() == 404 {
            return Err(ZoteroClientError::NotFound {
                key: request.parent_item_key,
            });
        }

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.json().await?)
    }

    /// Find an item by its BetterBibTeX citation key.
    ///
    /// Uses the dedicated /mcp/citekey endpoint which queries BetterBibTeX's
    /// internal database directly for accurate citation key lookup.
    ///
    /// # Arguments
    ///
    /// * `citation_key` - The citation key to search for
    /// * `_search_limit` - Deprecated, kept for API compatibility
    pub async fn find_by_citation_key(
        &self,
        citation_key: &str,
        _search_limit: u32,
    ) -> Result<Option<ZoteroItem>> {
        let url = format!("{}/citekey", self.base_url);
        let body = serde_json::json!({
            "citekey": citation_key
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(ZoteroClientError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let item: ZoteroItem = response.json().await?;
        Ok(Some(item))
    }
}
