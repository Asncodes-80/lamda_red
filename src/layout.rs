use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::{
    labels::{Fonts, Label},
    parsing::{Diagram, Direction, Position, Shape},
    routing::{route_edges, PlacedEdge},
};

const MARGIN: f32 = 26.0;
const COLUMN_GAP: f32 = 124.0;
const ROW_GAP: f32 = 72.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn offset(self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn towards(self, other: Self, fraction: f32) -> Self {
        self.offset((other.x - self.x) * fraction, (other.y - self.y) * fraction)
    }

    pub fn distance(self, other: Self) -> f32 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn centered(center: Point, width: f32, height: f32) -> Self {
        Self {
            left: center.x - width / 2.0,
            top: center.y - height / 2.0,
            width,
            height,
        }
    }

    pub fn center(self) -> Point {
        Point::new(self.left + self.width / 2.0, self.top + self.height / 2.0)
    }

    pub fn right(self) -> f32 {
        self.left + self.width
    }

    pub fn bottom(self) -> f32 {
        self.top + self.height
    }

    pub fn padded(self, padding: f32) -> Self {
        Self::centered(
            self.center(),
            self.width + 2.0 * padding,
            self.height + 2.0 * padding,
        )
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.left < other.right()
            && self.right() > other.left
            && self.top < other.bottom()
            && self.bottom() > other.top
    }

    pub fn crosses(self, start: Point, end: Point) -> bool {
        let mut enter: f32 = 0.0;
        let mut exit: f32 = 1.0;
        for (from, to, min, max) in [
            (start.x, end.x, self.left, self.right()),
            (start.y, end.y, self.top, self.bottom()),
        ] {
            let direction = to - from;
            if direction.abs() < f32::EPSILON {
                if from < min || from > max {
                    return false;
                }
                continue;
            }
            let a = (min - from) / direction;
            let b = (max - from) / direction;
            enter = enter.max(a.min(b));
            exit = exit.min(a.max(b));
            if enter > exit {
                return false;
            }
        }
        true
    }
}

pub struct PlacedNode {
    pub shape: Shape,
    pub bold: bool,
    pub bounds: Bounds,
    pub label: Label,
}

impl PlacedNode {
    fn new(shape: Shape, bold: bool, label: Label) -> Self {
        let width = label.width + 24.0;
        let height = label.height + 20.0;
        let (width, height) = match shape {
            Shape::Goal | Shape::Risk | Shape::Agent | Shape::Descriptive => (width + 36.0, height),
            Shape::Action => (width * 1.42, height * 1.42),
            Shape::Diamond => (width * 2.0, height * 2.0),
            Shape::Cloud => (width * 1.52, height * 1.65),
            Shape::Object | Shape::Note => (width, height),
        };
        let bounds = Bounds::centered(Point::new(0.0, 0.0), width.max(88.0), height.max(42.0));
        Self {
            shape,
            bold,
            bounds,
            label,
        }
    }

    pub fn vertices(&self) -> Vec<Point> {
        let Bounds {
            left: x,
            top: y,
            width: w,
            height: h,
        } = self.bounds;
        let relative: &[(f32, f32)] = match self.shape {
            Shape::Goal => &[(18.0, 0.0), (w, 0.0), (w - 18.0, h), (0.0, h)],
            Shape::Risk => &[(0.0, 0.0), (w - 18.0, 0.0), (w, h), (18.0, h)],
            Shape::Descriptive => &[(18.0, 0.0), (w - 18.0, 0.0), (w, h), (0.0, h)],
            Shape::Agent => &[
                (18.0, 0.0),
                (w - 18.0, 0.0),
                (w, h / 2.0),
                (w - 18.0, h),
                (18.0, h),
                (0.0, h / 2.0),
            ],
            Shape::Diamond => &[(w / 2.0, 0.0), (w, h / 2.0), (w / 2.0, h), (0.0, h / 2.0)],
            Shape::Object | Shape::Note => &[(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)],
            Shape::Action => return Vec::new(),
            Shape::Cloud => {
                let (mut start, curves) = self.cloud();
                let mut points = Vec::new();
                for (c1, c2, end) in curves {
                    for step in 0..8 {
                        let t = step as f32 / 8.0;
                        let a = start.towards(c1, t);
                        let b = c1.towards(c2, t);
                        let c = c2.towards(end, t);
                        points.push(a.towards(b, t).towards(b.towards(c, t), t));
                    }
                    start = end;
                }
                return points;
            }
        };
        relative
            .iter()
            .map(|&(dx, dy)| Point::new(x + dx, y + dy))
            .collect()
    }

