# 🎉 Claude-Flow Complete Setup Guide

## ✅ Installation Status

Your Claude-Flow system is now fully installed and configured!

### 🔧 What's Installed

1. **Global `klaude` Binary**: Available system-wide
   - Location: `/home/augment-agent/.local/bin/klaude`
   - Usage: `klaude <command> [options]`

2. **MCP Server**: Ready for Claude Desktop integration
   - Location: `/mnt/persist/workspace/claude-flow-mcp-server/`
   - Configuration: Generated and ready

3. **Project Files**: All dependencies installed
   - Main project: `/mnt/persist/workspace/`
   - Dependencies: Installed via bun

## 🚀 Quick Start

### Test Your Installation

```bash
# Test the global klaude command
klaude status

# Check available commands
klaude help

# Test MCP server
klaude mcp serve --test
```

### Start the MCP Server

```bash
# Start MCP server for Claude Desktop
klaude mcp serve

# Or start in background
nohup klaude mcp serve > mcp-server.log 2>&1 &
```

## 🔗 Claude Desktop Integration

### Step 1: Locate Your Claude Desktop Config

**macOS:**
```bash
~/Library/Application Support/Claude/claude_desktop_config.json
```

**Windows:**
```bash
%APPDATA%\Claude\claude_desktop_config.json
```

**Linux:**
```bash
~/.config/Claude/claude_desktop_config.json
```

### Step 2: Add MCP Server Configuration

Copy the contents of `claude-desktop-config.json` to your Claude Desktop config file:

```json
{
  "mcpServers": {
    "claude-flow": {
      "command": "node",
      "args": [
        "/mnt/persist/workspace/claude-flow-mcp-server/server.js"
      ],
      "env": {
        "CLAUDE_FLOW_PATH": "/home/augment-agent/.local/bin/klaude",
        "CLAUDE_FLOW_TIMEOUT": "30000",
        "CLAUDE_FLOW_LOG_LEVEL": "info"
      },
      "capabilities": {
        "tools": true,
        "prompts": false,
        "resources": false
      },
      "description": "Claude-Flow orchestration system MCP server"
    }
  }
}
```

### Step 3: Restart Claude Desktop

After adding the configuration, restart Claude Desktop to load the MCP server.

## 🧪 Testing the Setup

### 1. Test Global Binary

```bash
# Basic status check
klaude status

# Start a simple agent
klaude agent create test-agent

# Check system health
klaude monitor health
```

### 2. Test MCP Integration

In Claude Desktop, you should now have access to these tools:
- `claude_flow_status` - Check system status
- `claude_flow_execute` - Execute commands
- `claude_flow_agent_create` - Create agents
- `claude_flow_swarm_start` - Start swarms

### 3. Test Commands

```bash
# Start the orchestration system
klaude start

# Create and manage agents
klaude agent create myagent --type coding
klaude agent list
klaude agent status myagent

# Work with swarms
klaude swarm create myswarm
klaude swarm add-agent myswarm myagent
klaude swarm start myswarm

# Use SPARC methodology
klaude sparc init myproject
klaude sparc spec myproject "Build a REST API"
```

## 📚 Available Commands

### Core Commands
- `klaude start` - Start the orchestration system
- `klaude status` - Show system status
- `klaude help` - Show help information

### Agent Management
- `klaude agent create <name>` - Create a new agent
- `klaude agent list` - List all agents
- `klaude agent status <name>` - Check agent status
- `klaude agent delete <name>` - Delete an agent

### Swarm Operations
- `klaude swarm create <name>` - Create a new swarm
- `klaude swarm list` - List all swarms
- `klaude swarm start <name>` - Start a swarm
- `klaude swarm stop <name>` - Stop a swarm

### MCP Server
- `klaude mcp serve` - Start MCP server
- `klaude mcp status` - Check MCP server status
- `klaude mcp test` - Test MCP server

### SPARC Methodology
- `klaude sparc init <project>` - Initialize SPARC project
- `klaude sparc spec <project> <description>` - Create specification
- `klaude sparc arch <project>` - Design architecture
- `klaude sparc code <project>` - Generate code

## 🔧 Configuration

### Environment Variables

You can customize behavior with these environment variables:

```bash
export CLAUDE_FLOW_LOG_LEVEL=debug    # Logging level
export CLAUDE_FLOW_TIMEOUT=60000      # Command timeout (ms)
export CLAUDE_FLOW_MAX_AGENTS=10      # Maximum agents
export CLAUDE_FLOW_DATA_DIR=~/.claude-flow  # Data directory
```

### Config Files

- Global config: `~/.claude-flow/config.json`
- Project config: `./claude-flow.config.json`
- MCP config: `./claude-flow-mcp-server/config/`

## 🐛 Troubleshooting

### Common Issues

1. **`klaude` command not found**
   ```bash
   export PATH="/home/augment-agent/.local/bin:$PATH"
   # Or restart your terminal
   ```

2. **MCP server not connecting**
   ```bash
   # Check if server is running
   klaude mcp status
   
   # Check logs
   tail -f mcp-server.log
   ```

3. **Permission errors**
   ```bash
   # Make sure klaude is executable
   chmod +x ~/.local/bin/klaude
   ```

### Getting Help

- Run `klaude help` for command help
- Check logs in `~/.claude-flow/logs/`
- Review MCP server logs
- Check the project documentation

## 🎯 Next Steps

1. **Explore the System**: Try different commands and see what's available
2. **Create Your First Agent**: `klaude agent create my-first-agent`
3. **Start a Swarm**: Create multiple agents and coordinate them
4. **Use SPARC**: Try the methodology for structured development
5. **Integrate with Claude Desktop**: Use the MCP tools in conversations

## 📖 Additional Resources

- [Claude-Flow Documentation](./README.md)
- [MCP Server Guide](./claude-flow-mcp-server/README.md)
- [SPARC Methodology](./docs/sparc-methodology.md)
- [Agent Development Guide](./docs/agent-development.md)

---

🎉 **Congratulations!** Your Claude-Flow system is ready to use. Start with `klaude help` to explore all available commands.
