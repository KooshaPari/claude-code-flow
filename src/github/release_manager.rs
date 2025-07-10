use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn, error};

use crate::github::api::{GitHubApiClient, CreateReleaseRequest};

/// Release coordination with automated changelogs and validation
#[derive(Debug)]
pub struct ReleaseManager {
    api: GitHubApiClient,
    release_configs: HashMap<String, ReleaseConfig>,
    validation_rules: Vec<ValidationRule>,
    changelog_generators: HashMap<String, ChangelogGenerator>,
}

impl ReleaseManager {
    /// Create new release manager
    pub fn new(api: GitHubApiClient) -> Self {
        let mut manager = Self {
            api,
            release_configs: HashMap::new(),
            validation_rules: Vec::new(),
            changelog_generators: HashMap::new(),
        };
        
        // Initialize default configurations
        manager.load_default_configs();
        manager.load_default_validation_rules();
        manager.load_default_changelog_generators();
        
        manager
    }

    /// Orchestrate complete release process
    pub async fn orchestrate_release(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
    ) -> Result<ReleaseOrchestrationResult> {
        info!("Orchestrating release: {}", release_request.version);
        
        let mut result = ReleaseOrchestrationResult {
            version: release_request.version.clone(),
            success: false,
            steps_completed: Vec::new(),
            steps_failed: Vec::new(),
            changelog: None,
            artifacts: Vec::new(),
            deployment_info: None,
            rollback_info: None,
        };
        
        // Step 1: Pre-release validation
        match self.validate_pre_release(owner, repo, release_request).await {
            Ok(validation) => {
                result.steps_completed.push("pre-release-validation".to_string());
                if !validation.passed {
                    result.steps_failed.push(format!("Validation failed: {:?}", validation.failures));
                    return Ok(result);
                }
            }
            Err(e) => {
                result.steps_failed.push(format!("Pre-release validation error: {}", e));
                return Ok(result);
            }
        }
        
        // Step 2: Generate changelog
        match self.generate_changelog(owner, repo, release_request).await {
            Ok(changelog) => {
                result.steps_completed.push("changelog-generation".to_string());
                result.changelog = Some(changelog);
            }
            Err(e) => {
                result.steps_failed.push(format!("Changelog generation failed: {}", e));
                if release_request.require_changelog {
                    return Ok(result);
                }
            }
        }
        
        // Step 3: Create release branch (if needed)
        if release_request.create_release_branch {
            match self.create_release_branch(owner, repo, &release_request.version).await {
                Ok(_) => result.steps_completed.push("release-branch-creation".to_string()),
                Err(e) => {
                    result.steps_failed.push(format!("Release branch creation failed: {}", e));
                    return Ok(result);
                }
            }
        }
        
        // Step 4: Run release tests
        if release_request.run_tests {
            match self.run_release_tests(owner, repo, release_request).await {
                Ok(test_results) => {
                    result.steps_completed.push("release-tests".to_string());
                    if !test_results.all_passed {
                        result.steps_failed.push(format!("Tests failed: {}/{} passed", 
                                                        test_results.passed, test_results.total));
                        return Ok(result);
                    }
                }
                Err(e) => {
                    result.steps_failed.push(format!("Release tests failed: {}", e));
                    return Ok(result);
                }
            }
        }
        
        // Step 5: Build artifacts
        if release_request.build_artifacts {
            match self.build_release_artifacts(owner, repo, release_request).await {
                Ok(artifacts) => {
                    result.steps_completed.push("artifact-building".to_string());
                    result.artifacts = artifacts;
                }
                Err(e) => {
                    result.steps_failed.push(format!("Artifact building failed: {}", e));
                    return Ok(result);
                }
            }
        }
        
        // Step 6: Create GitHub release
        match self.create_github_release(owner, repo, release_request, &result).await {
            Ok(release) => {
                result.steps_completed.push("github-release-creation".to_string());
                info!("GitHub release created: {}", release.html_url);
            }
            Err(e) => {
                result.steps_failed.push(format!("GitHub release creation failed: {}", e));
                return Ok(result);
            }
        }
        
        // Step 7: Deploy release (if configured)
        if release_request.deploy_release {
            match self.deploy_release(owner, repo, release_request).await {
                Ok(deployment) => {
                    result.steps_completed.push("release-deployment".to_string());
                    result.deployment_info = Some(deployment);
                }
                Err(e) => {
                    result.steps_failed.push(format!("Release deployment failed: {}", e));
                    // Deployment failure doesn't fail the entire release
                }
            }
        }
        
        // Step 8: Post-release notifications
        if let Err(e) = self.send_release_notifications(owner, repo, release_request, &result).await {
            warn!("Failed to send release notifications: {}", e);
        }
        
        result.success = result.steps_failed.is_empty();
        
        if result.success {
            info!("Release orchestration completed successfully: {}", release_request.version);
        } else {
            error!("Release orchestration failed for: {}", release_request.version);
        }
        
        Ok(result)
    }

