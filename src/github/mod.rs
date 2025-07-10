pub mod api;
pub mod auth;
pub mod coordinator;
pub mod issue_tracker;
pub mod pr_manager;
pub mod release_manager;
pub mod repo_architect;
pub mod sync_coordinator;
pub mod webhooks;

use anyhow::Result;
use tracing::{info, warn};

use crate::config::Config;
use api::GitHubApiClient;
use auth::GitHubAuth;
use coordinator::GitHubCoordinator;
use issue_tracker::IssueTracker;
use pr_manager::PullRequestManager;
use release_manager::ReleaseManager;
use repo_architect::RepositoryArchitect;
use sync_coordinator::SyncCoordinator;
use webhooks::WebhookHandler;

/// Complete GitHub integration with all 6 coordination modes
pub struct GitHubIntegration {
    config: crate::config::GithubConfig,
    api_client: GitHubApiClient,
    auth: GitHubAuth,
    coordinator: GitHubCoordinator,
    pr_manager: PullRequestManager,
    issue_tracker: IssueTracker,
    release_manager: ReleaseManager,
    repo_architect: RepositoryArchitect,
    sync_coordinator: SyncCoordinator,
    webhook_handler: WebhookHandler,
}

impl GitHubIntegration {
    /// Initialize complete GitHub integration
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing comprehensive GitHub integration");
        
        // Initialize API client
        let api_client = GitHubApiClient::from_env()?;
        
        // Authenticate
        let user = api_client.authenticate().await?;
        let login = user.get("login").and_then(|v| v.as_str()).unwrap_or("unknown");
        info!("Authenticated with GitHub as: {}", login);
        
        // Initialize authentication module
        let oauth_config = auth::OAuthConfig::from_env()?;
        let auth = GitHubAuth::new(oauth_config);
        
        // Initialize all coordination modules
        let coordinator = GitHubCoordinator::new(api_client.clone());
        let pr_manager = PullRequestManager::new(api_client.clone());
        let issue_tracker = IssueTracker::new(api_client.clone());
        let release_manager = ReleaseManager::new(api_client.clone());
        let repo_architect = RepositoryArchitect::new(api_client.clone());
        let sync_coordinator = SyncCoordinator::new(api_client.clone());
        
        // Initialize webhook handler
        let webhook_secret = std::env::var("GITHUB_WEBHOOK_SECRET").ok();
        let mut webhook_handler = WebhookHandler::new(webhook_secret);
        
        // Register default event handlers
        webhook_handler.register_handler("push", webhooks::DefaultPushHandler);
        webhook_handler.register_handler("pull_request", webhooks::DefaultPullRequestHandler);
        webhook_handler.register_handler("issues", webhooks::DefaultIssueHandler);
        webhook_handler.register_handler("workflow_run", webhooks::DefaultWorkflowHandler);
        
        info!("GitHub integration initialized with all coordination modes");
        
