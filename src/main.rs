extern crate pest;
//#[macro_use]
extern crate pest_derive;

use anyhow::{bail, Result};
use clap::Parser as ClapParser;
use log::info;
#[allow(unused_imports)]
use pest::Parser as PestParser;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs;
use std::path::PathBuf;
use crate::mermaid::parse_mermaid;
use crate::runner::{GraphMachine, Traverse};

pub mod mermaid;
pub mod graph;
pub mod runner;

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

    let flowchart_graph = parse_mermaid(&flowchart_string)?;

    let mut flowchart_runner = GraphMachine::new(String::from("Start"), flowchart_graph);

    flowchart_runner.run()?;

    info!("does this only appear in log file if simplelogger isn't initialzed?");
    println!("Hello, world!");
    Ok(())
}

