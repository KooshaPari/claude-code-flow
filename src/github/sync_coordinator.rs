use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn, error};

use crate::github::api::GitHubApiClient;

/// Multi-package synchronization and coordination
#[derive(Debug)]
pub struct SyncCoordinator {
    api: GitHubApiClient,
    sync_configs: HashMap<String, serde_json::Value>,
    dependency_graph: DependencyGraph,
    sync_history: Vec<SyncOperation>,
}

impl SyncCoordinator {
    /// Create new sync coordinator
    pub fn new(api: GitHubApiClient) -> Self {
        Self {
            api,
            sync_configs: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
            sync_history: Vec::new(),
        }
    }

    /// Synchronize packages across multiple repositories
    pub async fn synchronize_packages(
        &mut self,
        sync_request: &PackageSyncRequest,
    ) -> Result<PackageSyncResult> {
        info!("Starting package synchronization for {} repositories", sync_request.repositories.len());
        
        let mut result = PackageSyncResult {
            sync_id: uuid::Uuid::new_v4().to_string(),
            started_at: Utc::now(),
            completed_at: None,
            success: false,
            repositories_processed: 0,
            repositories_failed: 0,
            package_updates: Vec::new(),
            dependency_conflicts: Vec::new(),
            created_pull_requests: Vec::new(),
            error_summary: Vec::new(),
        };
        
        // Validate repositories and access
        let validated_repos = self.validate_repositories(&sync_request.repositories).await?;
        if validated_repos.len() != sync_request.repositories.len() {
            return Err(anyhow!("Some repositories are not accessible"));
        }
        
        // Analyze current package versions across repositories
        let current_state = self.analyze_current_package_state(&sync_request.repositories).await?;
        
        // Determine synchronization strategy
        let sync_strategy = self.determine_sync_strategy(sync_request, &current_state)?;
        
        // Build dependency graph
        self.build_dependency_graph(&sync_request.repositories, &current_state).await?;
        
        // Calculate optimal sync order
        let sync_order = self.calculate_sync_order(&sync_request.repositories)?;
        
        // Execute synchronization for each repository
        for repo_info in sync_order {
            match self.sync_repository_packages(repo_info, sync_request, &current_state, &sync_strategy).await {
                Ok(repo_result) => {
                    result.repositories_processed += 1;
                    result.package_updates.extend(repo_result.package_updates);
                    result.created_pull_requests.extend(repo_result.pull_requests);
                    
                    // Check for dependency conflicts
                    let conflicts = self.check_dependency_conflicts(&repo_result, &current_state);
                    result.dependency_conflicts.extend(conflicts);
                }
                Err(e) => {
                    result.repositories_failed += 1;
                    result.error_summary.push(format!("{}: {}", repo_info.name, e));
                    error!("Failed to sync repository {}: {}", repo_info.name, e);
                    
                    if sync_request.fail_fast {
                        break;
                    }
                }
            }
        }
        
        // Validate final state
        if result.repositories_failed == 0 {
            match self.validate_final_sync_state(&sync_request.repositories, sync_request).await {
                Ok(_) => {
                    result.success = true;
                    info!("Package synchronization completed successfully");
                }
                Err(e) => {
                    result.error_summary.push(format!("Final validation failed: {}", e));
                    warn!("Package synchronization completed with validation errors");
                }
            }
        }
        
        result.completed_at = Some(Utc::now());
        
        // Record sync operation
        let operation = SyncOperation {
            sync_id: result.sync_id.clone(),
            operation_type: SyncOperationType::PackageSync,
            started_at: result.started_at,
            completed_at: result.completed_at,
            success: result.success,
            repositories: sync_request.repositories.iter().map(|r| r.name.clone()).collect(),
            details: serde_json::to_value(&result).unwrap_or_default(),
        };
        self.sync_history.push(operation);
        
        Ok(result)
    }

