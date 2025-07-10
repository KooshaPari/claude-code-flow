// Integration tests for Claude Flow 2.0
use claude_flow::prelude::*;
use tempfile::tempdir;
use tokio::test;

mod agent_tests;
mod coordination_tests;
mod memory_tests;
mod swarm_tests;

#[test]
async fn test_full_system_integration() -> Result<()> {
    // Create temporary config
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    // Write test config
    let config_content = r#"
[core]
log_level = "debug"
max_concurrent_tasks = 5
session_timeout = 300

[agents]
max_agents = 10
default_timeout = 60

[memory]
backend = "memory"
max_entries = 1000

[swarm]
default_topology = "hierarchical"
max_swarm_size = 5
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    
    // Load config
    let config = Config::load(&config_path).await?;
    
    // Initialize core system
    let core = Core::new(&config).await?;
    core.initialize(false).await?;
    
    // Initialize agent manager
    let agent_manager = AgentManager::new(&config).await?;
    agent_manager.initialize_hive_mind().await?;
    
    // Initialize coordinator
    let coordinator = Coordinator::new(&config).await?;
    
    // Initialize memory manager
    let memory_manager = MemoryManager::new(&config).await?;
    memory_manager.initialize().await?;
    
    // Initialize swarm orchestrator
    let swarm_orchestrator = SwarmOrchestrator::new(&config).await?;
    swarm_orchestrator.initialize().await?;
    
    // Test basic operations
    
    // 1. Test memory operations
    memory_manager.store("test_key", "test_value", "default").await?;
    let entry = memory_manager.retrieve("default", "test_key").await?;
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().value, "test_value");
    
    // 2. Test agent spawning
    let agents = agent_manager.spawn_agents_for_task("test task", 3, "hierarchical").await?;
    assert_eq!(agents.len(), 3);
    
    // 3. Test coordination
    coordinator.execute_task("integration test task").await?;
    
    // 4. Test swarm operations
    let swarm_id = swarm_orchestrator.initialize_swarm("mesh", 5).await?;
    swarm_orchestrator.execute_task("swarm test task", "parallel").await?;
    
    // 5. Verify system status
    let core_status = core.get_status().await?;
    assert!(core_status.initialized);
    
    let agent_status = agent_manager.get_status().await?;
    assert!(agent_status.total_agents > 0);
    
    let memory_stats = memory_manager.get_stats().await?;
    assert!(memory_stats.total_entries > 0);
    
    Ok(())
}

#[test]
async fn test_hive_mind_coordination() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[core]
log_level = "info"

[agents]
max_agents = 50

[memory]
backend = "memory"

[swarm]
default_topology = "hierarchical"
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    // Initialize systems
    let agent_manager = AgentManager::new(&config).await?;
    agent_manager.initialize_hive_mind().await?;
    
    let coordinator = Coordinator::new(&config).await?;
    let swarm_orchestrator = SwarmOrchestrator::new(&config).await?;
    swarm_orchestrator.initialize().await?;
    
    // Test hive-mind coordination
    let agents = agent_manager.spawn_agents_for_task(
        "complex coordination task",
        8,
        "hierarchical"
    ).await?;
    
    assert!(agents.len() <= 8);
    
    // Verify Queen agent exists
    let hive_status = agent_manager.get_hive_mind_status().await?;
    assert!(hive_status.queen_active);
    assert!(hive_status.worker_count > 0);
    assert!(hive_status.consensus_active);
    
    // Test coordination execution
    coordinator.execute_with_claude_integration("hive-mind test task").await?;
    
    // Verify swarm status
    let swarm_status = swarm_orchestrator.get_status().await?;
    assert!(swarm_status.health_score > 0.0);
    
    Ok(())
}

#[test]
async fn test_performance_benchmarks() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[core]
log_level = "warn"
max_concurrent_tasks = 20

[agents]
max_agents = 100

[memory]
backend = "memory"
max_entries = 10000

