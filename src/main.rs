use std::{env, error::Error, fs, path::PathBuf, process::ExitCode};

mod export;
mod labels;
mod layout;
mod parsing;
mod routing;

const USAGE: &str = "Usage: lamda_red [input.zz] [output-stem]";

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
    let mut args = env::args_os().skip(1);
    let input = args.next().unwrap_or_else(|| "sample.zz".into());
    if input == "--help" || input == "-h" {
        println!("{USAGE}\nDefaults: sample.zz and output");
        return Ok(());
    }
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
    let png_path = output_path(".png");
    let xml_path = output_path(".xml");
    let source_path = input.canonicalize()?;
    for path in [&png_path, &xml_path] {
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
    let png = export::png(&diagram, &mut layout, &mut fonts)?;
    let xml = export::xml(&diagram, &layout);
    fs::write(&png_path, png).map_err(|error| format!("{}: {error}", png_path.display()))?;
    fs::write(&xml_path, xml).map_err(|error| format!("{}: {error}", xml_path.display()))?;
    println!("Wrote {} and {}", png_path.display(), xml_path.display());
    Ok(())
}
