# Hierarchical Agent Examples

This directory contains comprehensive examples demonstrating the hierarchical agent system where TASK() agents can spawn their own TASK() agents, creating full org-like scaffolds of communicating and working agents.

## Overview

The hierarchical agent system enables:
- **Agent Spawning**: Agents can spawn specialized sub-agents for tasks
- **Organizational Structures**: Create startup, enterprise, or custom org structures  
- **Inter-Agent Communication**: Structured communication channels and protocols
- **Task Delegation**: Hierarchical task delegation with oversight
- **Lifecycle Management**: Full lifecycle management from spawn to termination
- **Performance Monitoring**: Real-time metrics and organizational health

## Quick Start Examples

### 1. Basic Agent Spawning

```bash
# Create a simple organization
claude-flow org create "TechStartup" --template startup

# Execute a task that spawns specialized agents
claude-flow org task <orgId> "Build user authentication system" --spawn --department eng
```

### 2. Complex Organizational Task

```bash
# Create enterprise organization
claude-flow org create "Enterprise" --template enterprise --departments "eng,product,sales,marketing"

# Execute multi-department coordination task
claude-flow org task <orgId> "Launch new product feature" --collaborate --approval
```

### 3. Agent Communication

```bash
# Direct agent communication
claude-flow org communicate <fromAgent> <toAgent> "Please review the authentication implementation"

# Department-wide broadcast
claude-flow org communicate <leadAgent> eng-team "Sprint planning meeting in 10 minutes"
```

## Example Scenarios

### Scenario 1: Software Development Team
- **CEO Agent**: Strategic oversight and resource allocation
- **Engineering Lead**: Technical leadership and code review
- **Developer Agents**: Feature implementation and bug fixes
- **QA Agent**: Testing and quality assurance
- **DevOps Agent**: Deployment and infrastructure

### Scenario 2: Research Organization  
- **Director Agent**: Research strategy and funding
- **Research Leads**: Project management and coordination
- **Researcher Agents**: Data collection and analysis
- **Analyst Agents**: Statistical analysis and reporting
- **Writer Agents**: Publication and documentation

### Scenario 3: Customer Service Organization
- **Manager Agent**: Team coordination and escalation handling
- **Senior Support Agents**: Complex issue resolution and training
- **Support Agents**: Customer interaction and ticket resolution
- **Specialist Agents**: Technical expertise for specific products
- **Escalation Agents**: Crisis management and executive communication

## File Structure

```
examples/hierarchical-agents/
├── README.md                          # This overview
├── basic-examples/
│   ├── simple-spawn.js                # Basic agent spawning
│   ├── parent-child-communication.js  # Communication patterns
│   └── task-delegation.js             # Task delegation
├── organizational-structures/
│   ├── startup-team.js                # Startup organization
│   ├── enterprise-structure.js       # Large enterprise
│   ├── research-lab.js                # Research organization
│   └── custom-org.js                  # Custom structure
├── advanced-patterns/
│   ├── multi-level-hierarchy.js      # Deep hierarchies
│   ├── cross-functional-teams.js     # Matrix organizations
│   ├── dynamic-scaling.js            # Auto-scaling agents
│   └── fault-tolerance.js            # Error handling
├── communication-examples/
│   ├── broadcast-patterns.js         # Broadcasting messages
│   ├── peer-to-peer.js               # Lateral communication
│   ├── reporting-structures.js       # Hierarchical reporting
│   └── emergency-protocols.js        # Crisis communication
├── lifecycle-management/
│   ├── agent-lifecycle.js            # Lifecycle monitoring
│   ├── performance-management.js     # Performance tracking
│   ├── resource-optimization.js      # Resource management
│   └── succession-planning.js        # Agent replacement
└── real-world-scenarios/
    ├── software-development/          # Complete dev team
    ├── research-project/              # Research organization
    ├── customer-support/              # Support team
    └── content-creation/              # Content production
```

## Key Concepts

### Agent Hierarchies
- **Parent-Child Relationships**: Agents can spawn and manage child agents
- **Sibling Communication**: Peer-to-peer communication between same-level agents
- **Supervision**: Parents monitor and guide child agent behavior

### Organizational Roles
- **Executive**: Strategic decision making and resource allocation
- **Manager**: Team coordination and tactical decisions  
- **Specialist**: Domain expertise and task execution
- **Support**: Auxiliary functions and assistance

### Communication Patterns
- **Hierarchical**: Up/down the org chart
- **Peer-to-Peer**: Same level communication
- **Broadcast**: One-to-many messaging
- **Direct**: Point-to-point communication

### Task Delegation
- **Authority Levels**: Different decision-making powers
- **Approval Workflows**: Multi-stage approval processes
- **Escalation Paths**: Issue escalation procedures
- **Performance Monitoring**: Task completion tracking

## Usage Patterns

### Pattern 1: Specialized Task Teams
When an agent receives a complex task, it can spawn a team of specialized agents:

```javascript
// CEO receives major project
TASK("Develop new AI product line", {
  spawnAgent: true,
  agentType: "coordinator", 
  delegateToChild: true,
  collaborationMode: "team"
});

// This spawns:
// - Product Manager (requirements gathering)
// - Tech Lead (architecture planning)  
// - Researchers (market analysis)
// - Developers (implementation)
```

### Pattern 2: Departmental Scaling
Departments can grow dynamically based on workload:

```javascript
// Engineering lead detects high workload
if (taskQueue.length > threshold) {
  TASK("Scale engineering team", {
    spawnAgent: true,
    agentType: "developer",
    departmentScope: "engineering",
    resourceRequirements: { specializedTools: ["react", "node.js"] }
  });
}
```

### Pattern 3: Cross-Functional Coordination
Projects requiring multiple departments:

```javascript
// Marketing campaign requiring multiple teams
TASK("Launch Q4 marketing campaign", {
  collaborationMode: "cross-functional",
  departments: ["marketing", "design", "engineering", "sales"],
  approvalRequired: true,
  escalationLevel: 2
});
```

## Best Practices

### 1. Hierarchy Design
- Keep hierarchies shallow when possible (max 5 levels)
- Define clear roles and responsibilities
- Establish communication protocols
- Plan for succession and redundancy

### 2. Task Delegation
- Use appropriate delegation levels
- Set clear expectations and deadlines
- Implement progress monitoring
- Plan escalation procedures

### 3. Communication
- Establish regular reporting schedules
- Use appropriate communication channels
- Document important decisions
- Maintain communication history

### 4. Performance Management
- Monitor agent performance metrics
- Implement feedback loops
- Plan for training and development
- Handle underperforming agents

### 5. Resource Management
- Monitor resource utilization
- Plan for peak workloads
- Implement auto-scaling policies
- Optimize resource allocation

## Integration with Claude Code

The hierarchical agent system integrates seamlessly with Claude Code's batch tools:

- **TodoWrite/TodoRead**: Track organizational tasks and progress
- **Task Tool**: Execute coordinated batch operations
- **Memory**: Share organizational knowledge and context
- **Agent Tool**: Spawn and manage agent instances

## Next Steps

1. Try the basic examples to understand core concepts
2. Explore organizational structure templates
3. Experiment with communication patterns
4. Implement custom organizational structures
5. Monitor and optimize performance

For detailed API documentation, see the TypeScript interfaces in the source code.
For specific implementation examples, check the scenario directories.