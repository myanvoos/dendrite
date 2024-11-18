use std::{error::Error};
use clap::{Arg, ArgMatches, Command, Parser};

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
                    let absolute_path = relative_to_absolute_path(&local_path);
                    println!("Processing file path {:?}...", absolute_path);

                    // process_local_path(&local_path)
                }
            },
            Some(("remote", remote_matches)) => {
                if remote_matches.contains_id("url") {
                    let remote_path: String = remote_matches
                      .get_one::<String>("url")
                      .expect("contains_id")
                      .to_string();
                    
                    match validate_url(&remote_path) {
                        Ok(url) => {
                            println!("Processing file path {:?}...", url);

                            // process_local_path(&local_path)
                        },
                        Err(err) => {
                            eprintln!(
                                "Error: The provided input '{}' is not a valid URL. Details: {}",
                                remote_path,
                                err
                            )
                        },
                        _ => unreachable!()
                    }
                }
            }
            _ => unreachable!()
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
