use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use pest::Parser;
use pest_derive::Parser;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Parser)]
#[grammar = "gram.pest"]
struct LamdaRed;

fn main() {
    read_input().unwrap();
}

/// Reads input lines.
pub fn read_input() -> io::Result<()> {
    let mut final_mxs: String = String::from("");
    let template: &str = "<?xml version='1.0' encoding='UTF-8'?>
        <mxfile host='app.diagrams.net' agent='Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36' version='24.7.7'>
        <diagram name='Page-1' id='Dud1fYsSzIhBKdsuW2Y5'>
            <mxGraphModel dx='1420' dy='795' grid='1' gridSize='10' guides='1' tooltips='1' connect='1' arrows='1' fold='1' page='1' pageScale='1' pageWidth='850' pageHeight='1100' math='0' shadow='0'>
            <root>
                <mxCell id='0' />
                <mxCell id='1' parent='0' />
               {CHILDREN}
            </root>
            </mxGraphModel>
        </diagram>
        </mxfile>";

    let path: &Path = Path::new("rules.txt");
    let file: File = File::open(&path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => final_mxs.push_str(&parsing_proc(&content)),
            Err(e) => println!("Error in reading line: {}", e),
        }
    }

    let result = &template.replace("{CHILDREN}", &final_mxs);

    let mut file = File::create("output.xml")?;
    file.write(result.as_bytes())?;

    Ok(())
}

pub fn parsing_proc(input: &str) -> String {
    let mut final_mxs: String = String::from("");

    let pairs = LamdaRed::parse(Rule::file, input)
        .unwrap_or_else(|e| panic!("Error in parsing grammar: {}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            for deep_inner_pair in inner_pair.into_inner() {
                let value: &str = deep_inner_pair.as_str();
                let shape_context: &str = &value[1..value.len() - 1];
                final_mxs.push_str(&mx_cell_builder(deep_inner_pair.as_rule(), shape_context));
            }
        }
    }

    return final_mxs;
}

/// # MXCell builder
///
/// WHAT: Returns `xml` shape as output file. All shapes are inside of `<mxCell><mxGeometry/></mxCell>` tag.
///
/// ## Overall Traits `<mxCell />`
///
/// + id: Create a random id for every shape.
/// + value: We need to define value inside of every `mxCell` tag.
/// + style: "shape={NAME};perimeter={PERIMETER};whiteSpace=wrap;html=1;fixedSize=1;flipH=(0, 1);flipV=(0, 1)"
///     + NAME:
///         + "parallelogram": Goal
///         + "parallelogram" with flipH=1: Risk
///         + "hexagon": Agent
///     + PERIMETER: Some perimeter with shame name and context.
///     + flipV: (0, 1)
///     + flipH: (0, 1)
///
/// ## Overall Traits `<mxGeometry />`
///
/// ### Coordination sets as pixel
///
/// + x
/// + y
///
/// ### Shape size (as GEOMETRY)
///
/// + width
/// + height
///
/// __NOTE__: We will have object size concern.
///
/// __NOTE__: All of `<mxCell />` children are inside of `<root>...</root>`.
///
/// ## Return should be:
///
/// ```xml
/// <mxCell id="4476NNJD" value="Goal shape" style="shape=parallelogram;perimeter=4476NNJD;whiteSpace=wrap;html=1;fixedSize=1;" vertex="1" parent="1">
///     <mxGeometry x="230" y="150" width="120" height="60" as="geometry" />
/// </mxCell>
/// ```
pub fn mx_cell_builder(shape_type: Rule, context: &str) -> String {
    let mut shape: &str = "";
    let mut flip_h: &str = "0";

    match shape_type {
        Rule::goal => shape = "parallelogram",
        Rule::risk => {
            shape = "parallelogram";
            flip_h = "1";
        }
        Rule::agent => shape = "hexagon",
        _ => println!("Error"),
    }

    let template = "
        <mxCell id='{SHAPE_ID}' value='{CONTEXT}' style='shape={SHAPE};perimeter=GOALSOMEID;whiteSpace=wrap;html=1;fixedSize=1;flipH={FLIPH}' vertex='1' parent='1'>
          <mxGeometry x='230' y='150' width='120' height='60' as='geometry' />
        </mxCell>
    ";

    return template
        .replace("{SHAPE_ID}", &random_shape_id())
        .replace("{CONTEXT}", context)
        .replace("{SHAPE}", &shape)
        .replace("{FLIPH}", &flip_h);
}

pub fn random_shape_id() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
}
