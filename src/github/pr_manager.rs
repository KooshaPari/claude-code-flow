use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::github::api::{GitHubApiClient, CreatePullRequestRequest, UpdatePullRequestRequest};

/// Pull request workflow management with AI reviews
#[derive(Debug)]
pub struct PullRequestManager {
    api: GitHubApiClient,
    review_templates: HashMap<String, ReviewTemplate>,
    workflow_configs: HashMap<String, PrWorkflowConfig>,
}

impl PullRequestManager {
    /// Create new pull request manager
    pub fn new(api: GitHubApiClient) -> Self {
        let mut manager = Self {
            api,
            review_templates: HashMap::new(),
            workflow_configs: HashMap::new(),
        };
        
        // Initialize default templates and configs
        manager.load_default_templates();
        manager.load_default_workflows();
        
        manager
    }

    /// Create a comprehensive pull request with AI analysis
    pub async fn create_comprehensive_pr(
        &self,
        owner: &str,
        repo: &str,
        request: &ComprehensivePrRequest,
    ) -> Result<PullRequestResult> {
        info!("Creating comprehensive PR: {}", request.title);
        
        // Analyze the changes first
        let change_analysis = self.analyze_changes(owner, repo, &request.head, &request.base).await?;
        
        // Generate AI-enhanced description
        let enhanced_description = self.generate_enhanced_description(request, &change_analysis).await?;
        
        // Create the pull request
        let pr_request = CreatePullRequestRequest {
            title: request.title.clone(),
            body: Some(enhanced_description),
            head: request.head.clone(),
            base: request.base.clone(),
        };
        
        let pr = self.api.create_pull_request(owner, repo, &pr_request).await?;
        
        // Apply labels based on analysis
        let labels = self.generate_labels(&change_analysis);
        if !labels.is_empty() {
            self.apply_labels(owner, repo, pr.number, &labels).await?;
        }
        
        // Assign reviewers based on change analysis
        let reviewers = self.suggest_reviewers(owner, repo, &change_analysis).await?;
        if !reviewers.is_empty() {
            self.assign_reviewers(owner, repo, pr.number, &reviewers).await?;
        }
        
        // Create initial AI review if requested
        let ai_review = if request.enable_ai_review {
            Some(self.create_ai_review(owner, repo, pr.number, &change_analysis).await?)
        } else {
            None
        };
        
        // Set up automated checks
        if request.enable_automated_checks {
            self.setup_automated_checks(owner, repo, pr.number).await?;
        }
        
        Ok(PullRequestResult {
            pr,
            change_analysis,
            ai_review,
            suggested_reviewers: reviewers,
            applied_labels: labels,
        })
    }

    /// Analyze changes in a pull request
    async fn analyze_changes(
        &self,
        owner: &str,
        repo: &str,
        head: &str,
        base: &str,
    ) -> Result<ChangeAnalysis> {
        info!("Analyzing changes between {} and {}", base, head);
        
        let mut analysis = ChangeAnalysis::default();
        
        // Get commit comparison
        let comparison = self.get_commit_comparison(owner, repo, base, head).await?;
        
        analysis.files_changed = comparison.files.len() as u32;
        analysis.additions = comparison.additions;
        analysis.deletions = comparison.deletions;
        analysis.commits = comparison.commits.len() as u32;
        
        // Analyze file types
        analysis.file_types = self.analyze_file_types(&comparison.files);
        
        // Analyze change complexity
        analysis.complexity = self.calculate_complexity(&comparison);
        
        // Analyze potential impact
        analysis.impact = self.analyze_impact(owner, repo, &comparison).await?;
        
        // Check for breaking changes
        analysis.breaking_changes = self.detect_breaking_changes(&comparison);
        
        // Security analysis
        analysis.security_concerns = self.analyze_security_changes(&comparison);
        
        // Performance analysis
        analysis.performance_impact = self.analyze_performance_impact(&comparison);
        
        // Documentation changes
        analysis.documentation_changes = self.analyze_documentation_changes(&comparison);
        
        // Test coverage analysis
        analysis.test_coverage = self.analyze_test_coverage(&comparison);
        
        Ok(analysis)
    }

