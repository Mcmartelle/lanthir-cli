use crate::graph::{Edge, Node};
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
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
            current_node: current_node,
            graph: graph,
        }
    }
}

impl Traverse for GraphMachine {
    fn run(&mut self) -> Result<()> {
        loop {
            match &self.choices() {
                Some(choices) => {
                    match choices.len() {
                        0 => {
                            println!("no choices");
                            break;
                        },
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
                                println!("Traversing to {}", choices[0].destination);
                                self.traverse(choices[0].destination.clone());
                            } else {
                                println!("exiting flowchart");
                                break;
                            }
                        },
                        _ => {                            
                            // use fuzzy select
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
                            self.traverse(choices[selection].destination.clone());
                        }
                    };
                }
                None => {
                    println!("This is the end.");
                    break;
                }
            };
        }
        Ok(())
    }

    fn choices(&self) -> Option<&Vec<Edge>> {
        let edges = match self.graph.get(&self.current_node) {
            Some(node) => &node.outputs,
            None => return None,
        };
        let choices = match edges.len() {
            0 => return None,
            _ => edges
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
