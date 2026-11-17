use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use pest::{error::InputLocation, iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lamda_red_grammar.pest"]
struct Drizzle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Goal,
    Risk,
    Agent,
    Object,
    Action,
    Note,
    Diamond,
    Descriptive,
    Cloud,
}

pub type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Marker {
    OpenCircle,
    FilledCircle,
    Zigzag,
    Bar,
}

pub struct Node {
    pub shape: Shape,
    pub bold: bool,
    pub label: String,
    pub position: Option<Position>,
}

pub struct Edge {
    pub sources: Vec<usize>,
    pub target: usize,
    pub dashed: bool,
    pub arrow_start: bool,
    pub arrow_end: bool,
    pub marker: Option<Marker>,
    pub orthogonal: bool,
    pub via: Option<Direction>,
    pub label: String,
}

pub struct Diagram {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub direction: Direction,
}

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    column: usize,
    message: String,
}

impl ParseError {
    fn at(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            column,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

pub fn parse(input: &str) -> Result<Diagram, ParseError> {
    let mut nodes = Vec::new();
    let mut ids = HashMap::new();
    let mut used_positions = HashMap::new();
    let mut unresolved_edges = Vec::new();
    let mut direction = None;

    for (index, line) in input.trim_start_matches('\u{feff}').lines().enumerate() {
        let line_number = index + 1;
        let content = line.trim_start_matches([' ', '\t']);
        if content.trim().is_empty() || content.starts_with("&&") {
            continue;
        }
        if let Some(column) = line
            .chars()
            .position(|c| (c < ' ' && c != '\t') || c == '\u{fffe}' || c == '\u{ffff}')
        {
            return Err(ParseError::at(
                line_number,
                column + 1,
                "invalid control character",
            ));
        }
        let statement = Drizzle::parse(Rule::line, line)
            .map_err(|error| {
                let byte = match error.location {
                    InputLocation::Pos(byte) | InputLocation::Span((byte, _)) => byte,
                };
                ParseError::at(
                    line_number,
                    line[..byte].chars().count() + 1,
                    "invalid Drizzle syntax",
                )
            })?
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap();

        match statement.as_rule() {
            Rule::layout => {
                if direction.is_some() {
                    return Err(at(line_number, &statement, "layout already declared"));
                }
                let value = statement.into_inner().next().unwrap();
                direction = Some(read_direction(value.as_str()));
            }
            Rule::named => {
                let mut parts = statement.into_inner();
                let id = parts.next().unwrap();
                if let Some(&(_, previous_line)) = ids.get(id.as_str()) {
                    return Err(at(
                        line_number,
                        &id,
                        format!(
                            "duplicate ID '{}' (first declared on line {previous_line})",
                            id.as_str()
                        ),
                    ));
                }
                let mut shape = parts.next().unwrap();
                let mut position = None;
                if shape.as_rule() == Rule::position {
                    let pos = read_position(line_number, shape.clone())?;
                    if let Some(previous_line) = used_positions.insert(pos, line_number) {
                        return Err(at(
                            line_number,
                            &shape,
                            format!(
                                "cell @{},{} is already occupied by a node on line {previous_line}",
                                pos.0, pos.1
                            ),
                        ));
                    }
                    position = Some(pos);
                    shape = parts.next().unwrap();
                }
                ids.insert(id.as_str(), (nodes.len(), line_number));
                nodes.push(read_node(shape, position));
            }
            Rule::anonymous => {
                nodes.extend(statement.into_inner().map(|shape| read_node(shape, None)))
            }
            Rule::edge => unresolved_edges.push((line_number, statement)),
            _ => unreachable!(),
        }
    }

    let edges = unresolved_edges
        .into_iter()
        .map(|(line, edge)| read_edge(line, edge, &ids))
        .collect::<Result<_, _>>()?;
    Ok(Diagram {
        nodes,
        edges,
        direction: direction.unwrap_or_default(),
    })
}

fn read_direction(value: &str) -> Direction {
    match value {
        "right" => Direction::Right,
        "left" => Direction::Left,
        "up" | "top" => Direction::Up,
        "down" | "bottom" => Direction::Down,
        _ => unreachable!(),
    }
}

fn at(line: usize, pair: &Pair<'_, Rule>, message: impl Into<String>) -> ParseError {
    ParseError::at(line, pair.as_span().start_pos().line_col().1, message)
}

fn read_position(line: usize, pair: Pair<'_, Rule>) -> Result<Position, ParseError> {
    let read_number = |number: Pair<'_, Rule>| {
        number
            .as_str()
            .parse::<usize>()
            .map_err(|_| at(line, &number, "grid coordinate is too large"))
    };
    let mut numbers = pair.into_inner();
    Ok((
        read_number(numbers.next().unwrap())?,
        read_number(numbers.next().unwrap())?,
    ))
}

fn read_node(pair: Pair<'_, Rule>, position: Option<Position>) -> Node {
    let mut parts = pair.into_inner();
    let first = parts.next().unwrap();
    let bold = first.as_rule() == Rule::bold;
    let symbol = if bold { parts.next().unwrap() } else { first };
    let shape = match symbol.as_rule() {
        Rule::goal => Shape::Goal,
        Rule::risk => Shape::Risk,
        Rule::agent => Shape::Agent,
        Rule::object => Shape::Object,
        Rule::action => Shape::Action,
        Rule::note => Shape::Note,
        Rule::diamond => Shape::Diamond,
        Rule::descriptive => Shape::Descriptive,
        Rule::cloud => Shape::Cloud,
        _ => unreachable!(),
    };
    let raw = symbol.as_str();
    let delimiter = if shape == Shape::Cloud { 2 } else { 1 };
    Node {
        shape,
        bold,
        label: unescape(&raw[delimiter..raw.len() - delimiter]),
        position,
    }
}

fn read_edge(
    line: usize,
    pair: Pair<'_, Rule>,
    ids: &HashMap<&str, (usize, usize)>,
) -> Result<Edge, ParseError> {
    let resolve_id = |id: Pair<'_, Rule>| {
        ids.get(id.as_str())
            .map(|&(node, _)| node)
            .ok_or_else(|| at(line, &id, format!("unknown node '{}'", id.as_str())))
    };
    let mut parts = pair.into_inner();
    let mut seen = HashSet::new();
    let sources = parts
        .next()
        .unwrap()
        .into_inner()
        .map(|id| {
            let source = resolve_id(id.clone())?;
            if !seen.insert(source) {
                return Err(at(line, &id, "duplicate source in a branch"));
            }
            Ok(source)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let operator = parts.next().unwrap().as_str();
    let target_id = parts.next().unwrap();
    let target = resolve_id(target_id.clone())?;
    if sources.len() > 1 && seen.contains(&target) {
        return Err(at(
            line,
            &target_id,
            "a branch cannot contain its own target",
        ));
    }
    let mut edge = Edge {
        sources,
        target,
        dashed: operator.contains("--"),
        arrow_start: operator.starts_with('<'),
        arrow_end: operator.ends_with('>'),
        marker: None,
        orthogonal: false,
        via: None,
        label: String::new(),
    };
    for part in parts {
        match part.as_rule() {
            Rule::modifier if part.as_str() == "[ortho]" => edge.orthogonal = true,
            Rule::modifier => {
                if edge.marker.is_some() {
                    return Err(at(line, &part, "an edge may have one marker"));
                }
                edge.marker = Some(match part.as_str() {
                    "[o]" => Marker::OpenCircle,
                    "[*]" => Marker::FilledCircle,
                    "[zigzag]" => Marker::Zigzag,
                    "[bar]" => Marker::Bar,
                    _ => unreachable!(),
                });
            }
            Rule::via => {
                edge.via = Some(read_direction(part.into_inner().next().unwrap().as_str()))
            }
            Rule::edge_label => edge.label = unescape(&part.as_str()[1..]),
            _ => unreachable!(),
        }
    }
    Ok(edge)
}

fn unescape(raw: &str) -> String {
    let mut text = String::new();
    let mut chars = raw.trim().chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => match chars.next().expect("the grammar requires a valid escape") {
                'n' => text.push('\n'),
                escaped => text.push(escaped),
            },
            '\t' => text.push(' '),
            ch => text.push(ch),
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::{parse, Direction, Marker, Shape};

    #[test]
    fn accepts_legacy_shapes_and_unicode_labels() {
        let source = "\u{feff}  && comment\r\n  /Goal/ \\Risk\\\r\n<عامل کنترل>\r\n";
        let diagram = parse(source).unwrap();
        assert_eq!(diagram.nodes.len(), 3);
        assert_eq!(diagram.nodes[0].shape, Shape::Goal);
        assert_eq!(diagram.nodes[1].shape, Shape::Risk);
        assert_eq!(diagram.nodes[2].label, "عامل کنترل");
    }

    #[test]
    fn resolves_connections_and_optional_arrows() {
        let source = r#"A-->B : speed\nlocation
A__>B
A<__B
A<-->B
A__B [o]
A--B
A @ 1,2 = [A & B]
B = <مقدار \> ۰>"#;
        let diagram = parse(source).unwrap();
        let formats = diagram
            .edges
            .iter()
            .map(|edge| {
                (
                    edge.dashed,
                    edge.arrow_start,
                    edge.arrow_end,
                    edge.marker.is_some(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            formats,
            [
                (true, false, true, false),
                (false, false, true, false),
                (false, true, false, false),
                (true, true, true, false),
                (false, false, false, true),
                (true, false, false, false),
            ]
        );
        assert!(diagram
            .edges
            .iter()
            .all(|edge| edge.sources == [0] && edge.target == 1));
        assert_eq!(diagram.edges[0].label, "speed\nlocation");
        assert_eq!(diagram.nodes[0].position, Some((1, 2)));
        assert_eq!(diagram.nodes[1].label, "مقدار > ۰");
    }

    #[test]
    fn points_to_the_unresolved_node() {
        let error = parse("A = /Goal/\n  A --> Missing").err().unwrap();
        assert_eq!(error.to_string(), "2:9: unknown node 'Missing'");
    }

    #[test]
    fn parses_leaf_shapes_branch_markers_and_routing() {
        let diagram = parse(
            r#"layout up
G = !/Goal/
A = ~Property \~ bounds~
B = {{مانعی است برای}}
A, B __> G [*] [ortho]
B --> G [bar] via bottom : Obstruction
A __ B [zigzag]"#,
        )
        .unwrap();
        assert_eq!(diagram.direction, Direction::Up);
        assert!(diagram.nodes[0].bold);
        assert_eq!(diagram.nodes[1].shape, Shape::Descriptive);
        assert_eq!(diagram.nodes[1].label, "Property ~ bounds");
        assert_eq!(diagram.nodes[2].shape, Shape::Cloud);
        assert_eq!(diagram.edges[0].sources, [1, 2]);
        assert_eq!(diagram.edges[0].marker, Some(Marker::FilledCircle));
        assert!(diagram.edges[0].orthogonal);
        assert_eq!(diagram.edges[1].marker, Some(Marker::Bar));
        assert_eq!(diagram.edges[1].via, Some(Direction::Down));
        assert_eq!(diagram.edges[2].marker, Some(Marker::Zigzag));
    }
}