[swarm]
default_topology = "mesh"
max_swarm_size = 20
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    // Initialize systems
    let memory_manager = MemoryManager::new(&config).await?;
    memory_manager.initialize().await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    let coordinator = Coordinator::new(&config).await?;
    
    // Performance benchmarks
    
    // 1. Memory throughput test
    let start_time = std::time::Instant::now();
    
    for i in 0..1000 {
        memory_manager.store(
            &format!("key_{}", i),
            &format!("value_{}", i),
            "benchmark"
        ).await?;
    }
    
    let memory_duration = start_time.elapsed();
    println!("Memory throughput: {} ops/sec", 
             1000.0 / memory_duration.as_secs_f64());
    
    // 2. Agent spawning test
    let start_time = std::time::Instant::now();
    
    let agents = agent_manager.spawn_agents_for_task(
        "performance test",
        20,
        "mesh"
    ).await?;
    
    let agent_duration = start_time.elapsed();
    println!("Agent spawning: {} agents/sec", 
             agents.len() as f64 / agent_duration.as_secs_f64());
    
    // 3. Task coordination test
    let start_time = std::time::Instant::now();
    
    for i in 0..100 {
        coordinator.execute_task(&format!("benchmark task {}", i)).await?;
    }
    
    let coordination_duration = start_time.elapsed();
    println!("Task coordination: {} tasks/sec", 
             100.0 / coordination_duration.as_secs_f64());
    
    // Verify performance requirements
    assert!(memory_duration.as_millis() < 5000); // <5s for 1000 operations
    assert!(agent_duration.as_millis() < 2000);  // <2s for 20 agents
    assert!(coordination_duration.as_millis() < 10000); // <10s for 100 tasks
    
    Ok(())
}

#[test]
async fn test_error_handling_and_recovery() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[core]
log_level = "debug"

[agents]
max_agents = 5

[memory]
backend = "memory"

[swarm]
default_topology = "star"
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    let memory_manager = MemoryManager::new(&config).await?;
    memory_manager.initialize().await?;
    
    // Test error recovery scenarios
    
    // 1. Test invalid memory operations
    let result = memory_manager.retrieve("nonexistent", "invalid_key").await;
    assert!(result.is_ok()); // Should handle gracefully
    
    // 2. Test agent limits
    let result = agent_manager.spawn_agents_for_task(
        "overflow test",
        1000, // Exceeds max_agents
        "hierarchical"
    ).await;
    assert!(result.is_ok()); // Should handle gracefully with limits
    
    // 3. Test configuration validation
    let validation = config.validate().await?;
    assert!(validation.valid || !validation.errors.is_empty());
    
    // 4. Test system status under stress
    let core = Core::new(&config).await?;
    core.initialize(false).await?;
    
    let status = core.get_status().await?;
    assert!(status.initialized);
    
    Ok(())
}

#[test]
async fn test_concurrent_operations() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
[core]
log_level = "info"
max_concurrent_tasks = 50

[agents]
max_agents = 50

[memory]
backend = "memory"

[swarm]
default_topology = "mesh"
"#;
    
    tokio::fs::write(&config_path, config_content).await?;
    let config = Config::load(&config_path).await?;
    
    let memory_manager = MemoryManager::new(&config).await?;
    memory_manager.initialize().await?;
    
    let agent_manager = AgentManager::new(&config).await?;
    
    // Test concurrent operations
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent memory operations
    for i in 0..10 {
        let memory_manager = memory_manager.clone();
        let handle = tokio::spawn(async move {
            for j in 0..10 {
                let key = format!("concurrent_{}_{}", i, j);
                let value = format!("value_{}_{}", i, j);
                memory_manager.store(&key, &value, "concurrent").await.unwrap();
                
                let entry = memory_manager.retrieve("concurrent", &key).await.unwrap();
                assert!(entry.is_some());
                assert_eq!(entry.unwrap().value, value);
            }
        });
        handles.push(handle);
    }
    
    // Spawn concurrent agent operations
    for i in 0..5 {
        let agent_manager = agent_manager.clone();
        let handle = tokio::spawn(async move {
            let agents = agent_manager.spawn_agents_for_task(
                &format!("concurrent task {}", i),
                3,
                "mesh"
            ).await.unwrap();
            assert!(agents.len() <= 3);
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await?;
    }
    
    // Verify final state
    let memory_stats = memory_manager.get_stats().await?;
    assert!(memory_stats.total_entries >= 100); // 10*10 concurrent stores
    
    let agent_status = agent_manager.get_status().await?;
    assert!(agent_status.total_agents > 0);
    
    Ok(())
}