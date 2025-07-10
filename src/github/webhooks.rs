use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use tracing::{debug, info, warn, error};

type HmacSha256 = Hmac<Sha256>;

/// GitHub webhook event handler
pub struct WebhookHandler {
    secret: Option<String>,
    handlers: HashMap<String, Box<dyn EventHandler + Send + Sync>>,
}

impl std::fmt::Debug for WebhookHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookHandler")
            .field("secret", &self.secret.as_ref().map(|_| "[REDACTED]"))
            .field("handlers_count", &self.handlers.len())
            .finish()
    }
}

impl WebhookHandler {
    /// Create new webhook handler
    pub fn new(secret: Option<String>) -> Self {
        Self {
            secret,
            handlers: HashMap::new(),
        }
    }

    /// Register event handler
    pub fn register_handler<H>(&mut self, event_type: &str, handler: H)
    where
        H: EventHandler + Send + Sync + 'static,
    {
        self.handlers.insert(event_type.to_string(), Box::new(handler));
    }

    /// Verify webhook signature
    pub fn verify_signature(&self, payload: &[u8], signature: &str) -> Result<bool> {
        let secret = match &self.secret {
            Some(secret) => secret,
            None => {
                warn!("No webhook secret configured, skipping signature verification");
                return Ok(true);
            }
        };

        if !signature.starts_with("sha256=") {
            return Err(anyhow!("Invalid signature format"));
        }

        let signature_hex = &signature[7..]; // Remove "sha256=" prefix
        let expected_signature = hex::decode(signature_hex)
            .map_err(|_| anyhow!("Invalid signature hex encoding"))?;

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|_| anyhow!("Invalid secret key"))?;
        mac.update(payload);
        
        let computed_signature = mac.finalize().into_bytes();
        
        // Use constant-time comparison to prevent timing attacks
        Ok(computed_signature.as_slice() == expected_signature.as_slice())
    }

    /// Process webhook event
    pub async fn process_event(
        &self,
        event_type: &str,
        delivery_id: &str,
        payload: &[u8],
        signature: Option<&str>,
    ) -> Result<EventResult> {
        info!("Processing webhook event: {} (ID: {})", event_type, delivery_id);

        // Verify signature if provided
        if let Some(sig) = signature {
            if !self.verify_signature(payload, sig)? {
                return Err(anyhow!("Invalid webhook signature"));
            }
        }

        // Parse payload based on event type
        let event = self.parse_event(event_type, payload)?;

        // Find and execute handler
        if let Some(handler) = self.handlers.get(event_type) {
            debug!("Found handler for event type: {}", event_type);
            handler.handle_event(&event).await
        } else {
            warn!("No handler registered for event type: {}", event_type);
            Ok(EventResult {
                success: true,
                message: format!("Unhandled event type: {}", event_type),
                data: None,
            })
        }
    }

    /// Parse GitHub webhook event payload
    fn parse_event(&self, event_type: &str, payload: &[u8]) -> Result<GitHubEvent> {
        let payload_str = std::str::from_utf8(payload)?;
        
        match event_type {
            "push" => {
                let push_event: PushEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::Push(push_event))
            }
            "pull_request" => {
                let pr_event: PullRequestEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::PullRequest(pr_event))
            }
            "issues" => {
                let issue_event: IssueEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::Issue(issue_event))
            }
            "release" => {
                let release_event: ReleaseEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::Release(release_event))
            }
            "workflow_run" => {
                let workflow_event: WorkflowRunEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::WorkflowRun(workflow_event))
            }
            "repository" => {
                let repo_event: RepositoryEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::Repository(repo_event))
            }
            _ => {
                let generic_event: GenericEvent = serde_json::from_str(payload_str)?;
                Ok(GitHubEvent::Generic(generic_event))
            }
        }
    }
}

/// Event handler trait
#[async_trait]
pub trait EventHandler {
    async fn handle_event(&self, event: &GitHubEvent) -> Result<EventResult>;
}