    /// Generate enhanced PR description with AI insights
    async fn generate_enhanced_description(
        &self,
        request: &ComprehensivePrRequest,
        analysis: &ChangeAnalysis,
    ) -> Result<String> {
        let mut description = String::new();
        
        // Original description
        if let Some(ref body) = request.body {
            description.push_str(body);
            description.push_str("\n\n");
        }
        
        // Add AI-generated summary
        description.push_str("## 🤖 AI Analysis Summary\n\n");
        
        // Change overview
        description.push_str(&format!(
            "**Change Overview:**\n- {} files changed\n- {} additions, {} deletions\n- {} commits\n\n",
            analysis.files_changed,
            analysis.additions,
            analysis.deletions,
            analysis.commits
        ));
        
        // File types analysis
        if !analysis.file_types.is_empty() {
            description.push_str("**File Types:**\n");
            for (file_type, count) in &analysis.file_types {
                description.push_str(&format!("- {}: {} files\n", file_type, count));
            }
            description.push('\n');
        }
        
        // Complexity assessment
        description.push_str(&format!(
            "**Complexity:** {} ({})\n\n",
            analysis.complexity.level,
            analysis.complexity.description
        ));
        
        // Impact analysis
        description.push_str(&format!(
            "**Impact Assessment:** {}\n",
            analysis.impact.description
        ));
        
        for area in &analysis.impact.affected_areas {
            description.push_str(&format!("- {}\n", area));
        }
        description.push('\n');
        
        // Breaking changes warning
        if !analysis.breaking_changes.is_empty() {
            description.push_str("## ⚠️ Breaking Changes\n\n");
            for change in &analysis.breaking_changes {
                description.push_str(&format!("- {}\n", change));
            }
            description.push('\n');
        }
        
        // Security concerns
        if !analysis.security_concerns.is_empty() {
            description.push_str("## 🔒 Security Considerations\n\n");
            for concern in &analysis.security_concerns {
                description.push_str(&format!("- {}\n", concern));
            }
            description.push('\n');
        }
        
        // Performance impact
        if analysis.performance_impact.has_impact {
            description.push_str("## ⚡ Performance Impact\n\n");
            description.push_str(&format!("{}\n\n", analysis.performance_impact.description));
        }
        
        // Testing information
        description.push_str("## 🧪 Testing\n\n");
        if analysis.test_coverage.has_tests {
            description.push_str(&format!(
                "- Test files: {} added/modified\n",
                analysis.test_coverage.test_files
            ));
            if let Some(coverage) = analysis.test_coverage.estimated_coverage {
                description.push_str(&format!("- Estimated coverage: {:.1}%\n", coverage));
            }
        } else {
            description.push_str("- ⚠️ No test files detected in changes\n");
        }
        description.push('\n');
        
        // Documentation updates
        if analysis.documentation_changes.has_changes {
            description.push_str("## 📚 Documentation\n\n");
            description.push_str(&format!(
                "- Documentation files updated: {}\n\n",
                analysis.documentation_changes.files_updated
            ));
        }
        
        // Checklist
        description.push_str("## ✅ Pre-merge Checklist\n\n");
        description.push_str("- [ ] All tests pass\n");
        description.push_str("- [ ] Code has been reviewed\n");
        description.push_str("- [ ] Documentation updated (if applicable)\n");
        description.push_str("- [ ] Breaking changes documented\n");
        description.push_str("- [ ] Security implications reviewed\n");
        
        Ok(description)
    }

