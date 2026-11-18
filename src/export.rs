use std::fmt::Write;

use cosmic_text::Color;
use tiny_skia::{
    FillRule, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Rect, Stroke, StrokeDash,
    Transform,
};

use crate::{
    labels::{Fonts, Label},
    layout::{Bounds, Layout, PlacedNode, Point},
    parsing::{Diagram, Edge, Marker, Shape},
    routing::{bar_position, midpoint, PlacedEdge},
};

const INK: (u8, u8, u8) = (30, 33, 41);
const WHITE: (u8, u8, u8) = (255, 255, 255);
const NOTE_FILL: (u8, u8, u8) = (225, 249, 234);
const NOTE_BORDER: (u8, u8, u8) = (55, 122, 88);

pub fn png(
    diagram: &Diagram,
    layout: &mut Layout,
    fonts: &mut Fonts,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut image = Pixmap::new(layout.canvas.width as u32, layout.canvas.height as u32)
        .ok_or("could not allocate the PNG canvas")?;
    image.fill(tiny_skia::Color::WHITE);
    let transform = Transform::from_translate(-layout.canvas.left, -layout.canvas.top);

    for (edge, placed) in diagram.edges.iter().zip(&layout.edges) {
        stroke(
            &mut image,
            &polyline(&placed.points, false),
            INK,
            edge.dashed,
            1.25,
            transform,
        );
        let points = &placed.points;
        if placed.rail.len() > 1 {
            stroke(
                &mut image,
                &polyline(&placed.rail, false),
                INK,
                edge.dashed,
                1.25,
                transform,
            );
        }
        for (_, branch) in &placed.branches {
            stroke(
                &mut image,
                &polyline(branch, false),
                INK,
                edge.dashed,
                1.25,
                transform,
            );
            if edge.arrow_start {
                arrow(&mut image, branch[0], branch[1], transform);
            }
        }
        if edge.arrow_start && placed.branches.is_empty() {
            arrow(&mut image, points[0], points[1], transform);
        }
        if edge.arrow_end {
            arrow(
                &mut image,
                points[points.len() - 1],
                points[points.len() - 2],
                transform,
            );
        }
    }
    for (edge, placed) in diagram.edges.iter().zip(&layout.edges) {
        if let Some(marker) = marker_geometry(edge, placed) {
            let center = marker.bounds.center();
            if marker.circle {
                let circle = oval(marker.bounds);
                fill(
                    &mut image,
                    &circle,
                    if marker.filled { INK } else { WHITE },
                    transform,
                );
                stroke(&mut image, &circle, INK, false, 1.25, transform);
            } else {
                let (dy, dx) = marker.angle.sin_cos();
                let radius = marker.bounds.height / 2.0;
                let line = [
                    center.offset(-dy * radius, dx * radius),
                    center.offset(dy * radius, -dx * radius),
                ];
                stroke(
                    &mut image,
                    &polyline(&line, false),
                    INK,
                    false,
                    marker.bounds.width,
                    transform,
                );
            }
        }
    }
    for node in &mut layout.nodes {
        let path = match node.shape {
            Shape::Action => oval(node.bounds),
            Shape::Cloud => cloud(node),
            _ => polyline(&node.vertices(), true),
        };
        let is_note = node.shape == Shape::Note;
        let (background, border) = if is_note {
            (NOTE_FILL, NOTE_BORDER)
        } else {
            (WHITE, INK)
        };
        fill(&mut image, &path, background, transform);
        stroke(
            &mut image,
            &path,
            border,
            is_note,
            node_stroke(node),
            transform,
        );
        let center = node.bounds.center();
        let x = center.x - node.label.width / 2.0 - layout.canvas.left;
        let y = center.y - node.label.height / 2.0 - layout.canvas.top;
        fonts.draw(
            &mut image,
            &mut node.label,
            x,
            y,
            Color::rgb(INK.0, INK.1, INK.2),
        );
    }
    for edge in &mut layout.edges {
        if let Some(bounds) = edge.label_bounds {
            let rect =
                Rect::from_xywh(bounds.left, bounds.top, bounds.width, bounds.height).unwrap();
            image.fill_rect(rect, &paint(WHITE), transform, None);
            fonts.draw(
                &mut image,
                &mut edge.label,
                bounds.left + 4.0 - layout.canvas.left,
                bounds.top + 2.0 - layout.canvas.top,
                Color::rgb(INK.0, INK.1, INK.2),
            );
        }
    }
    Ok(image.encode_png()?)
}

