use crate::oats::{Grain, Marker};
use anyhow::{
    // bail,
    Result,
};
// use arboard::Clipboard;
// use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use log::info;

pub struct OatsMachine {
    list: Vec<Grain>,
}

impl OatsMachine {
    pub fn new(list: Vec<Grain>) -> OatsMachine {
        OatsMachine { list }
    }
}

pub trait Oatify {
    fn run(&mut self) -> Result<()>;
}

impl Oatify for OatsMachine {
    fn run(&mut self) -> Result<()> {
        info!("starting sequence");
        let start_prompt = "Start";
        let start_items = &["Okay"];
        let one_of_prompt = "Complete one of the following";
        let unordered_prompt = "Complete all of the following in any order";
        let and_then_items = &["Completed", "Skipping"];
        // let cb_node = &["Copy to Clipboard", "Skip"];
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(start_prompt)
            .default(0)
            .items(&start_items[..])
            .interact()
            .unwrap();
        match selection {
            0 => {
                info!("completed: {}", &start_prompt);
            }
            _ => {
                info!("skipping: {}", &start_prompt,);
            }
        };
        let _ =
            &self
                .list
                .chunk_by(|a, b| a.marker == b.marker)
                .for_each(|sublist| match sublist[0].marker {
                    Some(Marker::AndThen) => {
                        sublist
                            .into_iter()
                            .filter(|a| a.content.is_some())
                            .for_each(|step| {
                                let text = step.content.as_ref().unwrap();
                                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                    .with_prompt(text)
                                    .default(0)
                                    .items(and_then_items)
                                    .interact()
                                    .unwrap();
                                match selection {
                                    0 => {
                                        info!("completed: {}", &text);
                                    }
                                    _ => {
                                        info!("skipping: {}", &text);
                                    }
                                }
                            });
                    }
                    Some(Marker::Clipbo) => {}
                    Some(Marker::Unordered) => {
                        let mut next_unorders = sublist.to_vec();
                        loop {
                            let mut undone_unorders: Vec<Grain> = next_unorders
                                .into_iter()
                                .filter(|a| !a.done && a.content.is_some())
                                .collect();
                            if undone_unorders.len() <= 0 {
                                break;
                            };
                            let unordered_items: Vec<String> = undone_unorders
                                .iter()
                                .map(|a| a.content.as_ref().unwrap().clone())
                                .collect();
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(unordered_prompt)
                                .default(0)
                                .items(&unordered_items[..])
                                .interact()
                                .unwrap();
                            undone_unorders[selection].done = true;
                            info!("completed one of: {}", &unordered_items[selection]);
                            next_unorders = undone_unorders;
                        }
                    }
                    Some(Marker::OneOf) => {
                        let one_of_items: Vec<String> = sublist
                            .iter()
                            .map(|a| a.content.as_ref().unwrap().clone())
                            .collect();
                        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                            .with_prompt(one_of_prompt)
                            .default(0)
                            .items(&one_of_items[..])
                            .interact()
                            .unwrap();
                        info!("completed one of: {}", &one_of_items[selection]);
                    }
                    Some(Marker::Breaker) => {}
                    None => {}
                });

        info!("exiting sequence");
        Ok(())
    }
}

// fn handle_node_with_clipbo(String: cb_text) -> Result<()> {
// let before = node.before.clone();
// let after = node.after.clone();
// println!(
//     "{} {} {}",
//     before.unwrap_or(String::from("")),
//     style(cb_text.clone()).cyan(),
//     after.unwrap_or(String::from(""))
// );
// let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
//     .with_prompt(cb_text)
//     .default(0)
//     .items(&cb_node[..])
//     .interact()
//     .unwrap();
// match selection {
//     0 => {
//         let mut clipboard = Clipboard::new()?;
//         clipboard.set_text(cb_text.clone())?;
//         info!("copied {} to clipboard", &cb_text,);
//     }
//     _ => {
//         info!("skipped clipboard copy",);
//     }
// };
// Ok(())
// }
