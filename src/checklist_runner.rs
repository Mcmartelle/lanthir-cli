use crate::checklist::Vertex;
use anyhow::Result;
use arboard::Clipboard;
use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use log::info;

pub struct ChecklistMachine {
    current_node: String,
    list: Vec<Vertex>,
}

impl ChecklistMachine {
    pub fn new(current_node: String, list: Vec<Vertex>) -> ChecklistMachine {
        ChecklistMachine { current_node, list }
    }
}

pub trait Checkify {
    fn run(&mut self) -> Result<()>;
}

impl Checkify for ChecklistMachine {
    fn run(&mut self) -> Result<()> {
        info!("starting checklist");
        let start_node = &["Okay"];
        let text_node = &["Completed", "Skipping"];
        let cb_node = &["Copy to Clipboard", "Skip"];
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(&self.current_node)
            .default(0)
            .items(&start_node[..])
            .interact()
            .unwrap();
        match selection {
            0 => {
                info!("completed: {}", &self.current_node);
            }
            _ => {
                info!("skipping: {}", &self.current_node,);
            }
        };
        for node in &self.list {
            match &node.alone {
                Some(text) => {
                    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt(text)
                        .default(0)
                        .items(&text_node[..])
                        .interact()
                        .unwrap();
                    match selection {
                        0 => {
                            info!("completed: {}", &text);
                        }
                        _ => {
                            info!("skipping: {}", &text,);
                        }
                    };
                }
                None => match &node.wrapper {
                    Some(cb_text) => {
                        let before = node.before.clone();
                        let after = node.after.clone();
                        println!(
                            "{} {} {}",
                            before.unwrap_or(String::from("")),
                            style(cb_text.clone()).cyan(),
                            after.unwrap_or(String::from(""))
                        );
                        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                            .with_prompt(cb_text)
                            .default(0)
                            .items(&cb_node[..])
                            .interact()
                            .unwrap();
                        match selection {
                            0 => {
                                let mut clipboard = Clipboard::new()?;
                                clipboard.set_text(cb_text.clone())?;
                                info!("copied {} to clipboard", &cb_text,);
                            }
                            _ => {
                                info!("skipped clipboard copy",);
                            }
                        };
                    }
                    None => {
                        let before = node.before.clone();
                        let after = node.after.clone();
                        let prompt =
                            before.unwrap_or(String::from("")) + &after.unwrap_or(String::from(""));
                        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                            .with_prompt(&prompt)
                            .default(0)
                            .items(&text_node[..])
                            .interact()
                            .unwrap();
                        match selection {
                            0 => {
                                info!("completed: {}", &prompt);
                            }
                            _ => {
                                info!("skipping: {}", &prompt);
                            }
                        };
                    }
                },
            }
        }
        info!("exiting checklist");
        Ok(())
    }
}
