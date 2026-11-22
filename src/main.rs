use std::{env, error::Error, fs, path::PathBuf, process::ExitCode};

mod export;
mod labels;
mod layout;
mod parsing;
mod routing;

const USAGE: &str = "Usage: lmdred [--export-set] [input.zz] [output-stem]
       lmdred --help
       lmdred --version

Compile a Drizzle diagram into PNG and editable mxGraph XML.
Paths are relative to the current directory. Defaults: sample.zz and output.

Examples:
  lmdred sample.zz my-new-red
  lmdred \"my diagrams/train.zz\" \"my diagrams/train\"

Options:
  -h, --help       Show this help
  -V, --version    Show the compiler version
  --export-set     Write XML and three PNG sizes (-compact, -document, -high)
  --              Treat the remaining arguments as file paths";

const HIGH_PNG_SCALE: f32 = 4.0;
const PNG_VARIANTS: [(&str, f32); 3] = [
    ("-compact.png", 1.0),
    ("-document.png", 2.0),
    ("-high.png", HIGH_PNG_SCALE),
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut paths = Vec::new();
    let mut positional = false;
    let mut export_set = false;
    for arg in env::args_os().skip(1) {
        if !positional {
            if arg == "--" {
                positional = true;
                continue;
            }
            if arg == "--help" || arg == "-h" {
                println!("{USAGE}");
                return Ok(());
            }
            if arg == "--version" || arg == "-V" {
                println!("lmdred {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            if arg == "--export-set" {
                export_set = true;
                continue;
            }
            if arg.to_string_lossy().starts_with('-') {
                return Err(format!("Unknown option: {}\n\n{USAGE}", arg.to_string_lossy()).into());
            }
        }
        paths.push(arg);
    }
    let mut args = paths.into_iter();
    let input = args.next().unwrap_or_else(|| "sample.zz".into());
    let stem = PathBuf::from(args.next().unwrap_or_else(|| "output".into()));
    if args.next().is_some() || stem.file_name().is_none() {
        return Err(USAGE.into());
    }
    let input = PathBuf::from(input);
    let contents =
        fs::read_to_string(&input).map_err(|error| format!("{}: {error}", input.display()))?;
    let diagram =
        parsing::parse(&contents).map_err(|error| format!("{}:{error}", input.display()))?;
    let output_path = |extension: &str| {
        let mut path = stem.as_os_str().to_owned();
        path.push(extension);
        PathBuf::from(path)
    };
    let variants = if export_set {
        PNG_VARIANTS.as_slice()
    } else {
        &[(".png", 1.0)]
    };
    let png_paths: Vec<_> = variants
        .iter()
        .map(|(suffix, _)| output_path(suffix))
        .collect();
    let xml_path = output_path(".xml");
    let source_path = input.canonicalize()?;
    for path in png_paths.iter().chain([&xml_path]) {
        if path
            .canonicalize()
            .is_ok_and(|target| target == source_path)
        {
            return Err(format!(
                "{}: output would overwrite the Drizzle source",
                path.display()
            )
            .into());
        }
    }
    let mut fonts = labels::Fonts::new()?;
    let mut layout = layout::arrange(&diagram, &mut fonts)?;
    let scale = if export_set {
        (layout::max_png_scale(layout.canvas) / HIGH_PNG_SCALE).min(1.0)
    } else {
        1.0
    };
    let images = variants
        .iter()
        .map(|(_, multiplier)| export::png(&diagram, &mut layout, &mut fonts, scale * multiplier))
        .collect::<Result<Vec<_>, _>>()?;
    let xml = export::xml(&diagram, &layout);
    for (path, png) in png_paths.iter().zip(images) {
        fs::write(path, png).map_err(|error| format!("{}: {error}", path.display()))?;
    }
    fs::write(&xml_path, xml).map_err(|error| format!("{}: {error}", xml_path.display()))?;
    let output_names = png_paths
        .iter()
        .chain([&xml_path])
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    println!(
        "Wrote {}",
        output_names.join(if export_set { ", " } else { " and " })
    );
    Ok(())
}
