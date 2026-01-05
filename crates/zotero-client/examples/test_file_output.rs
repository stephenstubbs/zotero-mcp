//! Test file-based image rendering.
//! Run: cargo run --package zotero-client --features image --example test_file_output

use zotero_client::image::{render_page_to_file, ImageFormat};

fn main() {
    let pdf_path =
        "/home/stephenstubbs/Zotero/storage/B8YU42RN/Sommerville - 2016 - Software engineering.pdf";

    println!("Testing file-based image rendering\n");

    // Test different DPIs
    for (dpi, format) in [
        (50, ImageFormat::Jpeg),
        (150, ImageFormat::Png),
        (300, ImageFormat::Jpeg),
    ] {
        let ext = match format {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
        };
        let output_path = format!("/tmp/test-page-{dpi}dpi.{ext}");

        match render_page_to_file(pdf_path, 0, dpi, format, &output_path) {
            Ok(saved_path) => {
                let metadata = std::fs::metadata(&saved_path).unwrap();
                println!(
                    "✓ {dpi} DPI {:?}: {} ({:.1} KB)",
                    format,
                    saved_path,
                    metadata.len() / 1024
                );
            }
            Err(e) => {
                println!("✗ {dpi} DPI {:?}: ERROR - {}", format, e);
            }
        }
    }

    println!(
        "\nNow the MCP response only contains the file path (~100 bytes) instead of MB of base64!"
    );
}
