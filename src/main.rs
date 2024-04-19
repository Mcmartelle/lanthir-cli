extern crate pest;
//#[macro_use]
extern crate pest_derive;

use crate::mermaid::parse_mermaid;
use crate::runner::{GraphMachine, Traverse};
use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Local, Timelike};
use clap::Parser as ClapParser;
#[allow(unused_imports)]
use pest::Parser as PestParser;
use sha2::{Digest, Sha256};
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs;
use std::path::PathBuf;

pub mod graph;
pub mod mermaid;
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

    match args.input {
        Some(input) => {
            let flowchart_string = fs::read_to_string(&input)?;

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

                            let filename = generate_logfile_name(
                                input.file_stem().unwrap().to_str().unwrap(),
                                &flowchart_string,
                            )?;
                            path.push(filename);
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

            let flowchart_graph = parse_mermaid(&flowchart_string)?;
            let mut flowchart_runner = GraphMachine::new(String::from("Start"), flowchart_graph);
            flowchart_runner.run()?;
        }
        None => {
            bail!("no input file provided");
        }
    }

    Ok(())
}

fn generate_logfile_name(flowchart_name: &str, flowchart: &String) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(flowchart);
    let result = hasher.finalize();
    let hash: String = format!("{:x}", result).chars().take(6).collect();
    let timestamp = filename_timestamp()?;
    let filename = format!("{}-{}-{}.log", flowchart_name, hash, timestamp);
    Ok(filename)
}

fn filename_timestamp() -> Result<String> {
    let local: DateTime<Local> = Local::now();
    let timestamp = format!(
        "{:04}-{:02}-{:02}{}{:02}{:02}{:02}",
        local.year(),
        local.month(),
        local.day(),
        local.weekday(),
        local.hour(),
        local.minute(),
        local.second()
    );
    Ok(timestamp)
}
