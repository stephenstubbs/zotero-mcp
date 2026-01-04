//! Unit tests for the zotero-client library.

use crate::types::{
    AnnotationPosition, CreateAnnotationRequest, Creator, PingResponse, SearchResponse, Tag,
    ZoteroAnnotation, ZoteroAttachment, ZoteroItem,
};

#[test]
fn test_zotero_item_deserialization() {
    let json = r#"{
        "id": 123,
        "key": "ABC12345",
        "itemType": "journalArticle",
        "title": "Test Article",
        "creators": [
            {
                "creatorType": "author",
                "firstName": "John",
                "lastName": "Doe"
            }
        ],
        "date": "2024-01-01",
        "extra": "Citation Key: doe2024test",
        "tags": [
            {"tag": "machine-learning", "type": 0}
        ]
    }"#;

    let item: ZoteroItem = serde_json::from_str(json).unwrap();

    assert_eq!(item.id, Some(123));
    assert_eq!(item.key, "ABC12345");
    assert_eq!(item.item_type, "journalArticle");
    assert_eq!(item.title, Some("Test Article".to_string()));
    assert_eq!(item.creators.len(), 1);
    assert_eq!(item.creators[0].first_name, Some("John".to_string()));
    assert_eq!(item.creators[0].last_name, Some("Doe".to_string()));
    assert_eq!(item.date, Some("2024-01-01".to_string()));
    assert_eq!(item.extra, Some("Citation Key: doe2024test".to_string()));
    assert_eq!(item.tags.len(), 1);
    assert_eq!(item.tags[0].tag, "machine-learning");
}

#[test]
fn test_zotero_item_minimal() {
    let json = r#"{
        "key": "XYZ99999",
        "itemType": "book"
    }"#;

    let item: ZoteroItem = serde_json::from_str(json).unwrap();

    assert_eq!(item.id, None);
    assert_eq!(item.key, "XYZ99999");
    assert_eq!(item.item_type, "book");
    assert_eq!(item.title, None);
    assert!(item.creators.is_empty());
    assert!(item.tags.is_empty());
}

#[test]
fn test_creator_with_name() {
    let json = r#"{
        "creatorType": "author",
        "name": "Organization Name"
    }"#;

    let creator: Creator = serde_json::from_str(json).unwrap();

    assert_eq!(creator.creator_type, Some("author".to_string()));
    assert_eq!(creator.name, Some("Organization Name".to_string()));
    assert_eq!(creator.first_name, None);
    assert_eq!(creator.last_name, None);
}

#[test]
fn test_zotero_attachment_deserialization() {
    let json = r#"{
        "id": 456,
        "key": "PDF12345",
        "title": "Full Text PDF",
        "contentType": "application/pdf",
        "path": "/path/to/file.pdf",
        "itemType": "attachment"
    }"#;

    let attachment: ZoteroAttachment = serde_json::from_str(json).unwrap();

    assert_eq!(attachment.id, Some(456));
    assert_eq!(attachment.key, "PDF12345");
    assert_eq!(attachment.title, Some("Full Text PDF".to_string()));
    assert_eq!(attachment.content_type, Some("application/pdf".to_string()));
    assert_eq!(attachment.path, Some("/path/to/file.pdf".to_string()));
    assert_eq!(attachment.item_type, "attachment");
}

#[test]
fn test_zotero_annotation_deserialization() {
    let json = r##"{
        "id": 789,
        "key": "ANN12345",
        "parentItemKey": "PDF12345",
        "annotationType": "highlight",
        "text": "Important text",
        "comment": "My note",
        "color": "#ffd400",
        "pageLabel": "5",
        "position": {
            "pageIndex": 4,
            "rects": [[100, 200, 300, 220]]
        }
    }"##;

    let annotation: ZoteroAnnotation = serde_json::from_str(json).unwrap();

    assert_eq!(annotation.id, Some(789));
    assert_eq!(annotation.key, Some("ANN12345".to_string()));
    assert_eq!(annotation.parent_item_key, Some("PDF12345".to_string()));
    assert_eq!(annotation.annotation_type, Some("highlight".to_string()));
    assert_eq!(annotation.text, Some("Important text".to_string()));
    assert_eq!(annotation.comment, Some("My note".to_string()));
    assert_eq!(annotation.color, Some("#ffd400".to_string()));
    assert_eq!(annotation.page_label, Some("5".to_string()));

    let position = annotation.position.unwrap();
    assert_eq!(position.page_index, 4);
    assert_eq!(position.rects.len(), 1);
    assert_eq!(position.rects[0], vec![100.0, 200.0, 300.0, 220.0]);
}

