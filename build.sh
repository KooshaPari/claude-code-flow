#!/bin/bash

set -e

echo "🚀 Building Claude Flow 2.0 (Rust/Go Implementation)"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create build directory
mkdir -p build/bin
mkdir -p build/configs

echo -e "${BLUE}📦 Building Rust components...${NC}"

# Build Rust binary
echo "Building main Claude Flow CLI..."
cargo build --release

# Copy Rust binary
if [ -f "target/release/claude-flow" ]; then
    cp target/release/claude-flow build/bin/
    echo -e "${GREEN}✅ Rust binary built successfully${NC}"
else
    echo -e "${RED}❌ Rust binary build failed${NC}"
    exit 1
fi

echo -e "${BLUE}🐹 Building Go components...${NC}"

# Build Go Neural Engine
echo "Building Neural Engine (Go)..."
cd go-neural
go build -ldflags="-s -w" -o ../build/bin/claude-flow-neural main.go
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Neural Engine built successfully${NC}"
else
    echo -e "${RED}❌ Neural Engine build failed${NC}"
    exit 1
fi
cd ..

# Build Go MCP Manager
echo "Building MCP Manager (Go)..."
cd go-mcp
go build -ldflags="-s -w" -o ../build/bin/claude-flow-mcp main.go
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ MCP Manager built successfully${NC}"
else
    echo -e "${RED}❌ MCP Manager build failed${NC}"
    exit 1
fi
cd ..

echo -e "${BLUE}📋 Creating configuration files...${NC}"

# Create default configuration
cat > build/configs/config.toml << 'EOF'
[core]
log_level = "info"
max_concurrent_tasks = 10
session_timeout = 3600
auto_save_interval = 300

[agents]
max_agents = 50
default_timeout = 300
memory_limit_mb = 512
cpu_limit_percent = 50.0

[memory]
backend = "sqlite"
database_path = "~/.claude-flow/memory.db"
max_entries = 100000
compression_enabled = true
encryption_enabled = false

[neural]
enabled = true
model_path = "~/.claude-flow/models"
max_models = 10
training_enabled = true
gpu_enabled = false

[swarm]
default_topology = "hierarchical"
max_swarm_size = 20
coordination_timeout = 60
load_balancing_enabled = true

[github]
enabled = false

[mcp]
enabled = true
auto_permissions = true

[[mcp.servers]]
name = "claude-flow"
command = "./claude-flow-mcp"
args = []

[[mcp.servers]]
name = "neural-engine"
command = "./claude-flow-neural"
args = []

[ui]
terminal_ui_enabled = true
web_ui_enabled = true
web_ui_port = 8080
real_time_updates = true
EOF

echo -e "${GREEN}✅ Configuration file created${NC}"

# Create launch script
cat > build/claude-flow-complete.sh << 'EOF'
#!/bin/bash

# Claude Flow 2.0 Complete Launch Script
echo "🌊 Claude Flow 2.0 - AI Orchestration Platform"
echo "=============================================="

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
BIN_DIR="$SCRIPT_DIR/bin"
CONFIG_DIR="$SCRIPT_DIR/configs"

# Start Neural Engine in background
echo "🧠 Starting Neural Engine..."
"$BIN_DIR/claude-flow-neural" &
NEURAL_PID=$!

# Start MCP Manager in background
echo "🔧 Starting MCP Manager..."
"$BIN_DIR/claude-flow-mcp" &
MCP_PID=$!

# Wait a moment for services to start
sleep 3

# Start main Claude Flow CLI
echo "🚀 Starting Claude Flow CLI..."
"$BIN_DIR/claude-flow" --config "$CONFIG_DIR/config.toml" "$@"

# Cleanup function
cleanup() {
    echo "🔄 Shutting down services..."
    kill $NEURAL_PID 2>/dev/null
    kill $MCP_PID 2>/dev/null
    echo "✅ Shutdown complete"
}

# Trap exit signals
trap cleanup EXIT INT TERM

# Keep script running if main process exits
wait
EOF

chmod +x build/claude-flow-complete.sh

echo -e "${GREEN}✅ Launch script created${NC}"

# Create installation script
cat > build/install.sh << 'EOF'
#!/bin/bash

echo "📦 Installing Claude Flow 2.0..."

# Create installation directory
INSTALL_DIR="$HOME/.local/bin/claude-flow"
mkdir -p "$INSTALL_DIR"

# Copy binaries
cp bin/* "$INSTALL_DIR/"
cp configs/* "$INSTALL_DIR/"
cp claude-flow-complete.sh "$INSTALL_DIR/"

# Create symlink in PATH
ln -sf "$INSTALL_DIR/claude-flow-complete.sh" "$HOME/.local/bin/claude-flow"

# Make sure ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    echo "⚠️  Please run: source ~/.bashrc"
fi

echo "✅ Claude Flow 2.0 installed successfully!"
echo "🚀 Run 'claude-flow --help' to get started"
EOF

chmod +x build/install.sh

echo -e "${BLUE}📊 Build Summary${NC}"
echo "================"
echo "🦀 Rust CLI: $(du -h build/bin/claude-flow | cut -f1)"
echo "🧠 Neural Engine: $(du -h build/bin/claude-flow-neural | cut -f1)"
echo "🔧 MCP Manager: $(du -h build/bin/claude-flow-mcp | cut -f1)"
echo "📁 Total Build Size: $(du -sh build | cut -f1)"

echo ""
echo -e "${GREEN}🎉 Build completed successfully!${NC}"
echo ""
echo "To test the build:"
echo "  cd build && ./claude-flow-complete.sh --help"
echo ""
echo "To install system-wide:"
echo "  cd build && ./install.sh"
echo ""
echo "Available components:"
echo "  🦀 claude-flow         - Main CLI (Rust)"
echo "  🧠 claude-flow-neural  - Neural Engine (Go)"
echo "  🔧 claude-flow-mcp     - MCP Manager (Go)"
echo ""