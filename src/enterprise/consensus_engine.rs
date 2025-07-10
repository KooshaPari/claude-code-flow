//! Enterprise Consensus Engine
//! 
//! Advanced consensus algorithms for multi-level enterprise decision making

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use super::{EnterpriseConfig, ComplianceStandard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusEngine {
    config: EnterpriseConfig,
    consensus_algorithms: RwLock<HashMap<ConsensusType, Box<dyn ConsensusAlgorithm + Send + Sync>>>,
    active_votes: RwLock<HashMap<Uuid, ConsensusVote>>,
    vote_history: RwLock<Vec<CompletedVote>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConsensusType {
    Executive,      // Executive-level strategic decisions
    Management,     // Management-level tactical decisions  
    Team,          // Team-level operational decisions
    Individual,    // Individual task decisions
    CrossOrg,      // Cross-organizational federated decisions
    Byzantine,     // Byzantine fault-tolerant consensus
    Democratic,    // Democratic majority voting
    Weighted,      // Weighted voting based on expertise
    Delegated,     // Delegated proof-of-stake style
    Hybrid,        // Combination of multiple algorithms
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusVote {
    pub id: Uuid,
    pub proposal: Proposal,
    pub consensus_type: ConsensusType,
    pub participants: Vec<Participant>,
    pub votes: HashMap<Uuid, Vote>,
    pub status: VoteStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub deadline: chrono::DateTime<chrono::Utc>,
    pub quorum_required: f64, // 0.0 to 1.0
    pub approval_threshold: f64, // 0.0 to 1.0
    pub delegations: HashMap<Uuid, Uuid>, // Delegator -> Delegate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub impact_level: ImpactLevel,
    pub resource_requirements: ResourceRequirements,
    pub compliance_implications: Vec<ComplianceStandard>,
    pub alternatives: Vec<Alternative>,
    pub supporting_evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    Strategic,
    Tactical,
    Operational,
    Policy,
    Budget,
    Technical,
    Organizational,
    Compliance,
    Emergency,
    Routine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
    Transformational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub budget_usd: Option<f64>,
    pub personnel_count: Option<u32>,
    pub time_weeks: Option<f64>,
    pub technology_requirements: Vec<String>,
    pub compliance_requirements: Vec<ComplianceStandard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub cost_benefit_ratio: Option<f64>,
    pub risk_assessment: RiskAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub probability: f64, // 0.0 to 1.0
    pub impact: f64,      // 0.0 to 1.0
    pub mitigation_strategies: Vec<String>,
    pub contingency_plans: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub source: String,
    pub credibility_score: f64,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    DataAnalysis,
    MarketResearch,
    TechnicalAnalysis,
    FinancialAnalysis,
    RiskAssessment,
    StakeholderFeedback,
    BenchmarkStudy,
    ExpertOpinion,
    HistoricalData,
    Simulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: Uuid,
    pub name: String,
    pub role: ParticipantRole,
    pub voting_power: f64, // Weight in weighted voting systems
    pub expertise_areas: Vec<String>,
    pub delegation_allowed: bool,
    pub abstention_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Executive,
    Manager,
    TeamLead,
    Individual,
    SubjectMatterExpert,
    Stakeholder,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub participant_id: Uuid,
    pub decision: VoteDecision,
    pub confidence_level: f64, // 0.0 to 1.0
    pub rationale: String,
    pub conditions: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub delegated_from: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteDecision {
    Approve,
    Reject,
    Abstain,
    ApproveWithConditions,
    RejectWithCounterproposal,
    RequestMoreInformation,
    Delegate(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteStatus {
    Pending,
    Active,
    QuorumReached,
    Approved,
    Rejected,
    Expired,
    Cancelled,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedVote {
    pub vote: ConsensusVote,
    pub result: VoteResult,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub execution_status: ExecutionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResult {
    pub outcome: VoteOutcome,
    pub approval_percentage: f64,
    pub participation_rate: f64,
    pub consensus_score: f64, // How strong the consensus was
    pub dissenting_opinions: Vec<DissentingOpinion>,
    pub execution_plan: Option<ExecutionPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteOutcome {
    UnanimousApproval,
    MajorityApproval,
    SupermajorityApproval,
    NarrowApproval,
    Tie,
    NarrowRejection,
    MajorityRejection,
    UnanimousRejection,
    QuorumNotMet,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DissentingOpinion {
    pub participant_id: Uuid,
    pub opinion: String,
    pub alternative_proposal: Option<Uuid>,
    pub impact_concerns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub id: Uuid,
    pub phases: Vec<ExecutionPhase>,
    pub timeline: Timeline,
    pub resource_allocation: HashMap<String, f64>,
    pub success_metrics: Vec<SuccessMetric>,
    pub risk_mitigation: Vec<RiskMitigation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPhase {
    pub phase_number: u32,
    pub name: String,
    pub description: String,
    pub duration_weeks: f64,
    pub dependencies: Vec<u32>,
    pub deliverables: Vec<String>,
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub milestones: Vec<Milestone>,
    pub critical_path: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub name: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub review_criteria: Vec<String>,
    pub go_no_go_decision: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetric {
    pub name: String,
    pub target_value: f64,
    pub measurement_method: String,
    pub measurement_frequency: MeasurementFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementFrequency {
    RealTime,
    Daily,
    Weekly,
    Monthly,
    Milestone,
    Final,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMitigation {
    pub risk: String,
    pub probability: f64,
    pub impact: f64,
    pub mitigation_strategy: String,
    pub contingency_plan: String,
    pub responsible_party: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Planning,
    InProgress,
    OnHold,
    Completed,
    Failed,
    Cancelled,
}

// Trait for consensus algorithms
pub trait ConsensusAlgorithm: std::fmt::Debug + Send + Sync {
    fn calculate_result(&self, vote: &ConsensusVote) -> Result<VoteResult>;
    fn validate_vote(&self, vote: &Vote, consensus_vote: &ConsensusVote) -> Result<bool>;
    fn suggest_next_action(&self, vote: &ConsensusVote) -> Result<NextAction>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NextAction {
    ContinueVoting,
    ExtendDeadline,
    Escalate,
    Complete,
    Cancel,
    RequestMoreParticipants,
    CallEmergencySession,
}

// Majority Voting Algorithm
#[derive(Debug)]
pub struct MajorityVoting {
    pub threshold: f64, // 0.5 for simple majority, 0.67 for supermajority
}

impl ConsensusAlgorithm for MajorityVoting {
    fn calculate_result(&self, vote: &ConsensusVote) -> Result<VoteResult> {
        let total_participants = vote.participants.len() as f64;
        let votes_cast = vote.votes.len() as f64;
        let participation_rate = votes_cast / total_participants;
        
        if participation_rate < vote.quorum_required {
            return Ok(VoteResult {
                outcome: VoteOutcome::QuorumNotMet,
                approval_percentage: 0.0,
                participation_rate,
                consensus_score: 0.0,
                dissenting_opinions: vec![],
                execution_plan: None,
            });
        }
        
        let approvals = vote.votes.values()
            .filter(|v| matches!(v.decision, VoteDecision::Approve | VoteDecision::ApproveWithConditions))
            .count() as f64;
        
        let approval_percentage = approvals / votes_cast;
        
        let outcome = if approval_percentage >= self.threshold {
            match approval_percentage {
                p if p >= 0.95 => VoteOutcome::UnanimousApproval,
                p if p >= 0.75 => VoteOutcome::SupermajorityApproval,
                p if p >= 0.6 => VoteOutcome::MajorityApproval,
                _ => VoteOutcome::NarrowApproval,
            }
        } else {
            VoteOutcome::MajorityRejection
        };
        
        let consensus_score = calculate_consensus_score(&vote.votes);
        
        Ok(VoteResult {
            outcome,
            approval_percentage,
            participation_rate,
            consensus_score,
            dissenting_opinions: extract_dissenting_opinions(&vote.votes),
            execution_plan: None, // Generated separately if approved
        })
    }
    
    fn validate_vote(&self, vote: &Vote, consensus_vote: &ConsensusVote) -> Result<bool> {
        // Check if participant is authorized
        let participant_exists = consensus_vote.participants
            .iter()
            .any(|p| p.id == vote.participant_id);
        
        if !participant_exists {
            return Ok(false);
        }
        
        // Check if vote is within deadline
        if vote.timestamp > consensus_vote.deadline {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    fn suggest_next_action(&self, vote: &ConsensusVote) -> Result<NextAction> {
        let now = chrono::Utc::now();
        let time_remaining = vote.deadline - now;
        let participation_rate = vote.votes.len() as f64 / vote.participants.len() as f64;
        
        if time_remaining < chrono::Duration::hours(1) && participation_rate < vote.quorum_required {
            Ok(NextAction::ExtendDeadline)
        } else if participation_rate >= vote.quorum_required {
            Ok(NextAction::Complete)
        } else if time_remaining < chrono::Duration::zero() {
            Ok(NextAction::Escalate)
        } else {
            Ok(NextAction::ContinueVoting)
        }
    }
}

// Byzantine Fault Tolerant Algorithm
#[derive(Debug)]
pub struct ByzantineFaultTolerant {
    pub fault_tolerance: f64, // Percentage of Byzantine participants to tolerate
}

impl ConsensusAlgorithm for ByzantineFaultTolerant {
    fn calculate_result(&self, vote: &ConsensusVote) -> Result<VoteResult> {
        // Simplified BFT: requires 2/3 + 1 honest participants
        let min_honest_required = ((vote.participants.len() as f64 * 2.0 / 3.0) + 1.0).ceil();
        let votes_cast = vote.votes.len() as f64;
        
        if votes_cast < min_honest_required {
            return Ok(VoteResult {
                outcome: VoteOutcome::QuorumNotMet,
                approval_percentage: 0.0,
                participation_rate: votes_cast / vote.participants.len() as f64,
                consensus_score: 0.0,
                dissenting_opinions: vec![],
                execution_plan: None,
            });
        }
        
        // In real BFT, we'd have multiple rounds of voting
        // This is a simplified version
        let approvals = vote.votes.values()
            .filter(|v| matches!(v.decision, VoteDecision::Approve))
            .count() as f64;
        
        let approval_percentage = approvals / votes_cast;
        let consensus_threshold = 2.0 / 3.0; // BFT threshold
        
        let outcome = if approval_percentage >= consensus_threshold {
            VoteOutcome::SupermajorityApproval
        } else {
            VoteOutcome::MajorityRejection
        };
        
        Ok(VoteResult {
            outcome,
            approval_percentage,
            participation_rate: votes_cast / vote.participants.len() as f64,
            consensus_score: calculate_consensus_score(&vote.votes),
            dissenting_opinions: extract_dissenting_opinions(&vote.votes),
            execution_plan: None,
        })
    }
    
    fn validate_vote(&self, vote: &Vote, consensus_vote: &ConsensusVote) -> Result<bool> {
        // Enhanced validation for BFT
        let participant = consensus_vote.participants
            .iter()
            .find(|p| p.id == vote.participant_id)
            .ok_or_else(|| anyhow::anyhow!("Participant not found"))?;
        
        // Check reputation/trustworthiness (simplified)
        let is_trusted = vote.confidence_level >= 0.8;
        
        Ok(is_trusted && vote.timestamp <= consensus_vote.deadline)
    }
    
    fn suggest_next_action(&self, vote: &ConsensusVote) -> Result<NextAction> {
        let honest_threshold = (vote.participants.len() as f64 * 2.0 / 3.0).ceil();
        let current_votes = vote.votes.len() as f64;
        
        if current_votes >= honest_threshold {
            Ok(NextAction::Complete)
        } else {
            Ok(NextAction::ContinueVoting)
        }
    }
}

// Weighted Voting Algorithm (expertise-based)
#[derive(Debug)]
pub struct WeightedVoting {
    pub expertise_weight_factor: f64,
    pub role_weight_factor: f64,
}

impl ConsensusAlgorithm for WeightedVoting {
    fn calculate_result(&self, vote: &ConsensusVote) -> Result<VoteResult> {
        let total_weight: f64 = vote.participants.iter().map(|p| p.voting_power).sum();
        let votes_weight: f64 = vote.votes.values()
            .map(|v| {
                vote.participants
                    .iter()
                    .find(|p| p.id == v.participant_id)
                    .map(|p| p.voting_power)
                    .unwrap_or(0.0)
            })
            .sum();
        
        let participation_rate = votes_weight / total_weight;
        
        if participation_rate < vote.quorum_required {
            return Ok(VoteResult {
                outcome: VoteOutcome::QuorumNotMet,
                approval_percentage: 0.0,
                participation_rate,
                consensus_score: 0.0,
                dissenting_opinions: vec![],
                execution_plan: None,
            });
        }
        
        let approval_weight: f64 = vote.votes.values()
            .filter(|v| matches!(v.decision, VoteDecision::Approve | VoteDecision::ApproveWithConditions))
            .map(|v| {
                vote.participants
                    .iter()
                    .find(|p| p.id == v.participant_id)
                    .map(|p| p.voting_power * v.confidence_level)
                    .unwrap_or(0.0)
            })
            .sum();
        
        let approval_percentage = approval_weight / votes_weight;
        
        let outcome = if approval_percentage >= vote.approval_threshold {
            if approval_percentage >= 0.9 {
                VoteOutcome::SupermajorityApproval
            } else {
                VoteOutcome::MajorityApproval
            }
        } else {
            VoteOutcome::MajorityRejection
        };
        
        Ok(VoteResult {
            outcome,
            approval_percentage,
            participation_rate,
            consensus_score: calculate_consensus_score(&vote.votes),
            dissenting_opinions: extract_dissenting_opinions(&vote.votes),
            execution_plan: None,
        })
    }
    
    fn validate_vote(&self, vote: &Vote, consensus_vote: &ConsensusVote) -> Result<bool> {
        let participant = consensus_vote.participants
            .iter()
            .find(|p| p.id == vote.participant_id)
            .ok_or_else(|| anyhow::anyhow!("Participant not found"))?;
        
        // Validate based on expertise relevance
        let has_relevant_expertise = !participant.expertise_areas.is_empty();
        
        Ok(has_relevant_expertise && vote.timestamp <= consensus_vote.deadline)
    }
    
    fn suggest_next_action(&self, vote: &ConsensusVote) -> Result<NextAction> {
        let high_influence_participants: f64 = vote.participants
            .iter()
            .filter(|p| p.voting_power > 0.1) // High influence threshold
            .map(|p| p.voting_power)
            .sum();
        
        let high_influence_voted: f64 = vote.votes.values()
            .filter_map(|v| {
                vote.participants
                    .iter()
                    .find(|p| p.id == v.participant_id && p.voting_power > 0.1)
                    .map(|p| p.voting_power)
            })
            .sum();
        
        if high_influence_voted / high_influence_participants >= 0.8 {
            Ok(NextAction::Complete)
        } else {
            Ok(NextAction::ContinueVoting)
        }
    }
}

impl ConsensusEngine {
    pub fn new(config: &EnterpriseConfig) -> Result<Self> {
        let mut algorithms: HashMap<ConsensusType, Box<dyn ConsensusAlgorithm + Send + Sync>> = HashMap::new();
        
        // Initialize default algorithms
        algorithms.insert(
            ConsensusType::Democratic,
            Box::new(MajorityVoting { threshold: 0.5 }),
        );
        
        algorithms.insert(
            ConsensusType::Executive,
            Box::new(WeightedVoting {
                expertise_weight_factor: 0.7,
                role_weight_factor: 0.3,
            }),
        );
        
        algorithms.insert(
            ConsensusType::Byzantine,
            Box::new(ByzantineFaultTolerant {
                fault_tolerance: 0.33, // Tolerate up to 1/3 Byzantine participants
            }),
        );
        
        Ok(Self {
            config: config.clone(),
            consensus_algorithms: RwLock::new(algorithms),
            active_votes: RwLock::new(HashMap::new()),
            vote_history: RwLock::new(Vec::new()),
        })
    }
    
    pub async fn initiate_consensus(&self, proposal: Proposal, consensus_type: ConsensusType) -> Result<Uuid> {
        let vote_id = Uuid::new_v4();
        let participants = self.determine_participants(&proposal, &consensus_type).await?;
        
        let consensus_vote = ConsensusVote {
            id: vote_id,
            proposal,
            consensus_type: consensus_type.clone(),
            participants,
            votes: HashMap::new(),
            status: VoteStatus::Active,
            started_at: chrono::Utc::now(),
            deadline: chrono::Utc::now() + self.determine_voting_duration(&consensus_type),
            quorum_required: self.determine_quorum_requirement(&consensus_type),
            approval_threshold: self.determine_approval_threshold(&consensus_type),
            delegations: HashMap::new(),
        };
        
        let mut active_votes = self.active_votes.write().await;
        active_votes.insert(vote_id, consensus_vote);
        
        Ok(vote_id)
    }
    
    pub async fn cast_vote(&self, vote_id: Uuid, vote: Vote) -> Result<()> {
        let mut active_votes = self.active_votes.write().await;
        let consensus_vote = active_votes.get_mut(&vote_id)
            .ok_or_else(|| anyhow::anyhow!("Vote not found"))?;
        
        // Validate the vote using the appropriate algorithm
        let algorithms = self.consensus_algorithms.read().await;
        let algorithm = algorithms.get(&consensus_vote.consensus_type)
            .ok_or_else(|| anyhow::anyhow!("Algorithm not found"))?;
        
        if !algorithm.validate_vote(&vote, consensus_vote)? {
            return Err(anyhow::anyhow!("Invalid vote"));
        }
        
        consensus_vote.votes.insert(vote.participant_id, vote);
        
        // Check if we can complete the vote
        let next_action = algorithm.suggest_next_action(consensus_vote)?;
        
        match next_action {
            NextAction::Complete => {
                let result = algorithm.calculate_result(consensus_vote)?;
                let completed_vote = CompletedVote {
                    vote: consensus_vote.clone(),
                    result,
                    completed_at: chrono::Utc::now(),
                    execution_status: ExecutionStatus::Planning,
                };
                
                let mut history = self.vote_history.write().await;
                history.push(completed_vote);
                
                consensus_vote.status = match consensus_vote.status {
                    _ if matches!(result.outcome, VoteOutcome::UnanimousApproval | 
                                                  VoteOutcome::MajorityApproval | 
                                                  VoteOutcome::SupermajorityApproval |
                                                  VoteOutcome::NarrowApproval) => VoteStatus::Approved,
                    _ => VoteStatus::Rejected,
                };
            }
            NextAction::ExtendDeadline => {
                consensus_vote.deadline = chrono::Utc::now() + chrono::Duration::hours(24);
            }
            _ => {
                // Continue voting
            }
        }
        
        Ok(())
    }
    
    async fn determine_participants(&self, proposal: &Proposal, consensus_type: &ConsensusType) -> Result<Vec<Participant>> {
        // This would normally query the organization database
        // For now, return mock participants based on consensus type
        
        let mut participants = Vec::new();
        
        match consensus_type {
            ConsensusType::Executive => {
                participants.push(Participant {
                    id: Uuid::new_v4(),
                    name: "CEO".to_string(),
                    role: ParticipantRole::Executive,
                    voting_power: 0.4,
                    expertise_areas: vec!["Strategy".to_string(), "Leadership".to_string()],
                    delegation_allowed: false,
                    abstention_allowed: false,
                });
                
                participants.push(Participant {
                    id: Uuid::new_v4(),
                    name: "CTO".to_string(),
                    role: ParticipantRole::Executive,
                    voting_power: 0.35,
                    expertise_areas: vec!["Technology".to_string(), "Innovation".to_string()],
                    delegation_allowed: false,
                    abstention_allowed: false,
                });
                
                participants.push(Participant {
                    id: Uuid::new_v4(),
                    name: "CFO".to_string(),
                    role: ParticipantRole::Executive,
                    voting_power: 0.25,
                    expertise_areas: vec!["Finance".to_string(), "Risk".to_string()],
                    delegation_allowed: false,
                    abstention_allowed: false,
                });
            }
            
            ConsensusType::Management => {
                for i in 0..5 {
                    participants.push(Participant {
                        id: Uuid::new_v4(),
                        name: format!("Manager {}", i + 1),
                        role: ParticipantRole::Manager,
                        voting_power: 0.2,
                        expertise_areas: vec![
                            format!("Domain {}", i + 1),
                            "Management".to_string(),
                        ],
                        delegation_allowed: true,
                        abstention_allowed: true,
                    });
                }
            }
            
            ConsensusType::Team => {
                for i in 0..8 {
                    participants.push(Participant {
                        id: Uuid::new_v4(),
                        name: format!("Team Member {}", i + 1),
                        role: ParticipantRole::Individual,
                        voting_power: 0.125,
                        expertise_areas: vec![
                            format!("Skill {}", i + 1),
                        ],
                        delegation_allowed: true,
                        abstention_allowed: true,
                    });
                }
            }
            
            _ => {
                // Default set of participants
                participants.push(Participant {
                    id: Uuid::new_v4(),
                    name: "Default Participant".to_string(),
                    role: ParticipantRole::Individual,
                    voting_power: 1.0,
                    expertise_areas: vec![],
                    delegation_allowed: true,
                    abstention_allowed: true,
                });
            }
        }
        
        Ok(participants)
    }
    
    fn determine_voting_duration(&self, consensus_type: &ConsensusType) -> chrono::Duration {
        match consensus_type {
            ConsensusType::Executive => chrono::Duration::days(7),   // 1 week for strategic decisions
            ConsensusType::Management => chrono::Duration::days(3),  // 3 days for tactical decisions
            ConsensusType::Team => chrono::Duration::days(1),       // 1 day for operational decisions
            ConsensusType::Individual => chrono::Duration::hours(8), // 8 hours for individual tasks
            ConsensusType::Byzantine => chrono::Duration::hours(24), // 24 hours for BFT consensus
            _ => chrono::Duration::days(2), // Default 2 days
        }
    }
    
    fn determine_quorum_requirement(&self, consensus_type: &ConsensusType) -> f64 {
        match consensus_type {
            ConsensusType::Executive => 1.0,    // 100% participation required
            ConsensusType::Management => 0.8,   // 80% participation required
            ConsensusType::Team => 0.6,         // 60% participation required
            ConsensusType::Individual => 0.5,   // 50% participation required
            ConsensusType::Byzantine => 0.67,   // 67% for BFT
            _ => 0.5, // Default 50%
        }
    }
    
    fn determine_approval_threshold(&self, consensus_type: &ConsensusType) -> f64 {
        match consensus_type {
            ConsensusType::Executive => 0.8,    // 80% approval for strategic decisions
            ConsensusType::Management => 0.67,  // Supermajority for tactical decisions
            ConsensusType::Team => 0.6,         // 60% for operational decisions
            ConsensusType::Individual => 0.5,   // Simple majority for individual tasks
            ConsensusType::Byzantine => 0.67,   // 67% for BFT
            _ => 0.5, // Default simple majority
        }
    }
    
    pub async fn get_vote_status(&self, vote_id: Uuid) -> Result<Option<ConsensusVote>> {
        let active_votes = self.active_votes.read().await;
        Ok(active_votes.get(&vote_id).cloned())
    }
    
    pub async fn get_vote_history(&self) -> Result<Vec<CompletedVote>> {
        let history = self.vote_history.read().await;
        Ok(history.clone())
    }
}

// Helper functions
fn calculate_consensus_score(votes: &HashMap<Uuid, Vote>) -> f64 {
    if votes.is_empty() {
        return 0.0;
    }
    
    let avg_confidence: f64 = votes.values()
        .map(|v| v.confidence_level)
        .sum::<f64>() / votes.len() as f64;
    
    let agreement_score = {
        let approvals = votes.values()
            .filter(|v| matches!(v.decision, VoteDecision::Approve | VoteDecision::ApproveWithConditions))
            .count() as f64;
        let total = votes.len() as f64;
        
        // Calculate how close to unanimous the vote is
        let unanimity = (approvals / total - 0.5).abs() * 2.0; // 0 = split, 1 = unanimous
        unanimity
    };
    
    (avg_confidence + agreement_score) / 2.0
}

fn extract_dissenting_opinions(votes: &HashMap<Uuid, Vote>) -> Vec<DissentingOpinion> {
    votes.values()
        .filter(|v| matches!(v.decision, VoteDecision::Reject | VoteDecision::RejectWithCounterproposal))
        .map(|v| DissentingOpinion {
            participant_id: v.participant_id,
            opinion: v.rationale.clone(),
            alternative_proposal: None, // Would be filled if available
            impact_concerns: v.conditions.clone(),
        })
        .collect()
}