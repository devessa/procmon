use clap::{Parser, Subcommand};
use paris::Logger;
use std::path::PathBuf;
use std::{thread, time};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    Rust {},
    Node { index: Option<PathBuf> },
}

fn main() {
    let cli = Cli::parse();

    let mut logger = Logger::new();

    match cli.debug {
        1 => {
            logger.info("debug: on");
        }
        2 => {
            logger.info("debug: verbose");
        }
        3 => {
            logger.info("debug: very verbose");
        }
        _ => {}
    }

    monitor(&cli.command, cli.debug);
}

fn monitor(command: &Option<Subcommands>, debug: u8) {
    let lang;
    let mut index: Option<PathBuf> = None;
    match command {
        Some(Subcommands::Rust {}) => lang = "rust",
        Some(Subcommands::Node { index: main_file }) => {
            lang = "node";
            index = main_file.clone();
        }
        None => {
            lang = "";
        }
    }
    let mut log = Logger::new();
    if debug > 0 && lang == "node" {
        if let Some(value) = index {
            log.info(format!("received index file: {:?}", value));
        } else {
            log.info(format!(
                "no index file provided, using default of index.js/index.ts"
            ));
        }
    }
    log.loading(format!(
        "<i><white>starting {} application in development mode...</i>",
        lang
    ));
    // mock logic
    thread::sleep(time::Duration::from_secs(2));
    log.success("<b><bright green>Running!</b>");
}
