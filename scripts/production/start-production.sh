#!/bin/bash
# Claude Flow Production Startup Script
# Handles graceful startup, health checks, and service coordination

set -euo pipefail

# Production startup configuration
export CLAUDE_FLOW_ENV="production"
export RUST_LOG="${RUST_LOG:-info}"
export RUST_BACKTRACE="1"

# Directories
DATA_DIR="${CLAUDE_FLOW_DATA_DIR:-/app/data}"
CONFIG_DIR="${CLAUDE_FLOW_CONFIG_DIR:-/app/config}"
LOG_DIR="${CLAUDE_FLOW_LOG_DIR:-/var/log/claude-flow}"

# Create necessary directories
mkdir -p "$DATA_DIR" "$CONFIG_DIR" "$LOG_DIR"

# Logging function
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [STARTUP] $1" | tee -a "$LOG_DIR/startup.log"
}

# Error handling
handle_error() {
    log "ERROR: $1"
    exit 1
}

# Graceful shutdown handler
shutdown_handler() {
    log "Received shutdown signal, gracefully stopping services..."
    
    # Stop MCP service
    if [[ -n "${MCP_PID:-}" ]]; then
        log "Stopping MCP service (PID: $MCP_PID)"
        kill -TERM "$MCP_PID" 2>/dev/null || true
        wait "$MCP_PID" 2>/dev/null || true
    fi
    
    # Stop main service
    if [[ -n "${MAIN_PID:-}" ]]; then
        log "Stopping main service (PID: $MAIN_PID)"
        kill -TERM "$MAIN_PID" 2>/dev/null || true
        wait "$MAIN_PID" 2>/dev/null || true
    fi
    
    log "All services stopped gracefully"
    exit 0
}

# Set up signal handlers
trap shutdown_handler SIGTERM SIGINT

log "Starting Claude Flow Production Services..."

# Validate environment
log "Validating production environment..."

# Check required binaries
for binary in claude-flow claude-flow-mcp; do
    if ! command -v "$binary" >/dev/null 2>&1; then
        handle_error "Required binary '$binary' not found in PATH"
    fi
done

# Check configuration
if [[ ! -f "$CONFIG_DIR/production.json" ]]; then
    log "Creating default production configuration..."
    cat > "$CONFIG_DIR/production.json" << 'EOF'
{
  "environment": "production",
  "server": {
    "host": "0.0.0.0",
    "port": 8080,
    "workers": 4
  },
  "mcp": {
    "host": "0.0.0.0", 
    "port": 8082,
    "max_connections": 100
  },
  "database": {
    "url": "sqlite:///app/data/claude-flow.db",
    "pool_size": 10,
    "timeout": 30
  },
  "logging": {
    "level": "info",
    "format": "json",
    "file": "/var/log/claude-flow/claude-flow.log",
    "max_size": "100MB",
    "max_files": 10
  },
  "metrics": {
    "enabled": true,
    "endpoint": "/metrics",
    "interval": 30
  },
  "health": {
    "endpoint": "/health",
    "timeout": 5,
    "checks": ["database", "memory", "disk"]
  },
  "security": {
    "cors_enabled": true,
    "rate_limit": {
      "enabled": true,
      "requests_per_minute": 100
    }
  }
}
EOF
fi

# Initialize database
log "Initializing database..."
if [[ ! -f "$DATA_DIR/claude-flow.db" ]]; then
    claude-flow --config "$CONFIG_DIR/production.json" init --force || handle_error "Database initialization failed"
fi

# Start MCP service
log "Starting MCP service..."
claude-flow-mcp \
    --host "0.0.0.0" \
    --port "8082" \
    --config "$CONFIG_DIR/production.json" \
    >> "$LOG_DIR/mcp.log" 2>&1 &

MCP_PID=$!
log "MCP service started with PID: $MCP_PID"

# Wait for MCP service to be ready
log "Waiting for MCP service to be ready..."
for i in {1..30}; do
    if curl -f "http://localhost:8082/api/mcp/health" >/dev/null 2>&1; then
        log "MCP service is ready"
        break
    fi
    
    if [[ $i -eq 30 ]]; then
        handle_error "MCP service failed to start within 30 seconds"
    fi
    
    sleep 1
done

# Start main Claude Flow service
log "Starting main Claude Flow service..."
claude-flow \
    --config "$CONFIG_DIR/production.json" \
    >> "$LOG_DIR/claude-flow.log" 2>&1 &

MAIN_PID=$!
log "Main service started with PID: $MAIN_PID"

# Wait for main service to be ready
log "Waiting for main service to be ready..."
for i in {1..30}; do
    if curl -f "http://localhost:8080/health" >/dev/null 2>&1; then
        log "Main service is ready"
        break
    fi
    
    if [[ $i -eq 30 ]]; then
        handle_error "Main service failed to start within 30 seconds"
    fi
    
    sleep 1
done

# Perform initial health check
log "Performing comprehensive health check..."
if ! /app/bin/health-check.sh; then
    handle_error "Initial health check failed"
fi

# Setup monitoring
log "Setting up production monitoring..."

# Start metrics collection
(
    while true; do
        timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        
        # Collect system metrics
        cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
        memory_info=$(free | grep Mem | awk '{printf "%.1f", $3/$2 * 100.0}')
        disk_usage=$(df -h / | awk 'NR==2{printf "%s", $5}' | cut -d'%' -f1)
        
        # Log metrics
        echo "{\"timestamp\":\"$timestamp\",\"cpu_usage\":$cpu_usage,\"memory_usage\":$memory_info,\"disk_usage\":$disk_usage}" >> "$LOG_DIR/metrics.log"
        
        sleep 60
    done
) &

METRICS_PID=$!

# Production readiness banner
log "======================================"
log "🚀 CLAUDE FLOW PRODUCTION READY 🚀"
log "======================================"
log "Main Service:    http://localhost:8080"
log "MCP Service:     http://localhost:8082"
log "Health Check:    http://localhost:8080/health"
log "Metrics:         http://localhost:8080/metrics"
log "======================================"
log "Services running in production mode"
log "Monitoring active, logging configured"
log "Ready for enterprise workloads"
log "======================================"

# Keep script running and monitor services
while true; do
    # Check if MCP service is still running
    if ! kill -0 "$MCP_PID" 2>/dev/null; then
        handle_error "MCP service died unexpectedly"
    fi
    
    # Check if main service is still running
    if ! kill -0 "$MAIN_PID" 2>/dev/null; then
        handle_error "Main service died unexpectedly"
    fi
    
    # Periodic health check (every 5 minutes)
    if [[ $(($(date +%s) % 300)) -eq 0 ]]; then
        if ! /app/bin/health-check.sh >/dev/null 2>&1; then
            log "WARNING: Health check failed, services may be degraded"
        fi
    fi
    
    sleep 10
done