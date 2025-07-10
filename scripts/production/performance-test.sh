#!/bin/bash
# Claude Flow Production Performance Testing Script
# Comprehensive load testing and performance validation

set -euo pipefail

# Configuration
BASE_URL="${CLAUDE_FLOW_URL:-http://localhost:8080}"
MCP_URL="${CLAUDE_FLOW_MCP_URL:-http://localhost:8082}"
CONCURRENT_USERS="${PERF_CONCURRENT_USERS:-10}"
TEST_DURATION="${PERF_TEST_DURATION:-60}"
RAMP_UP_TIME="${PERF_RAMP_UP:-10}"

# Test results
RESULTS_DIR="./performance-results-$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

# Logging
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [PERF-TEST] $1" | tee -a "$RESULTS_DIR/test.log"
}

error() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [ERROR] $1" | tee -a "$RESULTS_DIR/test.log" >&2
    exit 1
}

# Check dependencies
check_dependencies() {
    log "Checking dependencies..."
    
    for cmd in curl ab wrk jq; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            error "Required command '$cmd' not found. Please install it."
        fi
    done
    
    log "All dependencies available"
}

# Health check before testing
pre_test_health_check() {
    log "Performing pre-test health check..."
    
    # Check main service
    if ! curl -f "$BASE_URL/health" >/dev/null 2>&1; then
        error "Main service health check failed at $BASE_URL/health"
    fi
    
    # Check MCP service
    if ! curl -f "$MCP_URL/api/mcp/health" >/dev/null 2>&1; then
        error "MCP service health check failed at $MCP_URL/api/mcp/health"
    fi
    
    log "Pre-test health check passed"
}

# Baseline performance test
baseline_test() {
    log "Running baseline performance test..."
    
    # Single request latency test
    local response_time=$(curl -w "%{time_total}" -s -o /dev/null "$BASE_URL/health")
    log "Baseline response time: ${response_time}s"
    
    # Save baseline
    echo "{\"baseline_response_time\": $response_time, \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" > "$RESULTS_DIR/baseline.json"
}

# HTTP load test using Apache Bench
load_test_ab() {
    log "Running Apache Bench load test..."
    
    local output_file="$RESULTS_DIR/ab_results.txt"
    
    ab -n 1000 -c "$CONCURRENT_USERS" -g "$RESULTS_DIR/ab_plot.txt" "$BASE_URL/health" > "$output_file" 2>&1 || {
        log "Warning: Apache Bench test had issues, but continuing..."
    }
    
    # Extract key metrics
    local rps=$(grep "Requests per second" "$output_file" | awk '{print $4}' || echo "0")
    local mean_time=$(grep "Time per request" "$output_file" | head -1 | awk '{print $4}' || echo "0")
    local p95_time=$(grep "95%" "$output_file" | awk '{print $2}' || echo "0")
    
    log "Apache Bench Results - RPS: $rps, Mean: ${mean_time}ms, P95: ${p95_time}ms"
    
    # Save results in JSON format
    cat > "$RESULTS_DIR/ab_summary.json" << EOF
{
  "tool": "apache_bench",
  "requests_per_second": $rps,
  "mean_response_time_ms": $mean_time,
  "p95_response_time_ms": $p95_time,
  "concurrent_users": $CONCURRENT_USERS,
  "total_requests": 1000
}
EOF
}

# HTTP load test using wrk
load_test_wrk() {
    log "Running wrk load test..."
    
    local output_file="$RESULTS_DIR/wrk_results.txt"
    
    wrk -t4 -c"$CONCURRENT_USERS" -d"${TEST_DURATION}s" --latency "$BASE_URL/health" > "$output_file" 2>&1 || {
        log "Warning: wrk test had issues, but continuing..."
    }
    
    # Extract metrics
    local rps=$(grep "Requests/sec" "$output_file" | awk '{print $2}' || echo "0")
    local avg_latency=$(grep "Latency" "$output_file" | awk '{print $2}' || echo "0")
    local p99_latency=$(grep "99%" "$output_file" | awk '{print $2}' || echo "0")
    
    log "wrk Results - RPS: $rps, Avg Latency: $avg_latency, P99: $p99_latency"
    
    # Save results
    cat > "$RESULTS_DIR/wrk_summary.json" << EOF
{
  "tool": "wrk",
  "requests_per_second": "$rps",
  "avg_latency": "$avg_latency",
  "p99_latency": "$p99_latency",
  "concurrent_users": $CONCURRENT_USERS,
  "duration_seconds": $TEST_DURATION
}
EOF
}

