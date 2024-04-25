use crate::graph::{Edge, Node};
use anyhow::{bail, Result};
use arboard::Clipboard;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
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
    fn get_node_type(&self) -> Result<(NodeTextType, &str)>;
    fn get_node_label(&self) -> Option<&str>;
    fn get_command(&self) -> Option<&str>;
    fn get_clipboard(&self) -> Option<&str>;
}

pub enum NodeTextType {
    Cmd,
    Cb,
    Label,
    Id,
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
        let process_node = &["Completed", "Skipping"];
        let cb_node = &["Copy to Clipboard", "Skip"];
        let cmd_node = &["Run Command", "Skip"];
        loop {
            match &self.choices() {
                Some(choices) => {
                    match choices.len() {
                        0 => {
                            unreachable!();
                            // info!("dead end at {}", &self.current_node);
                            // println!("no choices");
                            // break;
                        }
                        1 => {
                            let (node_type, text) = self.get_node_type()?;
                            match node_type {
                                NodeTextType::Cb => {
                                    let selection =
                                        FuzzySelect::with_theme(&ColorfulTheme::default())
                                            .with_prompt(text)
                                            .default(0)
                                            .items(&cb_node[..])
                                            .interact()
                                            .unwrap();
                                    match selection {
                                        0 => {
                                            let mut clipboard = Clipboard::new()?;
                                            clipboard.set_text(text)?;
                                            info!(
                                                "copied {} to clipboard, proceeding from {} to {}",
                                                text, &self.current_node, &choices[0].destination
                                            );
                                        }
                                        _ => {
                                            info!(
                                                "skipped clipboard copy, going from {} to {}",
                                                &self.current_node, &choices[0].destination
                                            );
                                        }
                                    };
                                    println!("Traversing to {}", &choices[0].destination);
                                    self.traverse(choices[0].destination.clone());
                                }
                                NodeTextType::Cmd => {
                                    let selection =
                                        FuzzySelect::with_theme(&ColorfulTheme::default())
                                            .with_prompt(text)
                                            .default(0)
                                            .items(&cmd_node[..])
                                            .interact()
                                            .unwrap();
                                    match selection {
                                        0 => {
                                            println!(
                                                "Running command (but not really yet): {}",
                                                text
                                            );
                                            info!(
                                                "ran command {}, proceeding from {} to {}",
                                                text, &self.current_node, &choices[0].destination
                                            );
                                        }
                                        _ => {
                                            info!(
                                                "skipped running command, going from {} to {}",
                                                &self.current_node, &choices[0].destination
                                            );
                                        }
                                    };
                                    println!("Traversing to {}", &choices[0].destination);
                                    self.traverse(choices[0].destination.clone());
                                }
                                NodeTextType::Label | NodeTextType::Id => {
                                    let selection =
                                        FuzzySelect::with_theme(&ColorfulTheme::default())
                                            .with_prompt(text)
                                            .default(0)
                                            .items(&process_node[..])
                                            .interact()
                                            .unwrap();
                                    match selection {
                                        0 => {
                                            info!(
                                                "completed {} going to {}",
                                                &self.current_node, &choices[0].destination
                                            );
                                        }
                                        _ => {
                                            info!(
                                                "skipping {} going to {}",
                                                &self.current_node, &choices[0].destination
                                            );
                                        }
                                    };
                                    println!("Traversing to {}", &choices[0].destination);
                                    self.traverse(choices[0].destination.clone());
                                }
                            }
                        }
                        _ => {
                            let (node_type, text) = self.get_node_type()?;
                            match node_type {
                                NodeTextType::Cb => {
                                    let selection =
                                        FuzzySelect::with_theme(&ColorfulTheme::default())
                                            .with_prompt(text)
                                            .default(0)
                                            .items(&cb_node[..])
                                            .interact()
                                            .unwrap();
                                    match selection {
                                        0 => {
                                            let mut clipboard = Clipboard::new()?;
                                            clipboard.set_text(text)?;
                                            println!("Copied to clipboard");
                                            info!(
                                                "copied {} to clipboard at {}",
                                                text, &self.current_node
                                            );
                                        }
                                        _ => {
                                            info!(
                                                "skipped clipboard copy at {}",
                                                &self.current_node
                                            );
                                        }
                                    };
                                }
                                NodeTextType::Cmd => {
                                    let selection =
                                        FuzzySelect::with_theme(&ColorfulTheme::default())
                                            .with_prompt(text)
                                            .default(0)
                                            .items(&cmd_node[..])
                                            .interact()
                                            .unwrap();
                                    match selection {
                                        0 => {
                                            println!(
                                                "Running command (but not really yet): {}",
                                                text
                                            );
                                            info!("ran command {} at {}", text, &self.current_node);
                                        }
                                        _ => {
                                            info!(
                                                "skipped running command at {}",
                                                &self.current_node
                                            );
                                        }
                                    };
                                }
                                NodeTextType::Label | NodeTextType::Id => {}
                            }
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(text)
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
                    let (node_type, text) = self.get_node_type()?;
                    match node_type {
                        NodeTextType::Cb => {
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(text)
                                .default(0)
                                .items(&cb_node[..])
                                .interact()
                                .unwrap();
                            match selection {
                                0 => {
                                    let mut clipboard = Clipboard::new()?;
                                    clipboard.set_text(text)?;
                                    println!("Copied to clipboard");
                                    info!("copied {} to clipboard at {}", text, &self.current_node);
                                }
                                _ => {
                                    info!("skipped clipboard copy at {}", &self.current_node);
                                }
                            };
                        }
                        NodeTextType::Cmd => {
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(text)
                                .default(0)
                                .items(&cmd_node[..])
                                .interact()
                                .unwrap();
                            match selection {
                                0 => {
                                    println!("Running command (but not really yet): {}", text);
                                    info!("ran command {} at {}", text, &self.current_node);
                                }
                                _ => {
                                    info!("skipped running command at {}", &self.current_node);
                                }
                            };
                        }
                        NodeTextType::Label | NodeTextType::Id => {
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(text)
                                .default(0)
                                .items(&process_node[..])
                                .interact()
                                .unwrap();
                            match selection {
                                0 => {
                                    info!("completed {}", &self.current_node);
                                }
                                _ => {
                                    info!("skipping {}", &self.current_node);
                                }
                            };
                        }
                    }
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

    fn get_node_type(&self) -> Result<(NodeTextType, &str)> {
        match self.graph.get(&self.current_node) {
            Some(node) => {
                if let Some(cb) = &node.cb {
                    return Ok((NodeTextType::Cb, &cb));
                }
                if let Some(cmd) = &node.cmd {
                    return Ok((NodeTextType::Cmd, &cmd));
                }
                if let Some(label) = &node.label {
                    return Ok((NodeTextType::Label, &label));
                }
                return Ok((NodeTextType::Id, &self.current_node));
            }
            None => bail!("no node corresponding wtih {}", &self.current_node),
        };
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

    fn get_clipboard(&self) -> Option<&str> {
        match self.graph.get(&self.current_node) {
            Some(node) => match &node.cb {
                Some(clip) => Some(clip.as_str()),
                None => None,
            },
            None => None,
        }
    }
}
