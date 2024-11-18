//! Issue board
//! 

use chrono::{DateTime, Local};
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueBoard {
  pub(crate) issues: Vec<Issue>
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct Issue {
  pub title: Option<String>,
  pub author: String,
  pub summary: String,
  pub url: String,
  pub recommended: String,
  pub date: Option<DateTime<Local>>
}

impl IssueBoard {
  // Get an iterator over issues
  pub fn issues(&self) -> Iter<'_, Issue> {
    self.issues.iter()
  }
}