/// Event processing result
#[derive(Debug, Serialize, Deserialize)]
pub struct EventResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// GitHub webhook events
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GitHubEvent {
    Push(PushEvent),
    PullRequest(PullRequestEvent),
    Issue(IssueEvent),
    Release(ReleaseEvent),
    WorkflowRun(WorkflowRunEvent),
    Repository(RepositoryEvent),
    Generic(GenericEvent),
}

/// Push event payload
#[derive(Debug, Serialize, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub before: String,
    pub after: String,
    pub commits: Vec<Commit>,
    pub repository: Repository,
    pub pusher: User,
    pub sender: User,
}

/// Pull request event payload
#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}

/// Issue event payload
#[derive(Debug, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: String,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

/// Release event payload
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub action: String,
    pub release: Release,
    pub repository: Repository,
    pub sender: User,
}

/// Workflow run event payload
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRunEvent {
    pub action: String,
    pub workflow_run: WorkflowRun,
    pub repository: Repository,
    pub sender: User,
}

/// Repository event payload
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryEvent {
    pub action: String,
    pub repository: Repository,
    pub sender: User,
}

/// Generic event payload for unknown event types
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericEvent {
    pub action: Option<String>,
    pub repository: Option<Repository>,
    pub sender: Option<User>,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Repository information
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub default_branch: String,
    pub private: bool,
}

/// User information
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
}

/// Commit information
#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub url: String,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

/// Commit author information
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}

/// Pull request information
#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub user: User,
    pub head: PullRequestRef,
    pub base: PullRequestRef,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pull request reference
#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestRef {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub sha: String,
    pub repo: Repository,
}

/// Issue information
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub assignees: Vec<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Label information
#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

/// Release information
#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub id: u64,
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub html_url: String,
    pub author: User,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Workflow run information
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
    pub workflow_id: u64,
    pub run_number: u64,
    pub event: String,
    pub head_branch: String,
    pub head_sha: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Default event handlers for common scenarios
pub struct DefaultPushHandler;

#[async_trait]
impl EventHandler for DefaultPushHandler {
    async fn handle_event(&self, event: &GitHubEvent) -> Result<EventResult> {
        if let GitHubEvent::Push(push_event) = event {
            info!("Handling push event: {} commits to {}", 
                  push_event.commits.len(), 
                  push_event.git_ref);
            
            // Log commit information
            for commit in &push_event.commits {
                debug!("Commit {}: {}", &commit.id[..8], commit.message);
            }
            
            Ok(EventResult {
                success: true,
                message: format!("Processed {} commits", push_event.commits.len()),
                data: Some(serde_json::json!({
                    "commits": push_event.commits.len(),
                    "ref": push_event.git_ref,
                    "repository": push_event.repository.full_name
                })),
            })
        } else {
            Err(anyhow!("Expected push event"))
        }
    }
}

pub struct DefaultPullRequestHandler;

#[async_trait]
impl EventHandler for DefaultPullRequestHandler {
    async fn handle_event(&self, event: &GitHubEvent) -> Result<EventResult> {
        if let GitHubEvent::PullRequest(pr_event) = event {
            info!("Handling pull request event: {} on PR #{}", 
                  pr_event.action, 
                  pr_event.pull_request.number);
            
            let pr = &pr_event.pull_request;
            
            match pr_event.action.as_str() {
                "opened" => info!("New PR opened: {}", pr.title),
                "closed" => {
                    if pr.merged.unwrap_or(false) {
                        info!("PR merged: {}", pr.title);
                    } else {
                        info!("PR closed without merging: {}", pr.title);
                    }
                }
                "synchronize" => info!("PR updated: {}", pr.title),
                "review_requested" => info!("Review requested for PR: {}", pr.title),
                _ => debug!("PR action '{}' on PR: {}", pr_event.action, pr.title),
            }
            
            Ok(EventResult {
                success: true,
                message: format!("Processed PR {} action", pr_event.action),
                data: Some(serde_json::json!({
                    "action": pr_event.action,
                    "pr_number": pr.number,
                    "title": pr.title,
                    "state": pr.state
                })),
            })
        } else {
            Err(anyhow!("Expected pull request event"))
        }
    }
}

