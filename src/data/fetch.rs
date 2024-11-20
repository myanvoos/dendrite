use octocrab::{self, models::Repository};

pub async fn fetch_github(owner: &str, repo: &str) -> Result<Repository, octocrab::Error> {
  let instance = octocrab::instance();
  let repositories = instance.repos(owner, repo);
  let repository = repositories.get().await?;
  let repository_content = repositories.get_content().send().await?;
  println!("{:?}", repository_content);
  let repository_readme = repositories.get_readme().send().await?;
  let repository_license = repositories.license().await?;

  let issues = instance.issues(owner, repo).list();

  Ok(repository)
}