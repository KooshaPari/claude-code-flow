# Claude-Flow MCP Server: LLM-to-LLM Delegation

A Model Context Protocol (MCP) server that enables LLMs to delegate complex tasks to Claude-Flow as if it were a subordinate LLM with specialized multi-agent capabilities.

## Overview

This MCP server transforms Claude-Flow into a **subordinate LLM** that your primary LLM can delegate work to. Instead of managing individual agents and tasks, you interact with Claude-Flow as a capable assistant that can:

- **Accept complex task delegations** with natural language descriptions
- **Automatically break down work** into subtasks and coordinate specialized agents
- **Provide progress updates** and accept guidance during execution
- **Deliver structured results** when work is complete
- **Maintain context** across multiple related tasks

Think of it as having a **project manager LLM** with a team of specialists at its disposal.

## Key Delegation Patterns

### 🎯 **Task Delegation**
```
You: "I need market research on AI development trends in 2024"
Claude-Flow: *spawns researcher agents, coordinates data gathering, provides structured report*
```

### 🔄 **Progress Monitoring**
```
You: "How's that research project going?"
Claude-Flow: *provides detailed progress, agent activities, preliminary findings*
```

### 💬 **Interactive Guidance**
```
You: "Focus more on enterprise AI adoption rates"
Claude-Flow: *adjusts research focus, reallocates agent priorities*
```

## Installation

### Prerequisites

- Node.js 18+ 
- Claude-Flow installed and accessible via CLI
- MCP-compatible client (Claude Desktop, etc.)

### Setup

1. **Install the MCP server:**
```bash
npm install -g claude-flow-mcp-server
```

2. **Or clone and install locally:**
```bash
git clone <repository-url>
cd claude-flow-mcp-server
npm install
```

3. **Ensure Claude-Flow is available:**
```bash
npx claude-flow --help
```

## Configuration

### MCP Client Configuration

Add to your MCP client configuration (e.g., Claude Desktop):

```json
{
  "mcpServers": {
    "claude-flow": {
      "command": "node",
      "args": ["path/to/claude-flow-mcp-server/server.js"],
      "env": {
        "CLAUDE_FLOW_PATH": "npx claude-flow",
        "CLAUDE_FLOW_TIMEOUT": "30000",
        "CLAUDE_FLOW_LOG_LEVEL": "info"
      }
    }
  }
}
```

### Environment Variables

- `CLAUDE_FLOW_PATH`: Path to claude-flow executable (default: "npx claude-flow")
- `CLAUDE_FLOW_TIMEOUT`: Command timeout in milliseconds (default: 30000)
- `CLAUDE_FLOW_MAX_OUTPUT`: Maximum output size in bytes (default: 1MB)
- `CLAUDE_FLOW_LOG_LEVEL`: Logging level (debug, info, warn, error)

## Core Delegation Tools

### 🎯 **Primary Delegation Interface**

#### `delegate_to_claude_flow`
**The main tool for delegating complex tasks to Claude-Flow as a subordinate LLM.**

Delegate any complex task that would benefit from multi-agent coordination. Claude-Flow will automatically:
- Analyze the task and determine the best approach
- Spawn appropriate specialized agents (researchers, implementers, analysts, etc.)
- Coordinate the work and manage dependencies
- Store context and maintain progress

**Parameters:**
- `task_description` (string, required): Clear description of what you need done
- `task_type` (enum, required): Primary work type - research, development, analysis, implementation, coordination, review
- `priority` (number): Task priority 1-10 (default: 5)
- `requirements` (array): Specific requirements or constraints
- `expected_deliverables` (array): What outputs you expect
- `context` (string): Additional background information

**Example:**
```json
{
  "task_description": "Research and analyze the competitive landscape for AI code generation tools, focusing on pricing models and feature differentiation",
  "task_type": "research",
  "priority": 8,
  "requirements": ["Include at least 10 major competitors", "Focus on enterprise pricing"],
  "expected_deliverables": ["Competitive analysis report", "Pricing comparison matrix", "Feature gap analysis"],
  "context": "We're planning to launch our own AI coding assistant and need to understand the market positioning"
}
```

#### `check_claude_flow_progress`
**Monitor the progress of delegated work.**

Get status updates on tasks you've delegated, including agent activities and preliminary results.

