use crate::oats::{Groat, Marker};
use anyhow::{
    // bail,
    Result,
};
use arboard::Clipboard;
// use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use log::info;

#[derive(Clone)]
pub struct Oatlet {
    pub marker: Marker,
    pub content: Option<String>,
    pub clipboard: Option<String>,
    pub done: bool,
}

pub struct OatsMachine {
    groats: Vec<Groat>,
}

impl OatsMachine {
    pub fn new(list: Vec<Groat>) -> OatsMachine {
        OatsMachine { groats: list }
    }
}

pub trait Oatify {
    fn run(&mut self) -> Result<()>;
}

impl Oatify for OatsMachine {
    fn run(&mut self) -> Result<()> {
        info!("starting sequence");
        let start_prompt = "Warning: Your clipboard may be overwritten. Start?";
        let start_items = &["Okay"];
        let one_of_prompt = "Complete one of";
        let unordered_prompt = "Complete all in any order";
        let and_then_items = &["Done"];
        let optional_items = &["Completed", "Skipping"];
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

        let oatlets = groats_to_oatlets(&self.groats);
        let _ = &oatlets
            .chunk_by(|a, b| a.marker == b.marker)
            .for_each(|sublist| match sublist[0].marker {
                Marker::AndThen => {
                    sublist
                        .into_iter()
                        .filter(|a| a.content.is_some())
                        .for_each(|step| {
                            let text: String;
                            let parts: [&str; 3];
                            if step.clipboard.is_some() {
                                parts = [
                                    step.content.as_ref().unwrap(),
                                    "\nCopied to Clipboard: ",
                                    step.clipboard.as_ref().unwrap(),
                                ];
                                text = parts.concat();
                                let _ = copy_to_clipboard(step.clipboard.as_ref().unwrap().clone());
                            } else {
                                text = step.content.as_ref().unwrap().to_owned();
                            }
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(&text)
                                .default(0)
                                .items(and_then_items)
                                .interact()
                                .unwrap();
                            match selection {
                                0 => {
                                    info!("done: {}", &text);
                                }
                                _ => {
                                    info!("skipping: {}", &text);
                                }
                            }
                        });
                }
                Marker::Optional => {
                    sublist
                        .into_iter()
                        .filter(|a| a.content.is_some())
                        .for_each(|step| {
                            let mut text: String;
                            let parts: [&str; 3];
                            if step.clipboard.is_some() {
                                parts = [
                                    step.content.as_ref().unwrap(),
                                    " (Optional)\nCopied to Clipboard: ",
                                    step.clipboard.as_ref().unwrap(),
                                ];
                                text = parts.concat();
                                let _ = copy_to_clipboard(step.clipboard.as_ref().unwrap().clone());
                            } else {
                                text = step.content.as_ref().unwrap().to_owned();
                                text.push_str(" (Optional)");
                            }
                            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .with_prompt(&text)
                                .default(0)
                                .items(optional_items)
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
                Marker::Unordered => {
                    let mut next_unorders = sublist.to_vec();
                    loop {
                        let mut undone_unorders: Vec<Oatlet> = next_unorders
                            .into_iter()
                            .filter(|a| !a.done && a.content.is_some())
                            .collect();
                        if undone_unorders.len() <= 0 {
                            break;
                        };
                        let unordered_items: Vec<String> = undone_unorders
                            .iter()
                            .map(|a| match &a.clipboard {
                                Some(clip) => [
                                    a.content.as_ref().unwrap().clone(),
                                    String::from(", Clipboard: "),
                                    clip.clone(),
                                ]
                                .concat(),
                                None => a.content.as_ref().unwrap().clone(),
                            })
                            .collect();
                        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                            .with_prompt(unordered_prompt)
                            .default(0)
                            .items(&unordered_items)
                            .interact()
                            .unwrap();
                        if undone_unorders[selection].clipboard.is_some() {
                            let _ = copy_to_clipboard(
                                undone_unorders[selection].clipboard.clone().unwrap(),
                            );
                        }
                        undone_unorders[selection].done = true;
                        info!("completed one of: {}", &unordered_items[selection]);
                        next_unorders = undone_unorders;
                    }
                }
                Marker::OneOf => {
                    let one_of_items: Vec<String> = sublist
                        .iter()
                        .map(|a| match &a.clipboard {
                            Some(clip) => [
                                a.content.as_ref().unwrap().clone(),
                                String::from(", Clipboard: "),
                                clip.clone(),
                            ]
                            .concat(),
                            None => a.content.as_ref().unwrap().clone(),
                        })
                        .collect();
                    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt(one_of_prompt)
                        .default(0)
                        .items(&one_of_items)
                        .interact()
                        .unwrap();
                    if sublist[selection].clipboard.is_some() {
                        let _ = copy_to_clipboard(sublist[selection].clipboard.clone().unwrap());
                    }
                    info!("completed one of: {}", &one_of_items[selection]);
                }
                Marker::Clipbo => {
                    unreachable!()
                }
                Marker::Breaker => {}
            });

        info!("exiting sequence");
        Ok(())
    }
}

pub fn groats_to_oatlets(groats: &Vec<Groat>) -> Vec<Oatlet> {
    let mut oatlets: Vec<Oatlet> = Vec::new();
    for groat in groats {
        if groat.marker.is_some() && groat.marker.unwrap() == Marker::Clipbo && oatlets.len() > 0 {
            oatlets.last_mut().unwrap().clipboard = groat.content.clone();
        } else if groat.marker.is_some() {
            oatlets.push(Oatlet {
                marker: groat.marker.unwrap(),
                content: groat.content.clone(),
                clipboard: None,
                done: false,
            });
        }
    }
    return oatlets;
}

fn copy_to_clipboard(cb_text: String) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    match clipboard.set_text(&cb_text) {
        Ok(_) => info!("copied {} to clipboard", &cb_text,),
        Err(e) => info!("unable to copy {} to clipboard due to: {}", &cb_text, e), // Is there really not a good way to handle Result types through closures?
    }
    Ok(())
}
