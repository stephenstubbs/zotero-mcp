//! Test script for creating multi-line highlights.
//!
//! Run with:
//! ```bash
//! cargo run --example multiline_highlight_test --features pdf
//! ```

use zotero_client::types::CreateAnnotationRequest;
use zotero_client::ZoteroClient;

const CITATION_KEY: &str = "bellemarePracticalDataMesh2022";
const HIGHLIGHT_PAGE: usize = 7; // 0-indexed, so page 8 in the PDF (labeled as page 6 in the book)
const HIGHLIGHT_TEXT: &str = "Adam Bellemare has created a  definitive resource that marries these two together. In fact, it may be the  missing manual I always needed back in my enterprise days: a practical  guide for building data systems that treats data as decentralized products  and puts these products at the heart of your architecture.  This book also does a great job of making data mesh more tangible. Adam  uses the data mesh principles as a set of logical guardrails that help readers  understand the trade-offs they need to consider. He then dives deep into  practical and opinionated implementation";
const HIGHLIGHT_COLOR: &str = "#a28ae5"; // Purple for visibility
const HIGHLIGHT_COMMENT: &str = "Rust MCP multiline test - NEW";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(60));
    println!("Zotero MCP API - Multi-line Highlight Test (Rust)");
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

    // Step 2: Look up item by citation key or title
    println!("\n[2] Looking up item...");
    let item = match client.find_by_citation_key(CITATION_KEY, 500).await? {
        Some(item) => item,
        None => {
            println!("Citation key not found, trying title search...");
            let items = client.search_items("Practical Data Mesh", 10).await?;
            items.into_iter().next().ok_or("Item not found")?
        }
    };
    println!("OK: Found item: {}", item.title.as_deref().unwrap_or("No title"));

    // Step 3: Get PDF attachment
    println!("\n[3] Looking for PDF attachments...");
    let pdfs = client.get_pdf_attachments(&item.key).await?;
    let pdf = pdfs.into_iter().next().ok_or("No PDF attachment found")?;
    println!("OK: Found PDF: {}", pdf.path.as_deref().unwrap_or("No path"));

    // Step 4: Search for text using MuPDF's native search (like PyMuPDF)
    #[cfg(feature = "pdf")]
    let highlight_rects: Vec<Vec<f64>> = {
        if let Some(pdf_path) = &pdf.path {
            println!("\n[4] Searching for text using MuPDF native search (like PyMuPDF)...");
            println!("    Page: {}", HIGHLIGHT_PAGE + 1);
            println!("    Search text length: {} chars", HIGHLIGHT_TEXT.len());
            
            // Use MuPDF's native search which handles multi-line text properly
            // This matches PyMuPDF's page.search_for() behavior exactly
            match zotero_client::pdf::search_for_rects(pdf_path, HIGHLIGHT_PAGE, HIGHLIGHT_TEXT) {
                Ok(rects) => {
                    if rects.is_empty() {
                        println!("Text not found on page {}", HIGHLIGHT_PAGE + 1);
                        println!("\n[5] Trying to extract page text for debugging...");
                        if let Ok(text) = zotero_client::pdf::extract_text(pdf_path, HIGHLIGHT_PAGE) {
                            println!("First 500 chars of page:");
                            println!("{}", text.chars().take(500).collect::<String>());
                        }
                        Vec::new()
                    } else {
                        println!("OK: Found {} rect(s) for multi-line highlight", rects.len());
                        for (i, rect) in rects.iter().enumerate() {
                            println!("  Rect {}: [{:.1}, {:.1}, {:.1}, {:.1}]", 
                                i + 1, rect[0], rect[1], rect[2], rect[3]);
                        }
                        rects.into_iter().map(|r| r.to_vec()).collect()
                    }
                }
                Err(e) => {
                    println!("Warning: Could not search for text: {}", e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    };

    #[cfg(not(feature = "pdf"))]
    let highlight_rects: Vec<Vec<f64>> = {
        println!("\n[4] PDF feature not enabled - no position data");
        Vec::new()
    };

    // Step 6: Create the highlight
    println!("\n[6] Creating multi-line highlight...");
    
    let request = CreateAnnotationRequest::highlight(
        &pdf.key,
        HIGHLIGHT_TEXT,
        HIGHLIGHT_PAGE as u32,
        highlight_rects.clone(),
    )
    .with_comment(HIGHLIGHT_COMMENT)
    .with_color(HIGHLIGHT_COLOR);

    println!("Request rects count: {}", highlight_rects.len());

    let result = client.create_annotation(request).await?;

    if result.success {
        println!("OK: Created highlight!");
        if let Some(ann) = &result.annotation {
            println!("    Annotation Key: {}", ann.key.as_deref().unwrap_or("unknown"));
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
