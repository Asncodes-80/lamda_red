extern crate cairo;
extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

use cairo::{Context, Format, ImageSurface};
use serde_derive::Deserialize;
use serde_json::{self, Value};

use crate::SurfaceSettings;

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
pub fn convert_to_png(file_name: &str, surface_settings: SurfaceSettings) {
    // Read `.xml` formatted file to construct it to `MXGraphModel` structure.
    let xml_data: String =
        std::fs::read_to_string(file_name).expect("Can't read `.xml` output file.");
    let mx_graph_model: MXGraphModel = serde_xml_rs::from_reader(xml_data.as_bytes())
        .unwrap_or_else(|e| panic!("Error in MXGraphModel parsing.\n{}", e));
    // println!("{:#?}", mx_graph_model);

    // Main exported `.png` frame size.
    let surface = ImageSurface::create(
        Format::ARgb32,
        surface_settings.width as i32,
        surface_settings.height as i32,
    )
    .expect("Can't create frame.");
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
                    // Gets style string from MXCell.
                    let style_str = &cell.style.unwrap_or("".to_owned());
                    let mut shape_label: String = cell.value.unwrap_or(String::from(""));

                    // Gets shape size and geometry settings from MXGeometry.
                    let mut x = geometry.x.unwrap_or(0.0);
                    let mut y = geometry.y.unwrap_or(0.0);
                    let mut width = geometry.width.unwrap_or(100.0);
                    let mut height = geometry.height.unwrap_or(50.0);

                    // Makes object width dynamically and fixes x axis position start point.
                    // TODO: it needs to be clean.
                    if shape_label.len() > 16 && shape_label.len() < 33 {
                        for _ in 0..shape_label.len() - 16 {
                            width += 2.5;
                            x -= 0.5;
                            y += 10.0;
                        }
                    } else {
                        for _ in 0..shape_label.len() - 25 {
                            width += 1.73;
                            height += 0.5;
                            x -= 0.57;
                            y += 0.5;
                        }
                        shape_label = multiline_label(shape_label, 8);
                    }

                    draw_diagram(&context, x, y, width, height, &style_str);
                    draw_multiline_label(&context, &shape_label, x, y + 35.0, 20.0);
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
/// # Output
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
    // Hexagon height
    let radius_y = height / (5.0 as f64).sqrt();

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

/// Modifies one line label to multiline label
///
/// Creates a multiline label by splitting the input text into lines with a
/// specific number of words per line (chunk).
///
/// - `text`: A `String` containing the text to be split into multi lines.
/// - `chunk`: The maximum number of words per line. If the number of words is
/// not a multiple of `chunk` the last line may contain fewer words.
///
/// # Returns
///
/// A `String` where the input text is split into lines, with each line
/// containing up to `chunk` words. Lines are separated by newline characters
/// (`\n`).
///
/// # Example
///
/// ```
///  let text: String = String::from("Test this after you got it work. This message is very long to demonstrate it proper in proper screen or any object without any size overflow (width overflow).");
///  let modified: String = multiline_label(text, 8);
///  let lines: Vec<&str> = modified.split("\n").collect();
///  assert_eq!(lines.len(), 4, "Should be equal 4 line");
/// ```
fn multiline_label(text: String, chunk_size: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut modified_label: String = String::from("");

    for chunk in words.chunks(chunk_size) {
        modified_label.push_str(&chunk.join(" "));
        modified_label.push('\n');
    }

    return modified_label.trim_end().to_owned();
}

/// Draw Specific Diagram
fn draw_diagram(context: &Context, x: f64, y: f64, width: f64, height: f64, style_str: &str) {
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
        _ => panic!("Shape panic: Error at shape rule."),
    }
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_line_width(1.0);
    context.stroke().expect("Failed to draw specific diagram.");
}

/// Draw multiline label with Cairo
fn draw_multiline_label(context: &Context, text: &str, x: f64, y: f64, line_spacing: f64) {
    context.set_source_rgb(0.0, 0.0, 0.0);
    // Initial position for the first line
    let mut current_y = y;

    for line in text.split('\n') {
        context.move_to(x, current_y);
        context
            .show_text(line)
            .expect("Cairo error: failed to draw text");
        current_y += line_spacing;
    }
}

#[cfg(test)]
mod test {
    use super::multiline_label;

    #[test]
    fn four_line_test() {
        let text: String = String::from("Test this after you got it work. This message is very long to demonstrate it proper in proper screen or any object without any size overflow (width overflow).");
        let modified: String = multiline_label(text, 8);
        let lines: Vec<&str> = modified.split('\n').collect();
        assert_eq!(lines.len(), 4, "Should be equal 4 line");
    }

    #[test]
    fn two_line_test() {
        let text: String = String::from("Test this after you got it work. This message is very long to demonstrate it proper in proper screen or any object without any size overflow (width overflow).");
        let modified: String = multiline_label(text, 14);
        let lines: Vec<&str> = modified.split('\n').collect();
        assert_eq!(lines.len(), 2, "Should be equal 2 line");
    }

    #[test]
    fn white_space_test() {
        let text: String = String::from("Test this after you got it work. This message is very long to demonstrate it proper in proper screen or any object without any size overflow (width overflow).");
        let modified: String = multiline_label(text, 8);
        let last_char = &modified.chars().nth(modified.len() - 1);
        assert_ne!(last_char.unwrap(), ' ', "Should not have any whitespace.");
    }

    #[test]
    fn one_line_test() {
        let text: String = String::from("Test this after you got it work. This message is very long to demonstrate it proper in proper screen or any object without any size overflow (width overflow).");
        let modified: String = multiline_label(text, 1);
        let lines: Vec<&str> = modified.split('\n').collect();
        assert_eq!(lines.len(), 28);
    }
}
