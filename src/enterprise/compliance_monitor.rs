//! Compliance Monitoring System
//! 
//! Monitors and ensures compliance with enterprise standards

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use super::{EnterpriseConfig, ComplianceStandard, ComplianceStatus, AuditEntry, ComplianceImpact, RiskLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMonitor {
    config: EnterpriseConfig,
    compliance_rules: RwLock<HashMap<ComplianceStandard, ComplianceRuleSet>>,
    audit_log: RwLock<Vec<AuditEntry>>,
    compliance_status: RwLock<ComplianceStatus>,
    active_monitors: RwLock<HashMap<Uuid, ComplianceCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRuleSet {
    pub standard: ComplianceStandard,
    pub rules: Vec<ComplianceRule>,
    pub severity: ComplianceSeverity,
    pub monitoring_frequency: MonitoringFrequency,
    pub auto_remediation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
    pub validation_logic: ValidationLogic,
    pub remediation_actions: Vec<RemediationAction>,
    pub exceptions: Vec<ComplianceException>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    DataProtection,
    AccessControl,
    AuditTrail,
    Encryption,
    DataRetention,
    IncidentResponse,
    RiskAssessment,
    TrainingCompliance,
    SystemSecurity,
    ChangeManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationLogic {
    pub condition: String,
    pub parameters: HashMap<String, String>,
    pub threshold: Option<f64>,
    pub automated_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    pub action_type: ActionType,
    pub description: String,
    pub automated: bool,
    pub required_approval: Option<ApprovalLevel>,
    pub estimated_time_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Alert,
    AutoCorrect,
    Quarantine,
    Block,
    Escalate,
    Document,
    Train,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalLevel {
    Team,
    Manager,
    ComplianceOfficer,
    Executive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceException {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub justification: String,
    pub approved_by: Uuid,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub id: Uuid,
    pub task_id: Uuid,
    pub standard: ComplianceStandard,
    pub check_type: CheckType,
    pub status: CheckStatus,
    pub findings: Vec<ComplianceFinding>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckType {
    Preventive,
    Detective,
    Corrective,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Pending,
    Running,
    Passed,
    Failed,
    Warning,
    Remediated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFinding {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub severity: ComplianceSeverity,
    pub description: String,
    pub evidence: Vec<Evidence>,
    pub impact_assessment: ImpactAssessment,
    pub remediation_plan: Option<RemediationPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub data: EvidenceData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Log,
    Screenshot,
    Configuration,
    Database,
    Network,
    File,
    UserAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceData {
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Reference(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub business_impact: BusinessImpact,
    pub technical_impact: TechnicalImpact,
    pub regulatory_impact: RegulatoryImpact,
    pub risk_score: f64, // 0.0 to 10.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnicalImpact {
    None,
    Performance,
    Availability,
    Security,
    DataIntegrity,
    SystemFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegulatoryImpact {
    None,
    Warning,
    MinorViolation,
    MajorViolation,
    Breach,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationPlan {
    pub id: Uuid,
    pub steps: Vec<RemediationStep>,
    pub estimated_completion: chrono::DateTime<chrono::Utc>,
    pub assigned_to: Vec<Uuid>,
    pub priority: RemediationPriority,
    pub dependencies: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationStep {
    pub step_number: u32,
    pub description: String,
    pub action_type: ActionType,
    pub estimated_time_minutes: u32,
    pub required_skills: Vec<String>,
    pub validation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationPriority {
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

impl ComplianceMonitor {
    pub fn new(config: &EnterpriseConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            compliance_rules: RwLock::new(HashMap::new()),
            audit_log: RwLock::new(Vec::new()),
            compliance_status: RwLock::new(ComplianceStatus {
                standards: HashMap::new(),
                audit_trail: Vec::new(),
                risk_level: RiskLevel::Medium,
            }),
            active_monitors: RwLock::new(HashMap::new()),
        })
    }

    pub async fn enable_auditing(&self) -> Result<()> {
        self.initialize_compliance_rules().await?;
        self.start_monitoring_loops().await?;
        self.setup_real_time_checks().await?;
        Ok(())
    }

    async fn initialize_compliance_rules(&self) -> Result<()> {
        let mut rules = self.compliance_rules.write().await;
        
        // SOC 2 Compliance Rules
        if self.config.compliance_requirements.contains(&ComplianceStandard::SOC2) {
            let soc2_rules = self.create_soc2_rules().await?;
            rules.insert(ComplianceStandard::SOC2, soc2_rules);
        }

        // GDPR Compliance Rules
        if self.config.compliance_requirements.contains(&ComplianceStandard::GDPR) {
            let gdpr_rules = self.create_gdpr_rules().await?;
            rules.insert(ComplianceStandard::GDPR, gdpr_rules);
        }

        // HIPAA Compliance Rules
        if self.config.compliance_requirements.contains(&ComplianceStandard::HIPAA) {
            let hipaa_rules = self.create_hipaa_rules().await?;
            rules.insert(ComplianceStandard::HIPAA, hipaa_rules);
        }

        // ISO 27001 Compliance Rules
        if self.config.compliance_requirements.contains(&ComplianceStandard::ISO27001) {
            let iso_rules = self.create_iso27001_rules().await?;
            rules.insert(ComplianceStandard::ISO27001, iso_rules);
        }

        // FedRAMP Compliance Rules
        if self.config.compliance_requirements.contains(&ComplianceStandard::FedRAMP) {
            let fedramp_rules = self.create_fedramp_rules().await?;
            rules.insert(ComplianceStandard::FedRAMP, fedramp_rules);
        }

        Ok(())
    }

    async fn create_soc2_rules(&self) -> Result<ComplianceRuleSet> {
        Ok(ComplianceRuleSet {
            standard: ComplianceStandard::SOC2,
            rules: vec![
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "Access Control".to_string(),
                    description: "Ensure proper access controls are in place".to_string(),
                    rule_type: RuleType::AccessControl,
                    validation_logic: ValidationLogic {
                        condition: "user_access_reviewed_quarterly".to_string(),
                        parameters: HashMap::new(),
                        threshold: None,
                        automated_check: true,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::Review,
                            description: "Review user access permissions".to_string(),
                            automated: false,
                            required_approval: Some(ApprovalLevel::Manager),
                            estimated_time_minutes: 120,
                        }
                    ],
                    exceptions: vec![],
                },
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "Data Encryption".to_string(),
                    description: "Ensure data is encrypted at rest and in transit".to_string(),
                    rule_type: RuleType::Encryption,
                    validation_logic: ValidationLogic {
                        condition: "data_encrypted".to_string(),
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("algorithm".to_string(), "AES-256".to_string());
                            params
                        },
                        threshold: None,
                        automated_check: true,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::AutoCorrect,
                            description: "Enable encryption for unencrypted data".to_string(),
                            automated: true,
                            required_approval: None,
                            estimated_time_minutes: 30,
                        }
                    ],
                    exceptions: vec![],
                },
            ],
            severity: ComplianceSeverity::High,
            monitoring_frequency: MonitoringFrequency::Daily,
            auto_remediation: true,
        })
    }

    async fn create_gdpr_rules(&self) -> Result<ComplianceRuleSet> {
        Ok(ComplianceRuleSet {
            standard: ComplianceStandard::GDPR,
            rules: vec![
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "Data Protection by Design".to_string(),
                    description: "Implement data protection by design and by default".to_string(),
                    rule_type: RuleType::DataProtection,
                    validation_logic: ValidationLogic {
                        condition: "privacy_impact_assessment_completed".to_string(),
                        parameters: HashMap::new(),
                        threshold: None,
                        automated_check: false,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::Document,
                            description: "Complete privacy impact assessment".to_string(),
                            automated: false,
                            required_approval: Some(ApprovalLevel::ComplianceOfficer),
                            estimated_time_minutes: 240,
                        }
                    ],
                    exceptions: vec![],
                },
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "Data Retention".to_string(),
                    description: "Ensure data is not retained longer than necessary".to_string(),
                    rule_type: RuleType::DataRetention,
                    validation_logic: ValidationLogic {
                        condition: "data_retention_policy_enforced".to_string(),
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("max_retention_days".to_string(), "2555".to_string()); // 7 years
                            params
                        },
                        threshold: Some(2555.0),
                        automated_check: true,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::AutoCorrect,
                            description: "Delete data exceeding retention period".to_string(),
                            automated: true,
                            required_approval: Some(ApprovalLevel::ComplianceOfficer),
                            estimated_time_minutes: 60,
                        }
                    ],
                    exceptions: vec![],
                },
            ],
            severity: ComplianceSeverity::Critical,
            monitoring_frequency: MonitoringFrequency::Daily,
            auto_remediation: false,
        })
    }

    async fn create_hipaa_rules(&self) -> Result<ComplianceRuleSet> {
        Ok(ComplianceRuleSet {
            standard: ComplianceStandard::HIPAA,
            rules: vec![
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "PHI Access Control".to_string(),
                    description: "Control access to Protected Health Information".to_string(),
                    rule_type: RuleType::AccessControl,
                    validation_logic: ValidationLogic {
                        condition: "phi_access_controlled".to_string(),
                        parameters: HashMap::new(),
                        threshold: None,
                        automated_check: true,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::Block,
                            description: "Block unauthorized PHI access".to_string(),
                            automated: true,
                            required_approval: None,
                            estimated_time_minutes: 5,
                        }
                    ],
                    exceptions: vec![],
                },
            ],
            severity: ComplianceSeverity::Critical,
            monitoring_frequency: MonitoringFrequency::RealTime,
            auto_remediation: true,
        })
    }

    async fn create_iso27001_rules(&self) -> Result<ComplianceRuleSet> {
        Ok(ComplianceRuleSet {
            standard: ComplianceStandard::ISO27001,
            rules: vec![
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "Information Security Management".to_string(),
                    description: "Implement comprehensive information security management".to_string(),
                    rule_type: RuleType::SystemSecurity,
                    validation_logic: ValidationLogic {
                        condition: "security_controls_implemented".to_string(),
                        parameters: HashMap::new(),
                        threshold: Some(95.0), // 95% of controls implemented
                        automated_check: true,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::Review,
                            description: "Review and implement missing security controls".to_string(),
                            automated: false,
                            required_approval: Some(ApprovalLevel::Manager),
                            estimated_time_minutes: 480, // 8 hours
                        }
                    ],
                    exceptions: vec![],
                },
            ],
            severity: ComplianceSeverity::High,
            monitoring_frequency: MonitoringFrequency::Weekly,
            auto_remediation: false,
        })
    }

    async fn create_fedramp_rules(&self) -> Result<ComplianceRuleSet> {
        Ok(ComplianceRuleSet {
            standard: ComplianceStandard::FedRAMP,
            rules: vec![
                ComplianceRule {
                    id: Uuid::new_v4(),
                    name: "Continuous Monitoring".to_string(),
                    description: "Implement continuous monitoring of security controls".to_string(),
                    rule_type: RuleType::SystemSecurity,
                    validation_logic: ValidationLogic {
                        condition: "continuous_monitoring_active".to_string(),
                        parameters: HashMap::new(),
                        threshold: Some(99.9), // 99.9% uptime
                        automated_check: true,
                    },
                    remediation_actions: vec![
                        RemediationAction {
                            action_type: ActionType::Alert,
                            description: "Alert on monitoring system failure".to_string(),
                            automated: true,
                            required_approval: None,
                            estimated_time_minutes: 1,
                        }
                    ],
                    exceptions: vec![],
                },
            ],
            severity: ComplianceSeverity::Critical,
            monitoring_frequency: MonitoringFrequency::RealTime,
            auto_remediation: true,
        })
    }

    async fn start_monitoring_loops(&self) -> Result<()> {
        let rules = self.compliance_rules.clone();
        let audit_log = self.audit_log.clone();
        let compliance_status = self.compliance_status.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                let rules_guard = rules.read().await;
                for (standard, rule_set) in rules_guard.iter() {
                    for rule in &rule_set.rules {
                        if rule.validation_logic.automated_check {
                            if let Err(e) = Self::check_compliance_rule(
                                rule, 
                                &audit_log, 
                                &compliance_status
                            ).await {
                                tracing::error!("Failed to check compliance rule {}: {}", rule.name, e);
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn setup_real_time_checks(&self) -> Result<()> {
        // Setup real-time compliance monitoring for critical rules
        Ok(())
    }

    async fn check_compliance_rule(
        rule: &ComplianceRule,
        audit_log: &RwLock<Vec<AuditEntry>>,
        compliance_status: &RwLock<ComplianceStatus>
    ) -> Result<()> {
        // Simulate compliance check
        let is_compliant = rand::random::<f64>() > 0.1; // 90% compliance rate
        
        let audit_entry = AuditEntry {
            timestamp: chrono::Utc::now(),
            action: format!("Compliance check: {}", rule.name),
            user_id: Uuid::new_v4(), // System user
            team_id: None,
            compliance_impact: if is_compliant {
                ComplianceImpact::None
            } else {
                ComplianceImpact::Medium
            },
        };
        
        let mut audit_guard = audit_log.write().await;
        audit_guard.push(audit_entry);
        
        if !is_compliant {
            tracing::warn!("Compliance violation detected for rule: {}", rule.name);
            
            // Trigger remediation if auto-remediation is enabled
            for action in &rule.remediation_actions {
                if action.automated {
                    tracing::info!("Auto-remediating compliance violation: {}", action.description);
                }
            }
        }
        
        Ok(())
    }

    pub async fn monitor_task_compliance(&self, task_id: &Uuid) -> Result<()> {
        let check_id = Uuid::new_v4();
        let compliance_check = ComplianceCheck {
            id: check_id,
            task_id: *task_id,
            standard: ComplianceStandard::SOC2, // Default to SOC2
            check_type: CheckType::Continuous,
            status: CheckStatus::Running,
            findings: vec![],
            started_at: chrono::Utc::now(),
            completed_at: None,
        };
        
        let mut monitors = self.active_monitors.write().await;
        monitors.insert(check_id, compliance_check);
        
        // Start background monitoring for this task
        let task_id = *task_id;
        let monitors_clone = self.active_monitors.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            
            for _ in 0..10 { // Monitor for 10 minutes
                interval.tick().await;
                
                // Perform compliance checks for the task
                let mut monitors_guard = monitors_clone.write().await;
                if let Some(check) = monitors_guard.get_mut(&check_id) {
                    // Simulate compliance monitoring
                    let compliance_score = rand::random::<f64>();
                    
                    if compliance_score < 0.8 { // 80% threshold
                        let finding = ComplianceFinding {
                            id: Uuid::new_v4(),
                            rule_id: Uuid::new_v4(),
                            severity: ComplianceSeverity::Medium,
                            description: format!("Task {} compliance score below threshold", task_id),
                            evidence: vec![
                                Evidence {
                                    evidence_type: EvidenceType::Log,
                                    timestamp: chrono::Utc::now(),
                                    source: "compliance_monitor".to_string(),
                                    data: EvidenceData::Text(format!("Compliance score: {:.2}", compliance_score)),
                                }
                            ],
                            impact_assessment: ImpactAssessment {
                                business_impact: BusinessImpact::Low,
                                technical_impact: TechnicalImpact::Performance,
                                regulatory_impact: RegulatoryImpact::MinorViolation,
                                risk_score: (1.0 - compliance_score) * 10.0,
                            },
                            remediation_plan: None,
                        };
                        
                        check.findings.push(finding);
                        check.status = CheckStatus::Warning;
                        
                        tracing::warn!("Compliance issue detected for task {}", task_id);
                    }
                }
            }
            
            // Complete the check
            let mut monitors_guard = monitors_clone.write().await;
            if let Some(check) = monitors_guard.get_mut(&check_id) {
                check.completed_at = Some(chrono::Utc::now());
                if check.status == CheckStatus::Running {
                    check.status = CheckStatus::Passed;
                }
            }
        });
        
        Ok(())
    }

    pub async fn get_status(&self) -> Result<ComplianceStatus> {
        let status = self.compliance_status.read().await;
        let audit_log = self.audit_log.read().await;
        
        let mut updated_status = status.clone();
        
        // Update compliance status based on recent audit entries
        for standard in &self.config.compliance_requirements {
            let recent_violations = audit_log.iter()
                .filter(|entry| {
                    matches!(entry.compliance_impact, ComplianceImpact::Medium | ComplianceImpact::High) &&
                    entry.timestamp > chrono::Utc::now() - chrono::Duration::hours(24)
                })
                .count();
            
            let is_compliant = recent_violations == 0;
            updated_status.standards.insert(standard.clone(), is_compliant);
        }
        
        // Update risk level based on violations
        updated_status.risk_level = if updated_status.standards.values().all(|&compliant| compliant) {
            RiskLevel::Low
        } else if updated_status.standards.values().filter(|&&compliant| !compliant).count() > 2 {
            RiskLevel::High
        } else {
            RiskLevel::Medium
        };
        
        // Update audit trail with recent entries
        updated_status.audit_trail = audit_log.iter()
            .rev()
            .take(100)
            .cloned()
            .collect();
        
        Ok(updated_status)
    }

    pub async fn generate_compliance_report(&self) -> Result<ComplianceReport> {
        let status = self.get_status().await?;
        let active_monitors = self.active_monitors.read().await;
        
        let compliance_score = status.standards.values()
            .map(|&compliant| if compliant { 1.0 } else { 0.0 })
            .sum::<f64>() / status.standards.len() as f64 * 100.0;
        
        let findings: Vec<ComplianceFinding> = active_monitors.values()
            .flat_map(|check| check.findings.clone())
            .collect();
        
        Ok(ComplianceReport {
            id: Uuid::new_v4(),
            generated_at: chrono::Utc::now(),
            reporting_period: ReportingPeriod {
                start: chrono::Utc::now() - chrono::Duration::days(30),
                end: chrono::Utc::now(),
            },
            compliance_score,
            status,
            findings,
            recommendations: self.generate_compliance_recommendations(&findings).await?,
        })
    }

    async fn generate_compliance_recommendations(&self, findings: &[ComplianceFinding]) -> Result<Vec<ComplianceRecommendation>> {
        let mut recommendations = Vec::new();
        
        for finding in findings {
            if finding.severity == ComplianceSeverity::High || finding.severity == ComplianceSeverity::Critical {
                recommendations.push(ComplianceRecommendation {
                    id: Uuid::new_v4(),
                    finding_id: finding.id,
                    priority: match finding.severity {
                        ComplianceSeverity::Critical => RecommendationPriority::Critical,
                        ComplianceSeverity::High => RecommendationPriority::High,
                        _ => RecommendationPriority::Medium,
                    },
                    title: format!("Address {}", finding.description),
                    description: format!("Immediate action required for: {}", finding.description),
                    suggested_actions: vec![
                        "Review compliance controls".to_string(),
                        "Implement corrective measures".to_string(),
                        "Update policies and procedures".to_string(),
                    ],
                    estimated_effort_hours: 8,
                    deadline: chrono::Utc::now() + chrono::Duration::days(7),
                });
            }
        }
        
        Ok(recommendations)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub id: Uuid,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub reporting_period: ReportingPeriod,
    pub compliance_score: f64,
    pub status: ComplianceStatus,
    pub findings: Vec<ComplianceFinding>,
    pub recommendations: Vec<ComplianceRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingPeriod {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRecommendation {
    pub id: Uuid,
    pub finding_id: Uuid,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub suggested_actions: Vec<String>,
    pub estimated_effort_hours: u32,
    pub deadline: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}