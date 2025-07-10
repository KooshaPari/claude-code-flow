// Production Metrics and Observability
// Comprehensive metrics collection for enterprise monitoring

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicF64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub system: SystemMetrics,
    pub application: ApplicationMetrics,
    pub business: BusinessMetrics,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_usage_bytes: u64,
    pub disk_total_bytes: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub open_file_descriptors: u64,
    pub process_count: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub response_time_avg_ms: f64,
    pub response_time_p95_ms: f64,
    pub response_time_p99_ms: f64,
    pub errors_total: u64,
    pub error_rate_percent: f64,
    pub active_connections: u64,
    pub database_connections_active: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_hit_rate_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMetrics {
    pub swarms_created: u64,
    pub agents_spawned: u64,
    pub tasks_completed: u64,
    pub memory_entries_stored: u64,
    pub neural_models_trained: u64,
    pub mcp_tools_executed: u64,
    pub github_operations: u64,
    pub workflow_executions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub gc_cycles: u64,
    pub heap_size_bytes: u64,
    pub thread_count: u64,
    pub cpu_time_user_ms: u64,
    pub cpu_time_system_ms: u64,
    pub page_faults: u64,
    pub context_switches: u64,
    pub token_usage_total: u64,
    pub token_rate_per_minute: f64,
}

pub struct MetricsCollector {
    // Atomic counters for thread-safe metrics
    requests_total: AtomicU64,
    errors_total: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    
    // Business metrics
    swarms_created: AtomicU64,
    agents_spawned: AtomicU64,
    tasks_completed: AtomicU64,
    memory_entries: AtomicU64,
    neural_models: AtomicU64,
    mcp_executions: AtomicU64,
    github_operations: AtomicU64,
    workflow_executions: AtomicU64,
    token_usage: AtomicU64,

    // Response time tracking
    response_times: Arc<RwLock<Vec<f64>>>,
    
    // Custom metrics
    custom_metrics: Arc<RwLock<HashMap<String, f64>>>,
    
