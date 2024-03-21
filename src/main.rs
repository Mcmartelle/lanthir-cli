extern crate pest;
//#[macro_use]
extern crate pest_derive;

use anyhow::{bail, Result};
use clap::Parser as ClapParser;
use log::info;
use pest::Parser as PestParser;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs;
use std::path::PathBuf;

#[derive(pest_derive::Parser)]
#[grammar = "mermaid.pest"]
pub struct MermaidParser;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(long)]
    log_path: Option<PathBuf>,
    #[arg(short, long)]
    log: Option<bool>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.log.unwrap_or(true) {
        let log_dir = match args.log_path {
            Some(mut path) => {
                path.push("test.log");
                path
            }
            None => match home::home_dir() {
                Some(mut path) => {
                    path.push(".lanthir");
                    path.push("logs");
                    path.push("test.log");
                    path
                }
                None => bail!("Unable to get home directory"),
            },
        };
        let config = ConfigBuilder::new().set_time_format_rfc3339().build();
        let _ = WriteLogger::init(
            LevelFilter::Info,
            config,
            fs::File::create(log_dir).unwrap(),
        );
    }

    let flowchart_string = fs::read_to_string(args.input.unwrap())?;

    let _flowchart_graph = parse_mermaid(&flowchart_string);

    info!("does this only appear in log file if simplelogger isn't initialzed?");
    println!("Hello, world!");
    Ok(())
}

fn parse_mermaid(flowchart_string: &str) -> Result<()> {
    let mermaid_parts = MermaidParser::parse(Rule::mmd, flowchart_string)
        .expect("unsuccessful pest parse")
        .next()
        .unwrap();

    for part in mermaid_parts.into_inner() {
        match part.as_rule() {
            Rule::line => println!("LINE: {:#?}", part),
            Rule::header => println!("HEADER: {:#?}", part),
            _ => println!("OTHER: {:#?}", part),
        }
    }

    Ok(())
}
