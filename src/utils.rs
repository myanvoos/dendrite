//! Utilities
//! 
//! These are utility functions, mainly path handling for now

use core::panic;
use std::{env, fs::canonicalize, path::{Path, PathBuf}};
use url::Url;

// Checks if a path is relative
pub fn is_relative_path(local_path: &String) -> bool {
  return local_path.starts_with("./") || local_path.starts_with("../") || !Path::new(local_path).is_absolute()
}

// Validates passed URL for remote repository access
pub fn validate_url(input: &String) -> Result<url::Url, url::ParseError> {
  Url::parse(&input)
}

// Converts relative paths to absolute paths for consistency across the application
pub fn relative_to_absolute_path(local_path: &String) -> Result<PathBuf, std::io::Error> {
  let current_path = env::current_dir().unwrap_or_else(|e| {
    eprintln!("Error getting current directory: {}", e);
    PathBuf::new()
  });
  let mut absolute_path = PathBuf::from(current_path);

  if is_relative_path(local_path) {
    absolute_path.push(local_path);
  } else {
    absolute_path = PathBuf::from(local_path);
  }

  canonicalize(absolute_path)
}

// Extract the owner and the repo information from the path string
pub fn extract_owner_repo(path: &str) -> (String, String) {
  let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
  println!("{:?}", parts);
  if parts.len() >= 2 {
    let owner = parts[0].to_string();
    let repo = parts[1].to_string();
    ( owner, repo )
  } else {
    panic!("Invalid path: {}. An example of a valid path is https://github.com/myanvoos/dendrite", path);
  }
}