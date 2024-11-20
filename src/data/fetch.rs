use octocrab::{self, models::Repository};

pub async fn fetch_github(owner: &str, repo: &str) -> Result<Repository, octocrab::Error> {
  let instance = octocrab::instance();
  let repos = instance.repos(owner, repo);
  let repo = repos.get().await?;

  Ok(repo)
}