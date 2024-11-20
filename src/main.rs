use anyhow::{anyhow, Result};
use clap::{command, Arg, ArgAction};
use spellcasting_parser::*;

fn main() -> Result<(), anyhow::Error> {
    let matches = command!()
        .arg(
            Arg::new("file")
                .help("Path to the input file")
                .short('f')
                .long("file")
                .help("path to readed file")
                .long_help("path to readed file, can`t be used with -s/--string")
                .num_args(1),
        )
        .arg(
            Arg::new("string")
                .help("Input string")
                .short('s')
                .long("string")
                .help("input string to parse")
                .long_help("input string to parse, can`t be used with -s/--string")
                .num_args(1),
        )
        .group(
            clap::ArgGroup::new("input")
                .args(["file", "string"])
                .required(true)
                .multiple(false),
        )
        .arg(
            Arg::new("output")
                .help("Path to the output file")
                .short('o')
                .long("output")
                .help("path to output file")
                .num_args(1),
        )
        .arg(
            Arg::new("print")
                .help("Print the output to the console")
                .short('p')
                .long("print")
                .help("flag to print in console")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let content = if let Some(file_path) = matches.get_one::<String>("file") {
        std::fs::read_to_string(file_path).map_err(|e| anyhow!("Failed to read file: {}", e))?
    } else if let Some(input_string) = matches.get_one::<String>("string") {
        input_string.clone()
    } else {
        return Err(anyhow!("No valid input provided (file or string)"));
    };

    let spells = parse_string(&content)?;

    let output = spells.to_string();

    if let Some(output_path) = matches.get_one::<String>("output") {
        std::fs::write(output_path, &output)
            .map_err(|e| anyhow!("Failed to write to file: {}", e))?;
        println!("Output written to {}", output_path);
    }

    if matches.get_flag("print") {
        println!("Spells Struct: {:#?}", spells);
        println!("{}", output);
    }

    Ok(())
}