    /// Synchronize configuration files across repositories
    pub async fn synchronize_configurations(
        &mut self,
        sync_request: &ConfigSyncRequest,
    ) -> Result<ConfigSyncResult> {
        info!("Starting configuration synchronization for {} repositories", sync_request.repositories.len());
        
        let mut result = ConfigSyncResult {
            sync_id: uuid::Uuid::new_v4().to_string(),
            started_at: Utc::now(),
            completed_at: None,
            success: false,
            configurations_synced: HashMap::new(),
            conflicts_resolved: Vec::new(),
            created_pull_requests: Vec::new(),
            error_summary: Vec::new(),
        };
        
        // Validate configuration templates
        self.validate_config_templates(&sync_request.config_templates)?;
        
        // Process each repository
        for repo_info in &sync_request.repositories {
            match self.sync_repository_configs(repo_info, sync_request).await {
                Ok(repo_result) => {
                    result.configurations_synced.insert(repo_info.name.clone(), repo_result.configs_updated);
                    result.conflicts_resolved.extend(repo_result.conflicts_resolved);
                    result.created_pull_requests.extend(repo_result.pull_requests);
                }
                Err(e) => {
                    result.error_summary.push(format!("{}: {}", repo_info.name, e));
                    error!("Failed to sync configurations for {}: {}", repo_info.name, e);
                    
                    if sync_request.fail_fast {
                        break;
                    }
                }
            }
        }
        
        result.success = result.error_summary.is_empty();
        result.completed_at = Some(Utc::now());
        
        // Record sync operation
        let operation = SyncOperation {
            sync_id: result.sync_id.clone(),
            operation_type: SyncOperationType::ConfigSync,
            started_at: result.started_at,
            completed_at: result.completed_at,
            success: result.success,
            repositories: sync_request.repositories.iter().map(|r| r.name.clone()).collect(),
            details: serde_json::to_value(&result).unwrap_or_default(),
        };
        self.sync_history.push(operation);
        
        Ok(result)
    }

    /// Synchronize workflows across repositories
    pub async fn synchronize_workflows(
        &mut self,
        sync_request: &WorkflowSyncRequest,
    ) -> Result<WorkflowSyncResult> {
        info!("Starting workflow synchronization for {} repositories", sync_request.repositories.len());
        
        let mut result = WorkflowSyncResult {
            sync_id: uuid::Uuid::new_v4().to_string(),
            started_at: Utc::now(),
            completed_at: None,
            success: false,
            workflows_synced: HashMap::new(),
            workflow_validations: Vec::new(),
            created_pull_requests: Vec::new(),
            error_summary: Vec::new(),
        };
        
        // Validate workflow templates
        self.validate_workflow_templates(&sync_request.workflow_templates)?;
        
        // Process each repository
        for repo_info in &sync_request.repositories {
            match self.sync_repository_workflows(repo_info, sync_request).await {
                Ok(repo_result) => {
                    result.workflows_synced.insert(repo_info.name.clone(), repo_result.workflows_updated);
                    result.workflow_validations.extend(repo_result.validations);
                    result.created_pull_requests.extend(repo_result.pull_requests);
                }
                Err(e) => {
                    result.error_summary.push(format!("{}: {}", repo_info.name, e));
                    error!("Failed to sync workflows for {}: {}", repo_info.name, e);
                    
                    if sync_request.fail_fast {
                        break;
                    }
                }
            }
        }
        
        result.success = result.error_summary.is_empty();
        result.completed_at = Some(Utc::now());
        
        // Record sync operation
        let operation = SyncOperation {
            sync_id: result.sync_id.clone(),
            operation_type: SyncOperationType::WorkflowSync,
            started_at: result.started_at,
            completed_at: result.completed_at,
            success: result.success,
            repositories: sync_request.repositories.iter().map(|r| r.name.clone()).collect(),
            details: serde_json::to_value(&result).unwrap_or_default(),
        };
        self.sync_history.push(operation);
        
        Ok(result)
    }

