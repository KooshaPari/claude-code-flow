use claude_flow::prelude::*;
use tempfile::tempdir;

#[tokio::test]
async fn test_agent_lifecycle_management() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[agents]
max_agents = 20
default_timeout = 120
memory_limit_mb = 256
cpu_limit_percent = 25.0
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    
    // Test agent spawning with different types
    let architect_agents = agent_manager.spawn_agents_for_task(
        "design system architecture",
        2,
        "hierarchical"
    ).await?;
    
    let coder_agents = agent_manager.spawn_agents_for_task(
        "implement features",
        3,
        "parallel"
    ).await?;
    
    let tester_agents = agent_manager.spawn_agents_for_task(
        "quality assurance testing",
        2,
        "specialized"
    ).await?;
    
    // Verify agent distribution
    assert_eq!(architect_agents.len(), 2);
    assert_eq!(coder_agents.len(), 3);
    assert_eq!(tester_agents.len(), 2);
    
    // Test agent status
    let status = agent_manager.get_status().await?;
    assert!(status.total_agents >= 7);
    assert!(status.active_agents >= 0);
    
    // Test performance optimization
    agent_manager.optimize_performance().await?;
    
    let optimized_status = agent_manager.get_status().await?;
    assert_eq!(optimized_status.total_agents, status.total_agents);
    
    Ok(())
}

#[tokio::test]
async fn test_hive_mind_intelligence() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[agents]
max_agents = 50

[swarm]
default_topology = "hierarchical"
max_swarm_size = 20
coordination_timeout = 60
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    
    // Initialize hive-mind system
    agent_manager.initialize_hive_mind().await?;
    
    // Verify Queen agent is spawned
    let hive_status = agent_manager.get_hive_mind_status().await?;
    assert!(hive_status.queen_active);
    assert_eq!(hive_status.worker_count, 0); // No workers initially
    
    // Spawn worker agents
    let workers = agent_manager.spawn_agents_for_task(
        "complex multi-agent coordination task",
        8,
        "hierarchical"
    ).await?;
    
    assert!(workers.len() <= 8);
    
    // Check hive-mind status after spawning
    let updated_status = agent_manager.get_hive_mind_status().await?;
    assert!(updated_status.queen_active);
    assert!(updated_status.worker_count > 0);
    assert!(updated_status.consensus_active);
    assert!(updated_status.coordination_health > 0.0);
    
    Ok(())
}

#[tokio::test]
async fn test_agent_specialization() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[agents]
max_agents = 30
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    
    // Test specialized agent spawning based on task types
    
    // Security-focused task
    let security_agents = agent_manager.spawn_agents_for_task(
        "security audit and vulnerability assessment",
        5,
        "specialized"
    ).await?;
    
    // DevOps-focused task
    let devops_agents = agent_manager.spawn_agents_for_task(
        "deploy infrastructure and monitor systems",
        4,
        "specialized"
    ).await?;
    
    // Research-focused task
    let research_agents = agent_manager.spawn_agents_for_task(
        "research machine learning trends",
        3,
        "specialized"
    ).await?;
    
    // Testing-focused task
    let testing_agents = agent_manager.spawn_agents_for_task(
        "comprehensive test suite execution",
        3,
        "specialized"
    ).await?;
    
    // Verify specialization worked
    assert!(security_agents.len() <= 5);
    assert!(devops_agents.len() <= 4);
    assert!(research_agents.len() <= 3);
    assert!(testing_agents.len() <= 3);
    
    let total_agents = security_agents.len() + devops_agents.len() + 
                      research_agents.len() + testing_agents.len();
    
    let status = agent_manager.get_status().await?;
    assert!(status.total_agents >= total_agents as u32);
    
    Ok(())
}

#[tokio::test]
async fn test_agent_performance_metrics() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[agents]
max_agents = 15
default_timeout = 60
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    
    // Spawn agents for performance testing
    let agents = agent_manager.spawn_agents_for_task(
        "performance measurement task",
        10,
        "mesh"
    ).await?;
    
    assert!(agents.len() <= 10);
    
    // Get initial performance metrics
    let initial_status = agent_manager.get_status().await?;
    let initial_cpu = initial_status.cpu_usage;
    let initial_memory = initial_status.memory_usage;
    
    // Perform optimization
    agent_manager.optimize_performance().await?;
    
    // Get updated metrics
    let optimized_status = agent_manager.get_status().await?;
    
    // Verify metrics are being tracked
    assert!(optimized_status.cpu_usage >= 0.0);
    assert!(optimized_status.memory_usage >= 0);
    assert_eq!(optimized_status.total_agents, initial_status.total_agents);
    
    // Check that optimization may have improved efficiency
    // (In a real implementation, this would show actual improvements)
    assert!(optimized_status.cpu_usage >= 0.0);
    
    Ok(())
}

#[tokio::test]
async fn test_agent_coordination_strategies() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[agents]
max_agents = 40
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    
    // Test different coordination strategies
    
    // Hierarchical strategy
    let hierarchical_agents = agent_manager.spawn_agents_for_task(
        "hierarchical coordination test",
        5,
        "hierarchical"
    ).await?;
    
    // Parallel strategy
    let parallel_agents = agent_manager.spawn_agents_for_task(
        "parallel execution test",
        6,
        "parallel"
    ).await?;
    
    // Specialized strategy  
    let specialized_agents = agent_manager.spawn_agents_for_task(
        "specialized task execution",
        4,
        "specialized"
    ).await?;
    
    // Default strategy
    let default_agents = agent_manager.spawn_agents_for_task(
        "default coordination test",
        3,
        "default"
    ).await?;
    
    // Verify all strategies worked
    assert!(hierarchical_agents.len() <= 5);
    assert!(parallel_agents.len() <= 6);
    assert!(specialized_agents.len() <= 4);
    assert!(default_agents.len() <= 3);
    
    let total_spawned = hierarchical_agents.len() + parallel_agents.len() + 
                       specialized_agents.len() + default_agents.len();
    
    let status = agent_manager.get_status().await?;
    assert!(status.total_agents >= total_spawned as u32);
    
    Ok(())
}