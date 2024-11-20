use octocrab::{self, models::Repository};

pub async fn fetch_github(owner: &str, repo: &str) {
  let instance = octocrab::instance();
  let repos = instance.repos(owner, repo);
  

}