//! Manual test for image extraction functionality.
//!
//! Run with: cargo run --example test_image_extraction --features image
//!
//! Prerequisites:
//! - Zotero must be running with the MCP plugin
//! - You need to know an attachment key for a PDF in your library
//!
//! Usage:
//!   cargo run --example test_image_extraction -- <attachment_key> [page_number]

use std::env;
use std::fs;
use zotero_client::image::{detect_figures, render_page, render_region, ImageFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <pdf_path> [page_number]", args[0]);
        eprintln!("\nExample: {} /path/to/paper.pdf 1", args[0]);
        std::process::exit(1);
    }

    let pdf_path = &args[1];
    let page_num: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
    let page_index = page_num.saturating_sub(1);

    println!("Testing image extraction on: {}", pdf_path);
    println!("Page: {} (0-indexed: {})\n", page_num, page_index);

    // Test 1: Render full page
    println!("=== Test 1: Render Full Page ===");
    match render_page(pdf_path, page_index, 150, ImageFormat::Png) {
        Ok(output) => {
            println!("  Success!");
            println!("  Size: {}x{} pixels", output.width, output.height);
            println!("  MIME: {}", output.mime_type);
            println!("  Base64 length: {} chars", output.data.len());

            // Optionally save to file
            let decoded =
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &output.data)?;
            let filename = format!("test_page_{}.png", page_num);
            fs::write(&filename, &decoded)?;
            println!("  Saved to: {}", filename);
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
    println!();

    // Test 2: Detect figures
    println!("=== Test 2: Detect Figures ===");
    match detect_figures(pdf_path, page_index) {
        Ok(figures) => {
            if figures.is_empty() {
                println!("  No figures detected on this page.");
            } else {
                println!("  Found {} figure(s):", figures.len());
                for fig in &figures {
                    println!(
                        "    Figure {}: {} at [{:.1}, {:.1}, {:.1}, {:.1}] (confidence: {:.2})",
                        fig.index,
                        fig.figure_type.description(),
                        fig.rect[0],
                        fig.rect[1],
                        fig.rect[2],
                        fig.rect[3],
                        fig.confidence
                    );
                }

                // Test 3: Render first figure
                if let Some(first_fig) = figures.first() {
                    println!("\n=== Test 3: Render First Figure ===");
                    match render_region(pdf_path, page_index, first_fig.rect, 150, ImageFormat::Png)
                    {
                        Ok(output) => {
                            println!("  Success!");
                            println!("  Size: {}x{} pixels", output.width, output.height);

                            let decoded = base64::Engine::decode(
                                &base64::engine::general_purpose::STANDARD,
                                &output.data,
                            )?;
                            let filename = format!("test_figure_{}_0.png", page_num);
                            fs::write(&filename, &decoded)?;
                            println!("  Saved to: {}", filename);
                        }
                        Err(e) => {
                            println!("  Error: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }

    println!("\nDone!");
    Ok(())
}