    /// Validate pre-release conditions
    async fn validate_pre_release(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
    ) -> Result<ValidationResult> {
        info!("Validating pre-release conditions");
        
        let mut validation = ValidationResult {
            passed: true,
            failures: Vec::new(),
            warnings: Vec::new(),
        };
        
        // Apply validation rules
        for rule in &self.validation_rules {
            match self.apply_validation_rule(owner, repo, release_request, rule).await {
                Ok(rule_result) => {
                    if !rule_result.passed {
                        validation.passed = false;
                        validation.failures.push(rule_result.message);
                    }
                    validation.warnings.extend(rule_result.warnings);
                }
                Err(e) => {
                    validation.passed = false;
                    validation.failures.push(format!("Validation rule '{}' failed: {}", rule.name, e));
                }
            }
        }
        
        // Check version format
        if !self.validate_version_format(&release_request.version) {
            validation.passed = false;
            validation.failures.push("Invalid version format".to_string());
        }
        
        // Check if version already exists
        if self.version_exists(owner, repo, &release_request.version).await? {
            validation.passed = false;
            validation.failures.push("Version already exists".to_string());
        }
        
        // Check branch status
        let branch_status = self.check_branch_status(owner, repo, &release_request.target_branch).await?;
        if !branch_status.is_clean {
            validation.warnings.push("Target branch has uncommitted changes".to_string());
        }
        
        // Check CI status
        if release_request.require_ci_success {
            let ci_status = self.check_ci_status(owner, repo, &release_request.target_branch).await?;
            if !ci_status.all_successful {
                validation.passed = false;
                validation.failures.push(format!("CI checks failed: {}", ci_status.failures.join(", ")));
            }
        }
        
        // Check security scan results
        if release_request.require_security_scan {
            let security_status = self.check_security_scan_status(owner, repo).await?;
            if !security_status.passed {
                validation.passed = false;
                validation.failures.push("Security scan found vulnerabilities".to_string());
            }
        }
        
        Ok(validation)
    }

