//! Data types for the Zotero client library.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Semantic highlight colors for annotations.
///
/// These colors follow a predefined scheme for consistent meaning:
/// - Section colors (Blue, Purple, Magenta) for organizational structure
/// - Assessment colors (Green = positive, Red = negative, Grey = detail)
/// - Special colors (Orange = code)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HighlightColor {
    /// Blue (#2ea8e5) - Section 1 / Primary organization
    Section1,
    /// Purple (#a28ae5) - Section 2 / Secondary organization
    Section2,
    /// Magenta (#e56eee) - Section 3 / Tertiary organization
    Section3,
    /// Green (#5fb236) - Positive point / Agreement / Support
    Positive,
    /// Grey (#aaaaaa) - Point detail / Neutral / Context
    Detail,
    /// Red (#ff6666) - Negative point / Disagreement / Criticism
    Negative,
    /// Orange (#f19837) - Code / Technical content
    Code,
}

impl HighlightColor {
    /// Get the hex color code for this semantic color.
    #[must_use]
    pub fn to_hex(&self) -> &'static str {
        match self {
            Self::Section1 => "#2ea8e5",
            Self::Section2 => "#a28ae5",
            Self::Section3 => "#e56eee",
            Self::Positive => "#5fb236",
            Self::Detail => "#aaaaaa",
            Self::Negative => "#ff6666",
            Self::Code => "#f19837",
        }
    }

    /// Get a human-readable description of this color's semantic meaning.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Section1 => "Section 1 / Primary organization",
            Self::Section2 => "Section 2 / Secondary organization",
            Self::Section3 => "Section 3 / Tertiary organization",
            Self::Positive => "Positive point / Agreement",
            Self::Detail => "Point detail / Context",
            Self::Negative => "Negative point / Criticism",
            Self::Code => "Code / Technical content",
        }
    }
}

impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<HighlightColor> for String {
    fn from(color: HighlightColor) -> Self {
        color.to_hex().to_string()
    }
}

/// A creator (author, editor, etc.) of a Zotero item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    /// Creator type (e.g., "author", "editor").
    pub creator_type: Option<String>,
    /// First name of the creator.
    pub first_name: Option<String>,
    /// Last name of the creator.
    pub last_name: Option<String>,
    /// Full name (used when first/last are not available).
    pub name: Option<String>,
}

/// A Zotero library item (book, article, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ZoteroItem {
    /// Internal database ID.
    pub id: Option<i64>,
    /// Unique item key.
    pub key: String,
    /// Type of item (e.g., "book", "journalArticle").
    pub item_type: String,
    /// Title of the item.
    pub title: Option<String>,
    /// List of creators (authors, editors, etc.).
    #[serde(default)]
    pub creators: Vec<Creator>,
    /// Publication date.
    pub date: Option<String>,
    /// Extra field (often contains citation key).
    pub extra: Option<String>,
    /// Abstract or summary.
    #[serde(rename = "abstract")]
    pub abstract_note: Option<String>,
    /// URL of the item.
    pub url: Option<String>,
    /// DOI of the item.
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    /// Tags associated with the item.
    #[serde(default)]
    pub tags: Vec<Tag>,
}

/// A tag attached to an item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// Tag text.
    pub tag: String,
    /// Tag type (0 = user, 1 = automatic).
    #[serde(rename = "type")]
    pub tag_type: Option<i32>,
}

/// A file attachment in Zotero.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ZoteroAttachment {
    /// Internal database ID.
    pub id: Option<i64>,
    /// Unique item key.
    pub key: String,
    /// Title of the attachment.
    pub title: Option<String>,
    /// MIME content type (e.g., "application/pdf").
    pub content_type: Option<String>,
    /// Local file path.
    pub path: Option<String>,
    /// Item type (should be "attachment").
    pub item_type: String,
}

/// An annotation on a PDF attachment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ZoteroAnnotation {
    /// Internal database ID.
    pub id: Option<i64>,
    /// Unique item key.
    pub key: Option<String>,
    /// Parent item key (the PDF attachment).
    pub parent_item_key: Option<String>,
    /// Type of annotation (e.g., "highlight", "note").
    pub annotation_type: Option<String>,
    /// Highlighted or selected text.
    pub text: Option<String>,
    /// User comment on the annotation.
    pub comment: Option<String>,
    /// Highlight color (hex code, e.g., "#ffd400").
    pub color: Option<String>,
    /// Page label (human-readable page number).
    pub page_label: Option<String>,
    /// Sort index for ordering annotations.
    pub sort_index: Option<String>,
    /// Position information for the annotation.
    pub position: Option<AnnotationPosition>,
}

/// Position information for a PDF annotation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationPosition {
    /// Zero-based page index.
    pub page_index: u32,
    /// List of rectangles defining the highlight area.
    /// Each rect is [x1, y1, x2, y2] in PDF coordinates.
    #[serde(default)]
    pub rects: Vec<Vec<f64>>,
}

