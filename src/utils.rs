use std::{env, path::{Path, PathBuf}};
use url::{Url, ParseError};
use clap::ArgMatches;

// Converts relative paths to absolute paths for consistency across the application
// If the URL has './' (example: ./src/dendrite), then remove it
pub fn relative_to_absolute_path(local_path: &String) -> PathBuf {
  let current_path = env::current_dir().unwrap_or_default();
  let mut absolute_path = PathBuf::from(current_path);

  if local_path.starts_with("./") {
    let stripped_path = local_path.strip_prefix("./").unwrap_or(&local_path);
    absolute_path.push(stripped_path);
  } else {
    absolute_path.push(local_path);
  }
  absolute_path
}

// Validate passed URL for remote repository access
pub fn validate_url(input: &String) -> Result<url::Url, url::ParseError> {
  Url::parse(&input)
}