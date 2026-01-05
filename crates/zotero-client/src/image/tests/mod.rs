//! Unit tests for image extraction module.

use super::*;

#[test]
fn test_image_format_mime_types() {
    assert_eq!(ImageFormat::Png.mime_type(), "image/png");
    assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
}

#[test]
fn test_image_format_default() {
    assert_eq!(ImageFormat::default(), ImageFormat::Png);
}

#[test]
fn test_figure_region_dimensions() {
    let fig = FigureRegion {
        index: 0,
        rect: [100.0, 200.0, 400.0, 500.0],
        figure_type: FigureType::Image,
        confidence: 0.8,
    };

    assert_eq!(fig.width(), 300.0);
    assert_eq!(fig.height(), 300.0);
    assert!((fig.aspect_ratio() - 1.0).abs() < 0.01);
}

#[test]
fn test_figure_region_aspect_ratio() {
    let wide_fig = FigureRegion {
        index: 0,
        rect: [0.0, 0.0, 400.0, 200.0],
        figure_type: FigureType::Chart,
        confidence: 0.7,
    };

    assert!((wide_fig.aspect_ratio() - 2.0).abs() < 0.01);

    let tall_fig = FigureRegion {
        index: 0,
        rect: [0.0, 0.0, 100.0, 400.0],
        figure_type: FigureType::Diagram,
        confidence: 0.6,
    };

    assert!((tall_fig.aspect_ratio() - 0.25).abs() < 0.01);
}

#[test]
fn test_figure_type_descriptions() {
    assert_eq!(FigureType::Image.description(), "image");
    assert_eq!(FigureType::Chart.description(), "chart");
    assert_eq!(FigureType::Diagram.description(), "diagram");
    assert_eq!(FigureType::Unknown.description(), "figure");
}

#[test]
fn test_image_output_structure() {
    let output = ImageOutput {
        data: "dGVzdA==".to_string(), // "test" in base64
        mime_type: "image/png".to_string(),
        width: 100,
        height: 200,
    };

    assert_eq!(output.width, 100);
    assert_eq!(output.height, 200);
    assert_eq!(output.mime_type, "image/png");
}

#[test]
fn test_embedded_image_structure() {
    let img = EmbeddedImage {
        index: 0,
        rect: [10.0, 20.0, 110.0, 120.0],
        width: 100,
        height: 100,
        data: "base64data".to_string(),
        mime_type: "image/png".to_string(),
    };

    assert_eq!(img.index, 0);
    assert_eq!(img.width, 100);
    assert_eq!(img.rect[0], 10.0);
}
