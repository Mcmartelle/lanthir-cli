extern crate pest;
//#[macro_use]
extern crate pest_derive;

use anyhow::{bail, Result};
use clap::Parser as ClapParser;
use log::info;
// use pest::Parser as PestParser;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::File;
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
        let _ = WriteLogger::init(LevelFilter::Info, config, File::create(log_dir).unwrap());
    }

    info!("does this only appear in log file if simplelogger isn't initialzed?");
    println!("Hello, world!");
    Ok(())
}