    // Start time for uptime calculation
    start_time: Instant,
    start_timestamp: SystemTime,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            errors_total: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            swarms_created: AtomicU64::new(0),
            agents_spawned: AtomicU64::new(0),
            tasks_completed: AtomicU64::new(0),
            memory_entries: AtomicU64::new(0),
            neural_models: AtomicU64::new(0),
            mcp_executions: AtomicU64::new(0),
            github_operations: AtomicU64::new(0),
            workflow_executions: AtomicU64::new(0),
            token_usage: AtomicU64::new(0),
            response_times: Arc::new(RwLock::new(Vec::new())),
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
            start_timestamp: SystemTime::now(),
        }
    }

    // Request metrics
    pub fn increment_requests(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_errors(&self) {
        self.errors_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_response_time(&self, duration: Duration) {
        let ms = duration.as_secs_f64() * 1000.0;
        
        // Keep only last 1000 response times for memory efficiency
        if let Ok(mut times) = self.response_times.write() {
            times.push(ms);
            if times.len() > 1000 {
                times.drain(0..500); // Remove oldest half
            }
        }
    }

    // Cache metrics
    pub fn increment_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_cache_misses(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    // Business metrics
    pub fn increment_swarms_created(&self) {
        self.swarms_created.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_agents_spawned(&self) {
        self.agents_spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_tasks_completed(&self) {
        self.tasks_completed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_memory_entries(&self) {
        self.memory_entries.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_neural_models(&self) {
        self.neural_models.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_mcp_executions(&self) {
        self.mcp_executions.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_github_operations(&self) {
        self.github_operations.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_workflow_executions(&self) {
        self.workflow_executions.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_token_usage(&self, tokens: u64) {
        self.token_usage.fetch_add(tokens, Ordering::Relaxed);
    }

    // Custom metrics
    pub fn set_custom_metric(&self, name: String, value: f64) {
        if let Ok(mut metrics) = self.custom_metrics.write() {
            metrics.insert(name, value);
        }
    }

    pub fn increment_custom_metric(&self, name: String, increment: f64) {
        if let Ok(mut metrics) = self.custom_metrics.write() {
            let current = metrics.get(&name).unwrap_or(&0.0);
            metrics.insert(name, current + increment);
        }
    }

    // Snapshot generation
    pub async fn snapshot(&self) -> MetricsSnapshot {
        let timestamp = Utc::now();
        
        let system = self.collect_system_metrics().await;
        let application = self.collect_application_metrics().await;
        let business = self.collect_business_metrics().await;
        let performance = self.collect_performance_metrics().await;

        MetricsSnapshot {
            timestamp,
            system,
            application,
            business,
            performance,
        }
    }

    async fn collect_system_metrics(&self) -> SystemMetrics {
        // Get system information
        let uptime_seconds = self.start_time.elapsed().as_secs();
        
        // Platform-specific system metrics collection
        let (cpu_usage, memory_usage, memory_total) = self.get_system_usage().await;
        let (disk_usage, disk_total) = self.get_disk_usage().await;
        let (network_sent, network_received) = self.get_network_stats().await;
        let open_fds = self.get_open_file_descriptors().await;
        let process_count = self.get_process_count().await;

        SystemMetrics {
            cpu_usage_percent: cpu_usage,
            memory_usage_bytes: memory_usage,
            memory_total_bytes: memory_total,
            disk_usage_bytes: disk_usage,
            disk_total_bytes: disk_total,
            network_bytes_sent: network_sent,
            network_bytes_received: network_received,
            open_file_descriptors: open_fds,
            process_count,
            uptime_seconds,
        }
    }

    async fn collect_application_metrics(&self) -> ApplicationMetrics {
        let requests_total = self.requests_total.load(Ordering::Relaxed);
        let errors_total = self.errors_total.load(Ordering::Relaxed);
        let cache_hits = self.cache_hits.load(Ordering::Relaxed);
        let cache_misses = self.cache_misses.load(Ordering::Relaxed);

        let uptime_seconds = self.start_time.elapsed().as_secs_f64();
        let requests_per_second = if uptime_seconds > 0.0 {
            requests_total as f64 / uptime_seconds
        } else {
            0.0
        };

        let error_rate_percent = if requests_total > 0 {
            (errors_total as f64 / requests_total as f64) * 100.0
        } else {
            0.0
        };

        let cache_total = cache_hits + cache_misses;
        let cache_hit_rate_percent = if cache_total > 0 {
            (cache_hits as f64 / cache_total as f64) * 100.0
        } else {
            0.0
        };

        // Calculate response time percentiles
        let (avg_response_time, p95_response_time, p99_response_time) = {
            if let Ok(times) = self.response_times.read() {
                if !times.is_empty() {
                    let mut sorted_times = times.clone();
                    sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    
                    let avg = sorted_times.iter().sum::<f64>() / sorted_times.len() as f64;
                    let p95_idx = (sorted_times.len() as f64 * 0.95) as usize;
                    let p99_idx = (sorted_times.len() as f64 * 0.99) as usize;
                    
                    let p95 = sorted_times.get(p95_idx.min(sorted_times.len() - 1)).unwrap_or(&0.0);
                    let p99 = sorted_times.get(p99_idx.min(sorted_times.len() - 1)).unwrap_or(&0.0);
                    
                    (avg, *p95, *p99)
                } else {
                    (0.0, 0.0, 0.0)
                }
            } else {
                (0.0, 0.0, 0.0)
            }
        };

        ApplicationMetrics {
            requests_total,
            requests_per_second,
            response_time_avg_ms: avg_response_time,
            response_time_p95_ms: p95_response_time,
            response_time_p99_ms: p99_response_time,
            errors_total,
            error_rate_percent,
            active_connections: self.get_active_connections().await,
            database_connections_active: self.get_db_connections().await,
            cache_hits,
            cache_misses,
            cache_hit_rate_percent,
        }
    }

    async fn collect_business_metrics(&self) -> BusinessMetrics {
        BusinessMetrics {
            swarms_created: self.swarms_created.load(Ordering::Relaxed),
            agents_spawned: self.agents_spawned.load(Ordering::Relaxed),
            tasks_completed: self.tasks_completed.load(Ordering::Relaxed),
            memory_entries_stored: self.memory_entries.load(Ordering::Relaxed),
            neural_models_trained: self.neural_models.load(Ordering::Relaxed),
            mcp_tools_executed: self.mcp_executions.load(Ordering::Relaxed),
            github_operations: self.github_operations.load(Ordering::Relaxed),
            workflow_executions: self.workflow_executions.load(Ordering::Relaxed),
        }
    }

    async fn collect_performance_metrics(&self) -> PerformanceMetrics {
        let token_usage_total = self.token_usage.load(Ordering::Relaxed);
        let uptime_minutes = self.start_time.elapsed().as_secs_f64() / 60.0;
        let token_rate_per_minute = if uptime_minutes > 0.0 {
            token_usage_total as f64 / uptime_minutes
        } else {
            0.0
        };

        PerformanceMetrics {
            gc_cycles: self.get_gc_cycles().await,
            heap_size_bytes: self.get_heap_size().await,
            thread_count: self.get_thread_count().await,
            cpu_time_user_ms: self.get_cpu_time_user().await,
            cpu_time_system_ms: self.get_cpu_time_system().await,
            page_faults: self.get_page_faults().await,
            context_switches: self.get_context_switches().await,
            token_usage_total,
            token_rate_per_minute,
        }
    }

    // Prometheus format export
    pub async fn export_prometheus(&self) -> String {
        let snapshot = self.snapshot().await;
        let mut output = String::new();

        // System metrics
        output.push_str(&format!(
            "# HELP claude_flow_cpu_usage_percent CPU usage percentage\n\
             # TYPE claude_flow_cpu_usage_percent gauge\n\
             claude_flow_cpu_usage_percent {}\n",
            snapshot.system.cpu_usage_percent
        ));

        output.push_str(&format!(
            "# HELP claude_flow_memory_usage_bytes Memory usage in bytes\n\
             # TYPE claude_flow_memory_usage_bytes gauge\n\
             claude_flow_memory_usage_bytes {}\n",
            snapshot.system.memory_usage_bytes
        ));

        // Application metrics
        output.push_str(&format!(
            "# HELP claude_flow_requests_total Total number of requests\n\
             # TYPE claude_flow_requests_total counter\n\
             claude_flow_requests_total {}\n",
            snapshot.application.requests_total
        ));

        output.push_str(&format!(
            "# HELP claude_flow_errors_total Total number of errors\n\
             # TYPE claude_flow_errors_total counter\n\
             claude_flow_errors_total {}\n",
            snapshot.application.errors_total
        ));

        output.push_str(&format!(
            "# HELP claude_flow_response_time_avg_ms Average response time in milliseconds\n\
             # TYPE claude_flow_response_time_avg_ms gauge\n\
             claude_flow_response_time_avg_ms {}\n",
            snapshot.application.response_time_avg_ms
        ));

        // Business metrics
        output.push_str(&format!(
            "# HELP claude_flow_swarms_created_total Total swarms created\n\
             # TYPE claude_flow_swarms_created_total counter\n\
             claude_flow_swarms_created_total {}\n",
            snapshot.business.swarms_created
        ));

        output.push_str(&format!(
            "# HELP claude_flow_agents_spawned_total Total agents spawned\n\
             # TYPE claude_flow_agents_spawned_total counter\n\
             claude_flow_agents_spawned_total {}\n",
            snapshot.business.agents_spawned
        ));

        output.push_str(&format!(
            "# HELP claude_flow_tasks_completed_total Total tasks completed\n\
             # TYPE claude_flow_tasks_completed_total counter\n\
             claude_flow_tasks_completed_total {}\n",
            snapshot.business.tasks_completed
        ));

        // Custom metrics
        if let Ok(custom) = self.custom_metrics.read() {
            for (name, value) in custom.iter() {
                output.push_str(&format!(
                    "# HELP claude_flow_custom_{} Custom metric: {}\n\
                     # TYPE claude_flow_custom_{} gauge\n\
                     claude_flow_custom_{} {}\n",
                    name, name, name, name, value
                ));
            }
        }

        output
    }

    // Platform-specific system information gathering
    async fn get_system_usage(&self) -> (f64, u64, u64) {
        // Placeholder implementation - would use platform-specific APIs
        (25.5, 512 * 1024 * 1024, 1024 * 1024 * 1024)
    }

    async fn get_disk_usage(&self) -> (u64, u64) {
        // Placeholder implementation
        (2 * 1024 * 1024 * 1024, 10 * 1024 * 1024 * 1024)
    }

    async fn get_network_stats(&self) -> (u64, u64) {
        // Placeholder implementation
        (1024 * 1024, 2 * 1024 * 1024)
    }

    async fn get_open_file_descriptors(&self) -> u64 {
        // Placeholder implementation
        42
    }

    async fn get_process_count(&self) -> u64 {
        // Placeholder implementation
        156
    }

    async fn get_active_connections(&self) -> u64 {
        // Placeholder implementation
        15
    }

    async fn get_db_connections(&self) -> u64 {
        // Placeholder implementation
        5
    }

    async fn get_gc_cycles(&self) -> u64 {
        // Placeholder implementation
        0
    }

    async fn get_heap_size(&self) -> u64 {
        // Placeholder implementation
        128 * 1024 * 1024
    }

    async fn get_thread_count(&self) -> u64 {
        // Placeholder implementation
        8
    }

    async fn get_cpu_time_user(&self) -> u64 {
        // Placeholder implementation
        1500
    }

    async fn get_cpu_time_system(&self) -> u64 {
        // Placeholder implementation
        500
    }

    async fn get_page_faults(&self) -> u64 {
        // Placeholder implementation
        1024
    }

    async fn get_context_switches(&self) -> u64 {
        // Placeholder implementation
        50000
    }
}