fn paint(color: (u8, u8, u8)) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.set_color_rgba8(color.0, color.1, color.2, 255);
    paint
}

fn fill(image: &mut Pixmap, path: &Path, color: (u8, u8, u8), transform: Transform) {
    image.fill_path(path, &paint(color), FillRule::Winding, transform, None);
}

fn stroke(
    image: &mut Pixmap,
    path: &Path,
    color: (u8, u8, u8),
    dashed: bool,
    width: f32,
    transform: Transform,
) {
    let stroke = Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        dash: if dashed {
            StrokeDash::new(vec![3.0, 4.0], 0.0)
        } else {
            None
        },
        ..Stroke::default()
    };
    image.stroke_path(path, &paint(color), &stroke, transform, None);
}

fn polyline(points: &[Point], closed: bool) -> Path {
    let mut path = PathBuilder::new();
    path.move_to(points[0].x, points[0].y);
    for point in &points[1..] {
        path.line_to(point.x, point.y);
    }
    if closed {
        path.close();
    }
    path.finish().unwrap()
}

fn oval(bounds: Bounds) -> Path {
    let rect = Rect::from_xywh(bounds.left, bounds.top, bounds.width, bounds.height).unwrap();
    PathBuilder::from_oval(rect).unwrap()
}

fn cloud(node: &PlacedNode) -> Path {
    let (start, curves) = node.cloud();
    let mut path = PathBuilder::new();
    path.move_to(start.x, start.y);
    for (a, b, end) in curves {
        path.cubic_to(a.x, a.y, b.x, b.y, end.x, end.y);
    }
    path.close();
    path.finish().unwrap()
}

fn node_stroke(node: &PlacedNode) -> f32 {
    if node.bold {
        3.1
    } else {
        1.25
    }
}

struct Mark {
    bounds: Bounds,
    angle: f32,
    circle: bool,
    filled: bool,
}

fn marker_geometry(edge: &Edge, placed: &PlacedEdge) -> Option<Mark> {
    match edge.marker? {
        Marker::OpenCircle | Marker::FilledCircle => {
            let center = if placed.branches.is_empty() {
                midpoint(&placed.points)
            } else {
                placed.points[0]
            };
            Some(Mark {
                bounds: Bounds::centered(center, 10.0, 10.0),
                angle: 0.0,
                circle: true,
                filled: edge.marker == Some(Marker::FilledCircle),
            })
        }
        Marker::Bar => {
            let (point, angle) = bar_position(edge, placed);
            Some(Mark {
                bounds: Bounds::centered(point, 1.7, 15.0),
                angle,
                circle: false,
                filled: true,
            })
        }
        Marker::Zigzag => None,
    }
}

fn arrow(image: &mut Pixmap, tip: Point, previous: Point, transform: Transform) {
    let length = tip.distance(previous);
    if length == 0.0 {
        return;
    }
    let base = tip.towards(previous, (9.0 / length).min(0.9));
    let dx = (previous.y - tip.y) / length * 4.0;
    let dy = (tip.x - previous.x) / length * 4.0;
    fill(
        image,
        &polyline(&[tip, base.offset(dx, dy), base.offset(-dx, -dy)], true),
        INK,
        transform,
    );
}