# Custom Claude Flow API tests
api_functionality_test() {
    log "Running Claude Flow API functionality tests..."
    
    local api_results="$RESULTS_DIR/api_tests.json"
    local results=()
    
    # Test swarm creation
    log "Testing swarm creation API..."
    local swarm_start=$(date +%s%N)
    local swarm_response=$(curl -s -X POST "$MCP_URL/api/mcp/tools/mcp__claude-flow__swarm_init/execute" \
        -H "Content-Type: application/json" \
        -d '{"parameters": {"topology": "mesh", "maxAgents": 5, "strategy": "balanced"}}' 2>/dev/null || echo '{"error": "failed"}')
    local swarm_end=$(date +%s%N)
    local swarm_time=$(((swarm_end - swarm_start) / 1000000))
    
    if echo "$swarm_response" | jq -e '.result' >/dev/null 2>&1; then
        log "Swarm creation test passed (${swarm_time}ms)"
        results+=("{\"test\": \"swarm_creation\", \"status\": \"pass\", \"response_time_ms\": $swarm_time}")
    else
        log "Swarm creation test failed"
        results+=("{\"test\": \"swarm_creation\", \"status\": \"fail\", \"response_time_ms\": $swarm_time}")
    fi
    
    # Test agent spawning
    log "Testing agent spawning API..."
    local agent_start=$(date +%s%N)
    local agent_response=$(curl -s -X POST "$MCP_URL/api/mcp/tools/mcp__claude-flow__agent_spawn/execute" \
        -H "Content-Type: application/json" \
        -d '{"parameters": {"type": "researcher", "capabilities": ["analysis", "coordination"]}}' 2>/dev/null || echo '{"error": "failed"}')
    local agent_end=$(date +%s%N)
    local agent_time=$(((agent_end - agent_start) / 1000000))
    
    if echo "$agent_response" | jq -e '.result' >/dev/null 2>&1; then
        log "Agent spawning test passed (${agent_time}ms)"
        results+=("{\"test\": \"agent_spawning\", \"status\": \"pass\", \"response_time_ms\": $agent_time}")
    else
        log "Agent spawning test failed"
        results+=("{\"test\": \"agent_spawning\", \"status\": \"fail\", \"response_time_ms\": $agent_time}")
    fi
    
    # Test memory operations
    log "Testing memory operations API..."
    local memory_start=$(date +%s%N)
    local memory_response=$(curl -s -X POST "$MCP_URL/api/mcp/tools/mcp__claude-flow__memory_usage/execute" \
        -H "Content-Type: application/json" \
        -d '{"parameters": {"action": "store", "key": "test_key", "value": "test_value", "namespace": "performance_test"}}' 2>/dev/null || echo '{"error": "failed"}')
    local memory_end=$(date +%s%N)
    local memory_time=$(((memory_end - memory_start) / 1000000))
    
    if echo "$memory_response" | jq -e '.result' >/dev/null 2>&1; then
        log "Memory operations test passed (${memory_time}ms)"
        results+=("{\"test\": \"memory_operations\", \"status\": \"pass\", \"response_time_ms\": $memory_time}")
    else
        log "Memory operations test failed"
        results+=("{\"test\": \"memory_operations\", \"status\": \"fail\", \"response_time_ms\": $memory_time}")
    fi
    
    # Save API test results
    printf '[%s]\n' "$(IFS=,; echo "${results[*]}")" > "$api_results"
}

# Memory and resource monitoring during tests
monitor_resources() {
    log "Starting resource monitoring..."
    
    local monitor_file="$RESULTS_DIR/resource_monitor.txt"
    local pid_file="$RESULTS_DIR/monitor.pid"
    
    # Start monitoring in background
    (
        while true; do
            timestamp=$(date '+%Y-%m-%d %H:%M:%S')
            
            # System resources
            cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1 2>/dev/null || echo "0")
            memory_info=$(free | grep Mem | awk '{printf "%.1f", $3/$2 * 100.0}' 2>/dev/null || echo "0")
            
            # Claude Flow process resources
            claude_cpu=$(ps aux | grep "claude-flow" | grep -v grep | awk '{sum += $3} END {print sum+0}' 2>/dev/null || echo "0")
            claude_mem=$(ps aux | grep "claude-flow" | grep -v grep | awk '{sum += $4} END {print sum+0}' 2>/dev/null || echo "0")
            
            echo "$timestamp,$cpu_usage,$memory_info,$claude_cpu,$claude_mem" >> "$monitor_file"
            sleep 5
        done
    ) &
    
    echo $! > "$pid_file"
    log "Resource monitoring started (PID: $(cat "$pid_file"))"
}

# Stop resource monitoring
stop_monitoring() {
    local pid_file="$RESULTS_DIR/monitor.pid"
    
    if [[ -f "$pid_file" ]]; then
        local monitor_pid=$(cat "$pid_file")
        if kill "$monitor_pid" 2>/dev/null; then
            log "Resource monitoring stopped"
        fi
        rm -f "$pid_file"
    fi
}