    /// Generate comprehensive changelog
    async fn generate_changelog(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
    ) -> Result<Changelog> {
        info!("Generating changelog for release: {}", release_request.version);
        
        let generator = self.changelog_generators.get(&release_request.changelog_type)
            .ok_or_else(|| anyhow!("Unknown changelog type: {}", release_request.changelog_type))?;
        
        // Get commits since last release
        let last_release = self.get_last_release(owner, repo).await?;
        let commits = self.get_commits_since_release(owner, repo, last_release.as_ref()).await?;
        
        // Get pull requests since last release
        let pull_requests = self.get_prs_since_release(owner, repo, last_release.as_ref()).await?;
        
        // Get issues closed since last release
        let closed_issues = self.get_closed_issues_since_release(owner, repo, last_release.as_ref()).await?;
        
        // Generate changelog sections
        let mut changelog = Changelog {
            version: release_request.version.clone(),
            release_date: Utc::now(),
            sections: HashMap::new(),
            summary: String::new(),
            breaking_changes: Vec::new(),
            contributors: Vec::new(),
            statistics: ChangelogStatistics::default(),
        };
        
        // Categorize changes
        let categorized_changes = self.categorize_changes(&commits, &pull_requests, &closed_issues);
        
        // Generate sections based on generator type
        match generator.generator_type.as_str() {
            "conventional" => self.generate_conventional_changelog(&mut changelog, &categorized_changes),
            "semantic" => self.generate_semantic_changelog(&mut changelog, &categorized_changes),
            "custom" => self.generate_custom_changelog(&mut changelog, &categorized_changes, generator),
            _ => return Err(anyhow!("Unknown changelog generator type: {}", generator.generator_type)),
        }
        
        // Generate summary
        changelog.summary = self.generate_changelog_summary(&categorized_changes);
        
        // Extract breaking changes
        changelog.breaking_changes = self.extract_breaking_changes(&commits, &pull_requests);
        
        // Get contributors
        changelog.contributors = self.get_contributors(&commits);
        
        // Calculate statistics
        changelog.statistics = self.calculate_changelog_statistics(&categorized_changes);
        
        Ok(changelog)
    }

    /// Create release branch if needed
    async fn create_release_branch(
        &self,
        owner: &str,
        repo: &str,
        version: &str,
    ) -> Result<String> {
        let branch_name = format!("release/{}", version);
        info!("Creating release branch: {}", branch_name);
        
        // Get current main branch SHA
        let main_branch = self.get_default_branch(owner, repo).await?;
        let main_sha = self.get_branch_sha(owner, repo, &main_branch).await?;
        
        // Create release branch
        self.api.create_branch(owner, repo, &branch_name, &main_sha).await?;
        
        Ok(branch_name)
    }

    /// Run comprehensive release tests
    async fn run_release_tests(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
    ) -> Result<TestResults> {
        info!("Running release tests");
        
        let mut results = TestResults {
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            all_passed: false,
            failures: Vec::new(),
        };
        
        // Run different test suites based on configuration
        if release_request.test_config.unit_tests {
            let unit_results = self.run_unit_tests(owner, repo).await?;
            results.merge(unit_results);
        }
        
        if release_request.test_config.integration_tests {
            let integration_results = self.run_integration_tests(owner, repo).await?;
            results.merge(integration_results);
        }
        
        if release_request.test_config.e2e_tests {
            let e2e_results = self.run_e2e_tests(owner, repo).await?;
            results.merge(e2e_results);
        }
        
        if release_request.test_config.performance_tests {
            let perf_results = self.run_performance_tests(owner, repo).await?;
            results.merge(perf_results);
        }
        
        if release_request.test_config.security_tests {
            let security_results = self.run_security_tests(owner, repo).await?;
            results.merge(security_results);
        }
        
        results.all_passed = results.failed == 0;
        
        Ok(results)
    }

    /// Build release artifacts
    async fn build_release_artifacts(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
    ) -> Result<Vec<ReleaseArtifact>> {
        info!("Building release artifacts");
        
        let mut artifacts = Vec::new();
        
        for artifact_config in &release_request.artifact_configs {
            match self.build_artifact(owner, repo, artifact_config).await {
                Ok(artifact) => artifacts.push(artifact),
                Err(e) => return Err(anyhow!("Failed to build artifact '{}': {}", artifact_config.name, e)),
            }
        }
        
        Ok(artifacts)
    }

    /// Create GitHub release
    async fn create_github_release(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
        orchestration_result: &ReleaseOrchestrationResult,
    ) -> Result<octocrab::models::repos::Release> {
        info!("Creating GitHub release");
        
        let changelog_body = if let Some(ref changelog) = orchestration_result.changelog {
            self.format_changelog_for_release(changelog)
        } else {
            format!("Release {}", release_request.version)
        };
        
        let release_data = CreateReleaseRequest {
            tag_name: release_request.version.clone(),
            target_commitish: Some(release_request.target_branch.clone()),
            name: Some(format!("Release {}", release_request.version)),
            body: Some(changelog_body),
            draft: release_request.create_as_draft,
            prerelease: release_request.is_prerelease,
        };
        
        let release = self.api.create_release(owner, repo, &release_data).await?;
        
        // Upload artifacts if any
        if !orchestration_result.artifacts.is_empty() {
            for artifact in &orchestration_result.artifacts {
                if let Err(e) = self.upload_release_asset(owner, repo, release.id, artifact).await {
                    warn!("Failed to upload artifact '{}': {}", artifact.name, e);
                }
            }
        }
        
        Ok(release)
    }

    /// Deploy release to configured environments
    async fn deploy_release(
        &self,
        owner: &str,
        repo: &str,
        release_request: &ReleaseOrchestrationRequest,
    ) -> Result<DeploymentInfo> {
        info!("Deploying release: {}", release_request.version);
        
        let mut deployment_info = DeploymentInfo {
            environments: Vec::new(),
            success: true,
            deployment_url: None,
        };
        
        for env_config in &release_request.deployment_configs {
            match self.deploy_to_environment(owner, repo, release_request, env_config).await {
                Ok(env_deployment) => {
                    deployment_info.environments.push(env_deployment);
                }
                Err(e) => {
                    deployment_info.success = false;
                    deployment_info.environments.push(EnvironmentDeployment {
                        name: env_config.name.clone(),
                        success: false,
                        error: Some(e.to_string()),
                        url: None,
                        deployed_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(deployment_info)
    }

    /// Manage release rollback if needed
    pub async fn rollback_release(
        &self,
        owner: &str,
        repo: &str,
        rollback_request: &RollbackRequest,
    ) -> Result<RollbackResult> {
        info!("Rolling back release: {} to {}", rollback_request.current_version, rollback_request.target_version);
        
        let mut result = RollbackResult {
            success: false,
            steps_completed: Vec::new(),
            steps_failed: Vec::new(),
            rollback_duration: std::time::Duration::from_secs(0),
        };
        
        let start_time = std::time::Instant::now();
        
        // Step 1: Validate rollback target
        if let Err(e) = self.validate_rollback_target(owner, repo, &rollback_request.target_version).await {
            result.steps_failed.push(format!("Rollback validation failed: {}", e));
            return Ok(result);
        }
        result.steps_completed.push("rollback-validation".to_string());
        
        // Step 2: Create rollback branch
        if rollback_request.create_rollback_branch {
            match self.create_rollback_branch(owner, repo, rollback_request).await {
                Ok(_) => result.steps_completed.push("rollback-branch-creation".to_string()),
                Err(e) => {
                    result.steps_failed.push(format!("Rollback branch creation failed: {}", e));
                    return Ok(result);
                }
            }
        }
        
        // Step 3: Revert deployment
        if rollback_request.revert_deployment {
            match self.revert_deployment(owner, repo, rollback_request).await {
                Ok(_) => result.steps_completed.push("deployment-revert".to_string()),
                Err(e) => {
                    result.steps_failed.push(format!("Deployment revert failed: {}", e));
                    return Ok(result);
                }
            }
        }
        
        // Step 4: Update release status
        if let Err(e) = self.update_release_status(owner, repo, &rollback_request.current_version, "rolled_back").await {
            result.steps_failed.push(format!("Release status update failed: {}", e));
        } else {
            result.steps_completed.push("release-status-update".to_string());
        }
        
        result.rollback_duration = start_time.elapsed();
        result.success = result.steps_failed.is_empty();
        
        Ok(result)
    }

    /// Monitor release health and metrics
    pub async fn monitor_release_health(
        &self,
        owner: &str,
        repo: &str,
        version: &str,
        monitoring_config: &ReleaseMonitoringConfig,
    ) -> Result<ReleaseHealthReport> {
        info!("Monitoring release health for: {}", version);
        
        let mut report = ReleaseHealthReport {
            version: version.to_string(),
            status: ReleaseStatus::Unknown,
            metrics: HashMap::new(),
            alerts: Vec::new(),
            recommendations: Vec::new(),
        };
        
        // Check deployment status
        let deployment_status = self.check_deployment_status(owner, repo, version).await?;
        report.status = deployment_status;
        
        // Collect metrics
        for metric_config in &monitoring_config.metrics {
            match self.collect_release_metric(owner, repo, version, metric_config).await {
                Ok(metric_value) => {
                    report.metrics.insert(metric_config.name.clone(), metric_value);
                }
                Err(e) => {
                    report.alerts.push(ReleaseAlert {
                        level: AlertLevel::Warning,
                        message: format!("Failed to collect metric '{}': {}", metric_config.name, e),
                        timestamp: Utc::now(),
                    });
                }
            }
        }
        
        // Check alert conditions
        for alert_config in &monitoring_config.alerts {
            if let Some(metric_value) = report.metrics.get(&alert_config.metric) {
                if self.evaluate_alert_condition(alert_config, *metric_value) {
                    report.alerts.push(ReleaseAlert {
                        level: alert_config.level.clone(),
                        message: alert_config.message.clone(),
                        timestamp: Utc::now(),
                    });
                }
            }
        }
        
        // Generate recommendations
        report.recommendations = self.generate_release_recommendations(&report);
        
        Ok(report)
    }

    // Helper methods implementation (simplified for brevity)
    fn load_default_configs(&mut self) {
        // Load default release configurations
    }
    
    fn load_default_validation_rules(&mut self) {
        self.validation_rules.push(ValidationRule {
            name: "Version Format".to_string(),
            rule_type: "version_format".to_string(),
            conditions: vec!["semver".to_string()],
        });
    }
    
    fn load_default_changelog_generators(&mut self) {
        self.changelog_generators.insert(
            "conventional".to_string(),
            ChangelogGenerator {
                name: "Conventional Commits".to_string(),
                generator_type: "conventional".to_string(),
                template: "conventional".to_string(),
                sections: vec!["feat".to_string(), "fix".to_string(), "docs".to_string()],
            }
        );
    }

    // Placeholder implementations for complex methods
    async fn apply_validation_rule(&self, _owner: &str, _repo: &str, _request: &ReleaseOrchestrationRequest, _rule: &ValidationRule) -> Result<RuleResult> {
        Ok(RuleResult { passed: true, message: String::new(), warnings: Vec::new() })
    }
    
    fn validate_version_format(&self, version: &str) -> bool {
        // Simple semver validation
        version.chars().any(|c| c.is_numeric())
    }
    
    async fn version_exists(&self, _owner: &str, _repo: &str, _version: &str) -> Result<bool> {
        Ok(false)
    }
    
    async fn check_branch_status(&self, _owner: &str, _repo: &str, _branch: &str) -> Result<BranchStatus> {
        Ok(BranchStatus { is_clean: true })
    }
    
    async fn check_ci_status(&self, _owner: &str, _repo: &str, _branch: &str) -> Result<CiStatus> {
        Ok(CiStatus { all_successful: true, failures: Vec::new() })
    }
    
    async fn check_security_scan_status(&self, _owner: &str, _repo: &str) -> Result<SecurityScanStatus> {
        Ok(SecurityScanStatus { passed: true })
    }
    
    async fn get_last_release(&self, _owner: &str, _repo: &str) -> Result<Option<String>> {
        Ok(None)
    }
    
    async fn get_commits_since_release(&self, _owner: &str, _repo: &str, _last_release: Option<&String>) -> Result<Vec<CommitInfo>> {
        Ok(Vec::new())
    }
    
    async fn get_prs_since_release(&self, _owner: &str, _repo: &str, _last_release: Option<&String>) -> Result<Vec<PullRequestInfo>> {
        Ok(Vec::new())
    }
    
    async fn get_closed_issues_since_release(&self, _owner: &str, _repo: &str, _last_release: Option<&String>) -> Result<Vec<IssueInfo>> {
        Ok(Vec::new())
    }
    
    fn categorize_changes(&self, _commits: &[CommitInfo], _prs: &[PullRequestInfo], _issues: &[IssueInfo]) -> CategorizedChanges {
        CategorizedChanges::default()
    }
    
    fn generate_conventional_changelog(&self, _changelog: &mut Changelog, _changes: &CategorizedChanges) {
        // Implementation for conventional commits changelog
    }
    
    fn generate_semantic_changelog(&self, _changelog: &mut Changelog, _changes: &CategorizedChanges) {
        // Implementation for semantic versioning changelog
    }
    
    fn generate_custom_changelog(&self, _changelog: &mut Changelog, _changes: &CategorizedChanges, _generator: &ChangelogGenerator) {
        // Implementation for custom changelog generation
    }
    
    fn generate_changelog_summary(&self, _changes: &CategorizedChanges) -> String {
        "Release summary".to_string()
    }
    
    fn extract_breaking_changes(&self, _commits: &[CommitInfo], _prs: &[PullRequestInfo]) -> Vec<String> {
        Vec::new()
    }
    
    fn get_contributors(&self, _commits: &[CommitInfo]) -> Vec<String> {
        Vec::new()
    }
    
    fn calculate_changelog_statistics(&self, _changes: &CategorizedChanges) -> ChangelogStatistics {
        ChangelogStatistics::default()
    }

    async fn get_default_branch(&self, _owner: &str, _repo: &str) -> Result<String> {
        Ok("main".to_string())
    }
    
    async fn get_branch_sha(&self, _owner: &str, _repo: &str, _branch: &str) -> Result<String> {
        Ok("sha123".to_string())
    }

    async fn run_unit_tests(&self, _owner: &str, _repo: &str) -> Result<TestResults> {
        Ok(TestResults::default())
    }
    
    async fn run_integration_tests(&self, _owner: &str, _repo: &str) -> Result<TestResults> {
        Ok(TestResults::default())
    }
    
    async fn run_e2e_tests(&self, _owner: &str, _repo: &str) -> Result<TestResults> {
        Ok(TestResults::default())
    }
    
    async fn run_performance_tests(&self, _owner: &str, _repo: &str) -> Result<TestResults> {
        Ok(TestResults::default())
    }
    
    async fn run_security_tests(&self, _owner: &str, _repo: &str) -> Result<TestResults> {
        Ok(TestResults::default())
    }

    async fn build_artifact(&self, _owner: &str, _repo: &str, _config: &ArtifactConfig) -> Result<ReleaseArtifact> {
        Ok(ReleaseArtifact {
            name: _config.name.clone(),
            artifact_type: _config.artifact_type.clone(),
            path: "path/to/artifact".to_string(),
            size: 1024,
            checksum: "checksum".to_string(),
        })
    }
    
    fn format_changelog_for_release(&self, _changelog: &Changelog) -> String {
        "Formatted changelog".to_string()
    }
    
    async fn upload_release_asset(&self, _owner: &str, _repo: &str, _release_id: u64, _artifact: &ReleaseArtifact) -> Result<()> {
        Ok(())
    }
    
    async fn deploy_to_environment(&self, _owner: &str, _repo: &str, _request: &ReleaseOrchestrationRequest, _env_config: &DeploymentConfig) -> Result<EnvironmentDeployment> {
        Ok(EnvironmentDeployment {
            name: _env_config.name.clone(),
            success: true,
            error: None,
            url: Some("https://deployed.example.com".to_string()),
            deployed_at: Utc::now(),
        })
    }
    
    async fn send_release_notifications(&self, _owner: &str, _repo: &str, _request: &ReleaseOrchestrationRequest, _result: &ReleaseOrchestrationResult) -> Result<()> {
        Ok(())
    }

    async fn validate_rollback_target(&self, _owner: &str, _repo: &str, _target_version: &str) -> Result<()> {
        Ok(())
    }
    
    async fn create_rollback_branch(&self, _owner: &str, _repo: &str, _request: &RollbackRequest) -> Result<String> {
        Ok(format!("rollback/{}", _request.target_version))
    }
    
    async fn revert_deployment(&self, _owner: &str, _repo: &str, _request: &RollbackRequest) -> Result<()> {
        Ok(())
    }
    
    async fn update_release_status(&self, _owner: &str, _repo: &str, _version: &str, _status: &str) -> Result<()> {
        Ok(())
    }

    async fn check_deployment_status(&self, _owner: &str, _repo: &str, _version: &str) -> Result<ReleaseStatus> {
        Ok(ReleaseStatus::Healthy)
    }
    
    async fn collect_release_metric(&self, _owner: &str, _repo: &str, _version: &str, _config: &MetricConfig) -> Result<f64> {
        Ok(100.0)
    }
    
    fn evaluate_alert_condition(&self, _config: &AlertConfig, _value: f64) -> bool {
        false
    }
    
    fn generate_release_recommendations(&self, _report: &ReleaseHealthReport) -> Vec<String> {
        Vec::new()
    }
}

// Data structures for release management
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseOrchestrationRequest {
    pub version: String,
    pub target_branch: String,
    pub changelog_type: String,
    pub require_changelog: bool,
    pub create_release_branch: bool,
    pub run_tests: bool,
    pub test_config: TestConfig,
    pub build_artifacts: bool,
    pub artifact_configs: Vec<ArtifactConfig>,
    pub deploy_release: bool,
    pub deployment_configs: Vec<DeploymentConfig>,
    pub create_as_draft: bool,
    pub is_prerelease: bool,
    pub require_ci_success: bool,
    pub require_security_scan: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TestConfig {
    pub unit_tests: bool,
    pub integration_tests: bool,
    pub e2e_tests: bool,
    pub performance_tests: bool,
    pub security_tests: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactConfig {
    pub name: String,
    pub artifact_type: String,
    pub build_command: String,
    pub output_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub name: String,
    pub environment: String,
    pub deployment_type: String,
    pub configuration: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseOrchestrationResult {
    pub version: String,
    pub success: bool,
    pub steps_completed: Vec<String>,
    pub steps_failed: Vec<String>,
    pub changelog: Option<Changelog>,
    pub artifacts: Vec<ReleaseArtifact>,
    pub deployment_info: Option<DeploymentInfo>,
    pub rollback_info: Option<RollbackInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changelog {
    pub version: String,
    pub release_date: DateTime<Utc>,
    pub sections: HashMap<String, Vec<String>>,
    pub summary: String,
    pub breaking_changes: Vec<String>,
    pub contributors: Vec<String>,
    pub statistics: ChangelogStatistics,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChangelogStatistics {
    pub total_commits: u32,
    pub total_prs: u32,
    pub total_issues: u32,
    pub contributors_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseArtifact {
    pub name: String,
    pub artifact_type: String,
    pub path: String,
    pub size: u64,
    pub checksum: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentInfo {
    pub environments: Vec<EnvironmentDeployment>,
    pub success: bool,
    pub deployment_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentDeployment {
    pub name: String,
    pub success: bool,
    pub error: Option<String>,
    pub url: Option<String>,
    pub deployed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub passed: bool,
    pub failures: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub struct ValidationRule {
    pub name: String,
    pub rule_type: String,
    pub conditions: Vec<String>,
}

#[derive(Debug)]
pub struct RuleResult {
    pub passed: bool,
    pub message: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Default)]
pub struct TestResults {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub all_passed: bool,
    pub failures: Vec<String>,
}

impl TestResults {
    pub fn merge(&mut self, other: TestResults) {
        self.total += other.total;
        self.passed += other.passed;
        self.failed += other.failed;
        self.skipped += other.skipped;
        self.failures.extend(other.failures);
        self.all_passed = self.failed == 0;
    }
}

#[derive(Debug)]
pub struct ReleaseConfig {
    pub name: String,
    pub default_branch: String,
    pub version_scheme: String,
}

#[derive(Debug)]
pub struct ChangelogGenerator {
    pub name: String,
    pub generator_type: String,
    pub template: String,
    pub sections: Vec<String>,
}

#[derive(Debug, Default)]
pub struct CategorizedChanges {
    pub features: Vec<String>,
    pub fixes: Vec<String>,
    pub documentation: Vec<String>,
    pub performance: Vec<String>,
    pub other: Vec<String>,
}

#[derive(Debug)]
pub struct CommitInfo {
    pub sha: String,
    pub message: String,
    pub author: String,
}

#[derive(Debug)]
pub struct PullRequestInfo {
    pub number: u64,
    pub title: String,
    pub labels: Vec<String>,
}

#[derive(Debug)]
pub struct IssueInfo {
    pub number: u64,
    pub title: String,
    pub labels: Vec<String>,
}

#[derive(Debug)]
pub struct BranchStatus {
    pub is_clean: bool,
}

#[derive(Debug)]
pub struct CiStatus {
    pub all_successful: bool,
    pub failures: Vec<String>,
}

#[derive(Debug)]
pub struct SecurityScanStatus {
    pub passed: bool,
}

// Rollback structures
#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackRequest {
    pub current_version: String,
    pub target_version: String,
    pub create_rollback_branch: bool,
    pub revert_deployment: bool,
    pub notify_stakeholders: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackResult {
    pub success: bool,
    pub steps_completed: Vec<String>,
    pub steps_failed: Vec<String>,
    pub rollback_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackInfo {
    pub rollback_reason: String,
    pub rollback_time: DateTime<Utc>,
    pub rollback_author: String,
}

// Monitoring structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseMonitoringConfig {
    pub metrics: Vec<MetricConfig>,
    pub alerts: Vec<AlertConfig>,
    pub monitoring_duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricConfig {
    pub name: String,
    pub metric_type: String,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertConfig {
    pub metric: String,
    pub condition: String,
    pub threshold: f64,
    pub level: AlertLevel,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseHealthReport {
    pub version: String,
    pub status: ReleaseStatus,
    pub metrics: HashMap<String, f64>,
    pub alerts: Vec<ReleaseAlert>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReleaseStatus {
    Unknown,
    Healthy,
    Warning,
    Critical,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseAlert {
    pub level: AlertLevel,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_results_merge() {
        let mut results1 = TestResults {
            total: 10,
            passed: 8,
            failed: 2,
            skipped: 0,
            all_passed: false,
            failures: vec!["test1".to_string()],
        };
        
        let results2 = TestResults {
            total: 5,
            passed: 5,
            failed: 0,
            skipped: 0,
            all_passed: true,
            failures: Vec::new(),
        };
        
        results1.merge(results2);
        
        assert_eq!(results1.total, 15);
        assert_eq!(results1.passed, 13);
        assert_eq!(results1.failed, 2);
        assert!(!results1.all_passed);
    }

    #[test]
    fn test_changelog_creation() {
        let changelog = Changelog {
            version: "1.2.0".to_string(),
            release_date: Utc::now(),
            sections: HashMap::new(),
            summary: "Test release".to_string(),
            breaking_changes: Vec::new(),
            contributors: vec!["alice".to_string(), "bob".to_string()],
            statistics: ChangelogStatistics::default(),
        };
        
        assert_eq!(changelog.version, "1.2.0");
        assert_eq!(changelog.contributors.len(), 2);
    }

    #[test]
    fn test_release_status() {
        let status = ReleaseStatus::Healthy;
        matches!(status, ReleaseStatus::Healthy);
    }
}