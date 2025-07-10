use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::config::Config;
use crate::coordination::CoordinationMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub total_agents: u32,
    pub active_agents: u32,
    pub idle_agents: u32,
    pub failed_agents: u32,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiveMindStatus {
    pub queen_active: bool,
    pub worker_count: u32,
    pub current_task: Option<String>,
    pub coordination_health: f32,
    pub consensus_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub agent_type: AgentType,
    pub name: String,
    pub status: AgentStatusType,
    pub capabilities: Vec<String>,
    pub current_task: Option<String>,
    pub created_at: u64,
    pub last_heartbeat: u64,
    pub performance_metrics: AgentMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    Queen,
    Architect,
    Coder,
    Tester,
    Analyst,
    Researcher,
    Security,
    DevOps,
    Coordinator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatusType {
    Initializing,
    Idle,
    Working,
    Coordinating,
    Failed,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub tasks_completed: u64,
    pub success_rate: f32,
    pub avg_response_time: f32,
    pub coordination_score: f32,
}

#[derive(Debug, Clone)]
pub struct AgentCapability {
    pub name: String,
    pub description: String,
    pub required_resources: ResourceRequirements,
    pub compatibility: Vec<AgentType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_percent: f32,
    pub memory_mb: u32,
    pub network_bandwidth: u32,
    pub storage_mb: u32,
}

pub struct AgentManager {
    config: crate::config::AgentConfig,
    agents: RwLock<HashMap<Uuid, Agent>>,
    capabilities: RwLock<HashMap<String, AgentCapability>>,
    message_tx: mpsc::UnboundedSender<CoordinationMessage>,
    message_rx: RwLock<Option<mpsc::UnboundedReceiver<CoordinationMessage>>>,
}

impl AgentManager {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing agent manager");
        
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        
        let mut capabilities = HashMap::new();
        Self::register_default_capabilities(&mut capabilities);
        
        Ok(Self {
            config: config.agents.clone(),
            agents: RwLock::new(HashMap::new()),
            capabilities: RwLock::new(capabilities),
            message_tx,
            message_rx: RwLock::new(Some(message_rx)),
        })
    }
    
    pub async fn initialize_hive_mind(&self) -> Result<()> {
        info!("Initializing hive-mind intelligence system");
        
        // Create Queen agent
        let queen = self.create_agent(
            AgentType::Queen,
            "Queen".to_string(),
            vec!["coordination".to_string(), "decision-making".to_string(), "strategic-planning".to_string()]
        ).await?;
        
        info!("Queen agent spawned: {}", queen.id);
        
        // Register Queen as the primary coordinator
        self.register_agent(queen).await?;
        
        info!("Hive-mind system initialized successfully");
        Ok(())
    }
    
    pub async fn spawn_agents_for_task(&self, task: &str, count: u32, strategy: &str) -> Result<Vec<Uuid>> {
        info!("Spawning {} agents for task: '{}'", count, task);
        
        let mut spawned_agents = Vec::new();
        
        // Determine agent types based on task and strategy
        let agent_plan = self.plan_agent_distribution(task, count, strategy).await?;
        
        for (agent_type, agent_count) in agent_plan {
            for i in 0..agent_count {
                let agent_name = format!("{:?}-{}", agent_type, i + 1);
                let capabilities = self.get_capabilities_for_type(&agent_type).await;
                
                let agent = self.create_agent(agent_type.clone(), agent_name, capabilities).await?;
                let agent_id = agent.id;
                
                self.register_agent(agent).await?;
                spawned_agents.push(agent_id);
                
                debug!("Spawned agent: {} ({})", agent_id, agent_type.as_str());
            }
        }
        
        info!("Successfully spawned {} agents", spawned_agents.len());
        Ok(spawned_agents)
    }
    
    pub async fn get_status(&self) -> Result<AgentStatus> {
        let agents = self.agents.read().await;
        
        let total_agents = agents.len() as u32;
        let mut active_agents = 0;
        let mut idle_agents = 0;
        let mut failed_agents = 0;
        let mut total_cpu = 0.0;
        let mut total_memory = 0;
        
        for agent in agents.values() {
            match agent.status {
                AgentStatusType::Working | AgentStatusType::Coordinating => active_agents += 1,
                AgentStatusType::Idle => idle_agents += 1,
                AgentStatusType::Failed => failed_agents += 1,
                _ => {}
            }
            
            // Estimate resource usage (in a real implementation, this would be measured)
            total_cpu += 10.0; // Base CPU usage per agent
            total_memory += 50 * 1024 * 1024; // 50MB per agent
        }
        
        Ok(AgentStatus {
            total_agents,
            active_agents,
            idle_agents,
            failed_agents,
            cpu_usage: total_cpu,
            memory_usage: total_memory,
        })
    }
    
    pub async fn get_hive_mind_status(&self) -> Result<HiveMindStatus> {
        let agents = self.agents.read().await;
        
        let queen_active = agents.values()
            .any(|agent| matches!(agent.agent_type, AgentType::Queen) && 
                 !matches!(agent.status, AgentStatusType::Failed | AgentStatusType::Shutdown));
        
        let worker_count = agents.values()
            .filter(|agent| !matches!(agent.agent_type, AgentType::Queen))
            .count() as u32;
        
        let current_task = agents.values()
            .find_map(|agent| agent.current_task.clone())
            .or_else(|| Some("No active task".to_string()));
        
        // Calculate coordination health based on agent performance
        let coordination_health = if worker_count > 0 {
            let avg_coordination_score: f32 = agents.values()
                .map(|agent| agent.performance_metrics.coordination_score)
                .sum::<f32>() / agents.len() as f32;
            avg_coordination_score
        } else {
            0.0
        };
        
        Ok(HiveMindStatus {
            queen_active,
            worker_count,
            current_task,
            coordination_health,
            consensus_active: queen_active && worker_count > 0,
        })
    }
    
    pub async fn optimize_performance(&self) -> Result<()> {
        info!("Optimizing agent performance");
        
        let mut agents = self.agents.write().await;
        
        for agent in agents.values_mut() {
            // Remove failed agents
            if matches!(agent.status, AgentStatusType::Failed) {
                continue;
            }
            
            // Optimize agent performance metrics
            if agent.performance_metrics.success_rate < 0.8 {
                warn!("Agent {} has low success rate: {:.2}", 
                      agent.id, agent.performance_metrics.success_rate);
                
                // Reset agent or apply optimization strategies
                agent.performance_metrics.coordination_score *= 1.1;
            }
            
            // Update last heartbeat
            agent.last_heartbeat = self.current_timestamp();
        }
        
        info!("Agent performance optimization complete");
        Ok(())
    }
    
    async fn create_agent(&self, agent_type: AgentType, name: String, capabilities: Vec<String>) -> Result<Agent> {
        let agent_id = Uuid::new_v4();
        let current_time = self.current_timestamp();
        
        Ok(Agent {
            id: agent_id,
            agent_type,
            name,
            status: AgentStatusType::Initializing,
            capabilities,
            current_task: None,
            created_at: current_time,
            last_heartbeat: current_time,
            performance_metrics: AgentMetrics {
                tasks_completed: 0,
                success_rate: 1.0,
                avg_response_time: 0.0,
                coordination_score: 0.8,
            },
        })
    }
    
    async fn register_agent(&self, mut agent: Agent) -> Result<()> {
        agent.status = AgentStatusType::Idle;
        agent.last_heartbeat = self.current_timestamp();
        
        let agent_id = agent.id;
        self.agents.write().await.insert(agent_id, agent);
        
        debug!("Agent registered: {}", agent_id);
        Ok(())
    }
    
    async fn plan_agent_distribution(&self, task: &str, count: u32, strategy: &str) -> Result<Vec<(AgentType, u32)>> {
        debug!("Planning agent distribution for task: '{}', count: {}, strategy: '{}'", task, count, strategy);
        
        let plan = match strategy {
            "hierarchical" => self.plan_hierarchical_distribution(task, count).await,
            "parallel" => self.plan_parallel_distribution(task, count).await,
            "specialized" => self.plan_specialized_distribution(task, count).await,
            _ => self.plan_default_distribution(task, count).await,
        };
        
        debug!("Agent distribution plan: {:?}", plan);
        Ok(plan)
    }
    
    async fn plan_hierarchical_distribution(&self, _task: &str, count: u32) -> Vec<(AgentType, u32)> {
        let mut plan = vec![(AgentType::Coordinator, 1)];
        
        let remaining = count.saturating_sub(1);
        if remaining > 0 {
            plan.push((AgentType::Architect, std::cmp::min(2, remaining)));
        }
        if remaining > 2 {
            plan.push((AgentType::Coder, std::cmp::min(3, remaining - 2)));
        }
        if remaining > 5 {
            plan.push((AgentType::Tester, remaining - 5));
        }
        
        plan
    }
    
    async fn plan_parallel_distribution(&self, _task: &str, count: u32) -> Vec<(AgentType, u32)> {
        let types = vec![
            AgentType::Coder,
            AgentType::Tester,
            AgentType::Analyst,
            AgentType::Researcher,
        ];
        
        let per_type = count / types.len() as u32;
        let remainder = count % types.len() as u32;
        
        let mut plan = Vec::new();
        for (i, agent_type) in types.into_iter().enumerate() {
            let mut allocation = per_type;
            if i < remainder as usize {
                allocation += 1;
            }
            if allocation > 0 {
                plan.push((agent_type, allocation));
            }
        }
        
        plan
    }
    
    async fn plan_specialized_distribution(&self, task: &str, count: u32) -> Vec<(AgentType, u32)> {
        // Analyze task to determine specializations needed
        let task_lower = task.to_lowercase();
        
        if task_lower.contains("security") || task_lower.contains("audit") {
            vec![(AgentType::Security, count)]
        } else if task_lower.contains("deploy") || task_lower.contains("infrastructure") {
            vec![(AgentType::DevOps, count)]
        } else if task_lower.contains("research") || task_lower.contains("analysis") {
            vec![(AgentType::Researcher, count)]
        } else if task_lower.contains("test") || task_lower.contains("qa") {
            vec![(AgentType::Tester, count)]
        } else {
            self.plan_default_distribution(task, count).await
        }
    }
    
    async fn plan_default_distribution(&self, _task: &str, count: u32) -> Vec<(AgentType, u32)> {
        match count {
            1 => vec![(AgentType::Coder, 1)],
            2..=3 => vec![(AgentType::Coder, count - 1), (AgentType::Tester, 1)],
            4..=5 => vec![
                (AgentType::Architect, 1),
                (AgentType::Coder, count - 2),
                (AgentType::Tester, 1)
            ],
            _ => vec![
                (AgentType::Architect, 1),
                (AgentType::Coder, count - 3),
                (AgentType::Tester, 1),
                (AgentType::Coordinator, 1)
            ],
        }
    }
    
    async fn get_capabilities_for_type(&self, agent_type: &AgentType) -> Vec<String> {
        match agent_type {
            AgentType::Queen => vec![
                "coordination".to_string(),
                "decision-making".to_string(),
                "strategic-planning".to_string(),
                "consensus-building".to_string(),
            ],
            AgentType::Architect => vec![
                "system-design".to_string(),
                "architecture-planning".to_string(),
                "technology-selection".to_string(),
                "scalability-planning".to_string(),
            ],
            AgentType::Coder => vec![
                "code-generation".to_string(),
                "implementation".to_string(),
                "debugging".to_string(),
                "refactoring".to_string(),
            ],
            AgentType::Tester => vec![
                "test-planning".to_string(),
                "test-execution".to_string(),
                "quality-assurance".to_string(),
                "bug-detection".to_string(),
            ],
            AgentType::Analyst => vec![
                "data-analysis".to_string(),
                "performance-analysis".to_string(),
                "requirements-analysis".to_string(),
                "reporting".to_string(),
            ],
            AgentType::Researcher => vec![
                "information-gathering".to_string(),
                "technology-research".to_string(),
                "best-practices".to_string(),
                "documentation".to_string(),
            ],
            AgentType::Security => vec![
                "security-analysis".to_string(),
                "vulnerability-scanning".to_string(),
                "compliance-checking".to_string(),
                "penetration-testing".to_string(),
            ],
            AgentType::DevOps => vec![
                "deployment".to_string(),
                "infrastructure-management".to_string(),
                "monitoring".to_string(),
                "automation".to_string(),
            ],
            AgentType::Coordinator => vec![
                "task-coordination".to_string(),
                "resource-management".to_string(),
                "progress-tracking".to_string(),
                "communication".to_string(),
            ],
        }
    }
    
    fn register_default_capabilities(capabilities: &mut HashMap<String, AgentCapability>) {
        let default_capabilities = vec![
            AgentCapability {
                name: "coordination".to_string(),
                description: "Coordinate tasks between agents".to_string(),
                required_resources: ResourceRequirements {
                    cpu_percent: 5.0,
                    memory_mb: 64,
                    network_bandwidth: 10,
                    storage_mb: 10,
                },
                compatibility: vec![AgentType::Queen, AgentType::Coordinator],
            },
            AgentCapability {
                name: "code-generation".to_string(),
                description: "Generate and write code".to_string(),
                required_resources: ResourceRequirements {
                    cpu_percent: 20.0,
                    memory_mb: 256,
                    network_bandwidth: 5,
                    storage_mb: 100,
                },
                compatibility: vec![AgentType::Coder],
            },
            AgentCapability {
                name: "system-design".to_string(),
                description: "Design system architecture".to_string(),
                required_resources: ResourceRequirements {
                    cpu_percent: 15.0,
                    memory_mb: 128,
                    network_bandwidth: 5,
                    storage_mb: 50,
                },
                compatibility: vec![AgentType::Architect],
            },
            // Add more capabilities as needed
        ];
        
        for capability in default_capabilities {
            capabilities.insert(capability.name.clone(), capability);
        }
    }
    
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl AgentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentType::Queen => "Queen",
            AgentType::Architect => "Architect",
            AgentType::Coder => "Coder",
            AgentType::Tester => "Tester",
            AgentType::Analyst => "Analyst",
            AgentType::Researcher => "Researcher",
            AgentType::Security => "Security",
            AgentType::DevOps => "DevOps",
            AgentType::Coordinator => "Coordinator",
        }
    }
}