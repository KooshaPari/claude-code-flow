use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::github::api::{GitHubApiClient, CreateRepositoryRequest};

/// Repository analysis and security coordination
#[derive(Debug)]
pub struct GitHubCoordinator {
    api: GitHubApiClient,
    analysis_cache: HashMap<String, RepositoryAnalysis>,
}

impl GitHubCoordinator {
    /// Create new GitHub coordinator
    pub fn new(api: GitHubApiClient) -> Self {
        Self {
            api,
            analysis_cache: HashMap::new(),
        }
    }

    /// Analyze repository structure and security
    pub async fn analyze_repository(&mut self, owner: &str, repo: &str) -> Result<RepositoryAnalysis> {
        let repo_key = format!("{}/{}", owner, repo);
        
        info!("Starting repository analysis for {}", repo_key);
        
        // Check cache first
        if let Some(cached) = self.analysis_cache.get(&repo_key) {
            if cached.is_fresh() {
                debug!("Using cached analysis for {}", repo_key);
                return Ok(cached.clone());
            }
        }
        
        // Perform comprehensive analysis
        let repository = self.api.get_repository(owner, repo).await?;
        let mut analysis = RepositoryAnalysis::new(repo_key.clone());
        
        // Basic repository information
        analysis.basic_info = BasicRepoInfo {
            name: repository.name.clone(),
            description: repository.description.clone(),
            language: repository.language.clone(),
            size: repository.size.unwrap_or(0) as u64,
            default_branch: repository.default_branch.clone(),
            is_private: repository.private.unwrap_or(false),
            is_fork: repository.fork.unwrap_or(false),
            is_archived: repository.archived.unwrap_or(false),
            created_at: repository.created_at.unwrap_or_else(Utc::now),
            updated_at: repository.updated_at.unwrap_or_else(Utc::now),
        };
        
        // Security analysis
        analysis.security = self.analyze_security(owner, repo).await?;
        
        // Code quality analysis
        analysis.code_quality = self.analyze_code_quality(owner, repo).await?;
        
        // CI/CD analysis
        analysis.cicd = self.analyze_cicd(owner, repo).await?;
        
        // Dependency analysis
        analysis.dependencies = self.analyze_dependencies(owner, repo).await?;
        
        // Documentation analysis
        analysis.documentation = self.analyze_documentation(owner, repo).await?;
        
        // Community health analysis
        analysis.community_health = self.analyze_community_health(owner, repo).await?;
        
        // Calculate overall score
        analysis.calculate_score();
        
        // Cache the analysis
        self.analysis_cache.insert(repo_key, analysis.clone());
        
        info!("Repository analysis completed with score: {:.1}/10", analysis.overall_score);
        Ok(analysis)
    }

    /// Analyze repository security
    async fn analyze_security(&self, owner: &str, repo: &str) -> Result<SecurityAnalysis> {
        info!("Analyzing security for {}/{}", owner, repo);
        
        let mut security = SecurityAnalysis::default();
        
        // Check for security policies
        security.has_security_policy = self.check_file_exists(owner, repo, "SECURITY.md").await
            || self.check_file_exists(owner, repo, ".github/SECURITY.md").await;
        
        // Check for code scanning
        security.code_scanning_enabled = self.check_code_scanning(owner, repo).await;
        
        // Check for dependency scanning
        security.dependency_scanning_enabled = self.check_dependency_scanning(owner, repo).await;
        
        // Check for secret scanning
        security.secret_scanning_enabled = self.check_secret_scanning(owner, repo).await;
        
        // Check branch protection
        security.branch_protection = self.analyze_branch_protection(owner, repo).await?;
        
        // Analyze permissions and access
        security.access_analysis = self.analyze_repository_access(owner, repo).await?;
        
        // Calculate security score
        security.calculate_score();
        
        Ok(security)
    }

    /// Analyze code quality
    async fn analyze_code_quality(&self, owner: &str, repo: &str) -> Result<CodeQualityAnalysis> {
        info!("Analyzing code quality for {}/{}", owner, repo);
        
        let mut quality = CodeQualityAnalysis::default();
        
        // Check for linting configuration
        quality.has_linting = self.check_linting_config(owner, repo).await;
        
        // Check for testing setup
        quality.has_tests = self.check_testing_setup(owner, repo).await;
        
        // Check for code formatting
        quality.has_formatting = self.check_formatting_config(owner, repo).await;
        
        // Check for pre-commit hooks
        quality.has_pre_commit = self.check_file_exists(owner, repo, ".pre-commit-config.yaml").await;
        
        // Analyze test coverage
        quality.test_coverage = self.analyze_test_coverage(owner, repo).await;
        
        // Check for code review requirements
        quality.enforces_reviews = self.check_review_requirements(owner, repo).await;
        
        // Calculate quality score
        quality.calculate_score();
        
        Ok(quality)
    }

