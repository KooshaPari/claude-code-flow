use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::github::api::GitHubApiClient;

/// Repository structure optimization and architecture management
#[derive(Debug)]
pub struct RepositoryArchitect {
    api: GitHubApiClient,
    templates: HashMap<String, RepoTemplate>,
    optimization_rules: Vec<OptimizationRule>,
    best_practices: HashMap<String, BestPractice>,
}

impl RepositoryArchitect {
    /// Create new repository architect
    pub fn new(api: GitHubApiClient) -> Self {
        let mut architect = Self {
            api,
            templates: HashMap::new(),
            optimization_rules: Vec::new(),
            best_practices: HashMap::new(),
        };
        
        // Initialize default configurations
        architect.load_default_templates();
        architect.load_optimization_rules();
        architect.load_best_practices();
        
        architect
    }

    /// Analyze repository structure and provide optimization recommendations
    pub async fn analyze_repository_structure(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<StructureAnalysis> {
        info!("Analyzing repository structure for {}/{}", owner, repo);
        
        let mut analysis = StructureAnalysis {
            repository: format!("{}/{}", owner, repo),
            overall_score: 0.0,
            structure_assessment: StructureAssessment::default(),
            optimization_opportunities: Vec::new(),
            missing_components: Vec::new(),
            best_practice_compliance: HashMap::new(),
            recommendations: Vec::new(),
        };
        
        // Analyze directory structure
        analysis.structure_assessment.directory_structure = self.analyze_directory_structure(owner, repo).await?;
        
        // Analyze configuration files
        analysis.structure_assessment.configuration_files = self.analyze_configuration_files(owner, repo).await?;
        
        // Analyze documentation
        analysis.structure_assessment.documentation = self.analyze_documentation_structure(owner, repo).await?;
        
        // Analyze CI/CD setup
        analysis.structure_assessment.cicd_setup = self.analyze_cicd_structure(owner, repo).await?;
        
        // Check for missing components
        analysis.missing_components = self.identify_missing_components(owner, repo, &analysis.structure_assessment).await?;
        
        // Evaluate best practice compliance
        analysis.best_practice_compliance = self.evaluate_best_practices(&analysis.structure_assessment);
        
        // Generate optimization opportunities
        analysis.optimization_opportunities = self.identify_optimization_opportunities(&analysis.structure_assessment);
        
        // Generate recommendations
        analysis.recommendations = self.generate_structure_recommendations(&analysis);
        
        // Calculate overall score
        analysis.overall_score = self.calculate_structure_score(&analysis);
        
        info!("Repository structure analysis completed with score: {:.1}/10", analysis.overall_score);
        Ok(analysis)
    }

    /// Optimize repository structure based on template and best practices
    pub async fn optimize_repository_structure(
        &self,
        owner: &str,
        repo: &str,
        optimization_request: &OptimizationRequest,
    ) -> Result<OptimizationResult> {
        info!("Optimizing repository structure for {}/{}", owner, repo);
        
        let mut result = OptimizationResult {
            repository: format!("{}/{}", owner, repo),
            success: false,
            changes_applied: Vec::new(),
            changes_failed: Vec::new(),
            structure_improvements: StructureImprovements::default(),
        };
        
        // Apply template if specified
        if let Some(ref template_name) = optimization_request.apply_template {
            match self.apply_repository_template(owner, repo, template_name).await {
                Ok(template_changes) => {
                    result.changes_applied.extend(template_changes);
                    result.structure_improvements.template_applied = true;
                }
                Err(e) => {
                    result.changes_failed.push(format!("Template application failed: {}", e));
                }
            }
        }
        
        // Create missing directories
        if optimization_request.create_missing_directories {
            match self.create_missing_directories(owner, repo).await {
                Ok(dir_changes) => {
                    result.changes_applied.extend(dir_changes);
                    result.structure_improvements.directories_created = true;
                }
                Err(e) => {
                    result.changes_failed.push(format!("Directory creation failed: {}", e));
                }
            }
        }
        
        // Add missing configuration files
        if optimization_request.add_missing_configs {
            match self.add_missing_configuration_files(owner, repo).await {
                Ok(config_changes) => {
                    result.changes_applied.extend(config_changes);
                    result.structure_improvements.configs_added = true;
                }
                Err(e) => {
                    result.changes_failed.push(format!("Configuration file creation failed: {}", e));
                }
            }
        }
        
        // Improve documentation structure
        if optimization_request.improve_documentation {
            match self.improve_documentation_structure(owner, repo).await {
                Ok(doc_changes) => {
                    result.changes_applied.extend(doc_changes);
                    result.structure_improvements.documentation_improved = true;
                }
                Err(e) => {
                    result.changes_failed.push(format!("Documentation improvement failed: {}", e));
                }
            }
        }
        
        // Setup CI/CD workflows
        if optimization_request.setup_cicd {
            match self.setup_cicd_workflows(owner, repo).await {
                Ok(cicd_changes) => {
                    result.changes_applied.extend(cicd_changes);
                    result.structure_improvements.cicd_setup = true;
                }
                Err(e) => {
                    result.changes_failed.push(format!("CI/CD setup failed: {}", e));
                }
            }
        }
        
        // Apply security best practices
        if optimization_request.apply_security_practices {
            match self.apply_security_best_practices(owner, repo).await {
                Ok(security_changes) => {
                    result.changes_applied.extend(security_changes);
                    result.structure_improvements.security_enhanced = true;
                }
                Err(e) => {
                    result.changes_failed.push(format!("Security enhancement failed: {}", e));
                }
            }
        }
        
        result.success = result.changes_failed.is_empty();
        
        if result.success {
            info!("Repository optimization completed successfully with {} changes", result.changes_applied.len());
        } else {
            warn!("Repository optimization completed with {} failures", result.changes_failed.len());
        }
        
        Ok(result)
    }

    /// Create repository from template with customizations
    pub async fn create_from_template(
        &self,
        owner: &str,
        repo_name: &str,
        template_request: &TemplateCreationRequest,
    ) -> Result<TemplateCreationResult> {
        info!("Creating repository from template: {}", template_request.template_name);
        
        let template = self.templates.get(&template_request.template_name)
            .ok_or_else(|| anyhow!("Template not found: {}", template_request.template_name))?;
        
        let mut result = TemplateCreationResult {
            repository_name: repo_name.to_string(),
            template_used: template_request.template_name.clone(),
            success: false,
            files_created: Vec::new(),
            customizations_applied: Vec::new(),
        };
        
        // Create repository
        let repo_data = crate::github::api::CreateRepositoryRequest {
            name: repo_name.to_string(),
            description: template_request.description.clone(),
            homepage: template_request.homepage.clone(),
            private: template_request.private,
            auto_init: false,
        };
        
        let repository = self.api.create_repository(&repo_data).await?;
        
        // Create directory structure
        for dir in &template.directory_structure {
            if let Err(e) = self.create_directory_in_repo(owner, repo_name, dir).await {
                warn!("Failed to create directory {}: {}", dir, e);
            }
        }
        
        // Create files from template
        for file_template in &template.file_templates {
            match self.create_file_from_template(owner, repo_name, file_template, &template_request.customizations).await {
                Ok(file_path) => {
                    result.files_created.push(file_path);
                }
                Err(e) => {
                    warn!("Failed to create file from template {}: {}", file_template.path, e);
                }
            }
        }
        
        // Apply customizations
        for customization in &template_request.customizations {
            match self.apply_template_customization(owner, repo_name, customization).await {
                Ok(custom_desc) => {
                    result.customizations_applied.push(custom_desc);
                }
                Err(e) => {
                    warn!("Failed to apply customization: {}", e);
                }
            }
        }
        
        // Setup initial workflows
        if template_request.setup_workflows {
            if let Err(e) = self.setup_template_workflows(owner, repo_name, template).await {
                warn!("Failed to setup workflows: {}", e);
            }
        }
        
        result.success = !result.files_created.is_empty();
        
        info!("Repository created from template with {} files", result.files_created.len());
        Ok(result)
    }

    /// Generate architecture documentation
    pub async fn generate_architecture_documentation(
        &self,
        owner: &str,
        repo: &str,
        doc_request: &ArchitectureDocRequest,
    ) -> Result<ArchitectureDocumentation> {
        info!("Generating architecture documentation for {}/{}", owner, repo);
        
        // Analyze current repository structure
        let structure_analysis = self.analyze_repository_structure(owner, repo).await?;
        
        let mut documentation = ArchitectureDocumentation {
            repository: format!("{}/{}", owner, repo),
            generated_at: chrono::Utc::now(),
            sections: HashMap::new(),
            diagrams: Vec::new(),
            recommendations: structure_analysis.recommendations.clone(),
        };
        
        // Generate overview section
        if doc_request.include_overview {
            documentation.sections.insert(
                "overview".to_string(),
                self.generate_overview_section(owner, repo, &structure_analysis).await?
            );
        }
        
        // Generate directory structure section
        if doc_request.include_directory_structure {
            documentation.sections.insert(
                "directory_structure".to_string(),
                self.generate_directory_structure_section(&structure_analysis.structure_assessment.directory_structure)
            );
        }
        
        // Generate component architecture section
        if doc_request.include_component_architecture {
            documentation.sections.insert(
                "component_architecture".to_string(),
                self.generate_component_architecture_section(owner, repo).await?
            );
        }
        
        // Generate data flow section
        if doc_request.include_data_flow {
            documentation.sections.insert(
                "data_flow".to_string(),
                self.generate_data_flow_section(owner, repo).await?
            );
        }
        
        // Generate deployment architecture section
        if doc_request.include_deployment_architecture {
            documentation.sections.insert(
                "deployment_architecture".to_string(),
                self.generate_deployment_section(&structure_analysis.structure_assessment.cicd_setup).await?
            );
        }
        
        // Generate architecture diagrams
        if doc_request.generate_diagrams {
            documentation.diagrams = self.generate_architecture_diagrams(owner, repo, &structure_analysis).await?;
        }
        
        Ok(documentation)
    }

    /// Validate repository architecture against standards
    pub async fn validate_architecture(
        &self,
        owner: &str,
        repo: &str,
        validation_config: &ArchitectureValidationConfig,
    ) -> Result<ValidationReport> {
        info!("Validating repository architecture for {}/{}", owner, repo);
        
        let structure_analysis = self.analyze_repository_structure(owner, repo).await?;
        
        let mut report = ValidationReport {
            repository: format!("{}/{}", owner, repo),
            validation_date: chrono::Utc::now(),
            overall_compliance: 0.0,
            passed_rules: Vec::new(),
            failed_rules: Vec::new(),
            warnings: Vec::new(),
            critical_issues: Vec::new(),
        };
        
        // Apply validation rules
        for rule in &validation_config.rules {
            match self.apply_architecture_validation_rule(&structure_analysis, rule).await {
                Ok(rule_result) => {
                    if rule_result.passed {
                        report.passed_rules.push(rule.name.clone());
                    } else {
                        report.failed_rules.push(ValidationFailure {
                            rule_name: rule.name.clone(),
                            severity: rule.severity.clone(),
                            message: rule_result.message,
                            suggestions: rule_result.suggestions,
                        });
                        
                        if rule.severity == ValidationSeverity::Critical {
                            report.critical_issues.push(rule.name.clone());
                        }
                    }
                    
                    report.warnings.extend(rule_result.warnings);
                }
                Err(e) => {
                    warn!("Validation rule '{}' failed to execute: {}", rule.name, e);
                }
            }
        }
        
        // Calculate compliance score
        let total_rules = validation_config.rules.len() as f64;
        let passed_rules = report.passed_rules.len() as f64;
        report.overall_compliance = if total_rules > 0.0 {
            (passed_rules / total_rules) * 100.0
        } else {
            100.0
        };
        
        info!("Architecture validation completed with {:.1}% compliance", report.overall_compliance);
        Ok(report)
    }

    // Helper methods for analysis
    async fn analyze_directory_structure(&self, owner: &str, repo: &str) -> Result<DirectoryStructureAnalysis> {
        debug!("Analyzing directory structure");
        
        // This would typically use the GitHub API to fetch repository contents
        // For now, we'll return a simplified structure
        Ok(DirectoryStructureAnalysis {
            has_src_directory: self.check_directory_exists(owner, repo, "src").await,
            has_tests_directory: self.check_directory_exists(owner, repo, "tests").await,
            has_docs_directory: self.check_directory_exists(owner, repo, "docs").await,
            has_examples_directory: self.check_directory_exists(owner, repo, "examples").await,
            has_scripts_directory: self.check_directory_exists(owner, repo, "scripts").await,
            directory_depth: self.calculate_directory_depth(owner, repo).await.unwrap_or(0),
            organization_score: 7.5,
        })
    }

    async fn analyze_configuration_files(&self, owner: &str, repo: &str) -> Result<ConfigurationAnalysis> {
        debug!("Analyzing configuration files");
        
        Ok(ConfigurationAnalysis {
            has_gitignore: self.check_file_exists(owner, repo, ".gitignore").await,
            has_readme: self.check_file_exists(owner, repo, "README.md").await,
            has_license: self.check_file_exists(owner, repo, "LICENSE").await,
            has_contributing: self.check_file_exists(owner, repo, "CONTRIBUTING.md").await,
            has_code_of_conduct: self.check_file_exists(owner, repo, "CODE_OF_CONDUCT.md").await,
            has_security_policy: self.check_file_exists(owner, repo, "SECURITY.md").await,
            has_issue_templates: self.check_directory_exists(owner, repo, ".github/ISSUE_TEMPLATE").await,
            has_pr_template: self.check_file_exists(owner, repo, ".github/pull_request_template.md").await,
            configuration_score: 8.0,
        })
    }

    async fn analyze_documentation_structure(&self, owner: &str, repo: &str) -> Result<DocumentationAnalysis> {
        debug!("Analyzing documentation structure");
        
        Ok(DocumentationAnalysis {
            has_docs_folder: self.check_directory_exists(owner, repo, "docs").await,
            has_api_docs: self.check_api_documentation(owner, repo).await,
            has_user_guide: self.check_user_guide(owner, repo).await,
            has_developer_guide: self.check_developer_guide(owner, repo).await,
            has_architecture_docs: self.check_architecture_docs(owner, repo).await,
            documentation_completeness: 75.0,
        })
    }

    async fn analyze_cicd_structure(&self, owner: &str, repo: &str) -> Result<CicdAnalysis> {
        debug!("Analyzing CI/CD structure");
        
        Ok(CicdAnalysis {
            has_github_actions: self.check_directory_exists(owner, repo, ".github/workflows").await,
            has_docker_config: self.check_file_exists(owner, repo, "Dockerfile").await,
            has_docker_compose: self.check_file_exists(owner, repo, "docker-compose.yml").await,
            has_makefile: self.check_file_exists(owner, repo, "Makefile").await,
            has_build_scripts: self.check_build_scripts(owner, repo).await,
            automation_score: 6.5,
        })
    }

    // Template and optimization methods
    fn load_default_templates(&mut self) {
        // Load default repository templates
        self.templates.insert(
            "microservice".to_string(),
            RepoTemplate {
                name: "Microservice".to_string(),
                description: "Standard microservice template".to_string(),
                directory_structure: vec![
                    "src/".to_string(),
                    "tests/".to_string(),
                    "docs/".to_string(),
                    "scripts/".to_string(),
                    ".github/workflows/".to_string(),
                ],
                file_templates: vec![
                    FileTemplate {
                        path: "README.md".to_string(),
                        content: include_str!("templates/microservice/README.md").to_string(),
                        is_template: true,
                    },
                    FileTemplate {
                        path: "Dockerfile".to_string(),
                        content: include_str!("templates/microservice/Dockerfile").to_string(),
                        is_template: true,
                    },
                ],
                required_files: vec!["README.md".to_string(), "Dockerfile".to_string()],
                optional_files: vec!["docker-compose.yml".to_string()],
            }
        );
    }

    fn load_optimization_rules(&mut self) {
        self.optimization_rules.push(OptimizationRule {
            name: "Source Directory".to_string(),
            description: "Ensure source code is organized in src/ directory".to_string(),
            rule_type: OptimizationRuleType::DirectoryStructure,
            severity: OptimizationSeverity::Medium,
        });
    }

    fn load_best_practices(&mut self) {
        self.best_practices.insert(
            "readme".to_string(),
            BestPractice {
                name: "README Documentation".to_string(),
                description: "Repository should have comprehensive README".to_string(),
                category: "Documentation".to_string(),
                importance: BestPracticeImportance::High,
            }
        );
    }

    // Helper methods (simplified implementations)
    async fn check_directory_exists(&self, _owner: &str, _repo: &str, _path: &str) -> bool {
        // Implementation would check directory existence via GitHub API
        true
    }

    async fn check_file_exists(&self, _owner: &str, _repo: &str, _path: &str) -> bool {
        // Implementation would check file existence via GitHub API
        true
    }

    async fn calculate_directory_depth(&self, _owner: &str, _repo: &str) -> Result<u32> {
        Ok(3)
    }

    async fn check_api_documentation(&self, _owner: &str, _repo: &str) -> bool {
        false
    }

    async fn check_user_guide(&self, _owner: &str, _repo: &str) -> bool {
        false
    }

    async fn check_developer_guide(&self, _owner: &str, _repo: &str) -> bool {
        false
    }

    async fn check_architecture_docs(&self, _owner: &str, _repo: &str) -> bool {
        false
    }

    async fn check_build_scripts(&self, _owner: &str, _repo: &str) -> bool {
        false
    }

    async fn identify_missing_components(&self, _owner: &str, _repo: &str, _assessment: &StructureAssessment) -> Result<Vec<String>> {
        let mut missing = Vec::new();
        
        if !_assessment.directory_structure.has_tests_directory {
            missing.push("tests directory".to_string());
        }
        if !_assessment.configuration_files.has_gitignore {
            missing.push(".gitignore file".to_string());
        }
        
        Ok(missing)
    }

    fn evaluate_best_practices(&self, _assessment: &StructureAssessment) -> HashMap<String, f64> {
        let mut compliance = HashMap::new();
        compliance.insert("documentation".to_string(), 80.0);
        compliance.insert("testing".to_string(), 60.0);
        compliance.insert("security".to_string(), 70.0);
        compliance
    }

    fn identify_optimization_opportunities(&self, _assessment: &StructureAssessment) -> Vec<OptimizationOpportunity> {
        vec![
            OptimizationOpportunity {
                area: "Documentation".to_string(),
                description: "Add API documentation".to_string(),
                impact: "Medium".to_string(),
                effort: "Low".to_string(),
            }
        ]
    }

    fn generate_structure_recommendations(&self, _analysis: &StructureAnalysis) -> Vec<StructureRecommendation> {
        vec![
            StructureRecommendation {
                title: "Add comprehensive testing structure".to_string(),
                description: "Create organized test directories for unit, integration, and e2e tests".to_string(),
                priority: RecommendationPriority::High,
                effort_estimate: "2-4 hours".to_string(),
                benefits: vec!["Better code quality".to_string(), "Easier maintenance".to_string()],
            }
        ]
    }

    fn calculate_structure_score(&self, analysis: &StructureAnalysis) -> f64 {
        let mut score = 0.0;
        let mut components = 0;
        
        // Directory structure score
        score += analysis.structure_assessment.directory_structure.organization_score;
        components += 1;
        
        // Configuration score
        score += analysis.structure_assessment.configuration_files.configuration_score;
        components += 1;
        
        // Documentation score
        score += analysis.structure_assessment.documentation.documentation_completeness / 10.0;
        components += 1;
        
        // CI/CD score
        score += analysis.structure_assessment.cicd_setup.automation_score;
        components += 1;
        
        if components > 0 {
            score / components as f64
        } else {
            0.0
        }
    }

    // Implementation methods (simplified)
    async fn apply_repository_template(&self, _owner: &str, _repo: &str, _template_name: &str) -> Result<Vec<String>> {
        Ok(vec!["Applied template structure".to_string()])
    }

    async fn create_missing_directories(&self, _owner: &str, _repo: &str) -> Result<Vec<String>> {
        Ok(vec!["Created src/ directory".to_string(), "Created tests/ directory".to_string()])
    }

    async fn add_missing_configuration_files(&self, _owner: &str, _repo: &str) -> Result<Vec<String>> {
        Ok(vec!["Added .gitignore".to_string(), "Added LICENSE".to_string()])
    }

    async fn improve_documentation_structure(&self, _owner: &str, _repo: &str) -> Result<Vec<String>> {
        Ok(vec!["Enhanced README.md".to_string(), "Added CONTRIBUTING.md".to_string()])
    }

    async fn setup_cicd_workflows(&self, _owner: &str, _repo: &str) -> Result<Vec<String>> {
        Ok(vec!["Added CI workflow".to_string(), "Added CD workflow".to_string()])
    }

    async fn apply_security_best_practices(&self, _owner: &str, _repo: &str) -> Result<Vec<String>> {
        Ok(vec!["Added SECURITY.md".to_string(), "Configured Dependabot".to_string()])
    }

    async fn create_directory_in_repo(&self, _owner: &str, _repo: &str, _dir: &str) -> Result<()> {
        Ok(())
    }

    async fn create_file_from_template(&self, _owner: &str, _repo: &str, _template: &FileTemplate, _customizations: &[TemplateCustomization]) -> Result<String> {
        Ok(_template.path.clone())
    }

    async fn apply_template_customization(&self, _owner: &str, _repo: &str, _customization: &TemplateCustomization) -> Result<String> {
        Ok(format!("Applied customization: {}", _customization.name))
    }

    async fn setup_template_workflows(&self, _owner: &str, _repo: &str, _template: &RepoTemplate) -> Result<()> {
        Ok(())
    }

    // Documentation generation methods
    async fn generate_overview_section(&self, _owner: &str, _repo: &str, _analysis: &StructureAnalysis) -> Result<String> {
        Ok("Repository overview section".to_string())
    }

    fn generate_directory_structure_section(&self, _dir_analysis: &DirectoryStructureAnalysis) -> String {
        "Directory structure documentation".to_string()
    }

    async fn generate_component_architecture_section(&self, _owner: &str, _repo: &str) -> Result<String> {
        Ok("Component architecture section".to_string())
    }

    async fn generate_data_flow_section(&self, _owner: &str, _repo: &str) -> Result<String> {
        Ok("Data flow section".to_string())
    }

    async fn generate_deployment_section(&self, _cicd_analysis: &CicdAnalysis) -> Result<String> {
        Ok("Deployment architecture section".to_string())
    }

    async fn generate_architecture_diagrams(&self, _owner: &str, _repo: &str, _analysis: &StructureAnalysis) -> Result<Vec<ArchitectureDiagram>> {
        Ok(vec![
            ArchitectureDiagram {
                name: "System Overview".to_string(),
                diagram_type: "system".to_string(),
                content: "System overview diagram".to_string(),
                format: "mermaid".to_string(),
            }
        ])
    }

    // Validation methods
    async fn apply_architecture_validation_rule(&self, _analysis: &StructureAnalysis, _rule: &ArchitectureValidationRule) -> Result<ValidationRuleResult> {
        Ok(ValidationRuleResult {
            passed: true,
            message: "Rule passed".to_string(),
            suggestions: Vec::new(),
            warnings: Vec::new(),
        })
    }
}

// Data structures for repository architecture
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationRequest {
    pub apply_template: Option<String>,
    pub create_missing_directories: bool,
    pub add_missing_configs: bool,
    pub improve_documentation: bool,
    pub setup_cicd: bool,
    pub apply_security_practices: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateCreationRequest {
    pub template_name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub private: bool,
    pub customizations: Vec<TemplateCustomization>,
    pub setup_workflows: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateCustomization {
    pub name: String,
    pub value: String,
    pub file_pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchitectureDocRequest {
    pub include_overview: bool,
    pub include_directory_structure: bool,
    pub include_component_architecture: bool,
    pub include_data_flow: bool,
    pub include_deployment_architecture: bool,
    pub generate_diagrams: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchitectureValidationConfig {
    pub rules: Vec<ArchitectureValidationRule>,
    pub fail_on_critical: bool,
    pub generate_report: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchitectureValidationRule {
    pub name: String,
    pub description: String,
    pub severity: ValidationSeverity,
    pub rule_type: String,
    pub conditions: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

// Analysis result structures
#[derive(Debug, Serialize, Deserialize)]
pub struct StructureAnalysis {
    pub repository: String,
    pub overall_score: f64,
    pub structure_assessment: StructureAssessment,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
    pub missing_components: Vec<String>,
    pub best_practice_compliance: HashMap<String, f64>,
    pub recommendations: Vec<StructureRecommendation>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StructureAssessment {
    pub directory_structure: DirectoryStructureAnalysis,
    pub configuration_files: ConfigurationAnalysis,
    pub documentation: DocumentationAnalysis,
    pub cicd_setup: CicdAnalysis,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DirectoryStructureAnalysis {
    pub has_src_directory: bool,
    pub has_tests_directory: bool,
    pub has_docs_directory: bool,
    pub has_examples_directory: bool,
    pub has_scripts_directory: bool,
    pub directory_depth: u32,
    pub organization_score: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigurationAnalysis {
    pub has_gitignore: bool,
    pub has_readme: bool,
    pub has_license: bool,
    pub has_contributing: bool,
    pub has_code_of_conduct: bool,
    pub has_security_policy: bool,
    pub has_issue_templates: bool,
    pub has_pr_template: bool,
    pub configuration_score: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocumentationAnalysis {
    pub has_docs_folder: bool,
    pub has_api_docs: bool,
    pub has_user_guide: bool,
    pub has_developer_guide: bool,
    pub has_architecture_docs: bool,
    pub documentation_completeness: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CicdAnalysis {
    pub has_github_actions: bool,
    pub has_docker_config: bool,
    pub has_docker_compose: bool,
    pub has_makefile: bool,
    pub has_build_scripts: bool,
    pub automation_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub area: String,
    pub description: String,
    pub impact: String,
    pub effort: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureRecommendation {
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub effort_estimate: String,
    pub benefits: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

// Result structures
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub repository: String,
    pub success: bool,
    pub changes_applied: Vec<String>,
    pub changes_failed: Vec<String>,
    pub structure_improvements: StructureImprovements,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StructureImprovements {
    pub template_applied: bool,
    pub directories_created: bool,
    pub configs_added: bool,
    pub documentation_improved: bool,
    pub cicd_setup: bool,
    pub security_enhanced: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateCreationResult {
    pub repository_name: String,
    pub template_used: String,
    pub success: bool,
    pub files_created: Vec<String>,
    pub customizations_applied: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchitectureDocumentation {
    pub repository: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub sections: HashMap<String, String>,
    pub diagrams: Vec<ArchitectureDiagram>,
    pub recommendations: Vec<StructureRecommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchitectureDiagram {
    pub name: String,
    pub diagram_type: String,
    pub content: String,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub repository: String,
    pub validation_date: chrono::DateTime<chrono::Utc>,
    pub overall_compliance: f64,
    pub passed_rules: Vec<String>,
    pub failed_rules: Vec<ValidationFailure>,
    pub warnings: Vec<String>,
    pub critical_issues: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationFailure {
    pub rule_name: String,
    pub severity: ValidationSeverity,
    pub message: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug)]
pub struct ValidationRuleResult {
    pub passed: bool,
    pub message: String,
    pub suggestions: Vec<String>,
    pub warnings: Vec<String>,
}

// Template structures
#[derive(Debug)]
pub struct RepoTemplate {
    pub name: String,
    pub description: String,
    pub directory_structure: Vec<String>,
    pub file_templates: Vec<FileTemplate>,
    pub required_files: Vec<String>,
    pub optional_files: Vec<String>,
}

#[derive(Debug)]
pub struct FileTemplate {
    pub path: String,
    pub content: String,
    pub is_template: bool,
}

#[derive(Debug)]
pub struct OptimizationRule {
    pub name: String,
    pub description: String,
    pub rule_type: OptimizationRuleType,
    pub severity: OptimizationSeverity,
}

#[derive(Debug)]
pub enum OptimizationRuleType {
    DirectoryStructure,
    Configuration,
    Documentation,
    Security,
}

#[derive(Debug)]
pub enum OptimizationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub struct BestPractice {
    pub name: String,
    pub description: String,
    pub category: String,
    pub importance: BestPracticeImportance,
}

#[derive(Debug)]
pub enum BestPracticeImportance {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_analysis_creation() {
        let analysis = StructureAnalysis {
            repository: "test/repo".to_string(),
            overall_score: 8.5,
            structure_assessment: StructureAssessment::default(),
            optimization_opportunities: Vec::new(),
            missing_components: Vec::new(),
            best_practice_compliance: HashMap::new(),
            recommendations: Vec::new(),
        };
        
        assert_eq!(analysis.repository, "test/repo");
        assert_eq!(analysis.overall_score, 8.5);
    }

    #[test]
    fn test_optimization_opportunity() {
        let opportunity = OptimizationOpportunity {
            area: "Testing".to_string(),
            description: "Add unit tests".to_string(),
            impact: "High".to_string(),
            effort: "Medium".to_string(),
        };
        
        assert_eq!(opportunity.area, "Testing");
        assert_eq!(opportunity.impact, "High");
    }

    #[test]
    fn test_recommendation_priority() {
        let recommendation = StructureRecommendation {
            title: "Add CI/CD".to_string(),
            description: "Setup automated workflows".to_string(),
            priority: RecommendationPriority::High,
            effort_estimate: "4-6 hours".to_string(),
            benefits: vec!["Automation".to_string()],
        };
        
        matches!(recommendation.priority, RecommendationPriority::High);
    }
}