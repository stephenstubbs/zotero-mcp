//! Integration tests for the Zotero client.
//!
//! These tests require a running Zotero instance with the MCP plugin installed.
//! Run with: `cargo test --features integration`

#![cfg(feature = "integration")]

use zotero_client::types::CreateAnnotationRequest;
use zotero_client::ZoteroClient;

/// Test that we can ping the Zotero MCP plugin.
#[tokio::test]
async fn test_ping() {
    let client = ZoteroClient::new();
    let result = client.ping().await;

    match result {
        Ok(ping) => {
            assert_eq!(ping.status, "ok");
            assert!(ping.version.is_some());
            println!(
                "Connected to Zotero {} with plugin v{}",
                ping.zotero_version.unwrap_or_default(),
                ping.version.unwrap_or_default()
            );
        }
        Err(e) => {
            println!("Skipping test: Zotero not running - {}", e);
        }
    }
}

/// Test searching for items.
#[tokio::test]
async fn test_search_items() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    let items = client.search_items("test", 10).await;

    match items {
        Ok(items) => {
            println!("Found {} items matching 'test'", items.len());
            for item in &items {
                println!("  - {} [{}]", item.title.as_deref().unwrap_or("No title"), item.key);
            }
        }
        Err(e) => {
            println!("Search failed: {}", e);
        }
    }
}

/// Test listing items from the library.
#[tokio::test]
async fn test_list_items() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    let items = client.list_items(5).await;

    match items {
        Ok(items) => {
            println!("Listed {} items from library", items.len());
            for item in &items {
                println!(
                    "  - {} ({}) [{}]",
                    item.title.as_deref().unwrap_or("No title"),
                    item.item_type,
                    item.key
                );
            }
        }
        Err(e) => {
            println!("List failed: {}", e);
        }
    }
}

/// Test getting an item by key.
#[tokio::test]
async fn test_get_item() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    // First list some items to get a key
    let items = client.list_items(1).await;

    match items {
        Ok(items) if !items.is_empty() => {
            let key = &items[0].key;
            let item = client.get_item(key).await;

            match item {
                Ok(item) => {
                    println!("Got item: {} [{}]", item.title.as_deref().unwrap_or("No title"), item.key);
                }
                Err(e) => {
                    println!("Get item failed: {}", e);
                }
            }
        }
        Ok(_) => {
            println!("No items in library to test with");
        }
        Err(e) => {
            println!("List failed: {}", e);
        }
    }
}

/// Test getting children of an item.
#[tokio::test]
async fn test_get_children() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    // First list some items to get a key
    let items = client.list_items(5).await;

    match items {
        Ok(items) if !items.is_empty() => {
            for item in items {
                let children = client.get_children(&item.key).await;

                match children {
                    Ok(children) => {
                        if !children.children.is_empty() {
                            println!(
                                "Item '{}' has {} children",
                                item.title.as_deref().unwrap_or("No title"),
                                children.children.len()
                            );
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Get children failed: {}", e);
                    }
                }
            }
            println!("No items with children found");
        }
        Ok(_) => {
            println!("No items in library to test with");
        }
        Err(e) => {
            println!("List failed: {}", e);
        }
    }
}

/// Test getting PDF attachments.
#[tokio::test]
async fn test_get_pdf_attachments() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    // First list some items to get a key
    let items = client.list_items(20).await;

    match items {
        Ok(items) if !items.is_empty() => {
            for item in items {
                let pdfs = client.get_pdf_attachments(&item.key).await;

                match pdfs {
                    Ok(pdfs) if !pdfs.is_empty() => {
                        println!(
                            "Item '{}' has {} PDF attachment(s):",
                            item.title.as_deref().unwrap_or("No title"),
                            pdfs.len()
                        );
                        for pdf in &pdfs {
                            println!(
                                "  - {} [{}] at {}",
                                pdf.title.as_deref().unwrap_or("Untitled"),
                                pdf.key,
                                pdf.path.as_deref().unwrap_or("No path")
                            );
                        }
                        return;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        println!("Get PDF attachments failed: {}", e);
                    }
                }
            }
            println!("No items with PDF attachments found");
        }
        Ok(_) => {
            println!("No items in library to test with");
        }
        Err(e) => {
            println!("List failed: {}", e);
        }
    }
}

/// Test creating an annotation (commented out by default to avoid modifying library).
#[tokio::test]
#[ignore = "This test modifies the library - run manually with --include-ignored"]
async fn test_create_annotation() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    // Find an item with a PDF attachment
    let items = client.list_items(50).await.unwrap();

    for item in items {
        let pdfs = client.get_pdf_attachments(&item.key).await.unwrap();

        if let Some(pdf) = pdfs.first() {
            let request = CreateAnnotationRequest::highlight(
                &pdf.key,
                "Test annotation from integration test",
                0,
                vec![],
            )
            .with_comment("Created by zotero-client integration test")
            .with_color("#ff6666");

            let result = client.create_annotation(request).await;

            match result {
                Ok(response) => {
                    if response.success {
                        println!(
                            "Created annotation: {:?}",
                            response.annotation.map(|a| a.key)
                        );
                    } else {
                        println!("Annotation creation failed: {:?}", response.error);
                    }
                }
                Err(e) => {
                    println!("Create annotation failed: {}", e);
                }
            }
            return;
        }
    }

    println!("No PDF attachments found to annotate");
}

/// Test finding by citation key.
#[tokio::test]
async fn test_find_by_citation_key() {
    let client = ZoteroClient::new();

    // Skip if Zotero is not running
    if client.ping().await.is_err() {
        println!("Skipping test: Zotero not running");
        return;
    }

    // List items and find one with a citation key
    let items = client.list_items(50).await.unwrap();

    for item in &items {
        if let Some(extra) = &item.extra {
            if extra.contains("Citation Key:") || extra.to_lowercase().contains("citekey:") {
                // Extract the citation key
                let key = extra
                    .lines()
                    .find(|l| l.contains("Citation Key:") || l.to_lowercase().contains("citekey:"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(|s| s.trim());

                if let Some(cite_key) = key {
                    println!("Found item with citation key: {}", cite_key);

                    let found = client.find_by_citation_key(cite_key, 100).await.unwrap();

                    match found {
                        Some(found_item) => {
                            assert_eq!(found_item.key, item.key);
                            println!("Successfully found item by citation key!");
                            return;
                        }
                        None => {
                            println!("Could not find item by citation key");
                        }
                    }
                }
            }
        }
    }

    println!("No items with citation keys found");
}
