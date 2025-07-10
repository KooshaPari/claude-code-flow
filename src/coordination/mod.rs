use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, Mutex};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::config::Config;
use crate::agents::{Agent, AgentType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationMessage {
    pub id: Uuid,
    pub from_agent: Uuid,
    pub to_agent: Option<Uuid>, // None for broadcast
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub priority: MessagePriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    TaskAssignment,
    TaskComplete,
    TaskFailed,
    StatusUpdate,
    ResourceRequest,
    ResourceResponse,
    Coordination,
    Heartbeat,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub assigned_to: Option<Uuid>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub dependencies: Vec<Uuid>,
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub estimated_duration: Option<u64>,
    pub actual_duration: Option<u64>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationStrategy {
    pub name: String,
    pub description: String,
    pub topology: TopologyType,
    pub load_balancing: bool,
    pub fault_tolerance: bool,
    pub consensus_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyType {
    Hierarchical,
    Mesh,
    Ring,
    Star,
    Hybrid,
}

pub struct Coordinator {
    config: crate::config::SwarmConfig,
    active_tasks: RwLock<HashMap<Uuid, Task>>,
    task_queue: Mutex<VecDeque<Task>>,
    agents: RwLock<HashMap<Uuid, Agent>>,
    message_bus: MessageBus,
    strategy: RwLock<CoordinationStrategy>,
    performance_metrics: RwLock<CoordinationMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationMetrics {
    pub tasks_coordinated: u64,
    pub average_completion_time: f32,
    pub success_rate: f32,
    pub resource_utilization: f32,
    pub communication_overhead: f32,
}

struct MessageBus {
    senders: RwLock<HashMap<Uuid, mpsc::UnboundedSender<CoordinationMessage>>>,
    broadcast_sender: mpsc::UnboundedSender<CoordinationMessage>,
    message_history: RwLock<VecDeque<CoordinationMessage>>,
}

impl Coordinator {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing coordination system");
        
        let (broadcast_sender, _) = mpsc::unbounded_channel();
        
        let message_bus = MessageBus {
            senders: RwLock::new(HashMap::new()),
            broadcast_sender,
            message_history: RwLock::new(VecDeque::new()),
        };
        
        let default_strategy = CoordinationStrategy {
            name: config.swarm.default_topology.clone(),
            description: "Default coordination strategy".to_string(),
            topology: Self::parse_topology(&config.swarm.default_topology),
            load_balancing: config.swarm.load_balancing_enabled,
            fault_tolerance: true,
            consensus_required: false,
        };
        
        Ok(Self {
            config: config.swarm.clone(),
            active_tasks: RwLock::new(HashMap::new()),
            task_queue: Mutex::new(VecDeque::new()),
            agents: RwLock::new(HashMap::new()),
            message_bus,
            strategy: RwLock::new(default_strategy),
            performance_metrics: RwLock::new(CoordinationMetrics {
                tasks_coordinated: 0,
                average_completion_time: 0.0,
                success_rate: 1.0,
                resource_utilization: 0.0,
                communication_overhead: 0.0,
            }),
        })
    }
    
    pub async fn execute_task(&self, task_description: &str) -> Result<()> {
        info!("Executing task: '{}'", task_description);
        
        // Create task
        let task = self.create_task(task_description).await?;
        let task_id = task.id;
        
        // Add to queue
        self.task_queue.lock().await.push_back(task);
        
        // Start coordination
        self.coordinate_task_execution(task_id).await?;
        
        info!("Task execution initiated: {}", task_id);
        Ok(())
    }
    
    pub async fn execute_with_claude_integration(&self, task_description: &str) -> Result<()> {
        info!("Executing task with Claude integration: '{}'", task_description);
        
        // Create enhanced task with Claude integration metadata
        let mut task = self.create_task(task_description).await?;
        task.metadata.insert(
            "claude_integration".to_string(),
            serde_json::json!(true)
        );
        task.metadata.insert(
            "coordination_mode".to_string(),
            serde_json::json!("claude_enhanced")
        );
        
        let task_id = task.id;
        
        // Add to queue with high priority
        task.priority = TaskPriority::High;
        self.task_queue.lock().await.push_front(task);
        
        // Start enhanced coordination
        self.coordinate_claude_enhanced_execution(task_id).await?;
        
        info!("Claude-enhanced task execution initiated: {}", task_id);
        Ok(())
    }
    
    pub async fn register_agent(&self, agent: Agent) -> Result<()> {
        let agent_id = agent.id;
        debug!("Registering agent for coordination: {}", agent_id);
        
        // Create message channel for agent
        let (tx, mut rx) = mpsc::unbounded_channel();
        self.message_bus.senders.write().await.insert(agent_id, tx);
        
        // Register agent
        self.agents.write().await.insert(agent_id, agent);
        
        // Start message handling for agent
        let message_bus = Arc::new(&self.message_bus);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                // Handle agent-specific messages
                Self::handle_agent_message(message_bus.clone(), message).await;
            }
        });
        
        debug!("Agent registered for coordination: {}", agent_id);
        Ok(())
    }
    
    async fn create_task(&self, description: &str) -> Result<Task> {
        let task_id = Uuid::new_v4();
        let current_time = self.current_timestamp();
        
        Ok(Task {
            id: task_id,
            title: Self::extract_task_title(description),
            description: description.to_string(),
            assigned_to: None,
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            dependencies: Vec::new(),
            created_at: current_time,
            started_at: None,
            completed_at: None,
            estimated_duration: None,
            actual_duration: None,
            metadata: HashMap::new(),
        })
    }
    
    async fn coordinate_task_execution(&self, task_id: Uuid) -> Result<()> {
        debug!("Coordinating task execution: {}", task_id);
        
        // Get task from queue
        let task = {
            let mut queue = self.task_queue.lock().await;
            queue.iter()
                .position(|t| t.id == task_id)
                .and_then(|pos| queue.remove(pos))
        };
        
        let mut task = task.ok_or_else(|| anyhow::anyhow!("Task not found in queue: {}", task_id))?;
        
        // Find suitable agent based on strategy
        let agent_id = self.select_agent_for_task(&task).await?;
        
        // Assign task
        task.assigned_to = Some(agent_id);
        task.status = TaskStatus::Assigned;
        task.started_at = Some(self.current_timestamp());
        
        // Store active task
        self.active_tasks.write().await.insert(task_id, task.clone());
        
        // Send task assignment message
        self.send_task_assignment(agent_id, task).await?;
        
        // Update metrics
        self.update_coordination_metrics().await;
        
        Ok(())
    }
    
    async fn coordinate_claude_enhanced_execution(&self, task_id: Uuid) -> Result<()> {
        info!("Coordinating Claude-enhanced task execution: {}", task_id);
        
        // Enhanced coordination with Claude integration
        // This would include additional steps for Claude API integration
        self.coordinate_task_execution(task_id).await?;
        
        // Additional Claude-specific coordination steps
        // TODO: Implement Claude API integration
        
        Ok(())
    }
    
    async fn select_agent_for_task(&self, task: &Task) -> Result<Uuid> {
        let agents = self.agents.read().await;
        let strategy = self.strategy.read().await;
        
        // Simple agent selection based on strategy
        match strategy.topology {
            TopologyType::Hierarchical => {
                // Find Queen or Coordinator first
                for agent in agents.values() {
                    if matches!(agent.agent_type, AgentType::Queen | AgentType::Coordinator) {
                        return Ok(agent.id);
                    }
                }
                // Fallback to any available agent
                agents.values().next()
                    .map(|agent| agent.id)
                    .ok_or_else(|| anyhow::anyhow!("No agents available"))
            }
            TopologyType::Mesh => {
                // Load balancing across all agents
                // TODO: Implement proper load balancing
                agents.values().next()
                    .map(|agent| agent.id)
                    .ok_or_else(|| anyhow::anyhow!("No agents available"))
            }
            TopologyType::Ring => {
                // Round-robin selection
                // TODO: Implement round-robin logic
                agents.values().next()
                    .map(|agent| agent.id)
                    .ok_or_else(|| anyhow::anyhow!("No agents available"))
            }
            TopologyType::Star => {
                // Central coordinator handles all tasks
                for agent in agents.values() {
                    if matches!(agent.agent_type, AgentType::Coordinator) {
                        return Ok(agent.id);
                    }
                }
                Err(anyhow::anyhow!("No coordinator agent available"))
            }
            TopologyType::Hybrid => {
                // Dynamic selection based on task type and agent capabilities
                // TODO: Implement capability-based selection
                agents.values().next()
                    .map(|agent| agent.id)
                    .ok_or_else(|| anyhow::anyhow!("No agents available"))
            }
        }
    }
    
    async fn send_task_assignment(&self, agent_id: Uuid, task: Task) -> Result<()> {
        let message = CoordinationMessage {
            id: Uuid::new_v4(),
            from_agent: Uuid::new_v4(), // Coordinator ID
            to_agent: Some(agent_id),
            message_type: MessageType::TaskAssignment,
            payload: serde_json::to_value(&task)?,
            timestamp: self.current_timestamp(),
            priority: match task.priority {
                TaskPriority::Low => MessagePriority::Low,
                TaskPriority::Normal => MessagePriority::Normal,
                TaskPriority::High => MessagePriority::High,
                TaskPriority::Critical => MessagePriority::Critical,
            },
        };
        
        self.send_message(message).await?;
        debug!("Task assignment sent to agent: {}", agent_id);
        
        Ok(())
    }
    
    async fn send_message(&self, message: CoordinationMessage) -> Result<()> {
        // Store message in history
        let mut history = self.message_bus.message_history.write().await;
        history.push_back(message.clone());
        
        // Keep only last 1000 messages
        if history.len() > 1000 {
            history.pop_front();
        }
        drop(history);
        
        // Send to specific agent or broadcast
        let senders = self.message_bus.senders.read().await;
        
        if let Some(to_agent) = message.to_agent {
            if let Some(sender) = senders.get(&to_agent) {
                sender.send(message.clone())
                    .map_err(|_| anyhow::anyhow!("Failed to send message to agent: {}", to_agent))?;
            } else {
                warn!("Agent not found for message delivery: {}", to_agent);
            }
        } else {
            // Broadcast to all agents
            for (agent_id, sender) in senders.iter() {
                if let Err(_) = sender.send(message.clone()) {
                    warn!("Failed to send broadcast message to agent: {}", agent_id);
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_agent_message(_message_bus: Arc<&MessageBus>, message: CoordinationMessage) {
        debug!("Handling agent message: {:?}", message.message_type);
        
        match message.message_type {
            MessageType::TaskComplete => {
                // Handle task completion
                info!("Task completed by agent: {}", message.from_agent);
            }
            MessageType::TaskFailed => {
                // Handle task failure
                warn!("Task failed by agent: {}", message.from_agent);
            }
            MessageType::StatusUpdate => {
                // Handle status update
                debug!("Status update from agent: {}", message.from_agent);
            }
            MessageType::Heartbeat => {
                // Handle heartbeat
                debug!("Heartbeat from agent: {}", message.from_agent);
            }
            _ => {
                debug!("Unhandled message type: {:?}", message.message_type);
            }
        }
    }
    
    async fn update_coordination_metrics(&self) {
        let mut metrics = self.performance_metrics.write().await;
        metrics.tasks_coordinated += 1;
        
        // Update other metrics based on active tasks
        let active_tasks = self.active_tasks.read().await;
        let completed_tasks: Vec<&Task> = active_tasks.values()
            .filter(|task| matches!(task.status, TaskStatus::Completed))
            .collect();
        
        if !completed_tasks.is_empty() {
            let total_duration: u64 = completed_tasks.iter()
                .filter_map(|task| task.actual_duration)
                .sum();
            
            metrics.average_completion_time = total_duration as f32 / completed_tasks.len() as f32;
            metrics.success_rate = completed_tasks.len() as f32 / active_tasks.len() as f32;
        }
        
        // Estimate resource utilization
        let agent_count = self.agents.read().await.len();
        if agent_count > 0 {
            metrics.resource_utilization = (active_tasks.len() as f32 / agent_count as f32).min(1.0);
        }
    }
    
    fn parse_topology(topology_str: &str) -> TopologyType {
        match topology_str.to_lowercase().as_str() {
            "hierarchical" => TopologyType::Hierarchical,
            "mesh" => TopologyType::Mesh,
            "ring" => TopologyType::Ring,
            "star" => TopologyType::Star,
            "hybrid" => TopologyType::Hybrid,
            _ => TopologyType::Hierarchical, // Default
        }
    }
    
    fn extract_task_title(description: &str) -> String {
        // Extract a title from the description (first 50 characters)
        let title = description.chars().take(50).collect::<String>();
        if description.len() > 50 {
            format!("{}...", title)
        } else {
            title
        }
    }
    
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}