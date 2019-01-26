extern crate clap;
extern crate haron_parser;
extern crate nom;

use clap::{crate_authors, crate_version};
use haron_parser::types::instruction::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file(name: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_data(data: &str) -> Vec<Instruction> {
    let input = haron_parser::types::Input::from(data);

    match haron_parser::parser::parse_string(input) {
        Ok((_, tree)) => tree,
        Err(err) => panic!("{}", err),
    }
}

fn parse_command(matches: &clap::ArgMatches) {
    // Safe unwrap, because argument is required
    let file_path = matches.value_of("file").unwrap();

    match read_file(file_path) {
        Ok(contents) => {
            let tree = parse_data(&contents);

            if tree.is_empty() {
                println!("Unable to parse file or file is empty!");
                std::process::exit(1);
            }

            for line in tree {
                println!("{}", &line)
            }
        }
        Err(err) => panic!("{}", err),
    };
}

fn main() {
    let parse_subcommand = clap::SubCommand::with_name("parse")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("parse RedCode file")
        .arg(
            clap::Arg::with_name("file")
                .help("A RedCode file to parse")
                .value_name("PATH")
                .takes_value(true)
                .required(true),
        );

    let app = clap::App::new("haron")
        .about("haron is a small expirement about parsing and interpeting RedCode CoreWars files")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(parse_subcommand)
        .get_matches();

    if let ("parse", Some(matches)) = app.subcommand() {
        parse_command(matches);
    }
}