# Generate performance report
generate_report() {
    log "Generating performance report..."
    
    local report_file="$RESULTS_DIR/performance_report.html"
    
    cat > "$report_file" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Claude Flow Performance Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; }
        .section { margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px; }
        .metric { display: inline-block; margin: 10px; padding: 10px; background: #f9f9f9; border-radius: 3px; }
        .pass { color: green; }
        .fail { color: red; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <div class="header">
        <h1>Claude Flow Performance Test Report</h1>
        <p>Generated: $(date)</p>
        <p>Test Configuration: $CONCURRENT_USERS concurrent users, ${TEST_DURATION}s duration</p>
    </div>
EOF

    # Add baseline results
    if [[ -f "$RESULTS_DIR/baseline.json" ]]; then
        local baseline_time=$(jq -r '.baseline_response_time' "$RESULTS_DIR/baseline.json")
        cat >> "$report_file" << EOF
    <div class="section">
        <h2>Baseline Performance</h2>
        <div class="metric">Response Time: ${baseline_time}s</div>
    </div>
EOF
    fi

    # Add load test results
    cat >> "$report_file" << EOF
    <div class="section">
        <h2>Load Test Results</h2>
        <table>
            <tr><th>Tool</th><th>Requests/sec</th><th>Avg Response Time</th><th>P95/P99</th></tr>
EOF

    if [[ -f "$RESULTS_DIR/ab_summary.json" ]]; then
        local ab_rps=$(jq -r '.requests_per_second' "$RESULTS_DIR/ab_summary.json")
        local ab_mean=$(jq -r '.mean_response_time_ms' "$RESULTS_DIR/ab_summary.json")
        local ab_p95=$(jq -r '.p95_response_time_ms' "$RESULTS_DIR/ab_summary.json")
        
        cat >> "$report_file" << EOF
            <tr><td>Apache Bench</td><td>$ab_rps</td><td>${ab_mean}ms</td><td>${ab_p95}ms (P95)</td></tr>
EOF
    fi

    if [[ -f "$RESULTS_DIR/wrk_summary.json" ]]; then
        local wrk_rps=$(jq -r '.requests_per_second' "$RESULTS_DIR/wrk_summary.json")
        local wrk_avg=$(jq -r '.avg_latency' "$RESULTS_DIR/wrk_summary.json")
        local wrk_p99=$(jq -r '.p99_latency' "$RESULTS_DIR/wrk_summary.json")
        
        cat >> "$report_file" << EOF
            <tr><td>wrk</td><td>$wrk_rps</td><td>$wrk_avg</td><td>$wrk_p99 (P99)</td></tr>
EOF
    fi

    cat >> "$report_file" << EOF
        </table>
    </div>
EOF

    # Add API test results
    if [[ -f "$RESULTS_DIR/api_tests.json" ]]; then
        cat >> "$report_file" << EOF
    <div class="section">
        <h2>API Functionality Tests</h2>
        <table>
            <tr><th>Test</th><th>Status</th><th>Response Time</th></tr>
EOF
        
        jq -r '.[] | "<tr><td>\(.test)</td><td class=\"\(.status)\">\(.status)</td><td>\(.response_time_ms)ms</td></tr>"' "$RESULTS_DIR/api_tests.json" >> "$report_file"
        
        cat >> "$report_file" << EOF
        </table>
    </div>
EOF
    fi

    cat >> "$report_file" << EOF
    <div class="section">
        <h2>Test Files</h2>
        <ul>
            <li><a href="ab_results.txt">Apache Bench Full Results</a></li>
            <li><a href="wrk_results.txt">wrk Full Results</a></li>
            <li><a href="resource_monitor.txt">Resource Monitoring Data</a></li>
            <li><a href="test.log">Test Execution Log</a></li>
        </ul>
    </div>
</body>
</html>
EOF

    log "Performance report generated: $report_file"
}

# Main test execution
main() {
    log "Starting Claude Flow performance testing..."
    
    # Setup
    check_dependencies
    pre_test_health_check
    
    # Start monitoring
    monitor_resources
    
    # Trap to ensure monitoring is stopped
    trap stop_monitoring EXIT
    
    try {
        # Run tests
        baseline_test
        api_functionality_test
        load_test_ab
        load_test_wrk
        
        # Stop monitoring
        stop_monitoring
        
        # Generate report
        generate_report
        
        log "Performance testing completed successfully"
        log "Results available in: $RESULTS_DIR"
        
    } catch {
        log "Performance testing failed: $1"
        stop_monitoring
        exit 1
    }
}

# Error handling
try() {
    eval "$1"
}

catch() {
    if [[ $? -ne 0 ]]; then
        eval "$1"
    fi
}

# Execute if script is run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi