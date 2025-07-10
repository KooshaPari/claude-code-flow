//! Cross-Team Coordination Module
//! 
//! Manages coordination between multiple teams and departments

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::{mpsc, RwLock};
use super::EnterpriseConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub department: String,
    pub specializations: Vec<TeamSpecialization>,
    pub members: Vec<TeamMember>,
    pub capacity: TeamCapacity,
    pub current_workload: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamSpecialization {
    Frontend,
    Backend,
    Infrastructure,
    Security,
    DataScience,
    QualityAssurance,
    DevOps,
    ProductManagement,
    Design,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub skills: Vec<String>,
    pub availability: f64, // 0.0 to 1.0
    pub performance_rating: f64, // 0.0 to 5.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamCapacity {
    pub max_concurrent_tasks: u32,
    pub max_story_points_per_sprint: u32,
    pub available_hours_per_week: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationChannel {
    pub id: Uuid,
    pub teams: Vec<Uuid>,
    pub channel_type: ChannelType,
    pub priority: Priority,
    pub communication_protocol: CommunicationProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    DirectCollaboration,
    StatusUpdates,
    BlockerResolution,
    ResourceSharing,
    KnowledgeTransfer,
    QualityGate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationProtocol {
    pub frequency: UpdateFrequency,
    pub format: MessageFormat,
    pub escalation_rules: Vec<EscalationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageFormat {
    Structured,
    FreeForm,
    Template,
    Metrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub condition: String,
    pub threshold: f64,
    pub action: EscalationAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationAction {
    NotifyManager,
    ReallocateResources,
    AdjustPriority,
    TriggerEmergencyProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossTeamTask {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub required_specializations: Vec<TeamSpecialization>,
    pub dependencies: Vec<TaskDependency>,
    pub resource_allocation: HashMap<Uuid, TeamAllocation>,
    pub coordination_plan: CoordinationPlan,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub task_id: Uuid,
    pub dependency_type: DependencyType,
    pub blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    SequentialHandoff,
    ParallelSync,
    SharedResource,
    KnowledgeDependency,
    QualityGate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamAllocation {
    pub team_id: Uuid,
    pub allocated_members: Vec<Uuid>,
    pub time_allocation_percent: f64,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPlan {
    pub phases: Vec<CoordinationPhase>,
    pub synchronization_points: Vec<SyncPoint>,
    pub communication_schedule: CommunicationSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPhase {
    pub name: String,
    pub duration_days: u32,
    pub participating_teams: Vec<Uuid>,
    pub deliverables: Vec<String>,
    pub quality_gates: Vec<QualityGate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPoint {
    pub name: String,
    pub scheduled_date: chrono::DateTime<chrono::Utc>,
    pub participants: Vec<Uuid>,
    pub agenda: Vec<String>,
    pub required_deliverables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSchedule {
    pub daily_standups: Vec<StandupConfig>,
    pub weekly_syncs: Vec<SyncConfig>,
    pub milestone_reviews: Vec<ReviewConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandupConfig {
    pub teams: Vec<Uuid>,
    pub time: String, // "09:00 UTC"
    pub duration_minutes: u32,
    pub format: StandupFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StandupFormat {
    Traditional, // What did I do, what will I do, blockers
    Async,       // Written updates
    Visual,      // Dashboard-based
    Hybrid,      // Combination
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub teams: Vec<Uuid>,
    pub frequency: WeeklyFrequency,
    pub focus_areas: Vec<SyncFocus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeeklyFrequency {
    Weekly,
    BiWeekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncFocus {
    TechnicalAlignment,
    ResourceCoordination,
    RiskMitigation,
    QualityReview,
    ProgressTracking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewConfig {
    pub milestone_name: String,
    pub stakeholders: Vec<Uuid>,
    pub review_criteria: Vec<String>,
    pub success_metrics: Vec<SuccessMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetric {
    pub name: String,
    pub target_value: f64,
    pub unit: String,
    pub measurement_method: MeasurementMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementMethod {
    Automated,
    Manual,
    Survey,
    Calculation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    pub name: String,
    pub criteria: Vec<QualityCriterion>,
    pub required_approvers: Vec<Uuid>,
    pub automated_checks: Vec<AutomatedCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCriterion {
    pub metric: String,
    pub threshold: f64,
    pub operator: ComparisonOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedCheck {
    pub name: String,
    pub tool: String,
    pub configuration: HashMap<String, String>,
    pub required_status: CheckStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Planning,
    InProgress,
    Blocked,
    Review,
    Complete,
    Cancelled,
}

pub struct CrossTeamCoordinator {
    config: EnterpriseConfig,
    teams: RwLock<HashMap<Uuid, Team>>,
    coordination_channels: RwLock<HashMap<Uuid, CoordinationChannel>>,
    active_tasks: RwLock<HashMap<Uuid, CrossTeamTask>>,
    message_bus: tokio::sync::broadcast::Sender<CoordinationMessage>,
}

#[derive(Debug, Clone)]
pub struct CoordinationMessage {
    pub from_team: Uuid,
    pub to_teams: Vec<Uuid>,
    pub message_type: MessageType,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub priority: Priority,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    StatusUpdate,
    BlockerAlert,
    ResourceRequest,
    QualityGateResult,
    MilestoneAchieved,
    EscalationRequired,
}

impl CrossTeamCoordinator {
    pub fn new(config: &EnterpriseConfig) -> Result<Self> {
        let (tx, _) = tokio::sync::broadcast::channel(1000);
        
        Ok(Self {
            config: config.clone(),
            teams: RwLock::new(HashMap::new()),
            coordination_channels: RwLock::new(HashMap::new()),
            active_tasks: RwLock::new(HashMap::new()),
            message_bus: tx,
        })
    }

    pub async fn initialize_channels(&self) -> Result<()> {
        self.setup_default_teams().await?;
        self.create_coordination_channels().await?;
        self.start_coordination_loops().await?;
        Ok(())
    }

    async fn setup_default_teams(&self) -> Result<()> {
        let mut teams = self.teams.write().await;
        
        // Frontend Team
        let frontend_team = Team {
            id: Uuid::new_v4(),
            name: "Frontend Engineering".to_string(),
            department: "Engineering".to_string(),
            specializations: vec![TeamSpecialization::Frontend, TeamSpecialization::Design],
            members: vec![
                TeamMember {
                    id: Uuid::new_v4(),
                    name: "Senior Frontend Engineer".to_string(),
                    role: "Lead".to_string(),
                    skills: vec!["React".to_string(), "TypeScript".to_string(), "CSS".to_string()],
                    availability: 0.9,
                    performance_rating: 4.5,
                },
                TeamMember {
                    id: Uuid::new_v4(),
                    name: "Frontend Engineer".to_string(),
                    role: "Developer".to_string(),
                    skills: vec!["Vue.js".to_string(), "JavaScript".to_string(), "HTML".to_string()],
                    availability: 0.8,
                    performance_rating: 4.0,
                },
            ],
            capacity: TeamCapacity {
                max_concurrent_tasks: 3,
                max_story_points_per_sprint: 25,
                available_hours_per_week: 72.0,
            },
            current_workload: 0.6,
        };

        // Backend Team
        let backend_team = Team {
            id: Uuid::new_v4(),
            name: "Backend Engineering".to_string(),
            department: "Engineering".to_string(),
            specializations: vec![TeamSpecialization::Backend, TeamSpecialization::Infrastructure],
            members: vec![
                TeamMember {
                    id: Uuid::new_v4(),
                    name: "Senior Backend Engineer".to_string(),
                    role: "Lead".to_string(),
                    skills: vec!["Rust".to_string(), "Go".to_string(), "PostgreSQL".to_string()],
                    availability: 0.9,
                    performance_rating: 4.8,
                },
                TeamMember {
                    id: Uuid::new_v4(),
                    name: "Backend Engineer".to_string(),
                    role: "Developer".to_string(),
                    skills: vec!["Python".to_string(), "Redis".to_string(), "Docker".to_string()],
                    availability: 0.85,
                    performance_rating: 4.2,
                },
            ],
            capacity: TeamCapacity {
                max_concurrent_tasks: 4,
                max_story_points_per_sprint: 30,
                available_hours_per_week: 76.0,
            },
            current_workload: 0.5,
        };

        teams.insert(frontend_team.id, frontend_team);
        teams.insert(backend_team.id, backend_team);

        Ok(())
    }

    async fn create_coordination_channels(&self) -> Result<()> {
        let teams = self.teams.read().await;
        let team_ids: Vec<Uuid> = teams.keys().cloned().collect();
        
        let mut channels = self.coordination_channels.write().await;

        // Direct collaboration channel
        let collaboration_channel = CoordinationChannel {
            id: Uuid::new_v4(),
            teams: team_ids.clone(),
            channel_type: ChannelType::DirectCollaboration,
            priority: Priority::High,
            communication_protocol: CommunicationProtocol {
                frequency: UpdateFrequency::RealTime,
                format: MessageFormat::Structured,
                escalation_rules: vec![
                    EscalationRule {
                        condition: "task_blocked".to_string(),
                        threshold: 2.0, // 2 hours
                        action: EscalationAction::NotifyManager,
                    },
                ],
            },
        };

        channels.insert(collaboration_channel.id, collaboration_channel);

        Ok(())
    }

    async fn start_coordination_loops(&self) -> Result<()> {
        // Start background tasks for coordination
        let mut receiver = self.message_bus.subscribe();
        
        tokio::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                // Process coordination messages
                match message.message_type {
                    MessageType::BlockerAlert => {
                        // Handle blocker escalation
                        tracing::info!("Processing blocker alert: {}", message.content);
                    }
                    MessageType::StatusUpdate => {
                        // Update task status
                        tracing::debug!("Status update: {}", message.content);
                    }
                    _ => {
                        tracing::debug!("Received coordination message: {:?}", message.message_type);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn coordinate_teams_for_task(&self, task_id: &Uuid, _resource_allocation: &str) -> Result<()> {
        // Create cross-team coordination plan
        let coordination_plan = self.create_coordination_plan(task_id).await?;
        
        // Setup communication channels
        self.setup_task_communication(task_id, &coordination_plan).await?;
        
        // Start monitoring and coordination
        self.start_task_monitoring(task_id).await?;
        
        Ok(())
    }

    async fn create_coordination_plan(&self, _task_id: &Uuid) -> Result<CoordinationPlan> {
        Ok(CoordinationPlan {
            phases: vec![
                CoordinationPhase {
                    name: "Planning & Design".to_string(),
                    duration_days: 5,
                    participating_teams: self.teams.read().await.keys().cloned().collect(),
                    deliverables: vec![
                        "Technical specification".to_string(),
                        "Resource allocation plan".to_string(),
                    ],
                    quality_gates: vec![
                        QualityGate {
                            name: "Design Review".to_string(),
                            criteria: vec![
                                QualityCriterion {
                                    metric: "review_score".to_string(),
                                    threshold: 4.0,
                                    operator: ComparisonOperator::GreaterThanOrEqual,
                                },
                            ],
                            required_approvers: vec![Uuid::new_v4()],
                            automated_checks: vec![],
                        },
                    ],
                },
            ],
            synchronization_points: vec![
                SyncPoint {
                    name: "Phase 1 Completion".to_string(),
                    scheduled_date: chrono::Utc::now() + chrono::Duration::days(5),
                    participants: self.teams.read().await.keys().cloned().collect(),
                    agenda: vec!["Review deliverables".to_string(), "Plan next phase".to_string()],
                    required_deliverables: vec!["Technical spec".to_string()],
                },
            ],
            communication_schedule: CommunicationSchedule {
                daily_standups: vec![
                    StandupConfig {
                        teams: self.teams.read().await.keys().cloned().collect(),
                        time: "09:00 UTC".to_string(),
                        duration_minutes: 15,
                        format: StandupFormat::Hybrid,
                    },
                ],
                weekly_syncs: vec![
                    SyncConfig {
                        teams: self.teams.read().await.keys().cloned().collect(),
                        frequency: WeeklyFrequency::Weekly,
                        focus_areas: vec![
                            SyncFocus::TechnicalAlignment,
                            SyncFocus::ResourceCoordination,
                        ],
                    },
                ],
                milestone_reviews: vec![],
            },
        })
    }

    async fn setup_task_communication(&self, _task_id: &Uuid, _plan: &CoordinationPlan) -> Result<()> {
        // Setup communication channels for this specific task
        Ok(())
    }

    async fn start_task_monitoring(&self, task_id: &Uuid) -> Result<()> {
        let task_id = *task_id;
        let sender = self.message_bus.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                // Send periodic status updates
                let message = CoordinationMessage {
                    from_team: Uuid::new_v4(), // System
                    to_teams: vec![],
                    message_type: MessageType::StatusUpdate,
                    content: format!("Task {} progress check", task_id),
                    timestamp: chrono::Utc::now(),
                    priority: Priority::Low,
                };
                
                if sender.send(message).is_err() {
                    break; // No receivers, stop monitoring
                }
            }
        });

        Ok(())
    }

    pub async fn get_efficiency(&self) -> Result<f64> {
        let teams = self.teams.read().await;
        let tasks = self.active_tasks.read().await;
        
        if teams.is_empty() || tasks.is_empty() {
            return Ok(0.0);
        }

        // Calculate coordination efficiency based on:
        // - Team utilization
        // - Task completion rate
        // - Communication effectiveness
        
        let avg_utilization: f64 = teams.values()
            .map(|t| t.current_workload)
            .sum::<f64>() / teams.len() as f64;

        let completed_tasks = tasks.values()
            .filter(|t| matches!(t.status, TaskStatus::Complete))
            .count() as f64;
        
        let completion_rate = if tasks.len() > 0 {
            completed_tasks / tasks.len() as f64
        } else {
            0.0
        };

        // Simple efficiency calculation
        let efficiency = (avg_utilization * 0.4) + (completion_rate * 0.6);
        
        Ok(efficiency.min(1.0))
    }
}