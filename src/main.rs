extern crate pest;
//#[macro_use]
extern crate pest_derive;

use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Local, Timelike};
use clap::Parser as ClapParser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use lanthir_cli::checklist::parse_checklist;
use lanthir_cli::checklist_runner::{Checkify, ChecklistMachine};
use lanthir_cli::graph_runner::{GraphMachine, Traverse};
use lanthir_cli::mermaid::parse_mermaid;
use lanthir_cli::oats::parse_oats;
use lanthir_cli::oats_runner::{Oatify, OatsMachine};
#[allow(unused_imports)]
use pest::Parser as PestParser;
use sha2::{Digest, Sha256};
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs;
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(long)]
    log_path: Option<PathBuf>,
    #[arg(short, long)]
    log: Option<bool>,
    #[arg(long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.input {
        Some(input) => match input.extension() {
            Some(os_ext) => match os_ext.to_str() {
                Some(ext) => match ext {
                    "mmd" => {
                        let flowchart_string = fs::read_to_string(&input)?;

                        setup_logging(args.log, args.log_path, &input, &flowchart_string)?;

                        let flowchart_graph = parse_mermaid(&flowchart_string, args.verbose)?;
                        let mut flowchart_runner =
                            GraphMachine::new(String::from("Start"), flowchart_graph);
                        flowchart_runner.run()?;
                    }

                    "oats" => {
                        let oats_string = fs::read_to_string(&input)?;
                        
                        setup_logging(args.log, args.log_path, &input, &oats_string)?;
                        
                        let oats = parse_oats(&oats_string, args.verbose)?;
                        let mut oats_runner = OatsMachine::new(oats);
                        oats_runner.run()?;
                    }

                    "ckl" | "txt" => {
                        let checklist_string = fs::read_to_string(&input)?;

                        setup_logging(args.log, args.log_path, &input, &checklist_string)?;

                        let checklist = parse_checklist(&checklist_string, args.verbose)?;
                        let mut checklist_runner =
                            ChecklistMachine::new(String::from("Start"), checklist);
                        checklist_runner.run()?;
                    }
                    _ => {
                        bail!("file extension not supported");
                    }
                },
                None => {
                    bail!("File extension unable to convert from OsStr");
                }
            },
            None => {
                bail!("No file extension");
            }
        },
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

fn setup_logging(log_flag: Option<bool>, custom_log_dir: Option<PathBuf>, input_filename: &PathBuf, input_file: &String ) -> Result<()> {
    if log_flag.unwrap_or(true) {
        let log_path: Option<PathBuf> = match custom_log_dir {
            Some(mut path) => {
                path.push("test.log");
                Some(path)
            }
            None => match home::home_dir() {
                Some(mut path) => {
                    path.push(".lanthir");
                    path.push("logs");

                    let filename = generate_logfile_name(
                        input_filename.file_stem().unwrap().to_str().unwrap(),
                        &input_file,
                    )?;
                    path.push(filename);
                    Some(path)
                }
                None => {
                    println!("Unable to get home directory for logging.");
                    println!("Use --log <path> to manually set a logging directory");
                    None
                },
            },
        };
        
        let should_create_log_file: bool = match log_path {
            Some(ref path) => {
                let log_dir = path.parent().expect("no parent of path");// Should be ~/.lanthir/
                match log_dir.exists() {
                    true => true,
                    false => {
                        let log_items = &["Yes", "No"];
                        let mut log_prompt = String::from("Create directory for log files at: ");
                        log_prompt.push_str(log_dir.to_str().expect("the path should be convertable to a string"));
                        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                            .with_prompt(log_prompt)
                            .default(0)
                            .items(&log_items[..])
                            .interact()
                            .unwrap();
                        match selection {
                            0 => {
                                fs::create_dir_all(log_dir)?;
                                true
                            }
                            _ => {
                                false
                            }
                        }
                    }
                }
            }
            None => false
        };
        
        if should_create_log_file {
            match log_path {
                Some(path) => {
                    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
                    let _ = WriteLogger::init(
                        LevelFilter::Info,
                        config,
                        fs::File::create(path).expect("the log dir should exist for the log file to be created in"),
                    );
                }
                None => {}
            }
        }
        
    }   
    Ok(())
}
