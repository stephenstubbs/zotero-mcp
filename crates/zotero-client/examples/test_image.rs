//! Quick test for image extraction.
//! Run: cargo run --package zotero-client --features image --example test_image

use zotero_client::image::{detect_figures, render_page, render_region, ImageFormat};

fn main() {
    // Use a known PDF from Zotero storage
    let pdf_path =
        "/home/stephenstubbs/Zotero/storage/B8YU42RN/Sommerville - 2016 - Software engineering.pdf";

    println!("Testing image extraction on: {}\n", pdf_path);

    // Test 1: Render page 1
    println!("=== Test 1: Render Page 1 (PNG, 150 DPI) ===");
    match render_page(pdf_path, 0, 150, ImageFormat::Png) {
        Ok(output) => {
            println!("  SUCCESS!");
            println!("  Dimensions: {}x{} pixels", output.width, output.height);
            println!("  MIME type: {}", output.mime_type);
            println!("  Base64 data length: {} chars", output.data.len());

            // Save to file for visual verification
            if let Ok(decoded) =
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &output.data)
            {
                if std::fs::write("/tmp/test_page_1.png", &decoded).is_ok() {
                    println!("  Saved to: /tmp/test_page_1.png");
                }
            }
        }
        Err(e) => println!("  ERROR: {}", e),
    }

    // Test 2: Render as JPEG
    println!("\n=== Test 2: Render Page 1 (JPEG, 100 DPI) ===");
    match render_page(pdf_path, 0, 100, ImageFormat::Jpeg) {
        Ok(output) => {
            println!("  SUCCESS!");
            println!("  Dimensions: {}x{} pixels", output.width, output.height);
            println!("  MIME type: {}", output.mime_type);
            println!(
                "  Base64 data length: {} chars (smaller than PNG)",
                output.data.len()
            );
        }
        Err(e) => println!("  ERROR: {}", e),
    }

    // Test 3: Detect figures
    println!("\n=== Test 3: Detect Figures on Page 1 ===");
    match detect_figures(pdf_path, 0) {
        Ok(figures) => {
            println!("  SUCCESS!");
            println!("  Detected {} figure region(s)", figures.len());
            for fig in &figures {
                println!(
                    "    - Figure {}: {} at [{:.0}, {:.0}, {:.0}, {:.0}] confidence={:.2}",
                    fig.index,
                    fig.figure_type.description(),
                    fig.rect[0],
                    fig.rect[1],
                    fig.rect[2],
                    fig.rect[3],
                    fig.confidence
                );
            }

            // Test 4: Render first detected figure
            if let Some(fig) = figures.first() {
                println!("\n=== Test 4: Render First Detected Figure ===");
                match render_region(pdf_path, 0, fig.rect, 150, ImageFormat::Png) {
                    Ok(output) => {
                        println!("  SUCCESS!");
                        println!(
                            "  Figure dimensions: {}x{} pixels",
                            output.width, output.height
                        );

                        if let Ok(decoded) = base64::Engine::decode(
                            &base64::engine::general_purpose::STANDARD,
                            &output.data,
                        ) {
                            if std::fs::write("/tmp/test_figure_0.png", &decoded).is_ok() {
                                println!("  Saved to: /tmp/test_figure_0.png");
                            }
                        }
                    }
                    Err(e) => println!("  ERROR: {}", e),
                }
            }
        }
        Err(e) => println!("  ERROR: {}", e),
    }

    // Test 5: Render a specific region
    println!("\n=== Test 5: Render Specific Region ===");
    let test_rect = [100.0, 100.0, 400.0, 300.0]; // A test region
    match render_region(pdf_path, 0, test_rect, 150, ImageFormat::Png) {
        Ok(output) => {
            println!("  SUCCESS!");
            println!(
                "  Region dimensions: {}x{} pixels",
                output.width, output.height
            );
        }
        Err(e) => println!("  ERROR: {}", e),
    }

    println!("\n=== All tests completed! ===");
}
