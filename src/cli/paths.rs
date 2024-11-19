//! Paths
//! 
//! Provides two main functions to process local and remote paths. If remote, calls Octocat API to fetch the repository

use url::Host;

pub fn process_local_path(local_path: &String) {
  todo!()
  // get_files_folders(local_path)

  // UI draw()
}

pub fn process_remote_path(host: &Host<&str>, owner: &String, repo: &String) {
  println!("{}", owner);
  todo!();
  match host {
    Host::Domain(domain) => match  *domain {
        "github.com" => {
          println!("Processing GitHub repository...");
          todo!()
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
    // Host reinforces Ipv4 and Ipv6 in its pattern matching. 
    // We're not actually using them.
    Host::Ipv4(addr) => {
      println!("IPv4 address: {}", addr);
    },
    Host::Ipv6(addr) => {
      println!("IPv6 address: {}", addr);
    }
  }
}