//! Test script for creating highlights on a specific citation key.
//!
//! This mirrors the Python test script's functionality.
//!
//! Run with:
//! ```bash
//! cargo run --example highlight_test --features pdf
//! ```
//!
//! Or without PDF features (won't have text positions):
//! ```bash
//! cargo run --example highlight_test
//! ```

use std::path::Path;

use zotero_client::types::CreateAnnotationRequest;
use zotero_client::ZoteroClient;

const CITATION_KEY: &str = "bellemarePracticalDataMesh2022";
const HIGHLIGHT_TEXT: &str = "Practical Data Mesh";
const HIGHLIGHT_COLOR: &str = "#ffd400"; // Yellow
const HIGHLIGHT_COMMENT: &str = "Test highlight via Rust MCP client";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(60));
    println!("Zotero MCP API Test Script (Rust)");
    println!("{}", "=".repeat(60));

    let client = ZoteroClient::new();

    // Step 1: Check if MCP plugin is active
    println!("\n[1] Checking for MCP Zotero API plugin...");
    let ping = client.ping().await?;
    println!(
        "OK: MCP Plugin v{} on Zotero {}",
        ping.version.as_deref().unwrap_or("unknown"),
        ping.zotero_version.as_deref().unwrap_or("unknown")
    );

    // Step 2: Look up item by citation key
    println!("\n[2] Looking up item with citation key: {}", CITATION_KEY);
    let item = match client.find_by_citation_key(CITATION_KEY, 500).await? {
        Some(item) => {
            println!(
                "OK: Found item: {}",
                item.title.as_deref().unwrap_or("No title")
            );
            println!("    Key: {}", item.key);
            println!("    Type: {}", item.item_type);
            item
        }
        None => {
            println!("Citation key not found in 'extra' field.");
            println!("Trying title search for 'Practical Data Mesh'...");

            let items = client.search_items("Practical Data Mesh", 10).await?;
            match items.into_iter().next() {
                Some(item) => {
                    println!(
                        "OK: Found item: {}",
                        item.title.as_deref().unwrap_or("No title")
                    );
                    println!("    Key: {}", item.key);
                    item
                }
                None => {
                    println!("ERROR: Could not find the item.");
                    println!("\nListing first 10 items in library:");
                    let items = client.list_items(10).await?;
                    for (i, item) in items.iter().enumerate() {
                        let title = item.title.as_deref().unwrap_or("No title");
                        let display_title: String = title.chars().take(60).collect();
                        println!("  {}. {} [{}]", i + 1, display_title, item.key);
                    }
                    return Err("Item not found".into());
                }
            }
        }
    };

    // Step 3: Get PDF attachment
    println!("\n[3] Looking for PDF attachments...");
    let pdfs = client.get_pdf_attachments(&item.key).await?;

    let pdf = match pdfs.into_iter().next() {
        Some(pdf) => {
            println!("OK: Found PDF attachment:");
            println!("    Key: {}", pdf.key);
            println!("    Path: {}", pdf.path.as_deref().unwrap_or("No path"));
            pdf
        }
        None => {
            println!("ERROR: No PDF attachment found.");
            println!("Checking all children:");
            let children = client.get_children(&item.key).await?;
            for child in &children.children {
                let item_type = child
                    .get("itemType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let title = child
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("No title");
                println!("  - {}: {}", item_type, title);
            }
            return Err("No PDF attachment found".into());
        }
    };

    // Step 4: Read PDF text (if pdf feature enabled)
    let mut highlight_rects: Vec<Vec<f64>> = vec![];

    #[cfg(feature = "pdf")]
    {
        if let Some(pdf_path) = &pdf.path {
            if Path::new(pdf_path).exists() {
                println!("\n[4] Reading PDF content...");

                match zotero_client::pdf::extract_text(pdf_path, 0) {
                    Ok(text) => {
                        println!("OK: Extracted {} characters from page 1.", text.len());
                        println!("\nFirst 500 characters:");
                        println!("{}", "-".repeat(40));
                        let preview: String = text.chars().take(500).collect();
                        println!("{}", preview);
                        println!("{}", "-".repeat(40));
                    }
                    Err(e) => {
                        println!("Warning: Could not extract text: {}", e);
                    }
                }

                println!("\n[5] Searching for text using MuPDF native search (like PyMuPDF)...");
                // Use search_for_rects which matches PyMuPDF's page.search_for() behavior
                match zotero_client::pdf::search_for_rects(pdf_path, 0, HIGHLIGHT_TEXT) {
                    Ok(rects) => {
                        if rects.is_empty() {
                            println!("Text '{}' not found on page 1", HIGHLIGHT_TEXT);
                        } else {
                            println!("OK: Found {} rect(s) for highlight.", rects.len());
                            for (i, rect) in rects.iter().enumerate() {
                                println!(
                                    "  Rect {}: [{:.1}, {:.1}, {:.1}, {:.1}]",
                                    i + 1,
                                    rect[0],
                                    rect[1],
                                    rect[2],
                                    rect[3]
                                );
                                highlight_rects.push(rect.to_vec());
                            }
                        }
                    }
                    Err(e) => {
                        println!("Warning: Could not search for text: {}", e);
                    }
                }
            } else {
                println!("\n[4] WARNING: PDF not found at: {}", pdf_path);
            }
        }
    }

    #[cfg(not(feature = "pdf"))]
    {
        println!("\n[4] PDF feature not enabled - skipping text extraction");
        println!("    Run with: cargo run --example highlight_test --features pdf");
    }

    // Step 5: Create a test highlight
    println!("\n[6] Creating a test highlight...");

    let request = CreateAnnotationRequest::highlight(&pdf.key, HIGHLIGHT_TEXT, 0, highlight_rects)
        .with_comment(HIGHLIGHT_COMMENT)
        .with_color(HIGHLIGHT_COLOR);

    let result = client.create_annotation(request).await?;

    if result.success {
        println!("OK: Created highlight!");
        if let Some(ann) = &result.annotation {
            println!(
                "    Annotation Key: {}",
                ann.key.as_deref().unwrap_or("unknown")
            );
            println!("    Text: {}", ann.text.as_deref().unwrap_or(""));
            println!("    Page: {}", ann.page_label.as_deref().unwrap_or(""));
        }
    } else {
        println!("ERROR: Failed to create highlight.");
        if let Some(err) = &result.error {
            println!("    Details: {}", err);
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("Test complete!");
    println!("{}", "=".repeat(60));

    Ok(())
}
