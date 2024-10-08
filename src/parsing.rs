use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use pest::Parser;
use pest_derive::Parser;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Parser)]
#[grammar = "lamda_red_grammar.pest"]
struct LamdaRed;

#[derive(Clone, Copy)]
pub struct Coordination {
    x: usize,
    y: usize,
}

/// Reads input lines.
pub fn read_input(file_name: &str) -> io::Result<()> {
    let mut proximity = Coordination { x: 120, y: 60 };

    let mut mx_cells: String = String::from("");
    let root_template: &str = "<?xml version='1.0' encoding='UTF-8'?>
    <mxGraphModel>
     <root>
       <mxCell id='0' />
       <mxCell id='1' parent='0' />
         {CHILDREN}
     </root>
    </mxGraphModel>";

    let path: &Path = Path::new(file_name);
    let file: File = File::open(&path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => {
                // Skips spaces
                if content.trim().len() != 0 {
                    // Reads line if started by twice "&" as comment and ignore it during compile.
                    if &content[0..2] == "&&" {
                        continue;
                    } else {
                        // This implementation caused to prevent additional shape `y` proximity incremental.
                        mx_cells.push_str(&parsing_proc(&content, proximity));
                        proximity.y += 100;
                    }
                }
            }
            Err(e) => println!("Error in reading line, syntax error: {}", e),
        }
    }

    let result = &root_template.replace("{CHILDREN}", &mx_cells);

    let mut file = File::create("output.xml")?;
    file.write(result.as_bytes())?;

    Ok(())
}

/// Parsing Process
///
/// Checks input file syntax and it must match to grammar.
pub fn parsing_proc(input: &str, proximity: Coordination) -> String {
    let mut final_mxs: String = String::from("");

    let pairs =
        LamdaRed::parse(Rule::file, input).unwrap_or_else(|e| panic!("Syntax error: {}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            for deep_inner_pair in inner_pair.into_inner() {
                // Gets main context (label) of grammar (entry > goal as Rule >
                // label: "Goal 1", "Goal 1" is main).
                let value: &str = deep_inner_pair.as_str();
                // Trims shape identifier from first and last chars.
                let shape_label: &str = &value[1..value.len() - 1];

                // I know maybe it is not necessary, TODO: check it after
                // complete todo list.
                if deep_inner_pair.as_rule() == Rule::entry {
                    continue;
                } else {
                    final_mxs.push_str(&mx_cell_builder(
                        deep_inner_pair.as_rule(),
                        shape_label,
                        proximity,
                    ));
                }
            }
        }
    }

    return final_mxs;
}

/// # MXCell builder
///
/// WHAT: Returns `xml` shape as output file. All shapes are inside of
/// `<mxCell><mxGeometry/></mxCell>` tag.
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
pub fn mx_cell_builder(shape_type: Rule, label: &str, proximity: Coordination) -> String {
    let mut shape: &str = "";
    let mut flip_h: &str = "0";

    match shape_type {
        Rule::goal => shape = "parallelogram",
        Rule::risk => {
            shape = "parallelogram";
            flip_h = "1";
        }
        Rule::agent => shape = "hexagon",
        _ => println!("Make <mxCell /> error by unknown Rule."),
    }

    let mx_cell_template = "
        <mxCell
            id='{SHAPE_ID}'
            value='{CONTEXT}'
            style='shape={SHAPE};perimeter={PERIMETER};whiteSpace=wrap;html=1;fixedSize=1;flipH={FLIP_H}'
            vertex='1'
            parent='1'
        >
          <mxGeometry
            x='{X_AXIS}'
            y='{Y_AXIS}'
            width='120'
            height='60'
            as='geometry'
          />
        </mxCell>
    ";

    return mx_cell_template
        .replace("{SHAPE_ID}", &random_shape_id(8))
        .replace("{CONTEXT}", label)
        .replace("{SHAPE}", &shape)
        .replace("{PERIMETER}", &random_shape_id(2).to_string())
        .replace("{X_AXIS}", &proximity.x.to_string())
        .replace("{Y_AXIS}", &proximity.y.to_string())
        .replace("{FLIP_H}", &flip_h);
}

fn random_shape_id(take: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(take)
        .map(char::from)
        .collect();
}
