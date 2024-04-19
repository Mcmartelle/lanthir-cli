use crate::graph::{Edge, Node};
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use log::info;
use std::collections::HashMap;

pub struct GraphMachine {
    current_node: String,
    graph: HashMap<String, Node>,
}

pub trait Traverse {
    fn run(&mut self) -> Result<()>;
    fn choices(&self) -> Option<&Vec<Edge>>; // (edge label, destination)
    fn traverse(&mut self, destination: String);
    fn get_node_label(&self) -> Option<&str>;
    fn get_command(&self) -> Option<&str>;
}

impl GraphMachine {
    pub fn new(current_node: String, graph: HashMap<String, Node>) -> GraphMachine {
        GraphMachine {
            current_node,
            graph,
        }
    }
}

impl Traverse for GraphMachine {
    fn run(&mut self) -> Result<()> {
        info!("starting flowchart");
        loop {
            match &self.choices() {
                Some(choices) => {
                    match choices.len() {
                        0 => {
                            info!("dead end at {}", &self.current_node);
                            println!("no choices");
                            break;
                        }
                        1 => {
                            let prompt = match self.get_node_label() {
                                Some(x) => x,
                                None => &self.current_node,
                            };
                            println!("{}", prompt);
                            if Confirm::with_theme(&ColorfulTheme::default())
                                .with_prompt("Continue?")
                                .default(true)
                                .show_default(false)
                                .interact()
                                .unwrap()
                            {
                                info!(
                                    "completed {} going to {}",
                                    &self.current_node, choices[0].destination
                                );
                                println!("Traversing to {}", choices[0].destination);
                                self.traverse(choices[0].destination.clone());
                            } else {
                                info!("at {} not continuing", &self.current_node);
                                println!("exiting flowchart");
                                break;
                            }
                        }
                        _ => {
                            let prompt = match self.get_node_label() {
                                Some(x) => x,
                                None => &self.current_node,
                            };
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(prompt)
                                .default(0)
                                .items(&choices[..])
                                .interact()
                                .unwrap();
                            info!("at {} chose {}", &self.current_node, choices[selection]);
                            self.traverse(choices[selection].destination.clone());
                        }
                    };
                }
                None => {
                    info!("dead end at {}", &self.current_node);
                    println!("This is the end.");
                    break;
                }
            };
        }
        info!("exiting flowchart");
        Ok(())
    }

    fn choices(&self) -> Option<&Vec<Edge>> {
        let edges = match self.graph.get(&self.current_node) {
            Some(node) => &node.outputs,
            None => return None,
        };
        let choices = match edges.len() {
            0 => return None,
            _ => edges,
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
