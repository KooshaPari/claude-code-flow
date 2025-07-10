//! Enterprise Hierarchy Manager
//! 
//! Manages organizational hierarchies and decision-making processes

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use super::{EnterpriseConfig, ComplianceStandard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyLevel {
    pub level: u8,
    pub name: String,
    pub decision_authority: DecisionAuthority,
    pub members: Vec<Uuid>,
    pub parent_level: Option<u8>,
    pub child_levels: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionAuthority {
    Executive,     // Strategic decisions
    Management,    // Tactical decisions
    Team,         // Operational decisions
    Individual,   // Task-level decisions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRequest {
    pub id: Uuid,
    pub task_definition: String,
    pub required_authority: DecisionAuthority,
    pub compliance_requirements: Vec<ComplianceStandard>,
    pub resource_requirements: ResourceRequirement,
    pub urgency: UrgencyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirement {
    pub cpu_cores: u32,
    pub memory_gb: f64,
    pub storage_gb: f64,
    pub estimated_duration_hours: f64,
    pub team_members: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub approved: bool,
    pub approvers: Vec<Uuid>,
    pub rejectors: Vec<Uuid>,
    pub conditions: Vec<String>,
    pub escalation_required: bool,
}

pub struct HierarchyManager {
    hierarchy: RwLock<HashMap<u8, HierarchyLevel>>,
    config: EnterpriseConfig,
    pending_decisions: RwLock<HashMap<Uuid, ConsensusRequest>>,
}

impl HierarchyManager {
    pub fn new(config: &EnterpriseConfig) -> Result<Self> {
        Ok(Self {
            hierarchy: RwLock::new(HashMap::new()),
            config: config.clone(),
            pending_decisions: RwLock::new(HashMap::new()),
        })
    }

    pub async fn setup_hierarchy(&self) -> Result<()> {
        let mut hierarchy = self.hierarchy.write().await;
        
        // Level 0: Executive
        hierarchy.insert(0, HierarchyLevel {
            level: 0,
            name: "Executive Level".to_string(),
            decision_authority: DecisionAuthority::Executive,
            members: vec![Uuid::new_v4()], // CEO/CTO
            parent_level: None,
            child_levels: vec![1],
        });

        // Level 1: Management
        hierarchy.insert(1, HierarchyLevel {
            level: 1,
            name: "Management Level".to_string(),
            decision_authority: DecisionAuthority::Management,
            members: vec![Uuid::new_v4(), Uuid::new_v4()], // VPs/Directors
            parent_level: Some(0),
            child_levels: vec![2],
        });

        // Level 2: Team Leads
        hierarchy.insert(2, HierarchyLevel {
            level: 2,
            name: "Team Lead Level".to_string(),
            decision_authority: DecisionAuthority::Team,
            members: vec![
                Uuid::new_v4(), Uuid::new_v4(), 
                Uuid::new_v4(), Uuid::new_v4()
            ], // Team Leads
            parent_level: Some(1),
            child_levels: vec![3],
        });

        // Level 3: Individual Contributors
        hierarchy.insert(3, HierarchyLevel {
            level: 3,
            name: "Individual Contributor Level".to_string(),
            decision_authority: DecisionAuthority::Individual,
            members: vec![
                Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(),
                Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(),
                Uuid::new_v4(), Uuid::new_v4()
            ], // Individual Contributors
            parent_level: Some(2),
            child_levels: vec![],
        });

        Ok(())
    }

    pub async fn get_consensus_for_task(&self, task_definition: &str) -> Result<ConsensusResult> {
        let request = self.analyze_task_requirements(task_definition).await?;
        let required_level = self.determine_required_authority_level(&request).await?;
        
        let hierarchy = self.hierarchy.read().await;
        let target_level = hierarchy.get(&required_level)
            .ok_or_else(|| anyhow::anyhow!("Invalid hierarchy level"))?;

        // Simulate consensus process
        let approval_threshold = match target_level.decision_authority {
            DecisionAuthority::Executive => 0.8,  // 80% approval
            DecisionAuthority::Management => 0.7, // 70% approval
            DecisionAuthority::Team => 0.6,      // 60% approval
            DecisionAuthority::Individual => 0.5, // 50% approval
        };

        let total_members = target_level.members.len();
        let required_approvals = (total_members as f64 * approval_threshold).ceil() as usize;
        let approvals = self.simulate_voting(&target_level.members, &request).await?;

        let approved = approvals.len() >= required_approvals;
        let rejectors = target_level.members.iter()
            .filter(|id| !approvals.contains(id))
            .cloned()
            .collect();

        Ok(ConsensusResult {
            approved,
            approvers: approvals,
            rejectors,
            conditions: self.generate_approval_conditions(&request).await?,
            escalation_required: !approved && required_level > 0,
        })
    }

    async fn analyze_task_requirements(&self, task_definition: &str) -> Result<ConsensusRequest> {
        // AI-powered task analysis
        let resource_requirements = if task_definition.contains("enterprise") || 
                                      task_definition.contains("production") {
            ResourceRequirement {
                cpu_cores: 16,
                memory_gb: 32.0,
                storage_gb: 500.0,
                estimated_duration_hours: 8.0,
                team_members: 5,
            }
        } else if task_definition.contains("development") || 
                  task_definition.contains("feature") {
            ResourceRequirement {
                cpu_cores: 8,
                memory_gb: 16.0,
                storage_gb: 100.0,
                estimated_duration_hours: 4.0,
                team_members: 3,
            }
        } else {
            ResourceRequirement {
                cpu_cores: 4,
                memory_gb: 8.0,
                storage_gb: 50.0,
                estimated_duration_hours: 2.0,
                team_members: 1,
            }
        };

        let urgency = if task_definition.contains("critical") || 
                         task_definition.contains("urgent") {
            UrgencyLevel::Critical
        } else if task_definition.contains("important") {
            UrgencyLevel::High
        } else {
            UrgencyLevel::Medium
        };

        Ok(ConsensusRequest {
            id: Uuid::new_v4(),
            task_definition: task_definition.to_string(),
            required_authority: DecisionAuthority::Team, // Default
            compliance_requirements: self.config.compliance_requirements.clone(),
            resource_requirements,
            urgency,
        })
    }

    async fn determine_required_authority_level(&self, request: &ConsensusRequest) -> Result<u8> {
        // Determine hierarchy level based on task complexity and resource requirements
        if request.resource_requirements.team_members > 10 || 
           request.resource_requirements.estimated_duration_hours > 40.0 ||
           matches!(request.urgency, UrgencyLevel::Critical) {
            Ok(0) // Executive level
        } else if request.resource_requirements.team_members > 5 || 
                  request.resource_requirements.estimated_duration_hours > 20.0 ||
                  matches!(request.urgency, UrgencyLevel::High) {
            Ok(1) // Management level
        } else if request.resource_requirements.team_members > 1 || 
                  request.resource_requirements.estimated_duration_hours > 8.0 {
            Ok(2) // Team level
        } else {
            Ok(3) // Individual level
        }
    }

    async fn simulate_voting(&self, members: &[Uuid], request: &ConsensusRequest) -> Result<Vec<Uuid>> {
        // Simulate voting based on task characteristics
        let approval_rate = match request.urgency {
            UrgencyLevel::Critical => 0.9,
            UrgencyLevel::High => 0.8,
            UrgencyLevel::Medium => 0.7,
            UrgencyLevel::Low => 0.6,
        };

        let approved_count = (members.len() as f64 * approval_rate).floor() as usize;
        Ok(members.iter().take(approved_count).cloned().collect())
    }

    async fn generate_approval_conditions(&self, request: &ConsensusRequest) -> Result<Vec<String>> {
        let mut conditions = Vec::new();

        if request.resource_requirements.estimated_duration_hours > 20.0 {
            conditions.push("Weekly progress reports required".to_string());
        }

        if request.resource_requirements.team_members > 5 {
            conditions.push("Daily standups with cross-team coordination".to_string());
        }

        if !request.compliance_requirements.is_empty() {
            conditions.push("Compliance audit checkpoints at 25%, 50%, 75% completion".to_string());
        }

        if matches!(request.urgency, UrgencyLevel::Critical | UrgencyLevel::High) {
            conditions.push("Real-time monitoring and escalation procedures".to_string());
        }

        Ok(conditions)
    }

    pub async fn escalate_decision(&self, request_id: Uuid) -> Result<ConsensusResult> {
        let mut pending = self.pending_decisions.write().await;
        let request = pending.get(&request_id)
            .ok_or_else(|| anyhow::anyhow!("Request not found"))?
            .clone();

        // Move to higher authority level
        let current_level = self.determine_required_authority_level(&request).await?;
        if current_level == 0 {
            return Err(anyhow::anyhow!("Already at highest authority level"));
        }

        let escalated_request = ConsensusRequest {
            required_authority: match current_level {
                3 => DecisionAuthority::Team,
                2 => DecisionAuthority::Management,
                1 => DecisionAuthority::Executive,
                _ => DecisionAuthority::Executive,
            },
            urgency: UrgencyLevel::High, // Escalation increases urgency
            ..request
        };

        pending.insert(request_id, escalated_request.clone());
        self.get_consensus_for_task(&escalated_request.task_definition).await
    }
}