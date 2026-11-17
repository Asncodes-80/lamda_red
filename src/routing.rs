use std::collections::HashMap;

use crate::{
    labels::{Fonts, Label},
    layout::{Bounds, PlacedNode, Point},
    parsing::{Diagram, Direction, Edge, Marker},
};

pub struct PlacedEdge {
    pub points: Vec<Point>,
    pub branches: Vec<Branch>,
    pub label: Label,
    pub label_bounds: Option<Bounds>,
}

impl PlacedEdge {
    pub fn paths(&self) -> impl Iterator<Item = &[Point]> {
        self.branches
            .iter()
            .map(|(_, path)| path.as_slice())
            .chain([self.points.as_slice()])
    }
}

pub fn route_edges(diagram: &Diagram, nodes: &[PlacedNode], fonts: &mut Fonts) -> Vec<PlacedEdge> {
    let edges = &diagram.edges;
    let key = |edge: &Edge| {
        (
            edge.sources[0].min(edge.target),
            edge.sources[0].max(edge.target),
        )
    };
    let mut counts = HashMap::new();
    for edge in edges {
        *counts.entry(key(edge)).or_insert(0) += 1;
    }
    let mut seen = HashMap::new();
    let mut side_use = HashMap::new();
    let mut lanes = Vec::with_capacity(edges.len());
    let mut placed = edges
        .iter()
        .map(|edge| {
            let pair = key(edge);
            let index = seen.entry(pair).or_insert(0);
            let count = counts[&pair];
            let lane = (*index as f32 - (count - 1) as f32 / 2.0)
                * (18.0_f32.min(26.0 / (count - 1).max(1) as f32));
            let side_index = edge.via.map_or(0, |side| {
                let index = side_use.entry(side).or_insert(0);
                *index += 1;
                *index - 1
            });
            let (mut points, branches) = if edge.sources.len() > 1 {
                branch(edge, nodes, diagram.direction, side_index)
            } else if edge.sources[0] == edge.target {
                (self_loop(edge, nodes, *index, side_index), Vec::new())
            } else {
                (
                    route(edge, nodes, lane, diagram.direction, side_index),
                    Vec::new(),
                )
            };
            if edge.marker == Some(Marker::Zigzag) {
                insert_zigzag(&mut points);
            }
            *index += 1;
            lanes.push(lane);
            PlacedEdge {
                points,
                branches,
                label: fonts.layout(&edge.label, 12.0, 165.0, 3),
                label_bounds: None,
            }
        })
        .collect::<Vec<_>>();
    for (index, lane) in lanes.into_iter().enumerate() {
        placed[index].label_bounds = place_label(index, &placed, nodes, lane);
    }
    placed
}

type Branch = (usize, Vec<Point>);

fn branch(
    edge: &Edge,
    nodes: &[PlacedNode],
    direction: Direction,
    side_index: usize,
) -> (Vec<Point>, Vec<Branch>) {
    let target = &nodes[edge.target];
    let aim = target.bounds.center();
    let vertical = edge.via.unwrap_or(direction).is_vertical();
    let axis = |point: Point| if vertical { point.y } else { point.x };
    let align = |point: Point, value| {
        if vertical {
            Point::new(point.x, value)
        } else {
            Point::new(value, point.y)
        }
    };
    let source_centers = edge
        .sources
        .iter()
        .map(|&index| nodes[index].bounds.center());
    let source_middle = source_centers.map(axis).sum::<f32>() / edge.sources.len() as f32;
    let source_is_before = source_middle < axis(aim);
    let face = |bounds: Bounds, after| match (vertical, after) {
        (true, true) => bounds.bottom(),
        (true, false) => bounds.top,
        (false, true) => bounds.right(),
        (false, false) => bounds.left,
    };
    let source_face = edge
        .sources
        .iter()
        .map(|&index| face(nodes[index].bounds, source_is_before))
        .fold(source_middle, |end, next| {
            if source_is_before {
                end.max(next)
            } else {
                end.min(next)
            }
        });
    let target_face = face(target.bounds, !source_is_before);
    let has_gap = if source_is_before {
        source_face + 16.0 < target_face
    } else {
        source_face - 16.0 > target_face
    };
    let rail = if let Some(side) = edge.via {
        outside(side, nodes, side_index)
    } else if has_gap {
        source_face * 0.5 + target_face * 0.5
    } else if vertical && source_is_before {
        outside(Direction::Up, nodes, 0)
    } else if vertical {
        outside(Direction::Down, nodes, 0)
    } else if source_is_before {
        outside(Direction::Left, nodes, 0)
    } else {
        outside(Direction::Right, nodes, 0)
    };
    let junction = align(aim, rail);
    let trunk = vec![junction, target.boundary(aim, junction)];
    let branches = edge
        .sources
        .iter()
        .map(|&index| {
            let node = &nodes[index];
            let center = node.bounds.center();
            let bend = align(center, rail);
            let mut points = vec![node.boundary(center, bend), bend, junction];
            points.dedup();
            (index, points)
        })
        .collect();
    (trunk, branches)
}