    pub fn cloud(&self) -> (Point, [(Point, Point, Point); 8]) {
        let point = |(x, y)| {
            Point::new(
                self.bounds.left + x * self.bounds.width,
                self.bounds.top + y * self.bounds.height,
            )
        };
        let start = (0.055, 0.55);
        let curves = [
            ((-0.03, 0.36), (0.04, 0.22), (0.19, 0.23)),
            ((0.21, 0.02), (0.39, -0.04), (0.50, 0.16)),
            ((0.61, 0.03), (0.80, 0.08), (0.83, 0.25)),
            ((0.98, 0.23), (1.03, 0.42), (0.94, 0.52)),
            ((1.05, 0.64), (0.98, 0.84), (0.83, 0.81)),
            ((0.76, 1.01), (0.59, 1.02), (0.48, 0.89)),
            ((0.38, 1.02), (0.18, 0.96), (0.16, 0.80)),
            ((0.02, 0.83), (-0.02, 0.64), start),
        ];
        (
            point(start),
            curves.map(|(a, b, end)| (point(a), point(b), point(end))),
        )
    }

    pub fn boundary(&self, from: Point, towards: Point) -> Point {
        let dx = towards.x - from.x;
        let dy = towards.y - from.y;
        if self.shape == Shape::Action {
            let center = self.bounds.center();
            let rx = self.bounds.width / 2.0;
            let ry = self.bounds.height / 2.0;
            let x = (from.x - center.x) / rx;
            let y = (from.y - center.y) / ry;
            let a = (dx / rx).powi(2) + (dy / ry).powi(2);
            let b = x * dx / rx + y * dy / ry;
            let c = x * x + y * y - 1.0;
            return from.towards(towards, (-b + (b * b - a * c).max(0.0).sqrt()) / a);
        }
        let vertices = self.vertices();
        let t = (0..vertices.len())
            .filter_map(|index| {
                let a = vertices[index];
                let b = vertices[(index + 1) % vertices.len()];
                let sx = b.x - a.x;
                let sy = b.y - a.y;
                let det = dx * sy - dy * sx;
                if det.abs() < f32::EPSILON {
                    return None;
                }
                let ax = a.x - from.x;
                let ay = a.y - from.y;
                let t = (ax * sy - ay * sx) / det;
                let u = (ax * dy - ay * dx) / det;
                (t >= 0.0 && (-0.0001..=1.0001).contains(&u)).then_some(t)
            })
            .fold(f32::INFINITY, f32::min);
        from.towards(towards, t)
    }
}

pub struct Layout {
    pub nodes: Vec<PlacedNode>,
    pub edges: Vec<PlacedEdge>,
    pub canvas: Bounds,
}

pub fn arrange(diagram: &Diagram, fonts: &mut Fonts) -> Result<Layout, &'static str> {
    let mut nodes = diagram
        .nodes
        .iter()
        .map(|node| {
            let max_width = match node.shape {
                Shape::Note => 248.0,
                Shape::Diamond => 114.0,
                _ => 180.0,
            };
            let size = if node.shape == Shape::Note {
                12.0
            } else {
                15.0
            };
            PlacedNode::new(
                node.shape,
                node.bold,
                fonts.layout(&node.label, size, max_width, 4),
            )
        })
        .collect::<Vec<_>>();

    let slots = grid_positions(diagram);
    let columns = grid_centers(
        slots
            .iter()
            .zip(&nodes)
            .map(|(&(col, _), node)| (col, node.bounds.width)),
        if diagram.direction.is_vertical() {
            62.0
        } else {
            COLUMN_GAP
        },
    );
    let rows = grid_centers(
        slots
            .iter()
            .zip(&nodes)
            .map(|(&(_, row), node)| (row, node.bounds.height)),
        ROW_GAP,
    );
    for (node, (col, row)) in nodes.iter_mut().zip(slots) {
        node.bounds = Bounds::centered(
            Point::new(columns[&col], rows[&row]),
            node.bounds.width,
            node.bounds.height,
        );
    }
    check_size(canvas_bounds(&nodes, &[]))?;
    let edges = route_edges(diagram, &nodes, fonts);
    let canvas = canvas_bounds(&nodes, &edges);
    check_size(canvas)?;
    Ok(Layout {
        nodes,
        edges,
        canvas,
    })
}