#[test]
fn test_annotation_position_serialization() {
    let position = AnnotationPosition {
        page_index: 0,
        rects: vec![vec![10.0, 20.0, 100.0, 35.0], vec![10.0, 5.0, 100.0, 20.0]],
    };

    let json = serde_json::to_string(&position).unwrap();

    assert!(json.contains("\"pageIndex\":0"));
    assert!(json.contains("\"rects\":"));
}

#[test]
fn test_create_annotation_request_highlight() {
    let request = CreateAnnotationRequest::highlight(
        "PDF_KEY",
        "Test text",
        0,
        vec![vec![100.0, 200.0, 300.0, 220.0]],
    );

    assert_eq!(request.parent_item_key, "PDF_KEY");
    assert_eq!(request.annotation_type, Some("highlight".to_string()));
    assert_eq!(request.text, Some("Test text".to_string()));
    assert_eq!(request.color, Some("#ffd400".to_string())); // Default yellow
    assert_eq!(request.page_label, Some("1".to_string())); // page_index + 1

    let position = request.position.unwrap();
    assert_eq!(position.page_index, 0);
    assert_eq!(position.rects.len(), 1);
}

#[test]
fn test_create_annotation_request_with_options() {
    let request = CreateAnnotationRequest::highlight("PDF_KEY", "Test", 5, vec![])
        .with_comment("Important!")
        .with_color("#ff0000");

    assert_eq!(request.comment, Some("Important!".to_string()));
    assert_eq!(request.color, Some("#ff0000".to_string()));
    assert_eq!(request.page_label, Some("6".to_string())); // page_index 5 + 1
}

#[test]
fn test_ping_response_deserialization() {
    let json = r#"{
        "status": "ok",
        "plugin": "mcp-zotero-api",
        "version": "1.0.0",
        "zoteroVersion": "7.0.5"
    }"#;

    let ping: PingResponse = serde_json::from_str(json).unwrap();

    assert_eq!(ping.status, "ok");
    assert_eq!(ping.plugin, Some("mcp-zotero-api".to_string()));
    assert_eq!(ping.version, Some("1.0.0".to_string()));
    assert_eq!(ping.zotero_version, Some("7.0.5".to_string()));
}

#[test]
fn test_search_response_deserialization() {
    let json = r#"{
        "results": [
            {"key": "ABC", "itemType": "book"},
            {"key": "DEF", "itemType": "article"}
        ],
        "total": 2
    }"#;

    let response: SearchResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.results.len(), 2);
    assert_eq!(response.total, Some(2));
    assert_eq!(response.results[0].key, "ABC");
    assert_eq!(response.results[1].key, "DEF");
}

#[test]
fn test_tag_deserialization() {
    let json = r#"{"tag": "important", "type": 1}"#;

    let tag: Tag = serde_json::from_str(json).unwrap();

    assert_eq!(tag.tag, "important");
    assert_eq!(tag.tag_type, Some(1));
}

#[test]
fn test_create_annotation_request_serialization() {
    let request = CreateAnnotationRequest::highlight("PDF123", "Highlighted text", 2, vec![]);

    let json = serde_json::to_string(&request).unwrap();

    assert!(json.contains("\"parentItemKey\":\"PDF123\""));
    assert!(json.contains("\"annotationType\":\"highlight\""));
    assert!(json.contains("\"text\":\"Highlighted text\""));
    assert!(json.contains("\"pageLabel\":\"3\"")); // page_index 2 + 1
}