    /// Analyze CI/CD setup
    async fn analyze_cicd(&self, owner: &str, repo: &str) -> Result<CicdAnalysis> {
        info!("Analyzing CI/CD for {}/{}", owner, repo);
        
        let mut cicd = CicdAnalysis::default();
        
        // Check for GitHub Actions
        cicd.has_github_actions = self.check_github_actions(owner, repo).await;
        
        // Check for other CI systems
        cicd.has_other_ci = self.check_other_ci_systems(owner, repo).await;
        
        // Analyze workflow quality
        cicd.workflow_analysis = self.analyze_workflows(owner, repo).await?;
        
        // Check deployment setup
        cicd.has_deployment = self.check_deployment_setup(owner, repo).await;
        
        // Check for automated releases
        cicd.has_automated_releases = self.check_automated_releases(owner, repo).await;
        
        // Calculate CI/CD score
        cicd.calculate_score();
        
        Ok(cicd)
    }

    /// Analyze dependencies
    async fn analyze_dependencies(&self, owner: &str, repo: &str) -> Result<DependencyAnalysis> {
        info!("Analyzing dependencies for {}/{}", owner, repo);
        
        let mut deps = DependencyAnalysis::default();
        
        // Check for dependency files
        deps.has_dependency_file = self.check_dependency_files(owner, repo).await;
        
        // Check for lock files
        deps.has_lock_file = self.check_lock_files(owner, repo).await;
        
        // Check for automated dependency updates
        deps.has_automated_updates = self.check_dependabot_config(owner, repo).await;
        
        // Analyze dependency vulnerabilities
        deps.vulnerability_analysis = self.analyze_vulnerabilities(owner, repo).await?;
        
        // Check for outdated dependencies
        deps.outdated_analysis = self.analyze_outdated_deps(owner, repo).await?;
        
        // Calculate dependency score
        deps.calculate_score();
        
        Ok(deps)
    }

    /// Analyze documentation
    async fn analyze_documentation(&self, owner: &str, repo: &str) -> Result<DocumentationAnalysis> {
        info!("Analyzing documentation for {}/{}", owner, repo);
        
        let mut docs = DocumentationAnalysis::default();
        
        // Check for README
        docs.has_readme = self.check_file_exists(owner, repo, "README.md").await
            || self.check_file_exists(owner, repo, "README.rst").await
            || self.check_file_exists(owner, repo, "README.txt").await;
        
        // Check for contributing guidelines
        docs.has_contributing = self.check_file_exists(owner, repo, "CONTRIBUTING.md").await
            || self.check_file_exists(owner, repo, ".github/CONTRIBUTING.md").await;
        
        // Check for code of conduct
        docs.has_code_of_conduct = self.check_file_exists(owner, repo, "CODE_OF_CONDUCT.md").await
            || self.check_file_exists(owner, repo, ".github/CODE_OF_CONDUCT.md").await;
        
        // Check for license
        docs.has_license = self.check_file_exists(owner, repo, "LICENSE").await
            || self.check_file_exists(owner, repo, "LICENSE.md").await
            || self.check_file_exists(owner, repo, "LICENSE.txt").await;
        
        // Check for API documentation
        docs.has_api_docs = self.check_api_documentation(owner, repo).await;
        
        // Check for changelog
        docs.has_changelog = self.check_file_exists(owner, repo, "CHANGELOG.md").await
            || self.check_file_exists(owner, repo, "HISTORY.md").await;
        
        // Calculate documentation score
        docs.calculate_score();
        
        Ok(docs)
    }

    /// Analyze community health
    async fn analyze_community_health(&self, owner: &str, repo: &str) -> Result<CommunityHealthAnalysis> {
        info!("Analyzing community health for {}/{}", owner, repo);
        
        let mut community = CommunityHealthAnalysis::default();
        
        // Check issue templates
        community.has_issue_templates = self.check_issue_templates(owner, repo).await;
        
        // Check PR templates
        community.has_pr_templates = self.check_pr_templates(owner, repo).await;
        
        // Analyze recent activity
        community.activity_analysis = self.analyze_repository_activity(owner, repo).await?;
        
        // Check for maintainer responsiveness
        community.maintainer_responsiveness = self.analyze_maintainer_responsiveness(owner, repo).await?;
        
        // Calculate community score
        community.calculate_score();
        
        Ok(community)
    }