pub fn xml(diagram: &Diagram, layout: &Layout) -> String {
    let mut result = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<mxGraphModel>\n  <root>\n    <mxCell id=\"0\"/>\n    <mxCell id=\"1\" parent=\"0\"/>\n");
    for (index, node) in layout.nodes.iter().enumerate() {
        let shape = match node.shape {
            Shape::Goal => {
                "shape=parallelogram;perimeter=parallelogramPerimeter;fixedSize=1;size=18;"
            }
            Shape::Risk => {
                "shape=parallelogram;perimeter=parallelogramPerimeter;fixedSize=1;size=18;flipH=1;"
            }
            Shape::Agent => "shape=hexagon;perimeter=hexagonPerimeter;fixedSize=1;size=18;",
            Shape::Object => "shape=rectangle;",
            Shape::Action => "shape=ellipse;perimeter=ellipsePerimeter;",
            Shape::Note => {
                "shape=rectangle;dashed=1;dashPattern=3 4;fillColor=#e1f9ea;strokeColor=#377a58;"
            }
            Shape::Diamond => "shape=rhombus;perimeter=rhombusPerimeter;",
            Shape::Descriptive => {
                "shape=trapezoid;perimeter=trapezoidPerimeter;fixedSize=1;size=18;"
            }
            Shape::Cloud => "shape=cloud;",
        };
        let bounds = node.bounds;
        writeln!(result,
            "    <mxCell id=\"node{index}\" value=\"{}\" vertex=\"1\" parent=\"1\" style=\"fillColor=#ffffff;strokeColor=#1e2129;fontColor=#1e2129;html=0;align=center;verticalAlign=middle;fontSize={};strokeWidth={};{shape}\">",
            escape_xml(&node.label.text), node.label.font_size, node_stroke(node)).unwrap();
        writeln!(result,
            "      <mxGeometry x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" as=\"geometry\"/>\n    </mxCell>",
            bounds.left - layout.canvas.left, bounds.top - layout.canvas.top, bounds.width, bounds.height).unwrap();
    }
    for (index, (edge, placed)) in diagram.edges.iter().zip(&layout.edges).enumerate() {
        let group = !placed.branches.is_empty();
        let (source_name, source_bounds) = if group {
            write_xml_branches(&mut result, layout, edge, placed, index)
        } else {
            let source = edge.sources[0];
            (format!("node{source}"), layout.nodes[source].bounds)
        };
        write_xml_link(
            &mut result,
            layout.canvas,
            edge.dashed,
            XmlLink {
                id: &format!("edge{index}"),
                source: (&source_name, source_bounds),
                target: (
                    &format!("node{}", edge.target),
                    layout.nodes[edge.target].bounds,
                ),
                points: &placed.points,
                arrows: (edge.arrow_start && !group, edge.arrow_end),
                label: placed.label_bounds.map(|bounds| (&placed.label, bounds)),
            },
        );
        if let Some(marker) = marker_geometry(edge, placed) {
            let middle = midpoint(&placed.points);
            let (shape, border) = if marker.circle {
                ("ellipse", "#1e2129")
            } else {
                ("rectangle", "none")
            };
            let fill = if marker.filled { "#1e2129" } else { "#ffffff" };
            writeln!(result,
                "    <mxCell id=\"marker{index}\" parent=\"edge{index}\" vertex=\"1\" connectable=\"0\" style=\"shape={shape};strokeColor={border};fillColor={fill};rotation={:.2};\">\n      <mxGeometry width=\"{:.2}\" height=\"{:.2}\" relative=\"1\" as=\"geometry\"><mxPoint x=\"{:.2}\" y=\"{:.2}\" as=\"offset\"/></mxGeometry>\n    </mxCell>",
                marker.angle.to_degrees(), marker.bounds.width, marker.bounds.height, marker.bounds.left - middle.x, marker.bounds.top - middle.y).unwrap();
        }
    }
    result.push_str("  </root>\n</mxGraphModel>\n");
    result
}