/// Request to create a new annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnnotationRequest {
    /// Key of the parent PDF attachment.
    pub parent_item_key: String,
    /// Type of annotation (default: "highlight").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_type: Option<String>,
    /// Text content of the annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// User comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Highlight color (hex code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Page label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_label: Option<String>,
    /// Sort index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_index: Option<String>,
    /// Position information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<AnnotationPosition>,
}

impl CreateAnnotationRequest {
    /// Create a new highlight annotation request.
    ///
    /// # Example
    ///
    /// ```
    /// use zotero_client::types::{CreateAnnotationRequest, AnnotationPosition};
    ///
    /// let request = CreateAnnotationRequest::highlight(
    ///     "ABCD1234",
    ///     "Important text",
    ///     0,
    ///     vec![[100.0, 200.0, 300.0, 220.0].to_vec()],
    /// );
    /// ```
    pub fn highlight(
        parent_item_key: impl Into<String>,
        text: impl Into<String>,
        page_index: u32,
        rects: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            parent_item_key: parent_item_key.into(),
            annotation_type: Some("highlight".to_string()),
            text: Some(text.into()),
            comment: None,
            color: Some("#ffd400".to_string()), // Default yellow
            page_label: Some((page_index + 1).to_string()),
            sort_index: None,
            position: Some(AnnotationPosition { page_index, rects }),
        }
    }

    /// Set the comment on this annotation.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Set the color of this annotation.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the color using a semantic `HighlightColor`.
    pub fn with_semantic_color(mut self, color: HighlightColor) -> Self {
        self.color = Some(color.to_hex().to_string());
        self
    }
}

/// Request to create an area/image annotation (for figures, diagrams, etc.).
///
/// Area annotations use `annotationType: "image"` and only require position
/// coordinates (no text content).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAreaAnnotationRequest {
    /// Key of the parent PDF attachment.
    pub parent_item_key: String,
    /// Type of annotation (always "image" for area annotations).
    pub annotation_type: String,
    /// User comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Highlight color (hex code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Page label (human-readable page number).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_label: Option<String>,
    /// Sort index for ordering annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_index: Option<String>,
    /// Position information (page index and rectangular region).
    pub position: AnnotationPosition,
}

impl CreateAreaAnnotationRequest {
    /// Create a new area annotation request.
    ///
    /// # Arguments
    ///
    /// * `parent_item_key` - The key of the parent PDF attachment
    /// * `page_index` - Zero-based page index
    /// * `rect` - Bounding rectangle [x1, y1, x2, y2] in PDF coordinates
    ///
    /// # Example
    ///
    /// ```
    /// use zotero_client::types::CreateAreaAnnotationRequest;
    ///
    /// let request = CreateAreaAnnotationRequest::new(
    ///     "ABCD1234",
    ///     0, // page index
    ///     [100.0, 200.0, 300.0, 400.0], // rect
    /// );
    /// ```
    pub fn new(parent_item_key: impl Into<String>, page_index: u32, rect: [f64; 4]) -> Self {
        Self {
            parent_item_key: parent_item_key.into(),
            annotation_type: "image".to_string(),
            comment: None,
            color: Some("#ffd400".to_string()), // Default yellow
            page_label: Some((page_index + 1).to_string()),
            sort_index: None,
            position: AnnotationPosition {
                page_index,
                rects: vec![rect.to_vec()],
            },
        }
    }

    /// Set the comment on this annotation.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Set the color of this annotation (hex code).
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the color using a semantic `HighlightColor`.
    pub fn with_semantic_color(mut self, color: HighlightColor) -> Self {
        self.color = Some(color.to_hex().to_string());
        self
    }
}

/// Response from the ping endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    /// Status (should be "ok").
    pub status: String,
    /// Plugin name.
    pub plugin: Option<String>,
    /// Plugin version.
    pub version: Option<String>,
    /// Zotero version.
    pub zotero_version: Option<String>,
}

/// Response from search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// List of matching items.
    pub results: Vec<ZoteroItem>,
    /// Total number of results.
    pub total: Option<i64>,
}

/// Response from items endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsResponse {
    /// List of items.
    pub items: Vec<ZoteroItem>,
}

/// Response from children endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenResponse {
    /// Parent item key.
    pub parent_key: Option<String>,
    /// List of child items (attachments, notes, annotations).
    pub children: Vec<serde_json::Value>,
}

/// Response from annotation creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAnnotationResponse {
    /// Whether the operation succeeded.
    pub success: bool,
    /// The created annotation.
    pub annotation: Option<ZoteroAnnotation>,
    /// Error message if failed.
    pub error: Option<String>,
}

/// A text fragment from a PDF with position information.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFragment {
    /// The text content.
    pub text: String,
    /// Zero-based page index.
    pub page: u32,
    /// Bounding rectangle [x1, y1, x2, y2] in PDF coordinates.
    pub rect: [f64; 4],
}
