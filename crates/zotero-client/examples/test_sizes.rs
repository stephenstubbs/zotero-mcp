//! Test to check base64 output sizes at different DPIs.
//! Run: cargo run --package zotero-client --features image --example test_sizes

use zotero_client::image::{render_page, ImageFormat};

fn main() {
    let pdf_path =
        "/home/stephenstubbs/Zotero/storage/B8YU42RN/Sommerville - 2016 - Software engineering.pdf";

    println!("Testing base64 sizes at different DPIs\n");
    println!(
        "{:<10} {:<15} {:<20} {:<20}",
        "DPI", "Dimensions", "PNG (KB)", "JPEG (KB)"
    );
    println!("{}", "=".repeat(70));

    for dpi in [50, 72, 100, 150] {
        // Test PNG
        let png_result = render_page(pdf_path, 0, dpi, ImageFormat::Png);
        let jpeg_result = render_page(pdf_path, 0, dpi, ImageFormat::Jpeg);

        match (png_result, jpeg_result) {
            (Ok(png), Ok(jpeg)) => {
                let png_kb = png.data.len() as f64 / 1024.0;
                let jpeg_kb = jpeg.data.len() as f64 / 1024.0;

                println!(
                    "{:<10} {:<15} {:<20.1} {:<20.1}",
                    dpi,
                    format!("{}x{}", png.width, png.height),
                    png_kb,
                    jpeg_kb
                );
            }
            _ => {
                println!("{:<10} ERROR", dpi);
            }
        }
    }
}
