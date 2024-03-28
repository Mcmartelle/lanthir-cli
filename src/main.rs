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
    // let mut graph: HashMap<String, Node> = HashMap::new();

    for part in mermaid_parts.into_inner() {
        match part.as_rule() {
            Rule::line => {
                // println!("LINE: {:#?}", part);
                println!("LINE");
                for pair in part.into_inner() {
                    match pair.as_rule() {
                        Rule::node_cluster => {
                            println!("> NODE CLUSTER");
                            for node in pair.into_inner() {
                                match node.as_rule() {
                                    Rule::node => {
                                        println!(">> NODE");
                                        for node_attr in node.into_inner() {
                                            match node_attr.as_rule() {
                                                Rule::node_id => {
                                                    println!(">>> NODE ID");
                                                    println!("=== {}", node_attr.as_str());
                                                },
                                                Rule::node_shape => {
                                                    println!(">>> NODE SHAPE");
                                                    for node_content in node_attr.into_inner() {
                                                        match node_content.as_rule() {
                                                            Rule::node_text => {
                                                                println!(">>>> NODE TEXT");
                                                                println!("==== {}", node_content.as_str());
                                                            },
                                                            Rule::shell_cmd => {
                                                                println!(">>>> SHELL CMD");
                                                                for shell_command in node_content.into_inner() {
                                                                    match shell_command.as_rule() {
                                                                        Rule::shell_text => {
                                                                            let mut command_string = String::new();
                                                                            for slice in shell_command.into_inner() {
                                                                                match slice.as_rule() {
                                                                                    Rule::non_double_quote => command_string.push_str(slice.as_str()),
                                                                                    Rule::double_quote => command_string.push('"'),
                                                                                    _ => unreachable!(),
                                                                                }
                                                                            }
                                                                            println!("==== {}", command_string)
                                                                        },
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            },
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                },
                                                _ => unreachable!(),
                                            }
                                        }
                                    },
                                    _ => {
                                        // println!("OTHER: {:#?}", node);
                                        unreachable!()
                                    },
                                }
                            }
                        },
                        Rule::edge => {
                            println!("> EDGE");
                            for edge in pair.into_inner() {
                                match edge.as_rule() {
                                    Rule::directed_edge => {
                                        println!(">> DIRECTED EDGE");
                                        for edge_part in edge.into_inner() {
                                            match edge_part.as_rule() {
                                                Rule::edge_piped_text => {
                                                    for edge_text in edge_part.into_inner() {
                                                        match edge_text.as_rule() {
                                                            Rule::edge_text => {
                                                                println!(">>> EDGE TEXT (piped)");
                                                                println!("=== {}", edge_text.as_str());
                                                            },
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                },
                                                Rule::edge_text => {                                                    
                                                    println!(">>> EDGE TEXT");
                                                    println!("=== {}", edge_part.as_str());
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    },
                                    Rule::undirected_edge => {
                                        println!(">> UNDIRECTED EDGE");
                                    },
                                    _ => unreachable!(),
                                }
                            }
                        },
                        _ => unreachable!(),
                    }
                }
            },
            Rule::header => println!("HEADER: {:#?}", part),
            _ => unreachable!(),
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
