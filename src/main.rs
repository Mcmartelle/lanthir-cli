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

fn parse_mermaid(flowchart_string: &str) -> Result<HashMap<String, Node>> {
    let mermaid_parts = MermaidParser::parse(Rule::mmd, flowchart_string)
        .expect("unsuccessful pest parse")
        .next()
        .unwrap();
    let mut graph: HashMap<String, Node> = HashMap::new();
    
    
    for part in mermaid_parts.into_inner() {
        match part.as_rule() {
            Rule::line => {
                let mut line_node_clusters: Vec<Vec<(String, Option<String>, Option<String>)>> = Vec::new(); 
                let mut line_edges: Vec<(bool, Option<String>)> = Vec::new(); 
                // let mut node_index: u8 = 0;
                // let mut edge_index: u8 = 0;
                // Line;
                println!("LINE");
                for pair in part.into_inner() {
                    match pair.as_rule() {
                        Rule::node_cluster => {
                            // Node Cluster
                            println!("> NODE CLUSTER");
                            let mut cluster_nodes: Vec<(String, Option<String>, Option<String>)> = Vec::new();
                            for node in pair.into_inner() {
                                match node.as_rule() {
                                    Rule::node => {
                                        // Node
                                        println!(">> NODE");
                                        let mut node_id: String = String::new();
                                        let mut node_text: Option<String> = None;
                                        let mut node_cmd: Option<String> = None;
                                        for node_attr in node.into_inner() {
                                            match node_attr.as_rule() {
                                                Rule::node_id => {
                                                    // Node ID
                                                    println!(">>> NODE ID");
                                                    println!("=== {}", node_attr.as_str());
                                                    node_id = String::from(node_attr.as_str());
                                                },
                                                Rule::node_shape => {
                                                    println!(">>> NODE SHAPE");
                                                    for node_content in node_attr.into_inner() {
                                                        match node_content.as_rule() {
                                                            Rule::node_text => {
                                                                // Node Text
                                                                println!(">>>> NODE TEXT");
                                                                println!(
                                                                    "==== {}",
                                                                    node_content.as_str()
                                                                );
                                                                node_text = Some(String::from(node_content.as_str()));
                                                            }
                                                            Rule::shell_cmd => {
                                                                // Shell CMD
                                                                println!(">>>> SHELL CMD");
                                                                for shell_command in
                                                                    node_content.into_inner()
                                                                {
                                                                    match shell_command.as_rule() {
                                                                        Rule::shell_text => {
                                                                            let mut command_string =
                                                                                String::new();
                                                                            for slice in
                                                                                shell_command
                                                                                    .into_inner()
                                                                            {
                                                                                match slice.as_rule() {
                                                                                    Rule::non_double_quote => command_string.push_str(slice.as_str()),
                                                                                    Rule::double_quote => command_string.push('"'),
                                                                                    _ => unreachable!(),
                                                                                }
                                                                            }
                                                                            println!(
                                                                                "==== {}",
                                                                                command_string
                                                                            );
                                                                            node_cmd = Some(command_string);
                                                                        }
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                },
                                                _ => unreachable!(),
                                            }
                                        }
                                        cluster_nodes.push((node_id, node_text, node_cmd));
                                    }
                                    _ => {
                                        // println!("OTHER: {:#?}", node);
                                        unreachable!()
                                    }
                                }
                            }
                            line_node_clusters.push(cluster_nodes);
                        }
                        Rule::edge => {
                            println!("> EDGE");
                            let mut line_edge: Option<String> = None;
                            let mut edge_is_directed = false;
                            for edge in pair.into_inner() {
                                match edge.as_rule() {
                                    Rule::directed_edge => {
                                        println!(">> DIRECTED EDGE");
                                        edge_is_directed = true;
                                        for edge_part in edge.into_inner() {
                                            match edge_part.as_rule() {
                                                Rule::edge_piped_text => {
                                                    for edge_text in edge_part.into_inner() {
                                                        match edge_text.as_rule() {
                                                            Rule::edge_text => {
                                                                println!(">>> EDGE TEXT (piped)");
                                                                println!(
                                                                    "=== {}",
                                                                    edge_text.as_str()
                                                                );
                                                                line_edge = Some(String::from(edge_text.as_str()));
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                }
                                                Rule::edge_text => {
                                                    println!(">>> EDGE TEXT");
                                                    println!("=== {}", edge_part.as_str());
                                                    line_edge = Some(String::from(edge_part.as_str()));
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                    Rule::undirected_edge => {
                                        println!(">> UNDIRECTED EDGE");
                                        edge_is_directed = false;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            line_edges.push((edge_is_directed, line_edge));
                        }
                        _ => unreachable!(),
                    }
                }
                // Process line data into graph here
                println!("line_node_clusters: {:#?}", line_node_clusters);
                println!("line_edges: {:#?}", line_edges);
                assert_eq!(line_node_clusters.len(), line_edges.len()+1);
                for index in 0..line_edges.len() {
                    if line_edges[index].0 {
                        for src_node in line_node_clusters[index].clone() {
                            for dest_node in line_node_clusters[index+1].clone() {
                                graph.entry(src_node.0.clone()).and_modify(|entry| modify_src_entry(entry, src_node.clone(), line_edges[index].clone(), dest_node.clone())).or_insert(Node { outputs: vec![Edge {destination: dest_node.0.clone(), label: line_edges[index].1.clone()} ], label: src_node.1.clone(), cmd: src_node.2.clone() });
                                graph.entry(dest_node.0.clone()).and_modify(|entry| modify_dest_entry(entry, dest_node.clone())).or_insert(Node { outputs: vec![], label: dest_node.1.clone(), cmd: dest_node.2.clone() });
                            }
                        }
                    }
                }
                for (key, val) in graph.iter() {
                    println!("key: {} val: {:#?}", key, val);
                }

            },
            Rule::header => println!("HEADER: {:#?}", part),
            _ => unreachable!(),
        }
    }

    Ok(graph)
}

fn modify_src_entry(entry: &mut Node, src_node: (String, Option<String>, Option<String>), edge: (bool, Option<String>), dest_node: (String, Option<String>, Option<String>)) {
    entry.outputs.push(Edge { destination: dest_node.0.clone(), label: edge.1.clone() });
    if src_node.1.is_some() { // overwriting previous values to match mermaid :rolling-eyes:
        entry.label = src_node.1;
    }
    if src_node.2.is_some() {
        entry.cmd = src_node.2;
    }
}

fn modify_dest_entry(entry: &mut Node, dest_node: (String, Option<String>, Option<String>)) {
    if dest_node.1.is_some() {
        entry.label = dest_node.1;
    }
    if dest_node.2.is_some() {
        entry.cmd = dest_node.2;
    }
}

#[derive(Debug)]
struct Node {
    outputs: Vec<Edge>,
    label: Option<String>,
    cmd: Option<String>,
}

#[derive(Debug)]
struct Edge {
    destination: String,
    label: Option<String>,
}

struct GraphMachine {
    current_node: String,
    graph: HashMap<String, Node>,
}

trait Runner {
    fn choices(&self) -> Option<Vec<(Option<String>, &str)>>;
    fn traverse(&mut self, destination: String);
    fn get_node_label(&self) -> Option<&str>;
    fn get_command(&self) -> Option<&str>;
}

impl Runner for GraphMachine {
    fn choices(&self) -> Option<Vec<(Option<String>, &str)>> {
        let edges = match self.graph.get(&self.current_node) {
            Some(node) => &node.outputs,
            None => return None,
        };
        let choices = match edges.len() {
            0 => return None,
            _ => edges
                .iter()
                .map(|x| (x.label.clone(), x.destination.as_str()))
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