**Parameters:**
- `task_id` (string): Specific task to check (optional - shows all if not provided)
- `include_details` (boolean): Include detailed progress info (default: true)

#### `ask_claude_flow_question`
**Interactive communication with Claude-Flow during task execution.**

Ask questions, request clarifications, or provide additional guidance while work is in progress.

**Parameters:**
- `question` (string, required): Your question or request
- `context_task_id` (string): Related task ID if applicable
- `urgency` (enum): low, normal, high, critical (default: normal)

### 📋 **Results & Guidance Tools**

#### `retrieve_claude_flow_results`
**Get completed work and deliverables from Claude-Flow.**

Retrieve the final results when delegated tasks are complete.

**Parameters:**
- `task_id` (string, required): Task ID to get results for
- `format` (enum): summary, detailed, raw (default: detailed)
- `include_artifacts` (boolean): Include files/artifacts created (default: true)

#### `instruct_claude_flow`
**Provide specific instructions or course corrections to Claude-Flow.**

Give guidance on how to approach work or make adjustments to ongoing tasks.

**Parameters:**
- `instruction` (string, required): Your instruction or guidance
- `task_id` (string): Related task ID (optional)
- `instruction_type` (enum): guidance, correction, clarification, priority_change, scope_change

#### `get_claude_flow_capabilities`
**Understand what Claude-Flow can do and how to best delegate to it.**

Get information about available agent types, delegation patterns, and capabilities.

**Parameters:**
- `detailed` (boolean): Get detailed capability information (default: false)

### Task Management

#### `claude_flow_task_create`
Create a new task in the system.

**Parameters:**
- `type` (string): Task type (research, implementation, analysis, review, coordination)
- `description` (string): Task description
- `priority` (number): Task priority (1-10)
- `assignTo` (string): Agent ID to assign task to
- `dependencies` (array): List of task IDs this task depends on

#### `claude_flow_task_list`
List all tasks in the system.

**Parameters:**
- `status` (string): Filter tasks by status
- `assignedTo` (string): Filter tasks assigned to specific agent
- `verbose` (boolean): Include detailed task information

### Memory Management

#### `claude_flow_memory_store`
Store information in the system memory.

**Parameters:**
- `key` (string): Memory key/identifier
- `value` (string): Value to store
- `namespace` (string): Memory namespace
- `tags` (array): Tags for categorization

#### `claude_flow_memory_query`
Query information from system memory.

**Parameters:**
- `query` (string): Search query or key
- `namespace` (string): Memory namespace to search in
- `limit` (number): Maximum number of results
- `tags` (array): Filter by tags

### Configuration Management

#### `claude_flow_config_get`
Get configuration values.

**Parameters:**
- `key` (string): Configuration key to retrieve
- `section` (string): Configuration section

#### `claude_flow_config_set`
Set configuration values.

**Parameters:**
- `key` (string): Configuration key to set
- `value` (string): Configuration value
- `section` (string): Configuration section

### Workflow & Session Management

#### `claude_flow_workflow_execute`
Execute a workflow configuration.

**Parameters:**
- `workflow` (string): Workflow name or path to workflow file
- `parameters` (object): Workflow parameters
- `async` (boolean): Execute workflow asynchronously

#### `claude_flow_session_list`
List active terminal sessions.

#### `claude_flow_execute_command`
Execute a command in a terminal session.

**Parameters:**
- `command` (string): Command to execute
- `sessionId` (string): Terminal session ID
- `timeout` (number): Command timeout in seconds
- `cwd` (string): Working directory

## LLM-to-LLM Delegation Examples

### 🎯 **Complete Delegation Workflow**

