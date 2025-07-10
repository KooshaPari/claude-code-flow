use criterion::{black_box, criterion_group, criterion_main, Criterion};
use claude_flow::prelude::*;
use tempfile::tempdir;
use tokio::runtime::Runtime;

fn bench_agent_spawning(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("agent_spawning_10", |b| {
        b.iter(|| {
            rt.block_on(async {
                let temp_dir = tempdir().unwrap();
                let config_path = temp_dir.path().join("config.toml");
                
                let config_content = r#"
[agents]
max_agents = 50
"#;
                
                tokio::fs::write(&config_path, config_content).await.unwrap();
                let config = Config::load(&config_path).await.unwrap();
                
                let agent_manager = AgentManager::new(&config).await.unwrap();
                
                let agents = agent_manager.spawn_agents_for_task(
                    black_box("benchmark task"),
                    black_box(10),
                    black_box("hierarchical")
                ).await.unwrap();
                
                black_box(agents);
            })
        })
    });
}

fn bench_memory_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("memory_store_retrieve_100", |b| {
        b.iter(|| {
            rt.block_on(async {
                let temp_dir = tempdir().unwrap();
                let config_path = temp_dir.path().join("config.toml");
                
                let config_content = r#"
[memory]
backend = "memory"
max_entries = 10000
"#;
                
                tokio::fs::write(&config_path, config_content).await.unwrap();
                let config = Config::load(&config_path).await.unwrap();
                
                let memory_manager = MemoryManager::new(&config).await.unwrap();
                memory_manager.initialize().await.unwrap();
                
                for i in 0..100 {
                    let key = format!("bench_key_{}", i);
                    let value = format!("bench_value_{}", i);
                    
                    memory_manager.store(
                        black_box(&key),
                        black_box(&value),
                        black_box("benchmark")
                    ).await.unwrap();
                    
                    let entry = memory_manager.retrieve(
                        black_box("benchmark"),
                        black_box(&key)
                    ).await.unwrap();
                    
                    black_box(entry);
                }
            })
        })
    });
}

fn bench_task_coordination(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("task_coordination_50", |b| {
        b.iter(|| {
            rt.block_on(async {
                let temp_dir = tempdir().unwrap();
                let config_path = temp_dir.path().join("config.toml");
                
                let config_content = r#"
[core]
max_concurrent_tasks = 50

[swarm]
default_topology = "mesh"
coordination_timeout = 30
"#;
                
                tokio::fs::write(&config_path, config_content).await.unwrap();
                let config = Config::load(&config_path).await.unwrap();
                
                let coordinator = Coordinator::new(&config).await.unwrap();
                
                for i in 0..50 {
                    coordinator.execute_task(
                        black_box(&format!("benchmark task {}", i))
                    ).await.unwrap();
                }
            })
        })
    });
}

fn bench_swarm_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("swarm_initialization", |b| {
        b.iter(|| {
            rt.block_on(async {
                let temp_dir = tempdir().unwrap();
                let config_path = temp_dir.path().join("config.toml");
                
                let config_content = r#"
[swarm]
default_topology = "hierarchical"
max_swarm_size = 20
"#;
                
                tokio::fs::write(&config_path, config_content).await.unwrap();
                let config = Config::load(&config_path).await.unwrap();
                
                let swarm_orchestrator = SwarmOrchestrator::new(&config).await.unwrap();
                swarm_orchestrator.initialize().await.unwrap();
                
                let swarm_id = swarm_orchestrator.initialize_swarm(
                    black_box("mesh"),
                    black_box(10)
                ).await.unwrap();
                
                swarm_orchestrator.execute_task(
                    black_box("benchmark swarm task"),
                    black_box("parallel")
                ).await.unwrap();
                
                black_box(swarm_id);
            })
        })
    });
}

fn bench_hive_mind_coordination(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("hive_mind_coordination", |b| {
        b.iter(|| {
            rt.block_on(async {
                let temp_dir = tempdir().unwrap();
                let config_path = temp_dir.path().join("config.toml");
                
                let config_content = r#"
[agents]
max_agents = 30

[swarm]
default_topology = "hierarchical"
"#;
                
                tokio::fs::write(&config_path, config_content).await.unwrap();
                let config = Config::load(&config_path).await.unwrap();
                
                let agent_manager = AgentManager::new(&config).await.unwrap();
                let coordinator = Coordinator::new(&config).await.unwrap();
                
                agent_manager.initialize_hive_mind().await.unwrap();
                
                let agents = agent_manager.spawn_agents_for_task(
                    black_box("hive coordination benchmark"),
                    black_box(8),
                    black_box("hierarchical")
                ).await.unwrap();
                
                coordinator.execute_with_claude_integration(
                    black_box("claude integration benchmark")
                ).await.unwrap();
                
                black_box(agents);
            })
        })
    });
}

fn bench_memory_query_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("memory_query_1000_entries", |b| {
        b.iter(|| {
            rt.block_on(async {
                let temp_dir = tempdir().unwrap();
                let config_path = temp_dir.path().join("config.toml");
                
                let config_content = r#"
[memory]
backend = "memory"
max_entries = 10000
"#;
                
                tokio::fs::write(&config_path, config_content).await.unwrap();
                let config = Config::load(&config_path).await.unwrap();
                
                let memory_manager = MemoryManager::new(&config).await.unwrap();
                memory_manager.initialize().await.unwrap();
                
                // Pre-populate memory
                for i in 0..1000 {
                    memory_manager.store(
                        &format!("query_key_{}", i),
                        &format!("query_value_{}", i),
                        "query_bench"
                    ).await.unwrap();
                }
                
                // Benchmark queries
                let results = memory_manager.query(
                    black_box("query_*"),
                    black_box(Some("query_bench"))
                ).await.unwrap();
                
                black_box(results);
            })
        })
    });
}

criterion_group!(
    benches,
    bench_agent_spawning,
    bench_memory_operations,
    bench_task_coordination,
    bench_swarm_operations,
    bench_hive_mind_coordination,
    bench_memory_query_performance
);

criterion_main!(benches);