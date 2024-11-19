//! CLI commands
//!
//! Commands for the Dendrite CLI tool, using the Rust crate `clap`.
//! Structure:
//!     `dendrite local [path]` -- shorthand --> `dendrite -l [path]`
//!     `dendrite remote [remote-url]` -- shorthand --> `dendrite -r [remote-url]`
//!     `dendrite help`, -- shorthand --> `dendrite -h`

use clap::{Arg, ArgMatches, Command, Parser};
use std::{error::Error, path::PathBuf};

use crate::utils::{relative_to_absolute_path, validate_url};

#[derive(Parser, Debug)]
#[command(name = "dendrite")]
#[command(version, about, long_about = None)]
pub struct CLI;

impl CLI {
    pub fn run() -> Result<(), Box<dyn Error>> {
        let matches = CLI::parse_args();

        match matches.subcommand() {
            Some(("local", local_matches)) => {
                if local_matches.contains_id("path") {
                    let local_path: String = local_matches
                        .get_one::<String>("path")
                        .expect("contains_id")
                        .to_string();

                    let absolute_path =
                        relative_to_absolute_path(&local_path).unwrap_or_else(|e| {
                            eprintln!("Error converting relative path to absolute path: {}", e);
                            PathBuf::from("/")
                        });
                    println!("Processing file path {:?}...", absolute_path);

                    // process_local_path(&local_path)
                }
            }
            Some(("remote", remote_matches)) => {
                if remote_matches.contains_id("url") {
                    let remote_path: String = remote_matches
                        .get_one::<String>("url")
                        .expect("contains_id")
                        .to_string();

                    match validate_url(&remote_path) {
                        Ok(raw_url) => {
                            let url = raw_url.as_str();
                            println!("Processing remote repository {:?}...", url);

                            // process_remote_path(&remote_path)
                        }
                        Err(err) => {
                            eprintln!(
                                "Error: The provided input '{}' is not a valid URL. Details: {}",
                                remote_path, err
                            )
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn parse_args() -> ArgMatches {
        Command::new("dendrite")
            .version("0.1.0")
            .about("Dendrite is a tool that visualises codebases and GitHub issues.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("local")
                    .about("Visualise a local codebase")
                    .short_flag('l')
                    .arg(
                        Arg::new("path")
                            .required(true)
                            .help("Path to the local repository")
                            .num_args(1), //.action(ArgAction::Set)
                    ),
            )
            .subcommand(
                Command::new("remote")
                    .about("Visualise a remote repository")
                    .short_flag('r')
                    .arg(
                        Arg::new("url")
                            .required(true)
                            .help("Path to remote GitHub repository")
                            .num_args(1),
                    ),
            )
            .get_matches()
    }
}