```javascript
// 1. Start Claude-Flow (if not already running)
await callTool('claude_flow_start', { daemon: true });

// 2. Understand what Claude-Flow can do
const capabilities = await callTool('get_claude_flow_capabilities', { detailed: true });
console.log("Claude-Flow can handle:", capabilities.delegation_patterns);

// 3. Delegate a complex research task
const delegation = await callTool('delegate_to_claude_flow', {
  task_description: "Research the competitive landscape for AI-powered project management tools, focusing on features, pricing, and market positioning",
  task_type: "research",
  priority: 8,
  requirements: [
    "Include at least 15 major competitors",
    "Focus on SaaS pricing models",
    "Analyze AI/ML feature differentiation",
    "Include market size estimates"
  ],
  expected_deliverables: [
    "Competitive analysis report with company profiles",
    "Feature comparison matrix",
    "Pricing analysis with recommendations",
    "Market opportunity assessment"
  ],
  context: "We're developing an AI project management tool and need comprehensive market intelligence to inform our product strategy and go-to-market approach"
});

console.log("Task delegated:", delegation.task_info);
console.log("Next steps:", delegation.next_steps);

// 4. Monitor progress periodically
const progress = await callTool('check_claude_flow_progress', {
  task_id: delegation.task_info.id,
  include_details: true
});

console.log("Current progress:", progress.progress_summary);

// 5. Provide additional guidance if needed
await callTool('ask_claude_flow_question', {
  question: "Please prioritize tools that have raised significant funding in the last 2 years, as these are likely our most serious competitors",
  context_task_id: delegation.task_info.id,
  urgency: "normal"
});

// 6. Give specific instructions for course correction
await callTool('instruct_claude_flow', {
  instruction: "Focus more on enterprise pricing tiers and less on individual user plans. Also include integration capabilities as a key differentiator.",
  task_id: delegation.task_info.id,
  instruction_type: "scope_change"
});

// 7. Retrieve final results when complete
const results = await callTool('retrieve_claude_flow_results', {
  task_id: delegation.task_info.id,
  format: "detailed",
  include_artifacts: true
});

console.log("Final deliverables:", results.deliverables);
```

### 🔄 **Multi-Phase Project Delegation**

```javascript
// Phase 1: Research
const research = await callTool('delegate_to_claude_flow', {
  task_description: "Research user authentication best practices and security standards for web applications",
  task_type: "research",
  priority: 9,
  expected_deliverables: ["Security requirements document", "Best practices guide", "Technology recommendations"]
});

// Phase 2: Implementation (after research is complete)
const implementation = await callTool('delegate_to_claude_flow', {
  task_description: "Implement secure user authentication system based on research findings",
  task_type: "implementation",
  priority: 9,
  requirements: ["Use research findings as specification", "Include multi-factor authentication", "Follow OWASP guidelines"],
  context: `Build upon the research completed in task ${research.task_info.id}`
});

// Phase 3: Review and Testing
const review = await callTool('delegate_to_claude_flow', {
  task_description: "Review and test the implemented authentication system for security vulnerabilities",
  task_type: "review",
  priority: 8,
  requirements: ["Security audit", "Penetration testing", "Code review"],
  context: `Review the implementation from task ${implementation.task_info.id}`
});
```

### 💬 **Interactive Collaboration**

```javascript
// Start a complex analysis task
const analysis = await callTool('delegate_to_claude_flow', {
  task_description: "Analyze our application's performance bottlenecks and recommend optimization strategies",
  task_type: "analysis",
  priority: 7
});

// Check progress and provide feedback
const progress = await callTool('check_claude_flow_progress', {
  task_id: analysis.task_info.id
});

// Ask for clarification
await callTool('ask_claude_flow_question', {
  question: "Are you finding any database-related bottlenecks? We've been seeing slow query performance on the user dashboard.",
  context_task_id: analysis.task_info.id,
  urgency: "high"
});

// Provide additional context
await callTool('instruct_claude_flow', {
  instruction: "Please also consider our recent migration to microservices - some performance issues might be related to inter-service communication overhead.",
  task_id: analysis.task_info.id,
  instruction_type: "clarification"
});
```

## Error Handling

The server provides structured error responses with:
- Success/failure status
- Error messages and codes
- Command output and stderr
- Exit codes from CLI commands

## Security Considerations

- Commands are executed in isolated processes
- Output size is limited to prevent memory issues
- Timeouts prevent hanging processes
- Only whitelisted commands are allowed
- Environment variables control access

## Development

### Running in Development Mode
```bash
npm run dev
```

### Testing
```bash
npm test
```

### Debugging
Set `CLAUDE_FLOW_LOG_LEVEL=debug` for detailed logging.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

For issues and questions:
- GitHub Issues: [repository-url]/issues
- Documentation: [repository-url]/docs
- Claude-Flow Documentation: https://github.com/ruvnet/claude-code-flow