fn self_loop(edge: &Edge, nodes: &[PlacedNode], index: usize, side_index: usize) -> Vec<Point> {
    let node = &nodes[edge.target];
    let center = node.bounds.center();
    let reach = 24.0 + index as f32 * 12.0;
    let (mut x, mut y) = (node.bounds.right() + reach, node.bounds.top - reach);
    if let Some(side) = edge.via {
        let coordinate = outside(side, nodes, side_index);
        if side.is_vertical() {
            y = coordinate;
        } else {
            x = coordinate;
        }
    }
    vec![
        node.boundary(center, Point::new(x, center.y)),
        Point::new(x, center.y),
        Point::new(x, y),
        Point::new(center.x, y),
        node.boundary(center, Point::new(center.x, y)),
    ]
}

fn route(
    edge: &Edge,
    nodes: &[PlacedNode],
    lane: f32,
    direction: Direction,
    side_index: usize,
) -> Vec<Point> {
    let source_index = edge.sources[0];
    let source = &nodes[source_index];
    let target = &nodes[edge.target];
    let a_center = source.bounds.center();
    let b_center = target.bounds.center();
    let dx = b_center.x - a_center.x;
    let dy = b_center.y - a_center.y;
    let orthogonal = edge.orthogonal || direction.is_vertical();
    let vertical = if let Some(side) = edge.via {
        side.is_vertical()
    } else if orthogonal {
        dy.abs() > 0.01 && (direction.is_vertical() || dx.abs() < 0.01)
    } else {
        dx.abs() < dy.abs()
    };
    let spread = |distance: f32| {
        if orthogonal && distance.abs() > 0.01 {
            distance.signum() * 9.0
        } else {
            0.0
        }
    };
    let shift = |offset: f32, size: f32| offset.clamp(-size * 0.38, size * 0.38);
    let (a, b) = if vertical {
        (
            a_center.offset(shift(lane + spread(dx), source.bounds.width), 0.0),
            b_center.offset(shift(lane - spread(dx), target.bounds.width), 0.0),
        )
    } else {
        (
            a_center.offset(0.0, shift(lane + spread(dy), source.bounds.height)),
            b_center.offset(0.0, shift(lane - spread(dy), target.bounds.height)),
        )
    };
    let obstacles = nodes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != source_index && *i != edge.target)
        .map(|(_, node)| node.bounds.padded(8.0))
        .collect::<Vec<_>>();

    let mut best = (f32::INFINITY, Vec::new());
    let consider = |mut points: Vec<Point>, best: &mut (f32, Vec<Point>)| {
        points.dedup();
        points[0] = source.boundary(a, points[1]);
        let last = points.len() - 1;
        points[last] = target.boundary(b, points[last - 1]);
        let bends = points.len().saturating_sub(2) as f32;
        let mut score = bends * 12.0;
        for segment in points.windows(2) {
            score += segment[0].distance(segment[1]);
            score += 1_000_000.0
                * obstacles
                    .iter()
                    .filter(|rect| rect.crosses(segment[0], segment[1]))
                    .count() as f32;
        }
        if score < best.0 {
            *best = (score, points);
        }
    };

    if edge.via.is_none() && (!orthogonal || a.x == b.x || a.y == b.y) {
        consider(vec![a, b], &mut best);
        if best.0 < 1_000_000.0 {
            return best.1;
        }
    }
    let middle = a.towards(b, 0.5);
    let x_band = |bounds: Bounds| (bounds.left, bounds.center().x, bounds.right());
    let y_band = |bounds: Bounds| (bounds.top, bounds.center().y, bounds.bottom());
    let source_cols = channels(a_center.x, nodes, x_band, lane);
    let target_cols = channels(b_center.x, nodes, x_band, lane);
    let source_rows = channels(a_center.y, nodes, y_band, lane);
    let target_rows = channels(b_center.y, nodes, y_band, lane);
    if let Some(side) = edge.via {
        let outer = outside(side, nodes, side_index);
        if side.is_vertical() {
            if (a.x - b.x).abs() > 0.01 && a_center.x != b_center.x {
                consider(
                    vec![a, Point::new(a.x, outer), Point::new(b.x, outer), b],
                    &mut best,
                );
            }
            for x1 in source_cols {
                for x2 in target_cols {
                    if x1 != x2 {
                        consider(
                            vec![
                                a,
                                Point::new(x1, a.y),
                                Point::new(x1, outer),
                                Point::new(x2, outer),
                                Point::new(x2, b.y),
                                b,
                            ],
                            &mut best,
                        );
                    }
                }
            }
        } else {
            if (a.y - b.y).abs() > 0.01 && a_center.y != b_center.y {
                consider(
                    vec![a, Point::new(outer, a.y), Point::new(outer, b.y), b],
                    &mut best,
                );
            }
            for y1 in source_rows {
                for y2 in target_rows {
                    if y1 != y2 {
                        consider(
                            vec![
                                a,
                                Point::new(a.x, y1),
                                Point::new(outer, y1),
                                Point::new(outer, y2),
                                Point::new(b.x, y2),
                                b,
                            ],
                            &mut best,
                        );
                    }
                }
            }
        }
        return best.1;
    }
    if orthogonal
        && vertical
        && (middle.y < source.bounds.top || middle.y > source.bounds.bottom())
        && (middle.y < target.bounds.top || middle.y > target.bounds.bottom())
    {
        consider(
            vec![a, Point::new(a.x, middle.y), Point::new(b.x, middle.y), b],
            &mut best,
        );
        if best.0 < 1_000_000.0 {
            return best.1;
        }
    }
    let rows = source_rows.into_iter().chain(target_rows);
    for x in [middle.x].into_iter().chain(source_cols).chain(target_cols) {
        if (x < source.bounds.left || x > source.bounds.right())
            && (x < target.bounds.left || x > target.bounds.right())
        {
            consider(
                vec![a, Point::new(x, a.y), Point::new(x, b.y), b],
                &mut best,
            );
        }
    }
    for y in [middle.y].into_iter().chain(rows.clone()) {
        if (y < source.bounds.top || y > source.bounds.bottom())
            && (y < target.bounds.top || y > target.bounds.bottom())
        {
            consider(
                vec![a, Point::new(a.x, y), Point::new(b.x, y), b],
                &mut best,
            );
        }
    }
    if best.0 >= 1_000_000.0 {
        // A path may need to change columns in a free row between the endpoints.
        for y in rows {
            for x1 in source_cols {
                for x2 in target_cols {
                    consider(
                        vec![
                            a,
                            Point::new(x1, a.y),
                            Point::new(x1, y),
                            Point::new(x2, y),
                            Point::new(x2, b.y),
                            b,
                        ],
                        &mut best,
                    );
                }
            }
        }
    }
    best.1
}