    /// Check if a file exists in the repository
    async fn check_file_exists(&self, owner: &str, repo: &str, path: &str) -> bool {
        match self.api.octocrab.repos(owner, repo).get_content().path(path).send().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Check for GitHub Actions workflows
    async fn check_github_actions(&self, owner: &str, repo: &str) -> bool {
        match self.api.list_workflow_runs(owner, repo).await {
            Ok(runs) => !runs.is_empty(),
            Err(_) => false,
        }
    }

    /// Generate security recommendations
    pub fn generate_security_recommendations(&self, analysis: &RepositoryAnalysis) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();
        
        if !analysis.security.has_security_policy {
            recommendations.push(SecurityRecommendation {
                level: RecommendationLevel::High,
                category: "Security Policy".to_string(),
                title: "Add Security Policy".to_string(),
                description: "Create a SECURITY.md file to document security procedures".to_string(),
                action: "Create a SECURITY.md file in the root or .github directory".to_string(),
            });
        }
        
        if !analysis.security.code_scanning_enabled {
            recommendations.push(SecurityRecommendation {
                level: RecommendationLevel::High,
                category: "Code Scanning".to_string(),
                title: "Enable Code Scanning".to_string(),
                description: "Enable GitHub's code scanning to detect vulnerabilities".to_string(),
                action: "Set up CodeQL analysis in GitHub Actions".to_string(),
            });
        }
        
        if analysis.security.branch_protection.score < 8.0 {
            recommendations.push(SecurityRecommendation {
                level: RecommendationLevel::Medium,
                category: "Branch Protection".to_string(),
                title: "Improve Branch Protection".to_string(),
                description: "Strengthen branch protection rules for main branch".to_string(),
                action: "Enable required reviews, status checks, and admin enforcement".to_string(),
            });
        }
        
        recommendations
    }

    /// Generate improvement plan
    pub fn generate_improvement_plan(&self, analysis: &RepositoryAnalysis) -> ImprovementPlan {
        let mut plan = ImprovementPlan::new();
        
        // Security improvements
        if analysis.security.score < 8.0 {
            plan.add_task(ImprovementTask {
                category: "Security".to_string(),
                priority: TaskPriority::High,
                title: "Enhance Security Configuration".to_string(),
                description: "Implement comprehensive security measures".to_string(),
                estimated_hours: 4,
                dependencies: Vec::new(),
            });
        }
        
        // Code quality improvements
        if analysis.code_quality.score < 7.0 {
            plan.add_task(ImprovementTask {
                category: "Code Quality".to_string(),
                priority: TaskPriority::Medium,
                title: "Improve Code Quality Tools".to_string(),
                description: "Set up linting, testing, and formatting".to_string(),
                estimated_hours: 6,
                dependencies: Vec::new(),
            });
        }
        
        // Documentation improvements
        if analysis.documentation.score < 6.0 {
            plan.add_task(ImprovementTask {
                category: "Documentation".to_string(),
                priority: TaskPriority::Medium,
                title: "Enhance Documentation".to_string(),
                description: "Add missing documentation files and improve existing ones".to_string(),
                estimated_hours: 8,
                dependencies: Vec::new(),
            });
        }
        
        plan
    }

    // Helper methods for specific checks would go here...
    // (Implementation details for each check method)
    
    async fn check_code_scanning(&self, _owner: &str, _repo: &str) -> bool {
        // Implementation would check for code scanning alerts via API
        false
    }
    
    async fn check_dependency_scanning(&self, _owner: &str, _repo: &str) -> bool {
        // Implementation would check for dependency scanning setup
        false
    }
    
    async fn check_secret_scanning(&self, _owner: &str, _repo: &str) -> bool {
        // Implementation would check for secret scanning alerts
        false
    }
    
    async fn analyze_branch_protection(&self, _owner: &str, _repo: &str) -> Result<BranchProtectionAnalysis> {
        // Implementation would analyze branch protection rules
        Ok(BranchProtectionAnalysis::default())
    }
    
    async fn analyze_repository_access(&self, _owner: &str, _repo: &str) -> Result<AccessAnalysis> {
        // Implementation would analyze repository access and permissions
        Ok(AccessAnalysis::default())
    }
    
