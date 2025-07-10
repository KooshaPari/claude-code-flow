use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
#[cfg(feature = "github")]
use octocrab::{Octocrab, OctocrabBuilder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// GitHub API rate limit information
#[derive(Debug, Clone)]
pub struct RateLimit {
    pub remaining: u32,
    pub reset_time: DateTime<Utc>,
    pub limit: u32,
}

/// GitHub API client with rate limiting and error handling
#[derive(Debug, Clone)]
pub struct GitHubApiClient {
    octocrab: Octocrab,
    rate_limit: Arc<Mutex<Option<RateLimit>>>,
    last_request: Arc<Mutex<Instant>>,
}

#[cfg(feature = "github")]
impl GitHubApiClient {
    /// Create a new GitHub API client
    pub fn new(token: String) -> Result<Self> {
        let octocrab = OctocrabBuilder::new()
            .personal_token(token)
            .build()?;

        Ok(Self {
            octocrab,
            rate_limit: Arc::new(Mutex::new(None)),
            last_request: Arc::new(Mutex::new(Instant::now())),
        })
    }

    /// Create client from environment variable
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| anyhow!("GITHUB_TOKEN environment variable not found"))?;
        Self::new(token)
    }

    /// Authenticate and verify token
    pub async fn authenticate(&self) -> Result<serde_json::Value> {
        info!("Authenticating with GitHub API");
        let user = self.octocrab.current().user().await?;
        info!("Authenticated as: {}", user.login);
        Ok(serde_json::to_value(user)?)
    }

    /// Check rate limit before making requests
    pub async fn check_rate_limit(&self) -> Result<()> {
        if let Ok(rate_limit) = self.rate_limit.lock() {
            if let Some(ref limit) = *rate_limit {
                if limit.remaining <= 1 {
                    let wait_time = limit.reset_time
                        .signed_duration_since(Utc::now())
                        .to_std()
                        .unwrap_or(Duration::from_secs(60));
                    
                    if !wait_time.is_zero() {
                        warn!("Rate limit exceeded, waiting {:?}", wait_time);
                        tokio::time::sleep(wait_time).await;
                    }
                }
            }
        }
        Ok(())
    }

    /// Update rate limit information from response headers
    pub fn update_rate_limit(&self, remaining: u32, reset: i64, limit: u32) {
        if let Ok(mut rate_limit) = self.rate_limit.lock() {
            *rate_limit = Some(RateLimit {
                remaining,
                reset_time: DateTime::from_timestamp(reset, 0).unwrap_or_else(Utc::now),
                limit,
            });
        }
    }

    /// Get repository information
    #[cfg(feature = "github")]
    pub async fn get_repository(&self, owner: &str, repo: &str) -> Result<octocrab::models::Repository> {
        self.check_rate_limit().await?;
        debug!("Getting repository: {}/{}", owner, repo);
        Ok(self.octocrab.repos(owner, repo).get().await?)
    }

    /// List repositories for the authenticated user
    #[cfg(feature = "github")]
    pub async fn list_repositories(&self) -> Result<Vec<octocrab::models::Repository>> {
        self.check_rate_limit().await?;
        debug!("Listing repositories");
        let page = self.octocrab
            .current()
            .list_repos_for_authenticated_user()
            .send()
            .await?;
        Ok(page.items)
    }

    /// Create a new repository
    pub async fn create_repository(&self, repo_data: &CreateRepositoryRequest) -> Result<octocrab::models::Repository> {
        self.check_rate_limit().await?;
        info!("Creating repository: {}", repo_data.name);
        
        let mut builder = self.octocrab.repos().create(&repo_data.name);
        
        if let Some(ref description) = repo_data.description {
            builder = builder.description(description);
        }
        
        if let Some(ref homepage) = repo_data.homepage {
            builder = builder.homepage(homepage);
        }
        
        if repo_data.private {
            builder = builder.private(true);
        }
        
        Ok(builder.send().await?)
    }

    /// List pull requests
    pub async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        state: Option<octocrab::params::State>,
    ) -> Result<Vec<octocrab::models::pulls::PullRequest>> {
        self.check_rate_limit().await?;
        debug!("Listing pull requests for {}/{}", owner, repo);
        
        let mut builder = self.octocrab.pulls(owner, repo).list();
        if let Some(state) = state {
            let pr_state = match state {
                octocrab::params::State::Open => octocrab::params::pulls::State::Open,
                octocrab::params::State::Closed => octocrab::params::pulls::State::Closed,
                octocrab::params::State::All => octocrab::params::pulls::State::All,
            };
            builder = builder.state(pr_state);
        }
        
        let page = builder.send().await?;
        Ok(page.items)
    }

    /// Create a pull request
    pub async fn create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        pr_data: &CreatePullRequestRequest,
    ) -> Result<octocrab::models::pulls::PullRequest> {
        self.check_rate_limit().await?;
        info!("Creating pull request: {}", pr_data.title);
        
        let pr = self.octocrab
            .pulls(owner, repo)
            .create(&pr_data.title, &pr_data.head, &pr_data.base)
            .body(pr_data.body.as_deref().unwrap_or(""))
            .send()
            .await?;
        
        Ok(pr)
    }

    /// Update a pull request
    pub async fn update_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        update_data: &UpdatePullRequestRequest,
    ) -> Result<octocrab::models::pulls::PullRequest> {
        self.check_rate_limit().await?;
        debug!("Updating pull request #{}", number);
        
        let mut builder = self.octocrab.pulls(owner, repo).update(number);
        
        if let Some(ref title) = update_data.title {
            builder = builder.title(title);
        }
        
        if let Some(ref body) = update_data.body {
            builder = builder.body(body);
        }
        
        if let Some(ref state) = update_data.state {
            builder = builder.state(state.clone());
        }
        
        Ok(builder.send().await?)
    }

    /// List issues
    pub async fn list_issues(
        &self,
        owner: &str,
        repo: &str,
        state: Option<octocrab::params::State>,
    ) -> Result<Vec<octocrab::models::issues::Issue>> {
        self.check_rate_limit().await?;
        debug!("Listing issues for {}/{}", owner, repo);
        
        let mut builder = self.octocrab.issues(owner, repo).list();
        if let Some(state) = state {
            builder = builder.state(state);
        }
        
        let page = builder.send().await?;
        Ok(page.items)
    }

    /// Create an issue
    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_data: &CreateIssueRequest,
    ) -> Result<octocrab::models::issues::Issue> {
        self.check_rate_limit().await?;
        info!("Creating issue: {}", issue_data.title);
        
        let mut builder = self.octocrab.issues(owner, repo).create(&issue_data.title);
        
        if let Some(ref body) = issue_data.body {
            builder = builder.body(body);
        }
        
        if !issue_data.labels.is_empty() {
            builder = builder.labels(issue_data.labels.clone());
        }
        
        if !issue_data.assignees.is_empty() {
            builder = builder.assignees(issue_data.assignees.clone());
        }
        
        Ok(builder.send().await?)
    }

    /// Update an issue
    pub async fn update_issue(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        update_data: &UpdateIssueRequest,
    ) -> Result<octocrab::models::issues::Issue> {
        self.check_rate_limit().await?;
        debug!("Updating issue #{}", number);
        
        let mut builder = self.octocrab.issues(owner, repo).update(number);
        
        if let Some(ref title) = update_data.title {
            builder = builder.title(title);
        }
        
        if let Some(ref body) = update_data.body {
            builder = builder.body(body);
        }
        
        if let Some(ref state) = update_data.state {
            builder = builder.state(state.clone());
        }
        
        Ok(builder.send().await?)
    }

    /// List releases
    pub async fn list_releases(&self, owner: &str, repo: &str) -> Result<Vec<octocrab::models::repos::Release>> {
        self.check_rate_limit().await?;
        debug!("Listing releases for {}/{}", owner, repo);
        
        let page = self.octocrab.repos(owner, repo).releases().list().send().await?;
        Ok(page.items)
    }

    /// Create a release
    pub async fn create_release(
        &self,
        owner: &str,
        repo: &str,
        release_data: &CreateReleaseRequest,
    ) -> Result<octocrab::models::repos::Release> {
        self.check_rate_limit().await?;
        info!("Creating release: {}", release_data.tag_name);
        
        let mut builder = self.octocrab
            .repos(owner, repo)
            .releases()
            .create(&release_data.tag_name);
        
        if let Some(ref target) = release_data.target_commitish {
            builder = builder.target_commitish(target);
        }
        
        if let Some(ref name) = release_data.name {
            builder = builder.name(name);
        }
        
        if let Some(ref body) = release_data.body {
            builder = builder.body(body);
        }
        
        if release_data.draft {
            builder = builder.draft(true);
        }
        
        if release_data.prerelease {
            builder = builder.prerelease(true);
        }
        
        Ok(builder.send().await?)
    }

    /// List workflow runs
    pub async fn list_workflow_runs(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<octocrab::models::workflows::Run>> {
        self.check_rate_limit().await?;
        debug!("Listing workflow runs for {}/{}", owner, repo);
        
        let page = self.octocrab
            .workflows(owner, repo)
            .list_runs("")
            .send()
            .await?;
        Ok(page.items)
    }

    /// Get workflow run
    pub async fn get_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: u64,
    ) -> Result<octocrab::models::workflows::Run> {
        self.check_rate_limit().await?;
        debug!("Getting workflow run {} for {}/{}", run_id, owner, repo);
        
        Ok(self.octocrab.workflows(owner, repo).get_run(run_id).await?)
    }

    /// List branches
    pub async fn list_branches(&self, owner: &str, repo: &str) -> Result<Vec<serde_json::Value>> {
        self.check_rate_limit().await?;
        debug!("Listing branches for {}/{}", owner, repo);
        
        let page = self.octocrab.repos(owner, repo).list_branches().send().await?;
        Ok(page.items)
    }

    /// Create a branch
    pub async fn create_branch(
        &self,
        owner: &str,
        repo: &str,
        branch_name: &str,
        sha: &str,
    ) -> Result<serde_json::Value> {
        self.check_rate_limit().await?;
        info!("Creating branch: {}", branch_name);
        
        Ok(self.octocrab
            .repos(owner, repo)
            .create_ref(&format!("refs/heads/{}", branch_name), sha)
            .await?)
    }
}

// Request/Response types for GitHub operations
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub private: bool,
    pub auto_init: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePullRequestRequest {
    pub title: String,
    pub body: Option<String>,
    pub head: String,
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePullRequestRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<octocrab::params::State>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIssueRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<octocrab::params::State>,
    pub labels: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReleaseRequest {
    pub tag_name: String,
    pub target_commitish: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_github_client_creation() {
        // This test requires a valid GitHub token
        if std::env::var("GITHUB_TOKEN").is_ok() {
            let client = GitHubApiClient::from_env();
            assert!(client.is_ok());
        }
    }

    #[test]
    fn test_rate_limit_struct() {
        let rate_limit = RateLimit {
            remaining: 5000,
            reset_time: Utc::now(),
            limit: 5000,
        };
        
        assert_eq!(rate_limit.remaining, 5000);
        assert_eq!(rate_limit.limit, 5000);
    }
}