    /// Monitor synchronization status across repositories
    pub async fn monitor_sync_status(
        &self,
        repositories: &[RepositoryInfo],
    ) -> Result<SyncStatusReport> {
        info!("Monitoring sync status for {} repositories", repositories.len());
        
        let mut report = SyncStatusReport {
            monitored_at: Utc::now(),
            repositories: Vec::new(),
            overall_sync_health: SyncHealth::Unknown,
            drift_detected: Vec::new(),
            recommendations: Vec::new(),
        };
        
        // Check each repository
        for repo_info in repositories {
            let repo_status = self.check_repository_sync_status(repo_info).await?;
            report.repositories.push(repo_status);
        }
        
        // Analyze overall sync health
        report.overall_sync_health = self.calculate_overall_sync_health(&report.repositories);
        
        // Detect configuration drift
        report.drift_detected = self.detect_configuration_drift(&report.repositories);
        
        // Generate recommendations
        report.recommendations = self.generate_sync_recommendations(&report);
        
        Ok(report)
    }

    /// Create synchronization plan for multiple repositories
    pub async fn create_sync_plan(
        &self,
        plan_request: &SyncPlanRequest,
    ) -> Result<SyncPlan> {
        info!("Creating synchronization plan for {} repositories", plan_request.repositories.len());
        
        // Analyze current state
        let current_state = self.analyze_multi_repo_state(&plan_request.repositories).await?;
        
        // Identify differences
        let differences = self.identify_sync_differences(&current_state, &plan_request.target_state);
        
        // Calculate dependencies
        let dependencies = self.calculate_sync_dependencies(&differences);
        
        // Generate sync steps
        let sync_steps = self.generate_sync_steps(&differences, &dependencies);
        
        // Estimate effort and risks
        let effort_estimate = self.estimate_sync_effort(&sync_steps);
        let risk_assessment = self.assess_sync_risks(&sync_steps, &current_state);
        
        let plan = SyncPlan {
            plan_id: uuid::Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            repositories: plan_request.repositories.clone(),
            target_state: plan_request.target_state.clone(),
            current_state,
            sync_steps,
            dependencies,
            effort_estimate,
            risk_assessment,
            execution_order: self.calculate_execution_order(&dependencies),
        };
        
        Ok(plan)
    }

    /// Execute a pre-planned synchronization
    pub async fn execute_sync_plan(
        &mut self,
        plan: &SyncPlan,
        execution_config: &PlanExecutionConfig,
    ) -> Result<PlanExecutionResult> {
        info!("Executing sync plan: {}", plan.plan_id);
        
        let mut result = PlanExecutionResult {
            plan_id: plan.plan_id.clone(),
            started_at: Utc::now(),
            completed_at: None,
            success: false,
            steps_completed: Vec::new(),
            steps_failed: Vec::new(),
            rollback_performed: false,
        };
        
        // Execute steps in order
        for step in &plan.execution_order {
            match self.execute_sync_step(step, execution_config).await {
                Ok(step_result) => {
                    result.steps_completed.push(step_result);
                    info!("Sync step completed: {}", step.name);
                }
                Err(e) => {
                    let failure = SyncStepFailure {
                        step_name: step.name.clone(),
                        error: e.to_string(),
                        failed_at: Utc::now(),
                    };
                    result.steps_failed.push(failure);
                    error!("Sync step failed: {} - {}", step.name, e);
                    
                    // Perform rollback if configured
                    if execution_config.rollback_on_failure {
                        match self.rollback_sync_steps(&result.steps_completed).await {
                            Ok(_) => {
                                result.rollback_performed = true;
                                info!("Rollback completed successfully");
                            }
                            Err(rollback_err) => {
                                error!("Rollback failed: {}", rollback_err);
                            }
                        }
                    }
                    
                    break;
                }
            }
        }
        
        result.success = result.steps_failed.is_empty();
        result.completed_at = Some(Utc::now());
        
        if result.success {
            info!("Sync plan execution completed successfully");
        } else {
            error!("Sync plan execution failed with {} errors", result.steps_failed.len());
        }
        
        Ok(result)
    }

