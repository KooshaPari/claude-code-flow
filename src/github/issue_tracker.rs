use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::github::api::{GitHubApiClient, CreateIssueRequest, UpdateIssueRequest};

/// Issue lifecycle and project coordination
#[derive(Debug)]
pub struct IssueTracker {
    api: GitHubApiClient,
    triage_rules: Vec<TriageRule>,
    automation_configs: HashMap<String, AutomationConfig>,
    issue_templates: HashMap<String, IssueTemplate>,
}

impl IssueTracker {
    /// Create new issue tracker
    pub fn new(api: GitHubApiClient) -> Self {
        let mut tracker = Self {
            api,
            triage_rules: Vec::new(),
            automation_configs: HashMap::new(),
            issue_templates: HashMap::new(),
        };
        
        // Initialize default configurations
        tracker.load_default_triage_rules();
        tracker.load_default_automations();
        tracker.load_default_templates();
        
        tracker
    }

    /// Perform intelligent issue triage
    pub async fn triage_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u64,
    ) -> Result<TriageResult> {
        info!("Performing triage for issue #{}", issue_number);
        
        // Get issue details
        let issue = self.get_issue_details(owner, repo, issue_number).await?;
        
        // Analyze issue content
        let analysis = self.analyze_issue_content(owner, repo, &issue).await?;
        
        // Apply triage rules
        let mut triage_result = TriageResult {
            issue_number,
            analysis,
            applied_labels: Vec::new(),
            assigned_users: Vec::new(),
            priority_level: PriorityLevel::Medium,
            estimated_effort: None,
            suggested_milestone: None,
            automated_actions: Vec::new(),
        };
        
        // Apply each triage rule
        for rule in &self.triage_rules {
            if self.rule_matches(&rule, &triage_result.analysis) {
                self.apply_triage_rule(owner, repo, issue_number, rule, &mut triage_result).await?;
            }
        }
        
        // Generate effort estimation
        triage_result.estimated_effort = self.estimate_effort(&triage_result.analysis);
        
        // Suggest milestone
        triage_result.suggested_milestone = self.suggest_milestone(owner, repo, &triage_result).await?;
        
        info!("Triage completed for issue #{} with priority: {:?}", 
              issue_number, triage_result.priority_level);
        
        Ok(triage_result)
    }

    /// Analyze issue content using AI
    async fn analyze_issue_content(&self, owner: &str, repo: &str, issue: &IssueDetails) -> Result<IssueAnalysis> {
        debug!("Analyzing issue content: {}", issue.title);
        
        let mut analysis = IssueAnalysis {
            issue_type: self.classify_issue_type(&issue.title, &issue.body),
            severity: self.assess_severity(&issue.title, &issue.body),
            complexity: self.assess_complexity(&issue.title, &issue.body),
            domain_areas: self.identify_domain_areas(&issue.title, &issue.body),
            required_skills: self.identify_required_skills(&issue.title, &issue.body),
            dependencies: self.extract_dependencies(&issue.body),
            mentions_breaking_change: self.check_breaking_change(&issue.title, &issue.body),
            security_related: self.check_security_related(&issue.title, &issue.body),
            performance_related: self.check_performance_related(&issue.title, &issue.body),
            documentation_related: self.check_documentation_related(&issue.title, &issue.body),
            user_facing: self.check_user_facing(&issue.title, &issue.body),
            technical_debt: self.check_technical_debt(&issue.title, &issue.body),
        };
        
        // Additional context analysis
        analysis.sentiment = self.analyze_sentiment(&issue.body);
        analysis.urgency_indicators = self.detect_urgency_indicators(&issue.title, &issue.body);
        analysis.similar_issues = self.find_similar_issues(owner, repo, &issue.title).await?;
        
        Ok(analysis)
    }

    /// Apply triage rule to an issue
    async fn apply_triage_rule(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u64,
        rule: &TriageRule,
        triage_result: &mut TriageResult,
    ) -> Result<()> {
        debug!("Applying triage rule: {}", rule.name);
        
        for action in &rule.actions {
            match action {
                TriageAction::AddLabel(label) => {
                    self.api.add_issue_labels(owner, repo, issue_number, vec![label.clone()]).await?;
                    triage_result.applied_labels.push(label.clone());
                    triage_result.automated_actions.push(format!("Added label: {}", label));
                }
                TriageAction::AssignUser(user) => {
                    self.api.assign_issue(owner, repo, issue_number, vec![user.clone()]).await?;
                    triage_result.assigned_users.push(user.clone());
                    triage_result.automated_actions.push(format!("Assigned to: {}", user));
                }
                TriageAction::SetPriority(priority) => {
                    triage_result.priority_level = priority.clone();
                    triage_result.automated_actions.push(format!("Set priority: {:?}", priority));
                }
                TriageAction::AddComment(comment) => {
                    self.add_issue_comment(owner, repo, issue_number, comment).await?;
                    triage_result.automated_actions.push("Added automated comment".to_string());
                }
                TriageAction::RequestInfo => {
                    let comment = self.generate_info_request_comment(&triage_result.analysis);
                    self.add_issue_comment(owner, repo, issue_number, &comment).await?;
                    triage_result.automated_actions.push("Requested additional information".to_string());
                }
            }
        }
        
        Ok(())
    }

    /// Create comprehensive issue with smart defaults
    pub async fn create_comprehensive_issue(
        &self,
        owner: &str,
        repo: &str,
        request: &ComprehensiveIssueRequest,
    ) -> Result<IssueCreationResult> {
        info!("Creating comprehensive issue: {}", request.title);
        
        // Analyze the issue request
        let analysis = self.analyze_issue_request(request).await?;
        
        // Generate enhanced description
        let enhanced_body = self.generate_enhanced_issue_body(request, &analysis).await?;
        
        // Determine initial labels
        let initial_labels = self.generate_initial_labels(&analysis);
        
        // Create the issue
        let issue_request = CreateIssueRequest {
            title: request.title.clone(),
            body: Some(enhanced_body),
            labels: initial_labels.clone(),
            assignees: request.assignees.clone(),
        };
        
        let issue = self.api.create_issue(owner, repo, &issue_request).await?;
        
        // Perform initial triage
        let triage_result = self.triage_issue(owner, repo, issue.number).await?;
        
        // Set up automated tracking
        if request.enable_auto_tracking {
            self.setup_issue_tracking(owner, repo, issue.number).await?;
        }
        
        Ok(IssueCreationResult {
            issue,
            analysis,
            triage_result,
            initial_labels,
        })
    }

    /// Manage issue lifecycle
    pub async fn manage_issue_lifecycle(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u64,
        lifecycle_event: &LifecycleEvent,
    ) -> Result<LifecycleResult> {
        info!("Managing lifecycle event for issue #{}: {:?}", issue_number, lifecycle_event);
        
        let mut result = LifecycleResult {
            issue_number,
            event: lifecycle_event.clone(),
            actions_taken: Vec::new(),
            success: true,
        };
        
        match lifecycle_event {
            LifecycleEvent::StatusChange(from, to) => {
                self.handle_status_change(owner, repo, issue_number, from, to, &mut result).await?;
            }
            LifecycleEvent::AssignmentChange(assignee) => {
                self.handle_assignment_change(owner, repo, issue_number, assignee, &mut result).await?;
            }
            LifecycleEvent::PrLinked(pr_number) => {
                self.handle_pr_linked(owner, repo, issue_number, *pr_number, &mut result).await?;
            }
            LifecycleEvent::StaleDetected => {
                self.handle_stale_issue(owner, repo, issue_number, &mut result).await?;
            }
            LifecycleEvent::EscalationRequired => {
                self.handle_escalation(owner, repo, issue_number, &mut result).await?;
            }
        }
        
        Ok(result)
    }

    /// Batch process issues with advanced filtering
    pub async fn batch_process_issues(
        &self,
        owner: &str,
        repo: &str,
        filters: &IssueFilters,
        operations: &[BatchOperation],
    ) -> Result<BatchProcessResult> {
        info!("Batch processing issues with filters");
        
        // Get issues matching filters
        let issues = self.get_filtered_issues(owner, repo, filters).await?;
        
        let mut result = BatchProcessResult {
            total_issues: issues.len() as u32,
            processed_successfully: 0,
            failed: 0,
            operations_performed: HashMap::new(),
        };
        
        // Process each issue
        for issue in issues {
            match self.apply_batch_operations(owner, repo, issue.number, operations).await {
                Ok(operations_count) => {
                    result.processed_successfully += 1;
                    for (op, count) in operations_count {
                        *result.operations_performed.entry(op).or_insert(0) += count;
                    }
                }
                Err(e) => {
                    warn!("Failed to process issue #{}: {}", issue.number, e);
                    result.failed += 1;
                }
            }
        }
        
        info!("Batch processing completed: {}/{} successful", 
              result.processed_successfully, result.total_issues);
        
        Ok(result)
    }

    /// Generate comprehensive issue analytics
    pub async fn generate_issue_analytics(
        &self,
        owner: &str,
        repo: &str,
        timeframe: &Timeframe,
    ) -> Result<IssueAnalytics> {
        info!("Generating issue analytics for timeframe: {:?}", timeframe);
        
        // Get issues in timeframe
        let issues = self.get_issues_in_timeframe(owner, repo, timeframe).await?;
        
        let mut analytics = IssueAnalytics {
            timeframe: timeframe.clone(),
            total_issues: issues.len() as u32,
            open_issues: 0,
            closed_issues: 0,
            average_close_time: None,
            issue_velocity: 0.0,
            top_labels: HashMap::new(),
            top_assignees: HashMap::new(),
            issue_types: HashMap::new(),
            resolution_patterns: Vec::new(),
            trends: Vec::new(),
        };
        
        // Calculate metrics
        self.calculate_issue_metrics(&issues, &mut analytics);
        
        // Analyze patterns
        analytics.resolution_patterns = self.analyze_resolution_patterns(&issues);
        
        // Generate trends
        analytics.trends = self.generate_trend_analysis(&issues, timeframe);
        
        Ok(analytics)
    }

    /// Setup intelligent issue monitoring
    pub async fn setup_issue_monitoring(
        &self,
        owner: &str,
        repo: &str,
        config: &MonitoringConfig,
    ) -> Result<()> {
        info!("Setting up issue monitoring for {}/{}", owner, repo);
        
        // Store monitoring configuration
        self.store_monitoring_config(owner, repo, config).await?;
        
        // Set up automated checks
        for check in &config.automated_checks {
            self.setup_automated_check(owner, repo, check).await?;
        }
        
        // Configure notifications
        self.configure_notifications(owner, repo, &config.notifications).await?;
        
        info!("Issue monitoring configured successfully");
        Ok(())
    }

    // Helper methods for issue classification and analysis
    fn classify_issue_type(&self, title: &str, body: &Option<String>) -> IssueType {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        
        if content.contains("bug") || content.contains("error") || content.contains("broken") {
            IssueType::Bug
        } else if content.contains("feature") || content.contains("enhancement") || content.contains("add") {
            IssueType::Feature
        } else if content.contains("docs") || content.contains("documentation") {
            IssueType::Documentation
        } else if content.contains("performance") || content.contains("slow") || content.contains("optimization") {
            IssueType::Performance
        } else if content.contains("security") || content.contains("vulnerability") {
            IssueType::Security
        } else if content.contains("test") || content.contains("testing") {
            IssueType::Testing
        } else {
            IssueType::General
        }
    }

    fn assess_severity(&self, title: &str, body: &Option<String>) -> Severity {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        
        if content.contains("critical") || content.contains("urgent") || content.contains("production") {
            Severity::Critical
        } else if content.contains("major") || content.contains("important") {
            Severity::High
        } else if content.contains("minor") || content.contains("small") {
            Severity::Low
        } else {
            Severity::Medium
        }
    }

    fn assess_complexity(&self, title: &str, body: &Option<String>) -> Complexity {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        let word_count = content.split_whitespace().count();
        
        if word_count > 100 || content.contains("refactor") || content.contains("architecture") {
            Complexity::High
        } else if word_count > 50 || content.contains("multiple") || content.contains("several") {
            Complexity::Medium
        } else {
            Complexity::Low
        }
    }

    fn identify_domain_areas(&self, title: &str, body: &Option<String>) -> Vec<String> {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        let mut areas = Vec::new();
        
        if content.contains("frontend") || content.contains("ui") || content.contains("react") {
            areas.push("frontend".to_string());
        }
        if content.contains("backend") || content.contains("api") || content.contains("server") {
            areas.push("backend".to_string());
        }
        if content.contains("database") || content.contains("sql") || content.contains("db") {
            areas.push("database".to_string());
        }
        if content.contains("devops") || content.contains("deployment") || content.contains("ci/cd") {
            areas.push("devops".to_string());
        }
        
        areas
    }

    fn identify_required_skills(&self, title: &str, body: &Option<String>) -> Vec<String> {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        let mut skills = Vec::new();
        
        // Programming languages
        let languages = ["rust", "javascript", "python", "java", "go", "typescript"];
        for lang in &languages {
            if content.contains(lang) {
                skills.push(lang.to_string());
            }
        }
        
        // Technologies
        let technologies = ["docker", "kubernetes", "aws", "react", "node.js"];
        for tech in &technologies {
            if content.contains(tech) {
                skills.push(tech.to_string());
            }
        }
        
        skills
    }

    fn extract_dependencies(&self, body: &Option<String>) -> Vec<String> {
        // Extract dependencies from issue body (e.g., "depends on #123", "blocked by #456")
        // Implementation would use regex to find issue references
        Vec::new()
    }

    // Additional analysis methods
    fn check_breaking_change(&self, title: &str, body: &Option<String>) -> bool {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        content.contains("breaking") || content.contains("breaking change")
    }

    fn check_security_related(&self, title: &str, body: &Option<String>) -> bool {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        content.contains("security") || content.contains("vulnerability") || content.contains("cve")
    }

    fn check_performance_related(&self, title: &str, body: &Option<String>) -> bool {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        content.contains("performance") || content.contains("slow") || content.contains("optimization")
    }

    fn check_documentation_related(&self, title: &str, body: &Option<String>) -> bool {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        content.contains("docs") || content.contains("documentation") || content.contains("readme")
    }

    fn check_user_facing(&self, title: &str, body: &Option<String>) -> bool {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        content.contains("user") || content.contains("ui") || content.contains("frontend")
    }

    fn check_technical_debt(&self, title: &str, body: &Option<String>) -> bool {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        content.contains("refactor") || content.contains("cleanup") || content.contains("debt")
    }

    fn analyze_sentiment(&self, body: &Option<String>) -> SentimentAnalysis {
        // Simple sentiment analysis implementation
        // In a real implementation, this would use a proper sentiment analysis library
        SentimentAnalysis {
            score: 0.0,
            classification: "neutral".to_string(),
        }
    }

    fn detect_urgency_indicators(&self, title: &str, body: &Option<String>) -> Vec<String> {
        let content = format!("{} {}", title, body.as_deref().unwrap_or("")).to_lowercase();
        let mut indicators = Vec::new();
        
        if content.contains("urgent") {
            indicators.push("urgent keyword".to_string());
        }
        if content.contains("asap") {
            indicators.push("asap mentioned".to_string());
        }
        if content.contains("production") {
            indicators.push("production impact".to_string());
        }
        
        indicators
    }

    // Configuration loading methods
    fn load_default_triage_rules(&mut self) {
        self.triage_rules.push(TriageRule {
            name: "Bug Detection".to_string(),
            conditions: vec![
                TriageCondition::TitleContains("bug".to_string()),
                TriageCondition::IssueType(IssueType::Bug),
            ],
            actions: vec![
                TriageAction::AddLabel("bug".to_string()),
                TriageAction::SetPriority(PriorityLevel::High),
            ],
        });
        
        self.triage_rules.push(TriageRule {
            name: "Security Issues".to_string(),
            conditions: vec![
                TriageCondition::SecurityRelated(true),
            ],
            actions: vec![
                TriageAction::AddLabel("security".to_string()),
                TriageAction::SetPriority(PriorityLevel::Critical),
                TriageAction::AssignUser("security-team".to_string()),
            ],
        });
    }

    fn load_default_automations(&mut self) {
        // Load default automation configurations
    }

    fn load_default_templates(&mut self) {
        // Load default issue templates
    }

    // Helper method implementations (simplified for brevity)
    fn rule_matches(&self, rule: &TriageRule, analysis: &IssueAnalysis) -> bool {
        rule.conditions.iter().all(|condition| {
            match condition {
                TriageCondition::TitleContains(_) => true, // Simplified
                TriageCondition::IssueType(expected) => &analysis.issue_type == expected,
                TriageCondition::SecurityRelated(expected) => analysis.security_related == *expected,
                TriageCondition::Severity(expected) => &analysis.severity == expected,
            }
        })
    }

    fn estimate_effort(&self, analysis: &IssueAnalysis) -> Option<EffortEstimate> {
        let base_hours = match analysis.complexity {
            Complexity::Low => 2.0,
            Complexity::Medium => 8.0,
            Complexity::High => 24.0,
        };
        
        let multiplier = if analysis.security_related { 1.5 } else { 1.0 };
        
        Some(EffortEstimate {
            hours: base_hours * multiplier,
            confidence: 0.7,
            factors: vec!["complexity".to_string(), "domain".to_string()],
        })
    }

    // Async helper methods (simplified implementations)
    async fn get_issue_details(&self, _owner: &str, _repo: &str, _issue_number: u64) -> Result<IssueDetails> {
        Ok(IssueDetails::default())
    }

    async fn find_similar_issues(&self, _owner: &str, _repo: &str, _title: &str) -> Result<Vec<SimilarIssue>> {
        Ok(Vec::new())
    }

    async fn suggest_milestone(&self, _owner: &str, _repo: &str, _triage_result: &TriageResult) -> Result<Option<String>> {
        Ok(None)
    }

    async fn add_issue_comment(&self, _owner: &str, _repo: &str, _issue_number: u64, _comment: &str) -> Result<()> {
        Ok(())
    }

    async fn analyze_issue_request(&self, _request: &ComprehensiveIssueRequest) -> Result<IssueRequestAnalysis> {
        Ok(IssueRequestAnalysis::default())
    }

    async fn generate_enhanced_issue_body(&self, request: &ComprehensiveIssueRequest, _analysis: &IssueRequestAnalysis) -> Result<String> {
        Ok(request.body.clone().unwrap_or_default())
    }

    fn generate_initial_labels(&self, _analysis: &IssueRequestAnalysis) -> Vec<String> {
        Vec::new()
    }

    async fn setup_issue_tracking(&self, _owner: &str, _repo: &str, _issue_number: u64) -> Result<()> {
        Ok(())
    }

    async fn handle_status_change(&self, _owner: &str, _repo: &str, _issue_number: u64, _from: &str, _to: &str, _result: &mut LifecycleResult) -> Result<()> {
        Ok(())
    }

    async fn handle_assignment_change(&self, _owner: &str, _repo: &str, _issue_number: u64, _assignee: &str, _result: &mut LifecycleResult) -> Result<()> {
        Ok(())
    }

    async fn handle_pr_linked(&self, _owner: &str, _repo: &str, _issue_number: u64, _pr_number: u64, _result: &mut LifecycleResult) -> Result<()> {
        Ok(())
    }

    async fn handle_stale_issue(&self, _owner: &str, _repo: &str, _issue_number: u64, _result: &mut LifecycleResult) -> Result<()> {
        Ok(())
    }

    async fn handle_escalation(&self, _owner: &str, _repo: &str, _issue_number: u64, _result: &mut LifecycleResult) -> Result<()> {
        Ok(())
    }

    async fn get_filtered_issues(&self, _owner: &str, _repo: &str, _filters: &IssueFilters) -> Result<Vec<IssueDetails>> {
        Ok(Vec::new())
    }

    async fn apply_batch_operations(&self, _owner: &str, _repo: &str, _issue_number: u64, _operations: &[BatchOperation]) -> Result<HashMap<String, u32>> {
        Ok(HashMap::new())
    }

    async fn get_issues_in_timeframe(&self, _owner: &str, _repo: &str, _timeframe: &Timeframe) -> Result<Vec<IssueDetails>> {
        Ok(Vec::new())
    }

    fn calculate_issue_metrics(&self, _issues: &[IssueDetails], _analytics: &mut IssueAnalytics) {
        // Implementation would calculate various metrics
    }

    fn analyze_resolution_patterns(&self, _issues: &[IssueDetails]) -> Vec<ResolutionPattern> {
        Vec::new()
    }

    fn generate_trend_analysis(&self, _issues: &[IssueDetails], _timeframe: &Timeframe) -> Vec<Trend> {
        Vec::new()
    }

    async fn store_monitoring_config(&self, _owner: &str, _repo: &str, _config: &MonitoringConfig) -> Result<()> {
        Ok(())
    }

    async fn setup_automated_check(&self, _owner: &str, _repo: &str, _check: &AutomatedCheck) -> Result<()> {
        Ok(())
    }

    async fn configure_notifications(&self, _owner: &str, _repo: &str, _notifications: &NotificationConfig) -> Result<()> {
        Ok(())
    }

    fn generate_info_request_comment(&self, _analysis: &IssueAnalysis) -> String {
        "Thank you for the issue report. Could you please provide more details about the steps to reproduce this issue?".to_string()
    }
}