fn outside(side: Direction, nodes: &[PlacedNode], index: usize) -> f32 {
    let distance = 34.0 + 17.0 * index as f32;
    match side {
        Direction::Right => {
            nodes
                .iter()
                .map(|node| node.bounds.right())
                .fold(0.0, f32::max)
                + distance
        }
        Direction::Down => {
            nodes
                .iter()
                .map(|node| node.bounds.bottom())
                .fold(0.0, f32::max)
                + distance
        }
        Direction::Left => {
            nodes
                .iter()
                .map(|node| node.bounds.left)
                .fold(0.0, f32::min)
                - distance
        }
        Direction::Up => nodes.iter().map(|node| node.bounds.top).fold(0.0, f32::min) - distance,
    }
}

fn channels(
    center: f32,
    nodes: &[PlacedNode],
    band: impl Fn(Bounds) -> (f32, f32, f32),
    lane: f32,
) -> [f32; 2] {
    let mut min = center;
    let mut max = center;
    for (start, mid, end) in nodes.iter().map(|node| band(node.bounds)) {
        if (mid - center).abs() < 0.01 {
            min = min.min(start);
            max = max.max(end);
        }
    }
    [min - 22.0 + lane, max + 22.0 + lane]
}

fn place_label(
    index: usize,
    edges: &[PlacedEdge],
    nodes: &[PlacedNode],
    lane: f32,
) -> Option<Bounds> {
    let edge = &edges[index];
    if edge.label.text.trim().is_empty() {
        return None;
    }
    let mut best = None;
    let mut best_score = f32::INFINITY;
    let middle = midpoint(&edge.points);
    let width = edge.label.width + 8.0;
    let height = edge.label.height + 4.0;
    let preferred_side = if lane > 0.0 { 1.0 } else { -1.0 };
    for segment in edge.points.windows(2) {
        let [a, b] = [segment[0], segment[1]];
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let length = dx.hypot(dy);
        if length == 0.0 {
            continue;
        }
        // Orient the normal towards the bottom or right so parallel labels
        // choose opposite sides even when the edges run in opposite directions.
        let direction = if dx.abs() >= dy.abs() {
            dx.signum()
        } else {
            -dy.signum()
        };
        let nx = -dy / length * direction;
        let ny = dx / length * direction;
        let half_extent = (nx.abs() * width + ny.abs() * height) / 2.0;
        for fraction in [0.5, 0.25, 0.75] {
            let anchor = a.towards(b, fraction);
            for side in [preferred_side, -preferred_side] {
                for gap in [5.0, 15.0, 27.0] {
                    let offset = side * (half_extent + gap);
                    let center = anchor.offset(nx * offset, ny * offset);
                    let bounds = Bounds::centered(center, width, height);
                    let mut score = gap + anchor.distance(middle) * 0.30;
                    if side != preferred_side {
                        score += 3.0;
                    }
                    score += 10_000.0
                        * nodes
                            .iter()
                            .filter(|node| bounds.overlaps(node.bounds.padded(4.0)))
                            .count() as f32;
                    score += 5_000.0
                        * edges
                            .iter()
                            .filter_map(|edge| edge.label_bounds)
                            .filter(|other| bounds.overlaps(*other))
                            .count() as f32;
                    score += 120.0
                        * edges
                            .iter()
                            .filter(|edge| {
                                edge.paths().any(|path| {
                                    path.windows(2)
                                        .any(|segment| bounds.crosses(segment[0], segment[1]))
                                })
                            })
                            .count() as f32;
                    if score < best_score {
                        best_score = score;
                        best = Some(bounds);
                    }
                }
            }
        }
    }
    best
}