    // Helper methods for package synchronization
    async fn validate_repositories(&self, repositories: &[RepositoryInfo]) -> Result<Vec<RepositoryInfo>> {
        let mut validated = Vec::new();
        
        for repo in repositories {
            match self.api.get_repository(&repo.owner, &repo.name).await {
                Ok(_) => validated.push(repo.clone()),
                Err(e) => {
                    warn!("Repository {}/{} is not accessible: {}", repo.owner, repo.name, e);
                }
            }
        }
        
        Ok(validated)
    }

    async fn analyze_current_package_state(&self, repositories: &[RepositoryInfo]) -> Result<MultiRepoPackageState> {
        let mut state = MultiRepoPackageState {
            repositories: HashMap::new(),
            package_matrix: HashMap::new(),
            version_conflicts: Vec::new(),
        };
        
        for repo in repositories {
            match self.get_repository_package_info(repo).await {
                Ok(package_info) => {
                    state.repositories.insert(format!("{}/{}", repo.owner, repo.name), package_info);
                }
                Err(e) => {
                    warn!("Failed to get package info for {}/{}: {}", repo.owner, repo.name, e);
                }
            }
        }
        
        // Build package matrix
        state.package_matrix = self.build_package_matrix(&state.repositories);
        
        // Detect version conflicts
        state.version_conflicts = self.detect_version_conflicts(&state.package_matrix);
        
        Ok(state)
    }

    fn determine_sync_strategy(&self, request: &PackageSyncRequest, state: &MultiRepoPackageState) -> Result<SyncStrategy> {
        let strategy = match request.strategy.as_str() {
            "latest" => SyncStrategy::UseLatest,
            "specific" => SyncStrategy::UseSpecificVersions(request.target_versions.clone()),
            "conservative" => SyncStrategy::Conservative,
            "aggressive" => SyncStrategy::Aggressive,
            _ => return Err(anyhow!("Unknown sync strategy: {}", request.strategy)),
        };
        
        Ok(strategy)
    }