fn write_xml_branches(
    result: &mut String,
    layout: &Layout,
    edge: &Edge,
    placed: &PlacedEdge,
    index: usize,
) -> (String, Bounds) {
    let junctions = placed.rail.iter().enumerate().map(|(segment, &point)| {
        let id = format!("junction{index}_{segment}");
        let bounds = Bounds::centered(point, 1.0, 1.0);
        writeln!(result,
            "    <mxCell id=\"{id}\" vertex=\"1\" parent=\"1\" style=\"opacity=0;\"><mxGeometry x=\"{:.2}\" y=\"{:.2}\" width=\"1\" height=\"1\" as=\"geometry\"/></mxCell>",
            bounds.left - layout.canvas.left, bounds.top - layout.canvas.top).unwrap();
        (id, bounds)
    }).collect::<Vec<_>>();
    let terminal_at = |point| placed.rail.iter().position(|&p| p == point).unwrap();
    let trunk = terminal_at(placed.points[0]);
    for segment in 0..placed.rail.len().saturating_sub(1) {
        let (from, to) = if segment < trunk {
            (segment, segment + 1)
        } else {
            (segment + 1, segment)
        };
        write_xml_link(
            result,
            layout.canvas,
            edge.dashed,
            XmlLink {
                id: &format!("edge{index}rail{segment}"),
                source: (&junctions[from].0, junctions[from].1),
                target: (&junctions[to].0, junctions[to].1),
                points: &[placed.rail[from], placed.rail[to]],
                arrows: (false, false),
                label: None,
            },
        );
    }
    for (arm, (source, points)) in placed.branches.iter().enumerate() {
        let (target, bounds) = &junctions[terminal_at(*points.last().unwrap())];
        write_xml_link(
            result,
            layout.canvas,
            edge.dashed,
            XmlLink {
                id: &format!("edge{index}arm{arm}"),
                source: (&format!("node{source}"), layout.nodes[*source].bounds),
                target: (target, *bounds),
                points,
                arrows: (edge.arrow_start, false),
                label: None,
            },
        );
    }
    junctions.into_iter().nth(trunk).unwrap()
}

struct XmlLink<'a> {
    id: &'a str,
    source: (&'a str, Bounds),
    target: (&'a str, Bounds),
    points: &'a [Point],
    arrows: (bool, bool),
    label: Option<(&'a Label, Bounds)>,
}

fn write_xml_link(result: &mut String, canvas: Bounds, dashed: bool, link: XmlLink<'_>) {
    let (source_id, source) = link.source;
    let (target_id, target) = link.target;
    let [start, end] = [link.points[0], *link.points.last().unwrap()];
    let arrow = |enabled| if enabled { "block" } else { "none" };
    let dash = if dashed {
        "dashed=1;dashPattern=3 4;"
    } else {
        ""
    };
    let (value, font_size) = link.label.map_or((String::new(), 12.0), |(label, _)| {
        (escape_xml(&label.text), label.font_size)
    });
    writeln!(result,
        "    <mxCell id=\"{}\" value=\"{value}\" edge=\"1\" parent=\"1\" source=\"{source_id}\" target=\"{target_id}\" style=\"noEdgeStyle=1;html=0;fontColor=#1e2129;fontSize={font_size};labelBackgroundColor=#ffffff;strokeColor=#1e2129;startArrow={};endArrow={};{dash}exitX={:.4};exitY={:.4};entryX={:.4};entryY={:.4};exitPerimeter=0;entryPerimeter=0;\">",
        link.id, arrow(link.arrows.0), arrow(link.arrows.1),
        (start.x - source.left) / source.width, (start.y - source.top) / source.height,
        (end.x - target.left) / target.width, (end.y - target.top) / target.height).unwrap();
    result.push_str("      <mxGeometry relative=\"1\" as=\"geometry\">\n");
    let waypoints = &link.points[1..link.points.len() - 1];
    if !waypoints.is_empty() {
        result.push_str("        <Array as=\"points\">\n");
        for point in waypoints {
            writeln!(
                result,
                "          <mxPoint x=\"{:.2}\" y=\"{:.2}\"/>",
                point.x - canvas.left,
                point.y - canvas.top
            )
            .unwrap();
        }
        result.push_str("        </Array>\n");
    }
    if let Some((_, bounds)) = link.label {
        let middle = midpoint(link.points);
        let center = bounds.center();
        writeln!(
            result,
            "        <mxPoint x=\"{:.2}\" y=\"{:.2}\" as=\"offset\"/>",
            center.x - middle.x,
            center.y - middle.y
        )
        .unwrap();
    }
    result.push_str("      </mxGeometry>\n    </mxCell>\n");
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('\n', "&#10;")
}