fn path_length(points: &[Point]) -> f32 {
    points
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum()
}

fn sample_path(points: &[Point], mut remaining: f32) -> (Point, f32) {
    for pair in points.windows(2) {
        let [a, b] = [pair[0], pair[1]];
        let length = a.distance(b);
        if length > 0.0 && remaining <= length {
            return (
                a.towards(b, remaining / length),
                (b.y - a.y).atan2(b.x - a.x),
            );
        }
        remaining -= length;
    }
    (*points.last().unwrap(), 0.0)
}

pub fn midpoint(points: &[Point]) -> Point {
    sample_path(points, path_length(points) / 2.0).0
}

pub fn bar_position(edge: &Edge, placed: &PlacedEdge) -> (Point, f32) {
    let length = path_length(&placed.points);
    let inset = 19.0_f32.min(length / 2.0);
    let distance = match (edge.arrow_start, edge.arrow_end) {
        (false, true) => length - inset,
        (true, false) => inset,
        _ => length / 2.0,
    };
    sample_path(&placed.points, distance)
}

fn insert_zigzag(points: &mut Vec<Point>) {
    let (index, segment) = points
        .windows(2)
        .enumerate()
        .max_by(|(_, a), (_, b)| a[0].distance(a[1]).total_cmp(&b[0].distance(b[1])))
        .unwrap();
    let [a, b] = [segment[0], segment[1]];
    let length = a.distance(b);
    let half_span = 10.0_f32.min(length * 0.35);
    let dx = (b.x - a.x) / length;
    let dy = (b.y - a.y) / length;
    let center = a.towards(b, 0.5);
    let point = |along, off| center.offset(dx * along - dy * off, dy * along + dx * off);
    points.splice(
        index + 1..index + 1,
        [
            point(-half_span, 0.0),
            point(-half_span * 0.37, 6.0),
            point(half_span * 0.37, -6.0),
            point(half_span, 0.0),
        ],
    );
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::{labels::Fonts, layout::arrange, parsing::parse};

    #[test]
    fn routes_around_nodes_in_a_full_grid_and_separates_reverse_edges() {
        let mut source = String::new();
        for row in 0..4 {
            for col in 0..4 {
                writeln!(source, "N{col}{row} @{col},{row} = [Node {col},{row}]").unwrap();
            }
        }
        source.push_str("N11 __> N33 : Forward\nN11 <-- N33 : Return");
        let diagram = parse(&source).unwrap();
        let layout = arrange(&diagram, &mut Fonts::new().unwrap()).unwrap();
        for (edge, path) in diagram.edges.iter().zip(&layout.edges) {
            assert!(path.points.len() > 2);
            for (index, node) in layout.nodes.iter().enumerate() {
                if index != edge.sources[0] && index != edge.target {
                    assert!(
                        path.points
                            .windows(2)
                            .all(|segment| !node.bounds.crosses(segment[0], segment[1])),
                        "route enters node {index}",
                    );
                }
            }
            let label = path.label_bounds.unwrap();
            assert!(path
                .points
                .windows(2)
                .all(|segment| !label.crosses(segment[0], segment[1])));
        }
        assert_ne!(layout.edges[0].points, layout.edges[1].points);
    }

    #[test]
    fn shares_the_junction_and_keeps_obstructions_on_outer_routes() {
        let diagram = parse(include_str!("../sample/de-morgan.zz")).unwrap();
        let layout = arrange(&diagram, &mut Fonts::new().unwrap()).unwrap();
        let branch = &layout.edges[0];
        assert_eq!(branch.branches.len(), 2);
        for (_, path) in &branch.branches {
            assert_eq!(path.last(), branch.points.first());
            assert!(path
                .windows(2)
                .all(|pair| pair[0].x == pair[1].x || pair[0].y == pair[1].y));
        }
        assert!(branch.points[1].y < branch.points[0].y);
        let bottom = layout
            .nodes
            .iter()
            .map(|node| node.bounds.bottom())
            .fold(0.0, f32::max);
        for placed in &layout.edges[4..] {
            assert!(placed.points.iter().any(|point| point.y > bottom));
        }
    }
}