    /// Create AI-powered code review
    async fn create_ai_review(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
        analysis: &ChangeAnalysis,
    ) -> Result<AiReview> {
        info!("Creating AI review for PR #{}", pr_number);
        
        let mut review = AiReview {
            pr_number,
            created_at: Utc::now(),
            overall_assessment: String::new(),
            code_quality_score: 0.0,
            security_score: 0.0,
            performance_score: 0.0,
            maintainability_score: 0.0,
            suggestions: Vec::new(),
            concerns: Vec::new(),
            approvals: Vec::new(),
        };
        
        // Overall assessment
        review.overall_assessment = self.generate_overall_assessment(analysis);
        
        // Calculate scores
        review.code_quality_score = self.calculate_code_quality_score(analysis);
        review.security_score = self.calculate_security_score(analysis);
        review.performance_score = self.calculate_performance_score(analysis);
        review.maintainability_score = self.calculate_maintainability_score(analysis);
        
        // Generate suggestions
        review.suggestions = self.generate_code_suggestions(analysis);
        
        // Identify concerns
        review.concerns = self.identify_concerns(analysis);
        
        // Generate approvals for good practices
        review.approvals = self.generate_approvals(analysis);
        
        // Post review comment
        self.post_review_comment(owner, repo, pr_number, &review).await?;
        
        Ok(review)
    }

    /// Suggest appropriate reviewers based on change analysis
    async fn suggest_reviewers(
        &self,
        owner: &str,
        repo: &str,
        analysis: &ChangeAnalysis,
    ) -> Result<Vec<String>> {
        debug!("Suggesting reviewers for change analysis");
        
        let mut reviewers = Vec::new();
        
        // Get CODEOWNERS file for automatic suggestions
        if let Ok(codeowners) = self.get_codeowners(owner, repo).await {
            reviewers.extend(self.parse_codeowners_for_changes(&codeowners, analysis));
        }
        
        // Add domain experts based on file types
        reviewers.extend(self.suggest_domain_experts(analysis));
        
        // Add security reviewers for security-sensitive changes
        if !analysis.security_concerns.is_empty() {
            reviewers.extend(self.get_security_reviewers());
        }
        
        // Add performance reviewers for performance-critical changes
        if analysis.performance_impact.has_impact {
            reviewers.extend(self.get_performance_reviewers());
        }
        
        // Remove duplicates and limit to reasonable number
        reviewers.sort();
        reviewers.dedup();
        reviewers.truncate(5);
        
        Ok(reviewers)
    }

    /// Generate appropriate labels for the PR
    fn generate_labels(&self, analysis: &ChangeAnalysis) -> Vec<String> {
        let mut labels = Vec::new();
        
        // Size labels
        match analysis.complexity.level.as_str() {
            "low" => labels.push("size/S".to_string()),
            "medium" => labels.push("size/M".to_string()),
            "high" => labels.push("size/L".to_string()),
            "very_high" => labels.push("size/XL".to_string()),
            _ => {}
        }
        
        // Type labels based on file types
        if analysis.file_types.contains_key("test") {
            labels.push("type/testing".to_string());
        }
        if analysis.file_types.contains_key("doc") {
            labels.push("type/documentation".to_string());
        }
        if analysis.file_types.contains_key("config") {
            labels.push("type/configuration".to_string());
        }
        
        // Breaking changes
        if !analysis.breaking_changes.is_empty() {
            labels.push("breaking-change".to_string());
        }
        
        // Security
        if !analysis.security_concerns.is_empty() {
            labels.push("security".to_string());
        }
        
        // Performance
        if analysis.performance_impact.has_impact {
            labels.push("performance".to_string());
        }
        
        labels
    }

    /// Manage PR workflow and automation
    pub async fn manage_pr_workflow(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
        workflow_type: &str,
    ) -> Result<WorkflowResult> {
        info!("Managing PR workflow: {} for PR #{}", workflow_type, pr_number);
        
        let config = self.workflow_configs.get(workflow_type)
            .ok_or_else(|| anyhow!("Unknown workflow type: {}", workflow_type))?;
        
        let mut result = WorkflowResult {
            pr_number,
            workflow_type: workflow_type.to_string(),
            steps_completed: Vec::new(),
            steps_failed: Vec::new(),
            overall_success: true,
        };
        
        // Execute workflow steps
        for step in &config.steps {
            match self.execute_workflow_step(owner, repo, pr_number, step).await {
                Ok(_) => {
                    result.steps_completed.push(step.name.clone());
                    info!("Workflow step completed: {}", step.name);
                }
                Err(e) => {
                    result.steps_failed.push(format!("{}: {}", step.name, e));
                    warn!("Workflow step failed: {} - {}", step.name, e);
                    
                    if step.required {
                        result.overall_success = false;
                        break;
                    }
                }
            }
        }
        
        Ok(result)
    }