    // Additional helper methods...
    async fn check_linting_config(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_testing_setup(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_formatting_config(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn analyze_test_coverage(&self, _owner: &str, _repo: &str) -> Option<f64> { None }
    async fn check_review_requirements(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_other_ci_systems(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn analyze_workflows(&self, _owner: &str, _repo: &str) -> Result<WorkflowAnalysis> { 
        Ok(WorkflowAnalysis::default()) 
    }
    async fn check_deployment_setup(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_automated_releases(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_dependency_files(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_lock_files(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_dependabot_config(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn analyze_vulnerabilities(&self, _owner: &str, _repo: &str) -> Result<VulnerabilityAnalysis> {
        Ok(VulnerabilityAnalysis::default())
    }
    async fn analyze_outdated_deps(&self, _owner: &str, _repo: &str) -> Result<OutdatedAnalysis> {
        Ok(OutdatedAnalysis::default())
    }
    async fn check_api_documentation(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_issue_templates(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn check_pr_templates(&self, _owner: &str, _repo: &str) -> bool { false }
    async fn analyze_repository_activity(&self, _owner: &str, _repo: &str) -> Result<ActivityAnalysis> {
        Ok(ActivityAnalysis::default())
    }
    async fn analyze_maintainer_responsiveness(&self, _owner: &str, _repo: &str) -> Result<ResponsivenessAnalysis> {
        Ok(ResponsivenessAnalysis::default())
    }
}

// Data structures for analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryAnalysis {
    pub repository: String,
    pub analyzed_at: DateTime<Utc>,
    pub basic_info: BasicRepoInfo,
    pub security: SecurityAnalysis,
    pub code_quality: CodeQualityAnalysis,
    pub cicd: CicdAnalysis,
    pub dependencies: DependencyAnalysis,
    pub documentation: DocumentationAnalysis,
    pub community_health: CommunityHealthAnalysis,
    pub overall_score: f64,
}

impl RepositoryAnalysis {
    pub fn new(repository: String) -> Self {
        Self {
            repository,
            analyzed_at: Utc::now(),
            basic_info: BasicRepoInfo::default(),
            security: SecurityAnalysis::default(),
            code_quality: CodeQualityAnalysis::default(),
            cicd: CicdAnalysis::default(),
            dependencies: DependencyAnalysis::default(),
            documentation: DocumentationAnalysis::default(),
            community_health: CommunityHealthAnalysis::default(),
            overall_score: 0.0,
        }
    }
    
    pub fn is_fresh(&self) -> bool {
        let age = Utc::now().signed_duration_since(self.analyzed_at);
        age.num_hours() < 24
    }
    
    pub fn calculate_score(&mut self) {
        self.overall_score = (
            self.security.score * 0.25 +
            self.code_quality.score * 0.25 +
            self.cicd.score * 0.15 +
            self.dependencies.score * 0.15 +
            self.documentation.score * 0.1 +
            self.community_health.score * 0.1
        );
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BasicRepoInfo {
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub size: u64,
    pub default_branch: String,
    pub is_private: bool,
    pub is_fork: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    pub has_security_policy: bool,
    pub code_scanning_enabled: bool,
    pub dependency_scanning_enabled: bool,
    pub secret_scanning_enabled: bool,
    pub branch_protection: BranchProtectionAnalysis,
    pub access_analysis: AccessAnalysis,
    pub score: f64,
}

impl SecurityAnalysis {
    pub fn calculate_score(&mut self) {
        let mut score: f64 = 0.0;
        if self.has_security_policy { score += 1.0; }
        if self.code_scanning_enabled { score += 2.0; }
        if self.dependency_scanning_enabled { score += 2.0; }
        if self.secret_scanning_enabled { score += 1.0; }
        score += self.branch_protection.score * 0.3;
        score += self.access_analysis.score * 0.4;
        self.score = score.min(10.0);
    }
}

// Additional analysis structures...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeQualityAnalysis {
    pub has_linting: bool,
    pub has_tests: bool,
    pub has_formatting: bool,
    pub has_pre_commit: bool,
    pub test_coverage: Option<f64>,
    pub enforces_reviews: bool,
    pub score: f64,
}

impl CodeQualityAnalysis {
    pub fn calculate_score(&mut self) {
        let mut score: f64 = 0.0;
        if self.has_linting { score += 1.5; }
        if self.has_tests { score += 2.0; }
        if self.has_formatting { score += 1.0; }
        if self.has_pre_commit { score += 1.0; }
        if let Some(coverage) = self.test_coverage {
            score += (coverage / 100.0) * 2.0;
        }
        if self.enforces_reviews { score += 2.5; }
        self.score = score.min(10.0);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CicdAnalysis {
    pub has_github_actions: bool,
    pub has_other_ci: bool,
    pub workflow_analysis: WorkflowAnalysis,
    pub has_deployment: bool,
    pub has_automated_releases: bool,
    pub score: f64,
}

impl CicdAnalysis {
    pub fn calculate_score(&mut self) {
        let mut score: f64 = 0.0;
        if self.has_github_actions || self.has_other_ci { score += 3.0; }
        score += self.workflow_analysis.score * 0.4;
        if self.has_deployment { score += 2.0; }
        if self.has_automated_releases { score += 1.5; }
        self.score = score.min(10.0);
    }
}

// More analysis structures would be defined here...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    pub has_dependency_file: bool,
    pub has_lock_file: bool,
    pub has_automated_updates: bool,
    pub vulnerability_analysis: VulnerabilityAnalysis,
    pub outdated_analysis: OutdatedAnalysis,
    pub score: f64,
}

impl DependencyAnalysis {
    pub fn calculate_score(&mut self) {
        let mut score: f64 = 0.0;
        if self.has_dependency_file { score += 2.0; }
        if self.has_lock_file { score += 2.0; }
        if self.has_automated_updates { score += 2.0; }
        score += self.vulnerability_analysis.score * 0.2;
        score += self.outdated_analysis.score * 0.2;
        self.score = score.min(10.0);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentationAnalysis {
    pub has_readme: bool,
    pub has_contributing: bool,
    pub has_code_of_conduct: bool,
    pub has_license: bool,
    pub has_api_docs: bool,
    pub has_changelog: bool,
    pub score: f64,
}

impl DocumentationAnalysis {
    pub fn calculate_score(&mut self) {
        let mut score: f64 = 0.0;
        if self.has_readme { score += 3.0; }
        if self.has_contributing { score += 1.5; }
        if self.has_code_of_conduct { score += 1.0; }
        if self.has_license { score += 2.0; }
        if self.has_api_docs { score += 1.5; }
        if self.has_changelog { score += 1.0; }
        self.score = score.min(10.0);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityHealthAnalysis {
    pub has_issue_templates: bool,
    pub has_pr_templates: bool,
    pub activity_analysis: ActivityAnalysis,
    pub maintainer_responsiveness: ResponsivenessAnalysis,
    pub score: f64,
}

impl CommunityHealthAnalysis {
    pub fn calculate_score(&mut self) {
        let mut score: f64 = 0.0;
        if self.has_issue_templates { score += 1.0; }
        if self.has_pr_templates { score += 1.0; }
        score += self.activity_analysis.score * 0.4;
        score += self.maintainer_responsiveness.score * 0.4;
        self.score = score.min(10.0);
    }
}

// Supporting analysis structures
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchProtectionAnalysis {
    pub score: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessAnalysis {
    pub score: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowAnalysis {
    pub score: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VulnerabilityAnalysis {
    pub score: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutdatedAnalysis {
    pub score: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivityAnalysis {
    pub score: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponsivenessAnalysis {
    pub score: f64,
}

// Recommendation and improvement structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub level: RecommendationLevel,
    pub category: String,
    pub title: String,
    pub description: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementPlan {
    pub tasks: Vec<ImprovementTask>,
    pub estimated_total_hours: u32,
}

impl ImprovementPlan {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            estimated_total_hours: 0,
        }
    }
    
    pub fn add_task(&mut self, task: ImprovementTask) {
        self.estimated_total_hours += task.estimated_hours;
        self.tasks.push(task);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementTask {
    pub category: String,
    pub priority: TaskPriority,
    pub title: String,
    pub description: String,
    pub estimated_hours: u32,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_analysis_creation() {
        let analysis = RepositoryAnalysis::new("test/repo".to_string());
        assert_eq!(analysis.repository, "test/repo");
        assert_eq!(analysis.overall_score, 0.0);
    }

    #[test]
    fn test_security_analysis_scoring() {
        let mut security = SecurityAnalysis::default();
        security.has_security_policy = true;
        security.code_scanning_enabled = true;
        security.calculate_score();
        assert!(security.score > 0.0);
    }

    #[test]
    fn test_improvement_plan() {
        let mut plan = ImprovementPlan::new();
        let task = ImprovementTask {
            category: "Security".to_string(),
            priority: TaskPriority::High,
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            estimated_hours: 4,
            dependencies: Vec::new(),
        };
        
        plan.add_task(task);
        assert_eq!(plan.tasks.len(), 1);
        assert_eq!(plan.estimated_total_hours, 4);
    }
}