fn grid_positions(diagram: &Diagram) -> Vec<Position> {
    let mut occupied = diagram
        .nodes
        .iter()
        .filter_map(|node| node.position)
        .collect::<HashSet<_>>();
    let layers = layers(diagram);
    let max_rank = layers.iter().copied().max().unwrap_or(0);
    diagram
        .nodes
        .iter()
        .zip(layers)
        .map(|(node, rank)| {
            node.position.unwrap_or_else(|| {
                let cell = |slot| match diagram.direction {
                    Direction::Right => (rank, slot),
                    Direction::Left => (max_rank - rank, slot),
                    Direction::Down => (slot, rank),
                    Direction::Up => (slot, max_rank - rank),
                };
                (0..).map(cell).find(|cell| occupied.insert(*cell)).unwrap()
            })
        })
        .collect()
}

fn layers(diagram: &Diagram) -> Vec<usize> {
    let count = diagram.nodes.len();
    let mut incoming = vec![0; count];
    let mut outgoing = vec![Vec::new(); count];
    for edge in &diagram.edges {
        for &source in &edge.sources {
            if source == edge.target {
                continue;
            }
            let (from, to) = match (edge.arrow_start, edge.arrow_end) {
                (false, true) => (source, edge.target),
                (true, false) => (edge.target, source),
                _ if edge.sources.len() > 1 => (source, edge.target),
                _ => (source.min(edge.target), source.max(edge.target)),
            };
            incoming[to] += 1;
            outgoing[from].push(to);
        }
    }
    let mut ready = incoming
        .iter()
        .enumerate()
        .filter_map(|(index, &n)| (n == 0).then_some(index))
        .collect::<VecDeque<_>>();
    let mut layers = vec![0; count];
    let mut assigned = vec![false; count];
    // A cycle has no starting node. Break ties in declaration order when the queue runs dry.
    for index in 0..count {
        if ready.is_empty() && !assigned[index] {
            ready.push_back(index);
        }
        while let Some(from) = ready.pop_front() {
            assigned[from] = true;
            for &to in &outgoing[from] {
                if !assigned[to] {
                    layers[to] = layers[to].max(layers[from] + 1);
                    incoming[to] -= 1;
                    if incoming[to] == 0 {
                        ready.push_back(to);
                    }
                }
            }
        }
    }
    layers
}

fn grid_centers(sizes: impl Iterator<Item = (usize, f32)>, gap: f32) -> BTreeMap<usize, f32> {
    let mut tracks: BTreeMap<usize, f32> = BTreeMap::new();
    for (index, size) in sizes {
        tracks
            .entry(index)
            .and_modify(|current| *current = current.max(size))
            .or_insert(size);
    }
    let mut end = 0.0;
    let mut previous = tracks.first_key_value().map_or(0, |(&index, _)| index);
    for (&index, size) in &mut tracks {
        let start = end + (index - previous) as f32 * gap;
        end = start + *size;
        *size = start + *size / 2.0;
        previous = index;
    }
    tracks
}

fn canvas_bounds(nodes: &[PlacedNode], edges: &[PlacedEdge]) -> Bounds {
    let mut min = Point::new(0.0, 0.0);
    let mut max = min;
    let mut include = |point: Point| {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    };
    for bounds in nodes
        .iter()
        .map(|node| node.bounds)
        .chain(edges.iter().filter_map(|edge| edge.label_bounds))
    {
        include(Point::new(bounds.left, bounds.top));
        include(Point::new(bounds.right(), bounds.bottom()));
    }
    for point in edges.iter().flat_map(|edge| edge.paths()).flatten() {
        include(*point);
    }
    Bounds {
        left: (min.x - MARGIN).floor(),
        top: (min.y - MARGIN).floor(),
        width: (max.x + MARGIN).ceil() - (min.x - MARGIN).floor(),
        height: (max.y + MARGIN).ceil() - (min.y - MARGIN).floor(),
    }
}

fn check_size(canvas: Bounds) -> Result<(), &'static str> {
    if canvas.width > 16_384.0
        || canvas.height > 16_384.0
        || canvas.width * canvas.height > 32_000_000.0
    {
        return Err("diagram is too large for PNG output. Bring the grid positions closer or split it into diagrams");
    }
    Ok(())
}
