//! Paths
//! 
//! Provides two main functions to process local and remote paths. If remote, calls Octocat API to fetch the repository

use url::Host;

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
pub fn process_remote_path(host: &Host<&str>, owner: &String, repo: &String) {
  match host {
    Host::Domain(domain) => match  *domain {
        "github.com" => {
          println!("Processing GitHub repository...");
          fetch_github
        },
        "gitlab.com" => {
          println!("Processing GitLab repository...");
          todo!()
        },
        "bitbucket.org" => {
          println!("Processing Bitbucket repository...");
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