    async fn build_dependency_graph(&mut self, repositories: &[RepositoryInfo], state: &MultiRepoPackageState) -> Result<()> {
        self.dependency_graph = DependencyGraph::new();
        
        for repo in repositories {
            let repo_key = format!("{}/{}", repo.owner, repo.name);
            if let Some(package_info) = state.repositories.get(&repo_key) {
                self.dependency_graph.add_node(repo_key.clone(), package_info.clone());
                
                // Add edges for dependencies
                for dep in &package_info.dependencies {
                    for other_repo in repositories {
                        let other_key = format!("{}/{}", other_repo.owner, other_repo.name);
                        if other_key != repo_key {
                            if let Some(other_package) = state.repositories.get(&other_key) {
                                if other_package.name == dep.name {
                                    self.dependency_graph.add_edge(&repo_key, &other_key);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    fn calculate_sync_order(&self, repositories: &[RepositoryInfo]) -> Result<Vec<RepositoryInfo>> {
        // Perform topological sort on dependency graph
        let sorted_keys = self.dependency_graph.topological_sort()?;
        
        let mut ordered_repos = Vec::new();
        for key in sorted_keys {
            for repo in repositories {
                let repo_key = format!("{}/{}", repo.owner, repo.name);
                if repo_key == key {
                    ordered_repos.push(repo.clone());
                    break;
                }
            }
        }
        
        // Add any remaining repositories that weren't in the dependency graph
        for repo in repositories {
            if !ordered_repos.iter().any(|r| r.name == repo.name && r.owner == repo.owner) {
                ordered_repos.push(repo.clone());
            }
        }
        
        Ok(ordered_repos)
    }

    async fn sync_repository_packages(
        &self,
        repo: &RepositoryInfo,
        sync_request: &PackageSyncRequest,
        current_state: &MultiRepoPackageState,
        strategy: &SyncStrategy,
    ) -> Result<RepositoryPackageSyncResult> {
        info!("Syncing packages for {}/{}", repo.owner, repo.name);
        
        let mut result = RepositoryPackageSyncResult {
            repository: format!("{}/{}", repo.owner, repo.name),
            package_updates: Vec::new(),
            pull_requests: Vec::new(),
            conflicts: Vec::new(),
        };
        
        let repo_key = format!("{}/{}", repo.owner, repo.name);
        let current_packages = current_state.repositories.get(&repo_key);
        
        if let Some(packages) = current_packages {
            // Determine target versions based on strategy
            let target_versions = self.calculate_target_versions(packages, strategy, &sync_request.target_versions);
            
            // Generate package updates
            for (package_name, target_version) in target_versions {
                if let Some(current_version) = packages.dependencies.iter().find(|d| d.name == package_name) {
                    if current_version.version != target_version {
                        result.package_updates.push(PackageUpdate {
                            package_name: package_name.clone(),
                            from_version: current_version.version.clone(),
                            to_version: target_version.clone(),
                            update_type: self.classify_update_type(&current_version.version, &target_version),
                        });
                    }
                }
            }
            
            // Create pull request if there are updates
            if !result.package_updates.is_empty() && sync_request.create_pull_requests {
                match self.create_package_sync_pr(repo, &result.package_updates).await {
                    Ok(pr) => result.pull_requests.push(pr),
                    Err(e) => warn!("Failed to create PR for {}: {}", repo_key, e),
                }
            }
        }
        
        Ok(result)
    }

    // Additional helper methods (simplified implementations)
    async fn get_repository_package_info(&self, _repo: &RepositoryInfo) -> Result<RepositoryPackageInfo> {
        Ok(RepositoryPackageInfo {
            name: "example-package".to_string(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            package_manager: PackageManager::Npm,
        })
    }

    fn build_package_matrix(&self, _repositories: &HashMap<String, RepositoryPackageInfo>) -> HashMap<String, Vec<PackageVersion>> {
        HashMap::new()
    }

    fn detect_version_conflicts(&self, _matrix: &HashMap<String, Vec<PackageVersion>>) -> Vec<VersionConflict> {
        Vec::new()
    }

    fn calculate_target_versions(&self, _packages: &RepositoryPackageInfo, _strategy: &SyncStrategy, _target_versions: &HashMap<String, String>) -> HashMap<String, String> {
        HashMap::new()
    }

    fn classify_update_type(&self, _from: &str, _to: &str) -> UpdateType {
        UpdateType::Minor
    }

    async fn create_package_sync_pr(&self, _repo: &RepositoryInfo, _updates: &[PackageUpdate]) -> Result<PullRequestInfo> {
        Ok(PullRequestInfo {
            number: 123,
            title: "Sync package versions".to_string(),
            url: "https://github.com/owner/repo/pull/123".to_string(),
        })
    }

    fn check_dependency_conflicts(&self, _result: &RepositoryPackageSyncResult, _state: &MultiRepoPackageState) -> Vec<DependencyConflict> {
        Vec::new()
    }

    async fn validate_final_sync_state(&self, _repositories: &[RepositoryInfo], _request: &PackageSyncRequest) -> Result<()> {
        Ok(())
    }

    // Configuration sync methods (simplified)
    fn validate_config_templates(&self, _templates: &[ConfigTemplate]) -> Result<()> {
        Ok(())
    }

    async fn sync_repository_configs(&self, _repo: &RepositoryInfo, _request: &ConfigSyncRequest) -> Result<RepositoryConfigSyncResult> {
        Ok(RepositoryConfigSyncResult {
            repository: format!("{}/{}", _repo.owner, _repo.name),
            configs_updated: Vec::new(),
            conflicts_resolved: Vec::new(),
            pull_requests: Vec::new(),
        })
    }

    // Workflow sync methods (simplified)
    fn validate_workflow_templates(&self, _templates: &[WorkflowTemplate]) -> Result<()> {
        Ok(())
    }

    async fn sync_repository_workflows(&self, _repo: &RepositoryInfo, _request: &WorkflowSyncRequest) -> Result<RepositoryWorkflowSyncResult> {
        Ok(RepositoryWorkflowSyncResult {
            repository: format!("{}/{}", _repo.owner, _repo.name),
            workflows_updated: Vec::new(),
            validations: Vec::new(),
            pull_requests: Vec::new(),
        })
    }

    // Monitoring methods (simplified)
    async fn check_repository_sync_status(&self, _repo: &RepositoryInfo) -> Result<RepositorySyncStatus> {
        Ok(RepositorySyncStatus {
            repository: format!("{}/{}", _repo.owner, _repo.name),
            last_sync: None,
            sync_health: SyncHealth::Healthy,
            drift_detected: false,
            issues: Vec::new(),
        })
    }

    fn calculate_overall_sync_health(&self, _statuses: &[RepositorySyncStatus]) -> SyncHealth {
        SyncHealth::Healthy
    }

    fn detect_configuration_drift(&self, _statuses: &[RepositorySyncStatus]) -> Vec<ConfigurationDrift> {
        Vec::new()
    }

    fn generate_sync_recommendations(&self, _report: &SyncStatusReport) -> Vec<String> {
        vec!["Consider regular sync schedule".to_string()]
    }

    // Planning methods (simplified)
    async fn analyze_multi_repo_state(&self, _repositories: &[RepositoryInfo]) -> Result<MultiRepoState> {
        Ok(MultiRepoState {
            repositories: HashMap::new(),
            common_patterns: Vec::new(),
            inconsistencies: Vec::new(),
        })
    }

    fn identify_sync_differences(&self, _current: &MultiRepoState, _target: &TargetState) -> Vec<SyncDifference> {
        Vec::new()
    }

    fn calculate_sync_dependencies(&self, _differences: &[SyncDifference]) -> Vec<SyncDependency> {
        Vec::new()
    }

    fn generate_sync_steps(&self, _differences: &[SyncDifference], _dependencies: &[SyncDependency]) -> Vec<SyncStep> {
        Vec::new()
    }

    fn estimate_sync_effort(&self, _steps: &[SyncStep]) -> EffortEstimate {
        EffortEstimate {
            total_hours: 4.0,
            complexity: "Medium".to_string(),
            risk_level: "Low".to_string(),
        }
    }

    fn assess_sync_risks(&self, _steps: &[SyncStep], _current_state: &MultiRepoState) -> RiskAssessment {
        RiskAssessment {
            overall_risk: "Low".to_string(),
            risks: Vec::new(),
            mitigation_strategies: Vec::new(),
        }
    }

    fn calculate_execution_order(&self, _dependencies: &[SyncDependency]) -> Vec<SyncStep> {
        Vec::new()
    }

    // Execution methods (simplified)
    async fn execute_sync_step(&self, _step: &SyncStep, _config: &PlanExecutionConfig) -> Result<SyncStepResult> {
        Ok(SyncStepResult {
            step_name: _step.name.clone(),
            completed_at: Utc::now(),
            changes_made: Vec::new(),
        })
    }

    async fn rollback_sync_steps(&self, _completed_steps: &[SyncStepResult]) -> Result<()> {
        Ok(())
    }
}

// Dependency graph implementation
#[derive(Debug, Default)]
struct DependencyGraph {
    nodes: HashMap<String, RepositoryPackageInfo>,
    edges: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    fn new() -> Self {
        Self::default()
    }
    
    fn add_node(&mut self, key: String, package_info: RepositoryPackageInfo) {
        self.nodes.insert(key.clone(), package_info);
        self.edges.entry(key).or_insert_with(Vec::new);
    }
    
    fn add_edge(&mut self, from: &str, to: &str) {
        self.edges.entry(from.to_string()).or_insert_with(Vec::new).push(to.to_string());
    }
    
    fn topological_sort(&self) -> Result<Vec<String>> {
        // Simple topological sort implementation
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        
        for node in self.nodes.keys() {
            if !visited.contains(node) {
                self.dfs_visit(node, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        result.reverse();
        Ok(result)
    }
    
    fn dfs_visit(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(node) {
            return Err(anyhow!("Circular dependency detected"));
        }
        
        if visited.contains(node) {
            return Ok(());
        }
        
        temp_visited.insert(node.to_string());
        
        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                self.dfs_visit(neighbor, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());
        
        Ok(())
    }
}

// Data structures for sync coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub owner: String,
    pub name: String,
    pub branch: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSyncRequest {
    pub repositories: Vec<RepositoryInfo>,
    pub strategy: String,
    pub target_versions: HashMap<String, String>,
    pub create_pull_requests: bool,
    pub fail_fast: bool,
    pub dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSyncRequest {
    pub repositories: Vec<RepositoryInfo>,
    pub config_templates: Vec<ConfigTemplate>,
    pub create_pull_requests: bool,
    pub fail_fast: bool,
    pub merge_strategy: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowSyncRequest {
    pub repositories: Vec<RepositoryInfo>,
    pub workflow_templates: Vec<WorkflowTemplate>,
    pub create_pull_requests: bool,
    pub fail_fast: bool,
    pub validate_workflows: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigTemplate {
    pub name: String,
    pub path: String,
    pub content: String,
    pub merge_strategy: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub name: String,
    pub path: String,
    pub content: String,
    pub variables: HashMap<String, String>,
}

// Result structures
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSyncResult {
    pub sync_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub repositories_processed: u32,
    pub repositories_failed: u32,
    pub package_updates: Vec<PackageUpdate>,
    pub dependency_conflicts: Vec<DependencyConflict>,
    pub created_pull_requests: Vec<PullRequestInfo>,
    pub error_summary: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSyncResult {
    pub sync_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub configurations_synced: HashMap<String, Vec<String>>,
    pub conflicts_resolved: Vec<ConfigConflict>,
    pub created_pull_requests: Vec<PullRequestInfo>,
    pub error_summary: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowSyncResult {
    pub sync_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub workflows_synced: HashMap<String, Vec<String>>,
    pub workflow_validations: Vec<WorkflowValidation>,
    pub created_pull_requests: Vec<PullRequestInfo>,
    pub error_summary: Vec<String>,
}

// Supporting data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub package_name: String,
    pub from_version: String,
    pub to_version: String,
    pub update_type: UpdateType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyConflict {
    pub package_name: String,
    pub conflicting_versions: Vec<String>,
    pub affected_repositories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestInfo {
    pub number: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct RepositoryPackageInfo {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<PackageDependency>,
    pub dev_dependencies: Vec<PackageDependency>,
    pub package_manager: PackageManager,
}

#[derive(Debug, Clone)]
pub struct PackageDependency {
    pub name: String,
    pub version: String,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone)]
pub enum DependencyType {
    Direct,
    Dev,
    Peer,
    Optional,
}

#[derive(Debug, Clone)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Cargo,
    Go,
}

#[derive(Debug)]
pub struct MultiRepoPackageState {
    pub repositories: HashMap<String, RepositoryPackageInfo>,
    pub package_matrix: HashMap<String, Vec<PackageVersion>>,
    pub version_conflicts: Vec<VersionConflict>,
}

#[derive(Debug)]
pub struct PackageVersion {
    pub repository: String,
    pub version: String,
}

#[derive(Debug)]
pub struct VersionConflict {
    pub package_name: String,
    pub versions: Vec<PackageVersion>,
}

#[derive(Debug)]
pub enum SyncStrategy {
    UseLatest,
    UseSpecificVersions(HashMap<String, String>),
    Conservative,
    Aggressive,
}

#[derive(Debug)]
pub struct RepositoryPackageSyncResult {
    pub repository: String,
    pub package_updates: Vec<PackageUpdate>,
    pub pull_requests: Vec<PullRequestInfo>,
    pub conflicts: Vec<DependencyConflict>,
}

#[derive(Debug)]
pub struct RepositoryConfigSyncResult {
    pub repository: String,
    pub configs_updated: Vec<String>,
    pub conflicts_resolved: Vec<ConfigConflict>,
    pub pull_requests: Vec<PullRequestInfo>,
}

#[derive(Debug)]
pub struct RepositoryWorkflowSyncResult {
    pub repository: String,
    pub workflows_updated: Vec<String>,
    pub validations: Vec<WorkflowValidation>,
    pub pull_requests: Vec<PullRequestInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigConflict {
    pub file_path: String,
    pub conflict_type: String,
    pub resolution: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowValidation {
    pub workflow_name: String,
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

// Monitoring structures
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatusReport {
    pub monitored_at: DateTime<Utc>,
    pub repositories: Vec<RepositorySyncStatus>,
    pub overall_sync_health: SyncHealth,
    pub drift_detected: Vec<ConfigurationDrift>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositorySyncStatus {
    pub repository: String,
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_health: SyncHealth,
    pub drift_detected: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncHealth {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationDrift {
    pub repository: String,
    pub file_path: String,
    pub drift_type: String,
    pub detected_at: DateTime<Utc>,
}

// Planning structures
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPlanRequest {
    pub repositories: Vec<RepositoryInfo>,
    pub target_state: TargetState,
    pub include_dependencies: bool,
    pub risk_tolerance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetState {
    pub package_versions: HashMap<String, String>,
    pub configurations: HashMap<String, String>,
    pub workflows: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPlan {
    pub plan_id: String,
    pub created_at: DateTime<Utc>,
    pub repositories: Vec<RepositoryInfo>,
    pub target_state: TargetState,
    pub current_state: MultiRepoState,
    pub sync_steps: Vec<SyncStep>,
    pub dependencies: Vec<SyncDependency>,
    pub effort_estimate: EffortEstimate,
    pub risk_assessment: RiskAssessment,
    pub execution_order: Vec<SyncStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiRepoState {
    pub repositories: HashMap<String, RepositoryState>,
    pub common_patterns: Vec<String>,
    pub inconsistencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryState {
    pub packages: HashMap<String, String>,
    pub configurations: HashMap<String, String>,
    pub workflows: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDifference {
    pub difference_type: String,
    pub repository: String,
    pub current_value: String,
    pub target_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDependency {
    pub from_step: String,
    pub to_step: String,
    pub dependency_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStep {
    pub name: String,
    pub step_type: String,
    pub repository: String,
    pub description: String,
    pub estimated_duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub total_hours: f64,
    pub complexity: String,
    pub risk_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk: String,
    pub risks: Vec<IdentifiedRisk>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiedRisk {
    pub risk_type: String,
    pub probability: String,
    pub impact: String,
    pub description: String,
}

// Execution structures
#[derive(Debug, Serialize, Deserialize)]
pub struct PlanExecutionConfig {
    pub rollback_on_failure: bool,
    pub parallel_execution: bool,
    pub max_parallel: u32,
    pub timeout_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanExecutionResult {
    pub plan_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub steps_completed: Vec<SyncStepResult>,
    pub steps_failed: Vec<SyncStepFailure>,
    pub rollback_performed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStepResult {
    pub step_name: String,
    pub completed_at: DateTime<Utc>,
    pub changes_made: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStepFailure {
    pub step_name: String,
    pub error: String,
    pub failed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncOperation {
    pub sync_id: String,
    pub operation_type: SyncOperationType,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub repositories: Vec<String>,
    pub details: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncOperationType {
    PackageSync,
    ConfigSync,
    WorkflowSync,
    FullSync,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_info() {
        let repo = RepositoryInfo {
            owner: "test".to_string(),
            name: "repo".to_string(),
            branch: Some("main".to_string()),
            access_token: None,
        };
        
        assert_eq!(repo.owner, "test");
        assert_eq!(repo.name, "repo");
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        let package_info = RepositoryPackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            package_manager: PackageManager::Npm,
        };
        
        graph.add_node("test/repo".to_string(), package_info);
        assert!(graph.nodes.contains_key("test/repo"));
    }

    #[test]
    fn test_package_update() {
        let update = PackageUpdate {
            package_name: "lodash".to_string(),
            from_version: "4.17.20".to_string(),
            to_version: "4.17.21".to_string(),
            update_type: UpdateType::Patch,
        };
        
        assert_eq!(update.package_name, "lodash");
        matches!(update.update_type, UpdateType::Patch);
    }
}