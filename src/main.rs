extern crate pest;
//#[macro_use]
extern crate pest_derive;

use anyhow::{bail, Result};
use clap::Parser as ClapParser;
use log::info;
use pest::Parser as PestParser;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::collections::HashMap;
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

struct Node {
    outputs: Vec<Edge>,
    label: Option<String>,
    cmd: Option<String>,
}

struct Edge {
    destination: String,
    label: String,
}

struct GraphMachine {
    current_node: String,
    graph: HashMap<String, Node>,
}

trait Runner {
    fn choices(&self) -> Option<Vec<(&str, &str)>>;
    fn traverse(&mut self, destination: String);
    fn get_node_label(&self) -> Option<&str>;
    fn get_command(&self) -> Option<&str>;
}

impl Runner for GraphMachine {
    fn choices(&self) -> Option<Vec<(&str, &str)>> {
        let edges = match self.graph.get(&self.current_node) {
            Some(node) => &node.outputs,
            None => return None,
        };
        let choices = match edges.len() {
            0 => return None,
            _ => edges
                .iter()
                .map(|x| (x.label.as_str(), x.destination.as_str()))
                .collect(),
        };
        Some(choices)
    }

    fn traverse(&mut self, destination: String) {
        self.current_node = destination;
    }

    fn get_node_label(&self) -> Option<&str> {
        match self.graph.get(&self.current_node) {
            Some(node) => match &node.label {
                Some(command) => Some(command.as_str()),
                None => None,
            },
            None => None,
        }
    }

    fn get_command(&self) -> Option<&str> {
        match self.graph.get(&self.current_node) {
            Some(node) => match &node.cmd {
                Some(command) => Some(command.as_str()),
                None => None,
            },
            None => None,
        }
    }
}