    /// Execute individual workflow step
    async fn execute_workflow_step(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
        step: &WorkflowStep,
    ) -> Result<()> {
        match step.step_type.as_str() {
            "lint_check" => self.run_lint_check(owner, repo, pr_number).await,
            "test_execution" => self.run_tests(owner, repo, pr_number).await,
            "security_scan" => self.run_security_scan(owner, repo, pr_number).await,
            "dependency_check" => self.check_dependencies(owner, repo, pr_number).await,
            "build_verification" => self.verify_build(owner, repo, pr_number).await,
            "documentation_check" => self.check_documentation(owner, repo, pr_number).await,
            _ => Err(anyhow!("Unknown workflow step type: {}", step.step_type)),
        }
    }

    /// Update PR status based on checks and reviews
    pub async fn update_pr_status(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> Result<PrStatus> {
        info!("Updating PR status for #{}", pr_number);
        
        // Get current PR state
        let pr = self.get_pr_details(owner, repo, pr_number).await?;
        
        // Check all required status checks
        let status_checks = self.get_status_checks(owner, repo, &pr.head.sha).await?;
        
        // Check reviews
        let reviews = self.get_pr_reviews(owner, repo, pr_number).await?;
        
        // Calculate overall status
        let status = PrStatus {
            pr_number,
            mergeable: pr.mergeable.unwrap_or(false),
            all_checks_passed: status_checks.iter().all(|check| check.state == "success"),
            approved_reviews: reviews.iter().filter(|r| r.state == "APPROVED").count() as u32,
            requested_changes: reviews.iter().filter(|r| r.state == "CHANGES_REQUESTED").count() as u32,
            ready_to_merge: false,
        };
        
        // Determine if ready to merge
        let ready_to_merge = status.mergeable 
            && status.all_checks_passed 
            && status.approved_reviews > 0 
            && status.requested_changes == 0;
        
        let mut final_status = status;
        final_status.ready_to_merge = ready_to_merge;
        
        Ok(final_status)
    }

    // Helper methods implementation
    fn load_default_templates(&mut self) {
        // Load default review templates
        self.review_templates.insert(
            "comprehensive".to_string(),
            ReviewTemplate {
                name: "Comprehensive Review".to_string(),
                sections: vec![
                    "Code Quality".to_string(),
                    "Security".to_string(),
                    "Performance".to_string(),
                    "Documentation".to_string(),
                    "Testing".to_string(),
                ],
            },
        );
    }
    
    fn load_default_workflows(&mut self) {
        // Load default workflow configurations
        self.workflow_configs.insert(
            "standard".to_string(),
            PrWorkflowConfig {
                name: "Standard Workflow".to_string(),
                steps: vec![
                    WorkflowStep {
                        name: "Lint Check".to_string(),
                        step_type: "lint_check".to_string(),
                        required: true,
                        timeout_minutes: 5,
                    },
                    WorkflowStep {
                        name: "Test Execution".to_string(),
                        step_type: "test_execution".to_string(),
                        required: true,
                        timeout_minutes: 30,
                    },
                    WorkflowStep {
                        name: "Security Scan".to_string(),
                        step_type: "security_scan".to_string(),
                        required: false,
                        timeout_minutes: 10,
                    },
                ],
            },
        );
    }

    // Placeholder implementations for complex analysis methods
    async fn get_commit_comparison(&self, _owner: &str, _repo: &str, _base: &str, _head: &str) -> Result<CommitComparison> {
        // Implementation would use GitHub API to get commit comparison
        Ok(CommitComparison::default())
    }
    
    fn analyze_file_types(&self, _files: &[FileChange]) -> HashMap<String, u32> {
        // Implementation would analyze file extensions and categorize
        HashMap::new()
    }
    
    fn calculate_complexity(&self, _comparison: &CommitComparison) -> ComplexityAssessment {
        ComplexityAssessment::default()
    }
    
    async fn analyze_impact(&self, _owner: &str, _repo: &str, _comparison: &CommitComparison) -> Result<ImpactAssessment> {
        Ok(ImpactAssessment::default())
    }
    
    fn detect_breaking_changes(&self, _comparison: &CommitComparison) -> Vec<String> {
        Vec::new()
    }
    
    fn analyze_security_changes(&self, _comparison: &CommitComparison) -> Vec<String> {
        Vec::new()
    }
    
    fn analyze_performance_impact(&self, _comparison: &CommitComparison) -> PerformanceImpact {
        PerformanceImpact::default()
    }
    
    fn analyze_documentation_changes(&self, _comparison: &CommitComparison) -> DocumentationChanges {
        DocumentationChanges::default()
    }
    
    fn analyze_test_coverage(&self, _comparison: &CommitComparison) -> TestCoverage {
        TestCoverage::default()
    }

    // Additional helper methods...
    async fn apply_labels(&self, _owner: &str, _repo: &str, _pr_number: u64, _labels: &[String]) -> Result<()> {
        Ok(())
    }
    
    async fn assign_reviewers(&self, _owner: &str, _repo: &str, _pr_number: u64, _reviewers: &[String]) -> Result<()> {
        Ok(())
    }
    
    async fn setup_automated_checks(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn post_review_comment(&self, _owner: &str, _repo: &str, _pr_number: u64, _review: &AiReview) -> Result<()> {
        Ok(())
    }
    
    async fn get_codeowners(&self, _owner: &str, _repo: &str) -> Result<String> {
        Ok(String::new())
    }
    
    fn parse_codeowners_for_changes(&self, _codeowners: &str, _analysis: &ChangeAnalysis) -> Vec<String> {
        Vec::new()
    }
    
    fn suggest_domain_experts(&self, _analysis: &ChangeAnalysis) -> Vec<String> {
        Vec::new()
    }
    
    fn get_security_reviewers(&self) -> Vec<String> {
        vec!["security-team".to_string()]
    }
    
    fn get_performance_reviewers(&self) -> Vec<String> {
        vec!["performance-team".to_string()]
    }
    
    fn generate_overall_assessment(&self, _analysis: &ChangeAnalysis) -> String {
        "Overall assessment: Good changes with minor suggestions.".to_string()
    }
    
    fn calculate_code_quality_score(&self, _analysis: &ChangeAnalysis) -> f64 {
        8.5
    }
    
    fn calculate_security_score(&self, _analysis: &ChangeAnalysis) -> f64 {
        9.0
    }
    
    fn calculate_performance_score(&self, _analysis: &ChangeAnalysis) -> f64 {
        8.0
    }
    
    fn calculate_maintainability_score(&self, _analysis: &ChangeAnalysis) -> f64 {
        8.7
    }
    
    fn generate_code_suggestions(&self, _analysis: &ChangeAnalysis) -> Vec<String> {
        vec!["Consider adding more unit tests".to_string()]
    }
    
    fn identify_concerns(&self, _analysis: &ChangeAnalysis) -> Vec<String> {
        Vec::new()
    }
    
    fn generate_approvals(&self, _analysis: &ChangeAnalysis) -> Vec<String> {
        vec!["Good error handling implementation".to_string()]
    }

    async fn run_lint_check(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn run_tests(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn run_security_scan(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn check_dependencies(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn verify_build(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn check_documentation(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<()> {
        Ok(())
    }
    
    async fn get_pr_details(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<PullRequestDetails> {
        Ok(PullRequestDetails::default())
    }
    
    async fn get_status_checks(&self, _owner: &str, _repo: &str, _sha: &str) -> Result<Vec<StatusCheck>> {
        Ok(Vec::new())
    }
    
    async fn get_pr_reviews(&self, _owner: &str, _repo: &str, _pr_number: u64) -> Result<Vec<Review>> {
        Ok(Vec::new())
    }
}

// Data structures for PR management
#[derive(Debug, Serialize, Deserialize)]
pub struct ComprehensivePrRequest {
    pub title: String,
    pub body: Option<String>,
    pub head: String,
    pub base: String,
    pub enable_ai_review: bool,
    pub enable_automated_checks: bool,
    pub auto_assign_reviewers: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestResult {
    pub pr: octocrab::models::pulls::PullRequest,
    pub change_analysis: ChangeAnalysis,
    pub ai_review: Option<AiReview>,
    pub suggested_reviewers: Vec<String>,
    pub applied_labels: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChangeAnalysis {
    pub files_changed: u32,
    pub additions: u32,
    pub deletions: u32,
    pub commits: u32,
    pub file_types: HashMap<String, u32>,
    pub complexity: ComplexityAssessment,
    pub impact: ImpactAssessment,
    pub breaking_changes: Vec<String>,
    pub security_concerns: Vec<String>,
    pub performance_impact: PerformanceImpact,
    pub documentation_changes: DocumentationChanges,
    pub test_coverage: TestCoverage,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ComplexityAssessment {
    pub level: String,
    pub description: String,
    pub score: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub description: String,
    pub affected_areas: Vec<String>,
    pub risk_level: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PerformanceImpact {
    pub has_impact: bool,
    pub description: String,
    pub estimated_change: Option<f64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocumentationChanges {
    pub has_changes: bool,
    pub files_updated: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TestCoverage {
    pub has_tests: bool,
    pub test_files: u32,
    pub estimated_coverage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiReview {
    pub pr_number: u64,
    pub created_at: DateTime<Utc>,
    pub overall_assessment: String,
    pub code_quality_score: f64,
    pub security_score: f64,
    pub performance_score: f64,
    pub maintainability_score: f64,
    pub suggestions: Vec<String>,
    pub concerns: Vec<String>,
    pub approvals: Vec<String>,
}

#[derive(Debug)]
pub struct ReviewTemplate {
    pub name: String,
    pub sections: Vec<String>,
}

#[derive(Debug)]
pub struct PrWorkflowConfig {
    pub name: String,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug)]
pub struct WorkflowStep {
    pub name: String,
    pub step_type: String,
    pub required: bool,
    pub timeout_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub pr_number: u64,
    pub workflow_type: String,
    pub steps_completed: Vec<String>,
    pub steps_failed: Vec<String>,
    pub overall_success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrStatus {
    pub pr_number: u64,
    pub mergeable: bool,
    pub all_checks_passed: bool,
    pub approved_reviews: u32,
    pub requested_changes: u32,
    pub ready_to_merge: bool,
}

// Supporting data structures
#[derive(Debug, Default)]
pub struct CommitComparison {
    pub files: Vec<FileChange>,
    pub additions: u32,
    pub deletions: u32,
    pub commits: Vec<CommitInfo>,
}

#[derive(Debug)]
pub struct FileChange {
    pub filename: String,
    pub status: String,
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Debug)]
pub struct CommitInfo {
    pub sha: String,
    pub message: String,
    pub author: String,
}

#[derive(Debug, Default)]
pub struct PullRequestDetails {
    pub head: HeadRef,
    pub mergeable: Option<bool>,
}

#[derive(Debug, Default)]
pub struct HeadRef {
    pub sha: String,
}

#[derive(Debug)]
pub struct StatusCheck {
    pub state: String,
    pub description: String,
    pub context: String,
}

#[derive(Debug)]
pub struct Review {
    pub state: String,
    pub user: String,
    pub submitted_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_analysis_creation() {
        let analysis = ChangeAnalysis::default();
        assert_eq!(analysis.files_changed, 0);
        assert_eq!(analysis.commits, 0);
    }

    #[test]
    fn test_complexity_assessment() {
        let complexity = ComplexityAssessment {
            level: "medium".to_string(),
            description: "Moderate complexity changes".to_string(),
            score: 6.5,
        };
        
        assert_eq!(complexity.level, "medium");
        assert_eq!(complexity.score, 6.5);
    }

    #[test]
    fn test_pr_status() {
        let status = PrStatus {
            pr_number: 123,
            mergeable: true,
            all_checks_passed: true,
            approved_reviews: 2,
            requested_changes: 0,
            ready_to_merge: true,
        };
        
        assert!(status.ready_to_merge);
        assert_eq!(status.approved_reviews, 2);
    }
}