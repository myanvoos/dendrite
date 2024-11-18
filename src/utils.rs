use std::{env, path::{Path, PathBuf}};
use url::{Url, ParseError};
use clap::ArgMatches;

// Converts relative paths to absolute paths for consistency across the application
// If the URL has './' (example: ./src/dendrite), then remove it
pub fn relative_to_absolute_path(local_matches: &ArgMatches) -> PathBuf {
  let mut local_path: String = local_matches
    .get_one::<String>("path")
    .expect("contains_id")
    .to_string();

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
pub fn validate_url(input: &PathBuf) -> Result<url::Url, url::ParseError> {
  let input_str = input.to_string_lossy();
  Url::parse(&input_str)
}