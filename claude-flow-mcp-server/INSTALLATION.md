# Claude-Flow MCP Server Installation Guide

This guide walks you through installing and configuring the Claude-Flow MCP Server to work with your MCP-compatible client.

## Quick Start

### 1. Prerequisites

- **Node.js 18+**: Download from [nodejs.org](https://nodejs.org/)
- **Claude-Flow**: Install the orchestration system
- **MCP Client**: Claude Desktop, or another MCP-compatible client

### 2. Install Claude-Flow

```bash
# Install Claude-Flow globally
npm install -g claude-flow

# Verify installation
npx claude-flow --help
```

### 3. Install MCP Server

```bash
# Clone or download the MCP server
git clone <repository-url>
cd claude-flow-mcp-server

# Install dependencies
npm install

# Run setup script
node setup.js
```

### 4. Configure Your MCP Client

Add the generated configuration to your MCP client. For Claude Desktop, edit your configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "claude-flow": {
      "command": "node",
      "args": ["/path/to/claude-flow-mcp-server/server.js"],
      "env": {
        "CLAUDE_FLOW_PATH": "npx claude-flow",
        "CLAUDE_FLOW_TIMEOUT": "30000",
        "CLAUDE_FLOW_LOG_LEVEL": "info"
      }
    }
  }
}
```

### 5. Test the Installation

Restart your MCP client and test the connection:

```
Use the claude_flow_status tool to check if the server is working.
```

## Detailed Installation

### Interactive Setup

For a guided setup experience:

```bash
node setup.js --interactive
```

This will prompt you for:
- Claude-Flow command path
- Timeout settings
- Log level preferences

### Manual Configuration

#### Environment Variables

You can configure the server using environment variables:

```bash
export CLAUDE_FLOW_PATH="npx claude-flow"
export CLAUDE_FLOW_TIMEOUT="30000"
export CLAUDE_FLOW_MAX_OUTPUT="1048576"
export CLAUDE_FLOW_LOG_LEVEL="info"
```

#### Configuration File

Create a custom configuration file:

```json
{
  "claudeFlow": {
    "path": "npx claude-flow",
    "timeout": 30000,
    "maxOutputSize": 1048576,
    "logLevel": "info"
  },
  "mcp": {
    "serverName": "claude-flow-mcp-server",
    "version": "1.0.0"
  }
}
```

### Advanced Installation Options

#### Docker Installation

```dockerfile
FROM node:18-alpine

WORKDIR /app
COPY . .

RUN npm install
RUN npm install -g claude-flow

EXPOSE 3000

CMD ["node", "server.js"]
```

```bash
# Build and run
docker build -t claude-flow-mcp .
docker run -p 3000:3000 claude-flow-mcp
```

#### System Service (Linux)

Create a systemd service file:

```ini
[Unit]
Description=Claude-Flow MCP Server
After=network.target

[Service]
Type=simple
User=your-user
WorkingDirectory=/path/to/claude-flow-mcp-server
ExecStart=/usr/bin/node server.js
Restart=always
Environment=CLAUDE_FLOW_PATH=npx claude-flow
Environment=CLAUDE_FLOW_LOG_LEVEL=info

[Install]
WantedBy=multi-user.target
```

```bash
# Install and start service
sudo cp claude-flow-mcp.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable claude-flow-mcp
sudo systemctl start claude-flow-mcp
```

## Client-Specific Setup

### Claude Desktop

1. **Find your config file**:
   - macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - Windows: `%APPDATA%\Claude\claude_desktop_config.json`

2. **Add server configuration**:
```json
{
  "mcpServers": {
    "claude-flow": {
      "command": "node",
      "args": ["/absolute/path/to/claude-flow-mcp-server/server.js"],
      "env": {
        "CLAUDE_FLOW_PATH": "npx claude-flow"
      }
    }
  }
}
```

3. **Restart Claude Desktop**

### Other MCP Clients

For other MCP clients, refer to their documentation for adding MCP servers. The general pattern is:

- **Command**: `node`
- **Args**: `["/path/to/server.js"]`
- **Transport**: `stdio`
- **Capabilities**: `tools`

## Troubleshooting

### Common Issues

#### "Command not found: claude-flow"

```bash
# Install Claude-Flow
npm install -g claude-flow

# Or specify full path
export CLAUDE_FLOW_PATH="/usr/local/bin/claude-flow"
```

#### "Permission denied"

```bash
# Make server executable
chmod +x server.js

# Or run with node explicitly
node server.js
```

#### "Server timeout"

Increase timeout in configuration:

```bash
export CLAUDE_FLOW_TIMEOUT="60000"  # 60 seconds
```

#### "Output too large"

Increase output limit:

```bash
export CLAUDE_FLOW_MAX_OUTPUT="10485760"  # 10MB
```

### Debug Mode

Enable debug logging:

```bash
export CLAUDE_FLOW_LOG_LEVEL="debug"
```

### Testing

Run the test suite:

```bash
npm test
```

Or test manually:

```bash
# Test server startup
node server.js &
SERVER_PID=$!

# Send test request
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | node server.js

# Cleanup
kill $SERVER_PID
```

## Verification

### Check Installation

```bash
# Verify Node.js
node --version

# Verify Claude-Flow
npx claude-flow --help

# Verify MCP Server
node server.js --help
```

### Test Tools

Once configured in your MCP client, test these tools:

1. **claude_flow_status** - Check system status
2. **claude_flow_agent_list** - List agents (should be empty initially)
3. **claude_flow_memory_query** - Query memory (should return empty results)

### Expected Output

Successful tool calls should return structured JSON responses:

```json
{
  "success": true,
  "status": {...},
  "error": "",
  "exitCode": 0
}
```

## Next Steps

After successful installation:

1. **Start Claude-Flow**: Use `claude_flow_start` tool
2. **Spawn Agents**: Create agents with `claude_flow_agent_spawn`
3. **Create Tasks**: Assign work with `claude_flow_task_create`
4. **Monitor System**: Check status with `claude_flow_monitor`

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Enable debug logging
3. Review server logs
4. Test with the included test script
5. Open an issue on GitHub

## Security Notes

- The server executes CLI commands - ensure proper access controls
- Output is limited to prevent memory issues
- Commands timeout to prevent hanging processes
- Only whitelisted commands are allowed
- Environment variables control access levels

For production use, consider additional security measures like sandboxing or containerization.