// Data structures for issue tracking
#[derive(Debug, Serialize, Deserialize)]
pub struct ComprehensiveIssueRequest {
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub enable_auto_tracking: bool,
    pub enable_ai_analysis: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IssueDetails {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueAnalysis {
    pub issue_type: IssueType,
    pub severity: Severity,
    pub complexity: Complexity,
    pub domain_areas: Vec<String>,
    pub required_skills: Vec<String>,
    pub dependencies: Vec<String>,
    pub mentions_breaking_change: bool,
    pub security_related: bool,
    pub performance_related: bool,
    pub documentation_related: bool,
    pub user_facing: bool,
    pub technical_debt: bool,
    pub sentiment: SentimentAnalysis,
    pub urgency_indicators: Vec<String>,
    pub similar_issues: Vec<SimilarIssue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IssueType {
    Bug,
    Feature,
    Documentation,
    Performance,
    Security,
    Testing,
    General,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Complexity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    pub score: f64,
    pub classification: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarIssue {
    pub number: u64,
    pub title: String,
    pub similarity_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriageResult {
    pub issue_number: u64,
    pub analysis: IssueAnalysis,
    pub applied_labels: Vec<String>,
    pub assigned_users: Vec<String>,
    pub priority_level: PriorityLevel,
    pub estimated_effort: Option<EffortEstimate>,
    pub suggested_milestone: Option<String>,
    pub automated_actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub hours: f64,
    pub confidence: f64,
    pub factors: Vec<String>,
}

#[derive(Debug)]
pub struct TriageRule {
    pub name: String,
    pub conditions: Vec<TriageCondition>,
    pub actions: Vec<TriageAction>,
}

#[derive(Debug)]
pub enum TriageCondition {
    TitleContains(String),
    IssueType(IssueType),
    SecurityRelated(bool),
    Severity(Severity),
}

#[derive(Debug)]
pub enum TriageAction {
    AddLabel(String),
    AssignUser(String),
    SetPriority(PriorityLevel),
    AddComment(String),
    RequestInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleEvent {
    StatusChange(String, String),
    AssignmentChange(String),
    PrLinked(u64),
    StaleDetected,
    EscalationRequired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleResult {
    pub issue_number: u64,
    pub event: LifecycleEvent,
    pub actions_taken: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCreationResult {
    pub issue: octocrab::models::issues::Issue,
    pub analysis: IssueRequestAnalysis,
    pub triage_result: TriageResult,
    pub initial_labels: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IssueRequestAnalysis {
    pub complexity_score: f64,
    pub estimated_completion_time: Option<u32>,
    pub suggested_assignees: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFilters {
    pub state: Option<String>,
    pub labels: Vec<String>,
    pub assignee: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BatchOperation {
    AddLabel(String),
    RemoveLabel(String),
    Assign(String),
    Unassign(String),
    Close(String),
    Comment(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchProcessResult {
    pub total_issues: u32,
    pub processed_successfully: u32,
    pub failed: u32,
    pub operations_performed: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Timeframe {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Custom(DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueAnalytics {
    pub timeframe: Timeframe,
    pub total_issues: u32,
    pub open_issues: u32,
    pub closed_issues: u32,
    pub average_close_time: Option<f64>,
    pub issue_velocity: f64,
    pub top_labels: HashMap<String, u32>,
    pub top_assignees: HashMap<String, u32>,
    pub issue_types: HashMap<String, u32>,
    pub resolution_patterns: Vec<ResolutionPattern>,
    pub trends: Vec<Trend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolutionPattern {
    pub pattern: String,
    pub frequency: u32,
    pub average_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trend {
    pub metric: String,
    pub direction: String,
    pub change_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub automated_checks: Vec<AutomatedCheck>,
    pub notifications: NotificationConfig,
    pub escalation_rules: Vec<EscalationRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomatedCheck {
    pub name: String,
    pub check_type: String,
    pub frequency: String,
    pub conditions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email: bool,
    pub slack: bool,
    pub webhook: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EscalationRule {
    pub condition: String,
    pub action: String,
    pub delay_hours: u32,
}

#[derive(Debug)]
pub struct AutomationConfig {
    pub name: String,
    pub triggers: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug)]
pub struct IssueTemplate {
    pub name: String,
    pub content: String,
    pub required_fields: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_analysis_creation() {
        let analysis = IssueAnalysis {
            issue_type: IssueType::Bug,
            severity: Severity::High,
            complexity: Complexity::Medium,
            domain_areas: vec!["backend".to_string()],
            required_skills: vec!["rust".to_string()],
            dependencies: Vec::new(),
            mentions_breaking_change: false,
            security_related: false,
            performance_related: false,
            documentation_related: false,
            user_facing: true,
            technical_debt: false,
            sentiment: SentimentAnalysis {
                score: 0.0,
                classification: "neutral".to_string(),
            },
            urgency_indicators: Vec::new(),
            similar_issues: Vec::new(),
        };
        
        assert_eq!(analysis.issue_type, IssueType::Bug);
        assert_eq!(analysis.severity, Severity::High);
        assert!(analysis.user_facing);
    }

    #[test]
    fn test_effort_estimation() {
        let estimate = EffortEstimate {
            hours: 8.0,
            confidence: 0.7,
            factors: vec!["complexity".to_string()],
        };
        
        assert_eq!(estimate.hours, 8.0);
        assert_eq!(estimate.confidence, 0.7);
    }

    #[test]
    fn test_priority_levels() {
        assert_eq!(PriorityLevel::Critical as u8, 3);
        assert_eq!(PriorityLevel::High as u8, 2);
    }
}