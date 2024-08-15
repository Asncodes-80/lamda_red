extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate cairo;

use cairo::{Context, Format, ImageSurface};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct MXGraphModel {
    root: Root,
}
#[derive(Debug, Deserialize)]
struct Root {
    mxCell: Vec<MxCell>,
}

#[derive(Debug, Deserialize)]
struct MxCell {
    id: Option<String>,
    value: Option<String>,
    style: Option<String>,
    vertex: Option<String>,
    mxGeometry: Option<MxGeometry>,
}

#[derive(Debug, Deserialize)]
struct MxGeometry {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
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
    // TODO: Make it dynamic
    let surface = ImageSurface::create(Format::ARgb32, 800, 600).expect("Can't create page frame.");
    let context = Context::new(&surface)
        .unwrap_or_else(|e| panic!("Can't get instance of page surface.\n{}", e));

    // Set the background color.
    context.set_source_rgb(1.0, 1.0, 1.0);
    context
        .paint()
        .expect("Failed to paint the background color");

    // Loop through mxCell tags `<mxCell><some_mx_geometry /></mxCell>`.
    for cell in mx_graph_model.root.mxCell {
        if let Some(vertex) = cell.vertex {
            if vertex == "1" {
                // MXGeometry settings like `x` and `y` coordinations or `width` and `height` size.
                if let Some(geometry) = cell.mxGeometry {
                    // Set red color for the objects.
                    context.set_source_rgb(1.0, 0.0, 0.0);
                    context.rectangle(
                        geometry.x.unwrap_or(0.0),
                        geometry.y.unwrap_or(0.0),
                        geometry.width.unwrap_or(100.0),
                        geometry.height.unwrap_or(50.0),
                    );
                    context.stroke().expect("Failed to draw rectangle.");

                    if let Some(value) = cell.value {
                        context.move_to(
                            geometry.x.unwrap_or(0.0) + 10.0,
                            geometry.y.unwrap_or(0.0) + 50.0,
                        );
                        context.set_font_size(14.0);
                        context.show_text(&value).expect("Failed to draw text");
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

/// Parse Style
///
/// ```xml
/// <mxCell
///     style='shape={SHAPE};perimeter={PERIMETER};whiteSpace=wrap;html=1;fixedSize=1;flipH={FLIP_H}'
/// />
/// ```
fn parse_style(style: &str) -> std::collections::HashMap<String, String> {
    style
        .split(";")
        .filter_map(|item| {
            let mut parts = item.split("=");
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                Some((key.to_string(), value.to_string()))
            } else {
                None
            }
        })
        .collect()
}