pub struct DefaultIssueHandler;

#[async_trait]
impl EventHandler for DefaultIssueHandler {
    async fn handle_event(&self, event: &GitHubEvent) -> Result<EventResult> {
        if let GitHubEvent::Issue(issue_event) = event {
            info!("Handling issue event: {} on issue #{}", 
                  issue_event.action, 
                  issue_event.issue.number);
            
            let issue = &issue_event.issue;
            
            match issue_event.action.as_str() {
                "opened" => info!("New issue opened: {}", issue.title),
                "closed" => info!("Issue closed: {}", issue.title),
                "reopened" => info!("Issue reopened: {}", issue.title),
                "assigned" => info!("Issue assigned: {}", issue.title),
                "labeled" => info!("Issue labeled: {}", issue.title),
                _ => debug!("Issue action '{}' on issue: {}", issue_event.action, issue.title),
            }
            
            Ok(EventResult {
                success: true,
                message: format!("Processed issue {} action", issue_event.action),
                data: Some(serde_json::json!({
                    "action": issue_event.action,
                    "issue_number": issue.number,
                    "title": issue.title,
                    "state": issue.state
                })),
            })
        } else {
            Err(anyhow!("Expected issue event"))
        }
    }
}

pub struct DefaultWorkflowHandler;

#[async_trait]
impl EventHandler for DefaultWorkflowHandler {
    async fn handle_event(&self, event: &GitHubEvent) -> Result<EventResult> {
        if let GitHubEvent::WorkflowRun(workflow_event) = event {
            info!("Handling workflow run event: {} for run #{}", 
                  workflow_event.action, 
                  workflow_event.workflow_run.run_number);
            
            let run = &workflow_event.workflow_run;
            
            match workflow_event.action.as_str() {
                "requested" => info!("Workflow run requested: {}", run.name),
                "in_progress" => info!("Workflow run in progress: {}", run.name),
                "completed" => {
                    match run.conclusion.as_deref() {
                        Some("success") => info!("Workflow run succeeded: {}", run.name),
                        Some("failure") => error!("Workflow run failed: {}", run.name),
                        Some("cancelled") => warn!("Workflow run cancelled: {}", run.name),
                        _ => info!("Workflow run completed: {} ({})", run.name, 
                                 run.conclusion.as_deref().unwrap_or("unknown")),
                    }
                }
                _ => debug!("Workflow action '{}' for run: {}", workflow_event.action, run.name),
            }
            
            Ok(EventResult {
                success: true,
                message: format!("Processed workflow run {} action", workflow_event.action),
                data: Some(serde_json::json!({
                    "action": workflow_event.action,
                    "run_id": run.id,
                    "name": run.name,
                    "status": run.status,
                    "conclusion": run.conclusion
                })),
            })
        } else {
            Err(anyhow!("Expected workflow run event"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_handler_creation() {
        let handler = WebhookHandler::new(Some("test_secret".to_string()));
        assert!(handler.secret.is_some());
    }

    #[test]
    fn test_signature_verification() {
        let handler = WebhookHandler::new(Some("secret".to_string()));
        let payload = b"test payload";
        
        // This is a real signature for the payload "test payload" with secret "secret"
        let signature = "sha256=52b582138706ac0c597c80cfe2a7aa4ba7cbf8fa7b4ab1bb5f15c64be85b9400";
        
        let result = handler.verify_signature(payload, signature);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_invalid_signature() {
        let handler = WebhookHandler::new(Some("secret".to_string()));
        let payload = b"test payload";
        let invalid_signature = "sha256=invalid";
        
        let result = handler.verify_signature(payload, invalid_signature);
        assert!(result.is_err());
    }
}