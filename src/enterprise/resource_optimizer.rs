//! Resource Allocation Optimizer
//! 
//! Optimizes resource allocation across teams and projects

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use super::{EnterpriseConfig, ResourceLimits, ResourceUtilization};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePool {
    pub id: Uuid,
    pub name: String,
    pub resource_type: ResourceType,
    pub total_capacity: f64,
    pub available_capacity: f64,
    pub allocated_resources: HashMap<Uuid, ResourceAllocation>,
    pub utilization_history: Vec<UtilizationRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Compute { cpu_cores: u32, memory_gb: f64 },
    Storage { capacity_gb: f64, iops: u32 },
    Network { bandwidth_mbps: f64, latency_ms: f64 },
    Human { skill_set: Vec<String>, availability_hours: f64 },
    Budget { amount_usd: f64, currency: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub task_id: Uuid,
    pub team_id: Uuid,
    pub allocated_amount: f64,
    pub allocation_start: chrono::DateTime<chrono::Utc>,
    pub allocation_end: chrono::DateTime<chrono::Utc>,
    pub priority: AllocationPriority,
    pub efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilizationRecord {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub utilization_percent: f64,
    pub efficiency_score: f64,
    pub bottlenecks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStrategy {
    pub strategy_type: StrategyType,
    pub parameters: HashMap<String, f64>,
    pub constraints: Vec<OptimizationConstraint>,
    pub objectives: Vec<OptimizationObjective>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    LoadBalancing,
    CostOptimization,
    PerformanceMaximization,
    ResourceMinimization,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConstraint {
    pub constraint_type: ConstraintType,
    pub limit: f64,
    pub hard_constraint: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    MaxCpuUtilization,
    MaxMemoryUtilization,
    MaxBudget,
    MinTeamSize,
    MaxTaskDuration,
    ComplianceRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationObjective {
    pub objective_type: ObjectiveType,
    pub weight: f64,
    pub target_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    MinimizeCost,
    MaximizePerformance,
    MaximizeUtilization,
    MinimizeLatency,
    MaximizeQuality,
    MinimizeRisk,
}

pub struct ResourceOptimizer {
    config: EnterpriseConfig,
    resource_pools: RwLock<HashMap<Uuid, ResourcePool>>,
    optimization_strategies: RwLock<HashMap<String, OptimizationStrategy>>,
    allocation_history: RwLock<Vec<ResourceAllocation>>,
}

impl ResourceOptimizer {
    pub fn new(config: &EnterpriseConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            resource_pools: RwLock::new(HashMap::new()),
            optimization_strategies: RwLock::new(HashMap::new()),
            allocation_history: RwLock::new(Vec::new()),
        })
    }

    pub async fn configure_limits(&self) -> Result<()> {
        self.initialize_resource_pools().await?;
        self.setup_optimization_strategies().await?;
        self.start_monitoring_loop().await?;
        Ok(())
    }

    async fn initialize_resource_pools(&self) -> Result<()> {
        let mut pools = self.resource_pools.write().await;
        
        // Compute Pool
        let compute_pool = ResourcePool {
            id: Uuid::new_v4(),
            name: "Enterprise Compute Pool".to_string(),
            resource_type: ResourceType::Compute {
                cpu_cores: 128,
                memory_gb: 512.0,
            },
            total_capacity: 100.0,
            available_capacity: 100.0,
            allocated_resources: HashMap::new(),
            utilization_history: Vec::new(),
        };

        // Storage Pool
        let storage_pool = ResourcePool {
            id: Uuid::new_v4(),
            name: "Enterprise Storage Pool".to_string(),
            resource_type: ResourceType::Storage {
                capacity_gb: 10000.0,
                iops: 50000,
            },
            total_capacity: 100.0,
            available_capacity: 100.0,
            allocated_resources: HashMap::new(),
            utilization_history: Vec::new(),
        };

        // Network Pool
        let network_pool = ResourcePool {
            id: Uuid::new_v4(),
            name: "Enterprise Network Pool".to_string(),
            resource_type: ResourceType::Network {
                bandwidth_mbps: 10000.0,
                latency_ms: 1.0,
            },
            total_capacity: 100.0,
            available_capacity: 100.0,
            allocated_resources: HashMap::new(),
            utilization_history: Vec::new(),
        };

        // Human Resources Pool
        let human_pool = ResourcePool {
            id: Uuid::new_v4(),
            name: "Engineering Team Pool".to_string(),
            resource_type: ResourceType::Human {
                skill_set: vec![
                    "Rust".to_string(),
                    "Go".to_string(),
                    "TypeScript".to_string(),
                    "React".to_string(),
                    "Docker".to_string(),
                    "Kubernetes".to_string(),
                ],
                availability_hours: 320.0, // 8 people * 40 hours
            },
            total_capacity: 100.0,
            available_capacity: 100.0,
            allocated_resources: HashMap::new(),
            utilization_history: Vec::new(),
        };

        // Budget Pool
        let budget_pool = ResourcePool {
            id: Uuid::new_v4(),
            name: "Project Budget Pool".to_string(),
            resource_type: ResourceType::Budget {
                amount_usd: 1000000.0,
                currency: "USD".to_string(),
            },
            total_capacity: 100.0,
            available_capacity: 100.0,
            allocated_resources: HashMap::new(),
            utilization_history: Vec::new(),
        };

        pools.insert(compute_pool.id, compute_pool);
        pools.insert(storage_pool.id, storage_pool);
        pools.insert(network_pool.id, network_pool);
        pools.insert(human_pool.id, human_pool);
        pools.insert(budget_pool.id, budget_pool);

        Ok(())
    }

    async fn setup_optimization_strategies(&self) -> Result<()> {
        let mut strategies = self.optimization_strategies.write().await;
        
        // Load Balancing Strategy
        let load_balancing = OptimizationStrategy {
            strategy_type: StrategyType::LoadBalancing,
            parameters: {
                let mut params = HashMap::new();
                params.insert("max_utilization".to_string(), 80.0);
                params.insert("rebalance_threshold".to_string(), 20.0);
                params
            },
            constraints: vec![
                OptimizationConstraint {
                    constraint_type: ConstraintType::MaxCpuUtilization,
                    limit: self.config.resource_limits.max_cpu_percent,
                    hard_constraint: true,
                },
                OptimizationConstraint {
                    constraint_type: ConstraintType::MaxMemoryUtilization,
                    limit: 85.0,
                    hard_constraint: true,
                },
            ],
            objectives: vec![
                OptimizationObjective {
                    objective_type: ObjectiveType::MaximizeUtilization,
                    weight: 0.6,
                    target_value: Some(75.0),
                },
                OptimizationObjective {
                    objective_type: ObjectiveType::MinimizeLatency,
                    weight: 0.4,
                    target_value: Some(100.0), // 100ms
                },
            ],
        };

        // Cost Optimization Strategy
        let cost_optimization = OptimizationStrategy {
            strategy_type: StrategyType::CostOptimization,
            parameters: {
                let mut params = HashMap::new();
                params.insert("cost_threshold".to_string(), 0.8);
                params.insert("efficiency_weight".to_string(), 0.7);
                params
            },
            constraints: vec![
                OptimizationConstraint {
                    constraint_type: ConstraintType::MaxBudget,
                    limit: 800000.0, // 80% of budget
                    hard_constraint: true,
                },
            ],
            objectives: vec![
                OptimizationObjective {
                    objective_type: ObjectiveType::MinimizeCost,
                    weight: 0.8,
                    target_value: None,
                },
                OptimizationObjective {
                    objective_type: ObjectiveType::MaximizeQuality,
                    weight: 0.2,
                    target_value: Some(4.0),
                },
            ],
        };

        // Performance Maximization Strategy
        let performance_max = OptimizationStrategy {
            strategy_type: StrategyType::PerformanceMaximization,
            parameters: {
                let mut params = HashMap::new();
                params.insert("performance_weight".to_string(), 0.9);
                params.insert("cost_weight".to_string(), 0.1);
                params
            },
            constraints: vec![
                OptimizationConstraint {
                    constraint_type: ConstraintType::MinTeamSize,
                    limit: 3.0,
                    hard_constraint: true,
                },
            ],
            objectives: vec![
                OptimizationObjective {
                    objective_type: ObjectiveType::MaximizePerformance,
                    weight: 0.9,
                    target_value: None,
                },
                OptimizationObjective {
                    objective_type: ObjectiveType::MinimizeRisk,
                    weight: 0.1,
                    target_value: Some(0.2),
                },
            ],
        };

        strategies.insert("load_balancing".to_string(), load_balancing);
        strategies.insert("cost_optimization".to_string(), cost_optimization);
        strategies.insert("performance_maximization".to_string(), performance_max);

        Ok(())
    }

    async fn start_monitoring_loop(&self) -> Result<()> {
        let pools = self.resource_pools.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // 1 minute
            
            loop {
                interval.tick().await;
                
                let pools_guard = pools.read().await;
                for pool in pools_guard.values() {
                    let utilization = 100.0 - pool.available_capacity;
                    
                    if utilization > 90.0 {
                        tracing::warn!(
                            "High resource utilization detected in pool {}: {:.1}%",
                            pool.name,
                            utilization
                        );
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn allocate_resources_for_task(&self, task_id: &Uuid, task_definition: &str) -> Result<String> {
        let resource_requirements = self.analyze_resource_requirements(task_definition).await?;
        let optimal_allocation = self.optimize_allocation(&resource_requirements).await?;
        
        // Apply allocation
        self.apply_allocation(task_id, &optimal_allocation).await?;
        
        Ok(format!("Allocated resources for task {}: {:?}", task_id, optimal_allocation))
    }

    async fn analyze_resource_requirements(&self, task_definition: &str) -> Result<HashMap<String, f64>> {
        let mut requirements = HashMap::new();
        
        // AI-powered analysis of task requirements
        if task_definition.contains("machine learning") || task_definition.contains("AI") {
            requirements.insert("compute".to_string(), 60.0); // High compute
            requirements.insert("memory".to_string(), 80.0);  // High memory
            requirements.insert("storage".to_string(), 40.0); // Medium storage
        } else if task_definition.contains("database") || task_definition.contains("storage") {
            requirements.insert("compute".to_string(), 30.0); // Medium compute
            requirements.insert("memory".to_string(), 50.0);  // Medium memory
            requirements.insert("storage".to_string(), 80.0); // High storage
        } else if task_definition.contains("web") || task_definition.contains("frontend") {
            requirements.insert("compute".to_string(), 25.0); // Low compute
            requirements.insert("memory".to_string(), 30.0);  // Low memory
            requirements.insert("network".to_string(), 60.0); // Medium network
        } else {
            // Default allocation
            requirements.insert("compute".to_string(), 40.0);
            requirements.insert("memory".to_string(), 40.0);
            requirements.insert("storage".to_string(), 20.0);
            requirements.insert("network".to_string(), 20.0);
        }

        // Always need human resources
        requirements.insert("human".to_string(), 30.0);
        requirements.insert("budget".to_string(), 25.0);

        Ok(requirements)
    }

    async fn optimize_allocation(&self, requirements: &HashMap<String, f64>) -> Result<HashMap<String, ResourceAllocation>> {
        let mut allocation = HashMap::new();
        let pools = self.resource_pools.read().await;
        
        for (resource_type, required_amount) in requirements {
            // Find best pool for this resource type
            if let Some((pool_id, pool)) = self.find_best_pool(&pools, resource_type, *required_amount).await? {
                let allocation_id = Uuid::new_v4();
                
                let resource_allocation = ResourceAllocation {
                    task_id: Uuid::new_v4(), // Will be updated when applied
                    team_id: Uuid::new_v4(), // Will be determined by team coordinator
                    allocated_amount: *required_amount,
                    allocation_start: chrono::Utc::now(),
                    allocation_end: chrono::Utc::now() + chrono::Duration::hours(8), // Default 8 hours
                    priority: AllocationPriority::Medium,
                    efficiency_score: self.calculate_efficiency_score(pool, *required_amount).await?,
                };
                
                allocation.insert(pool_id.to_string(), resource_allocation);
            }
        }
        
        Ok(allocation)
    }

    async fn find_best_pool(&self, pools: &HashMap<Uuid, ResourcePool>, resource_type: &str, required_amount: f64) -> Result<Option<(Uuid, &ResourcePool)>> {
        let mut best_pool = None;
        let mut best_score = 0.0;
        
        for (pool_id, pool) in pools {
            if self.pool_matches_type(pool, resource_type) && pool.available_capacity >= required_amount {
                let score = self.calculate_pool_score(pool, required_amount).await?;
                if score > best_score {
                    best_score = score;
                    best_pool = Some((*pool_id, pool));
                }
            }
        }
        
        Ok(best_pool)
    }

    fn pool_matches_type(&self, pool: &ResourcePool, resource_type: &str) -> bool {
        match (&pool.resource_type, resource_type) {
            (ResourceType::Compute { .. }, "compute") => true,
            (ResourceType::Storage { .. }, "storage") => true,
            (ResourceType::Network { .. }, "network") => true,
            (ResourceType::Human { .. }, "human") => true,
            (ResourceType::Budget { .. }, "budget") => true,
            _ => false,
        }
    }

    async fn calculate_pool_score(&self, pool: &ResourcePool, required_amount: f64) -> Result<f64> {
        // Score based on:
        // - Available capacity
        // - Historical efficiency
        // - Current utilization
        
        let capacity_score = pool.available_capacity / 100.0;
        let utilization_score = 1.0 - ((100.0 - pool.available_capacity) / 100.0);
        
        let efficiency_score = if !pool.utilization_history.is_empty() {
            let recent_efficiency: f64 = pool.utilization_history
                .iter()
                .rev()
                .take(10)
                .map(|r| r.efficiency_score)
                .sum::<f64>() / pool.utilization_history.len().min(10) as f64;
            recent_efficiency
        } else {
            0.8 // Default efficiency
        };

        // Weighted score
        let score = (capacity_score * 0.4) + (utilization_score * 0.3) + (efficiency_score * 0.3);
        
        Ok(score)
    }

    async fn calculate_efficiency_score(&self, pool: &ResourcePool, allocated_amount: f64) -> Result<f64> {
        // Calculate expected efficiency based on allocation size and pool characteristics
        let utilization_after_allocation = (100.0 - pool.available_capacity + allocated_amount) / 100.0;
        
        // Optimal utilization is around 70-80%
        let efficiency = if utilization_after_allocation < 0.5 {
            utilization_after_allocation * 1.5 // Under-utilized penalty
        } else if utilization_after_allocation > 0.9 {
            1.0 - (utilization_after_allocation - 0.9) * 2.0 // Over-utilization penalty
        } else {
            0.8 + (utilization_after_allocation - 0.5) * 0.5 // Sweet spot bonus
        };
        
        Ok(efficiency.max(0.0).min(1.0))
    }

    async fn apply_allocation(&self, task_id: &Uuid, allocation: &HashMap<String, ResourceAllocation>) -> Result<()> {
        let mut pools = self.resource_pools.write().await;
        let mut history = self.allocation_history.write().await;
        
        for (pool_id_str, resource_allocation) in allocation {
            if let Ok(pool_id) = pool_id_str.parse::<Uuid>() {
                if let Some(pool) = pools.get_mut(&pool_id) {
                    // Update pool availability
                    pool.available_capacity -= resource_allocation.allocated_amount;
                    
                    // Add to pool's allocations
                    let mut updated_allocation = resource_allocation.clone();
                    updated_allocation.task_id = *task_id;
                    pool.allocated_resources.insert(resource_allocation.task_id, updated_allocation.clone());
                    
                    // Add to history
                    history.push(updated_allocation);
                    
                    // Record utilization
                    let utilization_record = UtilizationRecord {
                        timestamp: chrono::Utc::now(),
                        utilization_percent: 100.0 - pool.available_capacity,
                        efficiency_score: resource_allocation.efficiency_score,
                        bottlenecks: Vec::new(),
                    };
                    pool.utilization_history.push(utilization_record);
                }
            }
        }
        
        Ok(())
    }

    pub async fn get_utilization(&self) -> Result<ResourceUtilization> {
        let pools = self.resource_pools.read().await;
        
        let mut cpu_usage = 0.0;
        let mut memory_usage = 0.0;
        let mut storage_usage = 0.0;
        let mut network_usage = 0.0;
        
        let mut cpu_pools = 0;
        let mut memory_pools = 0;
        let mut storage_pools = 0;
        let mut network_pools = 0;
        
        for pool in pools.values() {
            let utilization = 100.0 - pool.available_capacity;
            
            match &pool.resource_type {
                ResourceType::Compute { .. } => {
                    cpu_usage += utilization;
                    memory_usage += utilization;
                    cpu_pools += 1;
                    memory_pools += 1;
                }
                ResourceType::Storage { .. } => {
                    storage_usage += utilization;
                    storage_pools += 1;
                }
                ResourceType::Network { .. } => {
                    network_usage += utilization;
                    network_pools += 1;
                }
                _ => {}
            }
        }
        
        Ok(ResourceUtilization {
            cpu_usage: if cpu_pools > 0 { cpu_usage / cpu_pools as f64 } else { 0.0 },
            memory_usage: if memory_pools > 0 { memory_usage / memory_pools as f64 } else { 0.0 },
            storage_usage: if storage_pools > 0 { storage_usage / storage_pools as f64 } else { 0.0 },
            network_usage: if network_pools > 0 { network_usage / network_pools as f64 } else { 0.0 },
        })
    }

    pub async fn optimize_current_allocations(&self) -> Result<()> {
        let strategies = self.optimization_strategies.read().await;
        let load_balancing = strategies.get("load_balancing")
            .ok_or_else(|| anyhow::anyhow!("Load balancing strategy not found"))?;
        
        // Implement load balancing optimization
        self.rebalance_resources(load_balancing).await?;
        
        Ok(())
    }

    async fn rebalance_resources(&self, strategy: &OptimizationStrategy) -> Result<()> {
        let mut pools = self.resource_pools.write().await;
        
        // Find overutilized and underutilized pools
        let mut rebalance_candidates = Vec::new();
        
        for (pool_id, pool) in pools.iter() {
            let utilization = 100.0 - pool.available_capacity;
            let max_util = strategy.parameters.get("max_utilization").unwrap_or(&80.0);
            let threshold = strategy.parameters.get("rebalance_threshold").unwrap_or(&20.0);
            
            if utilization > *max_util {
                rebalance_candidates.push((*pool_id, utilization, "overutilized"));
            } else if utilization < threshold {
                rebalance_candidates.push((*pool_id, utilization, "underutilized"));
            }
        }
        
        // Log rebalancing recommendations
        for (pool_id, utilization, status) in rebalance_candidates {
            if let Some(pool) = pools.get(&pool_id) {
                tracing::info!(
                    "Rebalancing recommendation for pool '{}': {} at {:.1}% utilization",
                    pool.name,
                    status,
                    utilization
                );
            }
        }
        
        Ok(())
    }
}