extern crate cairo;
extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

use cairo::{Context, Format, ImageSurface};
use serde_derive::Deserialize;
use serde_json::{self, Value};

#[derive(Debug, Deserialize)]
struct MXGraphModel {
    root: Root,
}

#[derive(Debug, Deserialize)]
struct Root {
    #[serde(rename(deserialize = "mxCell"))]
    mx_cell: Vec<MxCell>,
}

#[derive(Debug, Deserialize)]
struct MxCell {
    id: Option<String>,
    value: Option<String>,
    style: Option<String>,
    vertex: Option<String>,
    #[serde(rename(deserialize = "mxGeometry"))]
    mx_geometry: Option<MxGeometry>,
}

#[derive(Debug, Deserialize)]
struct MxGeometry {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct ShapeStyleFromXml {
    html: Option<String>,
    #[serde(rename(deserialize = "whiteSpace"))]
    white_space: Option<String>,
    shape: Option<String>,
    perimeter: Option<String>,
    #[serde(rename(deserialize = "fixedSize"))]
    fixed_size: Option<String>,
    #[serde(rename(deserialize = "flipH"))]
    flip_h: Option<String>,
}

/// Convert to `.png`
pub fn convert_to_png(file_name: &str) {
    // Read `.xml` formatted file to construct it to `MXGraphModel` structure.
    let xml_data: String =
        std::fs::read_to_string(file_name).expect("Can't read `.xml` output file.");
    let mx_graph_model: MXGraphModel = serde_xml_rs::from_reader(xml_data.as_bytes())
        .unwrap_or_else(|e| panic!("Error in MXGraphModel parsing.\n{}", e));
    // println!("{:#?}", mx_graph_model);

    // Main exported `.png` frame size.
    // TODO: Make it dynamic surface (paper) size
    let surface = ImageSurface::create(Format::ARgb32, 800, 600).expect("Can't create page frame.");
    let context = Context::new(&surface)
        .unwrap_or_else(|e| panic!("Can't get instance of page surface.\n{}", e));

    // Set the background color.
    context.set_source_rgb(1.0, 1.0, 1.0);
    context
        .paint()
        .expect("Failed to paint the background color");

    // Loop through mxCell tags `<mxCell><some_mx_geometry /></mxCell>`.
    for cell in mx_graph_model.root.mx_cell {
        if let Some(vertex) = cell.vertex {
            if vertex == "1" {
                // MXGeometry settings like `x` and `y` coordinations or `width` and `height` size.
                if let Some(geometry) = cell.mx_geometry {
                    // Gets style string from MXCell. TODO: Needs to define default value.
                    let style_str = &cell.style.unwrap_or("".to_owned());

                    // Gets geometry settings from MXGeometry.
                    let x = geometry.x.unwrap_or(0.0);
                    let y = geometry.y.unwrap_or(0.0);
                    let width = geometry.width.unwrap_or(100.0);
                    let height = geometry.height.unwrap_or(50.0);

                    // Set red color for the objects.
                    context.set_source_rgb(1.0, 0.0, 0.0);

                    // Converts `serde_json` HashMap to predefine ShapeStyleFromXml struct.
                    let styles: ShapeStyleFromXml =
                        serde_json::from_value(Value::Object(parse_style(style_str))).unwrap();

                    match styles.shape.unwrap().as_str() {
                        "parallelogram" => {
                            let flip_h_value = styles
                                .flip_h
                                .unwrap_or("1".to_string())
                                .parse::<i8>()
                                .unwrap();
                            parallelogram(&context, x, y, width, height, flip_h_value);
                        }
                        "hexagon" => {
                            hexagon(&context, x, y, width, height);
                        }
                        _ => panic!("Error"),
                    }
                    context.set_source_rgb(0.0, 0.0, 0.0);
                    context.set_line_width(1.0);
                    context.stroke().expect("Failed to draw rectangle.");

                    if let Some(shape_label) = cell.value {
                        // Label positioning inside of shape
                        context.move_to(geometry.x.unwrap_or(0.0), geometry.y.unwrap_or(0.0));
                        context.set_font_size(14.0);
                        context
                            .show_text(&shape_label)
                            .expect("Failed to draw text");
                    }
                }
            }
        }
    }

    // Final result
    let mut output_png_file =
        std::fs::File::create("output.png").expect("Couldn't create output file.");
    surface
        .write_to_png(&mut output_png_file)
        .expect("Failed to write to png.");
}

/// Parse Style to `serde_json` HashMap
///
/// ```xml
/// <mxCell
///     style='shape={SHAPE};perimeter={PERIMETER};whiteSpace=wrap;html=1;fixedSize=1;flipH={FLIP_H}'
/// />
/// ```
///
/// ### Output
///
/// ```json
/// {
///     "shape": "hexagon",
///     "perimeter": "perimeterID",
///     "whiteSpace": "wrap",
///     "html": "1",
///     "fixedSize": "1",
///     "flipH": "0"
/// }
/// ```
fn parse_style(style: &str) -> serde_json::Map<String, Value> {
    style
        .split(";")
        .filter_map(|item| {
            let mut parts = item.split("=");
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                Some((key.to_string(), Value::String(value.to_string())))
            } else {
                None
            }
        })
        .collect()
}

/// Draw Parallelogram
///
/// Defines the vertices of the parallelogram.
///
/// Without horizontal flip:
///
/// ```txt
///        ●------------------●
///       /                  /
///      /      Context     /
///     /                  /
///    ●------------------●
/// ```
///
/// With horizontal flip:
///
/// ```txt
///    ●-------------------●
///     \                   \
///      \      Context      \
///       \                   \
///        ●-------------------●
/// ```
fn parallelogram(context: &Context, x: f64, y: f64, width: f64, height: f64, flip: i8) {
    let mut skew: f64 = 30.0;

    if flip == 1 {
        skew = -30.0;
    }

    if flip == 0 {
        skew = 30.0;
    }

    // Top-left corner
    context.move_to(x, y);
    // Top-right corner
    context.line_to(x + width, y);
    // Bottom-right corner that shifted left or right by `skew`.
    context.line_to(x + width - skew, y + height);
    // Bottom-left corner that shifted left or right by `skew`.
    context.line_to(x - skew, y + height);
    context.close_path();
}

/// Draw Hexagon
///
/// It's simple without any complication, limitation and condition.
///
/// ```txt
///     ●-------------------●
///   /                      \
///  /                        \
/// ●          Context         ●
///  \                        /
///   \                      /
///     ●-------------------●
/// ```
fn hexagon(context: &Context, x: f64, y: f64, width: f64, height: f64) {
    let radius_x = width / 2.0;
    let radius_y = height / (3.0 as f64).sqrt();

    for i in 0..6 {
        // Calculate the angle for each vertex of the hexagon
        let angle = std::f64::consts::PI / 3.0 * i as f64;
        let vertex_x = x + radius_x * angle.cos();
        let vertex_y = y + radius_y * angle.sin();

        if i == 0 {
            context.move_to(vertex_x, vertex_y);
        } else {
            context.line_to(vertex_x, vertex_y);
        }
    }
    context.close_path();
}