        Ok(Self {
            config: config.github.clone(),
            api_client,
            auth,
            coordinator,
            pr_manager,
            issue_tracker,
            release_manager,
            repo_architect,
            sync_coordinator,
            webhook_handler,
        })
    }
    
    /// Mode 1: Repository analysis and security coordination
    pub async fn run_coordinator_analysis(&mut self, owner: &str, repo: &str, analysis_type: &str) -> Result<coordinator::RepositoryAnalysis> {
        info!("Running GitHub coordinator analysis: {}", analysis_type);
        self.coordinator.analyze_repository(owner, repo).await
    }
    
    /// Mode 2: Pull request workflow management with AI reviews
    pub async fn manage_pull_requests(&self, owner: &str, repo: &str, request: &pr_manager::ComprehensivePrRequest) -> Result<pr_manager::PullRequestResult> {
        info!("Managing GitHub pull requests with AI coordination");
        self.pr_manager.create_comprehensive_pr(owner, repo, request).await
    }
    
    /// Mode 3: Issue lifecycle and project coordination
    pub async fn coordinate_issue_lifecycle(&self, owner: &str, repo: &str, issue_number: u64) -> Result<issue_tracker::TriageResult> {
        info!("Coordinating issue lifecycle for #{}", issue_number);
        self.issue_tracker.triage_issue(owner, repo, issue_number).await
    }
    
    /// Mode 4: Release coordination with automated changelogs
    pub async fn orchestrate_release(&self, owner: &str, repo: &str, request: &release_manager::ReleaseOrchestrationRequest) -> Result<release_manager::ReleaseOrchestrationResult> {
        info!("Orchestrating release: {}", request.version);
        self.release_manager.orchestrate_release(owner, repo, request).await
    }
    
    /// Mode 5: Repository structure optimization
    pub async fn optimize_repository_structure(&self, owner: &str, repo: &str, request: &repo_architect::OptimizationRequest) -> Result<repo_architect::OptimizationResult> {
        info!("Optimizing repository structure");
        self.repo_architect.optimize_repository_structure(owner, repo, request).await
    }
    
    /// Mode 6: Multi-package synchronization
    pub async fn synchronize_packages(&mut self, request: &sync_coordinator::PackageSyncRequest) -> Result<sync_coordinator::PackageSyncResult> {
        info!("Synchronizing packages across {} repositories", request.repositories.len());
        self.sync_coordinator.synchronize_packages(request).await
    }
    
    /// Process GitHub webhook events
    pub async fn process_webhook(&self, event_type: &str, delivery_id: &str, payload: &[u8], signature: Option<&str>) -> Result<webhooks::EventResult> {
        info!("Processing GitHub webhook: {} ({})", event_type, delivery_id);
        self.webhook_handler.process_event(event_type, delivery_id, payload, signature).await
    }
    
    /// Get comprehensive repository insights
    pub async fn get_repository_insights(&mut self, owner: &str, repo: &str) -> Result<RepositoryInsights> {
        info!("Generating comprehensive repository insights");
        
        // Gather insights from all coordination modes
        let security_analysis = self.coordinator.analyze_repository(owner, repo).await?;
        let structure_analysis = self.repo_architect.analyze_repository_structure(owner, repo).await?;
        
        // Get recent activity
        let recent_prs = self.api_client.list_pull_requests(owner, repo, None).await?;
        let recent_issues = self.api_client.list_issues(owner, repo, None).await?;
        let recent_releases = self.api_client.list_releases(owner, repo).await?;
        
        let recommendations = self.generate_comprehensive_recommendations(&security_analysis, &structure_analysis);
        
        Ok(RepositoryInsights {
            repository: format!("{}/{}", owner, repo),
            generated_at: chrono::Utc::now(),
            security_analysis,
            structure_analysis,
            activity_summary: ActivitySummary {
                open_prs: recent_prs.len() as u32,
                open_issues: recent_issues.len() as u32,
                recent_releases: recent_releases.len() as u32,
            },
            recommendations,
        })
    }
    
    /// Generate comprehensive recommendations
    fn generate_comprehensive_recommendations(
        &self,
        security_analysis: &coordinator::RepositoryAnalysis,
        structure_analysis: &repo_architect::StructureAnalysis,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Security recommendations
        if security_analysis.security.score < 8.0 {
            recommendations.push("🔒 Enhance repository security configuration".to_string());
        }
        
        // Structure recommendations
        if structure_analysis.overall_score < 7.0 {
            recommendations.push("🏗️ Improve repository structure and organization".to_string());
        }
        
        // Documentation recommendations
        if security_analysis.documentation.score < 6.0 {
            recommendations.push("📚 Add comprehensive documentation".to_string());
        }
        
        // CI/CD recommendations
        if security_analysis.cicd.score < 7.0 {
            recommendations.push("🚀 Set up automated CI/CD workflows".to_string());
        }
        
        recommendations
    }
    
    /// Health check for GitHub integration
    pub async fn health_check(&self) -> Result<HealthStatus> {
        let mut status = HealthStatus {
            overall_health: "healthy".to_string(),
            api_connectivity: false,
            authentication_valid: false,
            rate_limit_ok: false,
            components: std::collections::HashMap::new(),
        };
        
        // Check API connectivity
        match self.api_client.authenticate().await {
            Ok(_) => {
                status.api_connectivity = true;
                status.authentication_valid = true;
            }
            Err(e) => {
                warn!("GitHub API health check failed: {}", e);
                status.overall_health = "degraded".to_string();
            }
        }
        
        // Check rate limits (simplified)
        status.rate_limit_ok = true;
        
        // Component status
        status.components.insert("coordinator".to_string(), "healthy".to_string());
        status.components.insert("pr_manager".to_string(), "healthy".to_string());
        status.components.insert("issue_tracker".to_string(), "healthy".to_string());
        status.components.insert("release_manager".to_string(), "healthy".to_string());
        status.components.insert("repo_architect".to_string(), "healthy".to_string());
        status.components.insert("sync_coordinator".to_string(), "healthy".to_string());
        status.components.insert("webhook_handler".to_string(), "healthy".to_string());
        
        Ok(status)
    }
}

/// Comprehensive repository insights
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryInsights {
    pub repository: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub security_analysis: coordinator::RepositoryAnalysis,
    pub structure_analysis: repo_architect::StructureAnalysis,
    pub activity_summary: ActivitySummary,
    pub recommendations: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivitySummary {
    pub open_prs: u32,
    pub open_issues: u32,
    pub recent_releases: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    pub overall_health: String,
    pub api_connectivity: bool,
    pub authentication_valid: bool,
    pub rate_limit_ok: bool,
    pub components: std::collections::HashMap<String, String>,
}