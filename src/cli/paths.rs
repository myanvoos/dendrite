//! Paths
//! 
//! Provides two main functions to process local and remote paths. If remote, calls the appropriate handler to fetch the repository

use octocrab::{models::Repository, Error};
use url::Host;
use crate::data::fetch::{self, fetch_github};

pub fn process_local_path(local_path: &String) {
  todo!()
  // get_files_folders(local_path)

  // UI draw()
}


/// This handler processes a remote repository path by distinguishing
/// between supported domain types (e.g., GitHub, GitLab, Bitbucket) 
/// and executing the appropriate logic for each domain.
///
/// # Parameters
/// - `host`: The host information of the remote URL (e.g., "github.com").
/// - `owner`: The owner of the repository.
/// - `repo`: The repository name.
///
/// # Supported Hosts
/// - `github.com`
/// - `gitlab.com`
/// - `bitbucket.org`
///
/// For unsupported hosts, a fallback is provided to log an error.
///
/// IPv4 and IPv6 address handling is included but not actively used.
pub async fn process_remote_path(host: &Host<&str>, owner: &String, repo: &String) {
  match host {
    Host::Domain(domain) => match  *domain {
        "github.com" => {
          println!("...Identified the remote repository as a GitHub repository");
          
          match fetch_github(owner, repo).await {
            Ok(repo) => {
              println!("...Fetched repository details: {:?}\n\n\n", repo);
            },
            Err(e) => {
              match e {
                Error::Http { source, backtrace: _ } => {
                  eprintln!("Encountered HTTP error: {:?}", source);
                }
                Error::Json { source, backtrace: _ } => {
                  eprintln!("Encountered JSON error: {:?}", source);
                }
                Error::JWT { source, backtrace: _ } => {
                  eprintln!("Encountered a JWT error: {:?}", source);
                }
                Error::GitHub { source, backtrace: _ } => {
                  eprintln!("Encountered a GitHub error: {:?}", source);
                }
                _ => {
                  eprintln!("Unknown error: {:?}", e)
                }
              }
            }
          };
        },
        "gitlab.com" => {
          println!("...Identified the remote repository as a GitLab repository");
          todo!()
        },
        "bitbucket.org" => {
          println!("...Identified the remote repository as a Bitbucket repository");
          todo!()
        },
        _ => {
          println!("Unsupported domain: {}", domain);
        }
    },
    Host::Ipv4(addr) => {
      println!("IPv4 address: {}", addr);
    },
    Host::Ipv6(addr) => {
      println!("IPv6 address: {}", addr);
    }
  }
}