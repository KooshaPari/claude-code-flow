# Hierarchical Agents User Guide

## 🚀 Complete Guide to Using Hierarchical Agent Systems in Claude-Flow

### Table of Contents
1. [Quick Start](#quick-start)
2. [Basic Concepts](#basic-concepts)
3. [Setting Up Organizations](#setting-up-organizations)
4. [Agent Spawning](#agent-spawning)
5. [Task Delegation](#task-delegation)
6. [Communication Systems](#communication-systems)
7. [QuDAG Security Integration](#qudag-security-integration)
8. [Real-World Examples](#real-world-examples)
9. [CLI Commands Reference](#cli-commands-reference)
10. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Prerequisites
- Claude-Flow installed and configured
- Node.js environment set up
- Basic understanding of agent systems

### 30-Second Setup
```bash
# 1. Initialize hierarchical agent system
./claude-flow org create "MyCompany" --template startup

# 2. Create your first agent hierarchy
./claude-flow org add-agent coordinator "CEO" --permissions spawn-agent,delegate-task

# 3. Test agent spawning
./claude-flow org task <orgId> "Research market trends" --spawn --department research
```

---

## Basic Concepts

### What are Hierarchical Agents?

Hierarchical agents are AI agents that can spawn and manage other agents, creating organizational structures similar to companies, teams, or complex project hierarchies.

**Key Features:**
- **Parent-Child Relationships**: Agents can create and manage sub-agents
- **Organizational Structures**: Build teams, departments, and companies
- **Task Delegation**: Break down complex tasks and assign to specialists
- **Communication Networks**: Secure messaging between agents
- **Lifecycle Management**: Automatic agent creation, monitoring, and cleanup

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                   CEO Agent (Level 0)                      │
├─────────────────────┬───────────────────────────────────────┤
│   Engineering Lead │           Product Lead               │
│     (Level 1)      │             (Level 1)               │
├──────────┬──────────┼──────────────┬────────────────────────┤
│ Senior   │ Junior   │ UX Designer  │ Product Manager        │
│Developer │Developer │   (Level 2)  │    (Level 2)           │
│(Level 2) │(Level 2) │              │                        │
└──────────┴──────────┴──────────────┴────────────────────────┘
```

---

## Setting Up Organizations

### 1. Create Basic Organization

```bash
# Startup organization (2-3 levels, agile structure)
./claude-flow org create "TechStartup" --template startup

# Enterprise organization (4-5 levels, formal structure)
./claude-flow org create "MegaCorp" --template enterprise

# Custom organization
./claude-flow org create "CustomTeam" --template custom \
  --max-levels 3 \
  --departments "engineering,product,sales"
```

### 2. Programmatic Organization Setup

```javascript
import { OrganizationalScaffold } from './src/organization/org-scaffold.js';

// Create organization scaffold
const orgScaffold = new OrganizationalScaffold(
  hierarchicalSystem, taskSpawner, communicationSystem,
  agentManager, taskCoordinator, memory
);

// Create startup organization
const startup = await orgScaffold.createOrganization(
  'startup',
  'MyStartup',
  {
    structure: {
      maxLevels: 3,
      spanOfControl: { min: 2, max: 6, default: 4 }
    },
    departments: [
      {
        id: 'engineering',
        name: 'Engineering',
        requiredRoles: ['tech-lead', 'senior-developer', 'developer'],
        targetSize: { min: 3, max: 10, optimal: 6 }
      }
    ]
  }
);
```

### 3. Add Agents to Organization

```bash
# Add executive level agent
./claude-flow org add-agent coordinator "CTO" \
  --level 1 \
  --permissions spawn-agent,delegate-task,make-decision \
  --department engineering

# Add specialist agent
./claude-flow org add-agent specialist "Senior Developer" \
  --level 2 \
  --parent <cto-agent-id> \
  --capabilities code-generation,testing,review
```

---

## Agent Spawning

### 1. Basic Agent Spawning with TASK()

```javascript
import { HierarchicalTaskSpawner } from './src/agents/hierarchical-task-spawner.js';

// Enhanced TASK() function with spawning capabilities
const result = await TASK(
  "Develop user authentication system",
  {
    priority: 'high',
    spawnAgent: true,              // Enable agent spawning
    agentType: 'developer',        // Type of agent to spawn
    agentRole: 'Backend Developer', // Specific role
    requiresSpecialist: true,      // Needs specialized skills
    resourceRequirements: {
      specializedTools: ['node.js', 'postgresql', 'jwt'],
      teamSize: 1
    }
  },
  {
    agentId: 'parent-agent-001',   // Parent agent context
    permissions: ['spawn-agent', 'delegate-task'],
    currentTasks: [],
    children: []
  }
);

console.log(`Spawned agent: ${result.agentId}`);
console.log(`Task assigned: ${result.taskId}`);
```

### 2. Team Spawning

```javascript
// Spawn entire team for complex project
const teamResult = await TASK(
  "Build e-commerce platform",
  {
    priority: 'high',
    spawnAgent: true,
    collaborationMode: 'team',     // Spawn multiple agents
    resourceRequirements: {
      teamSize: 4,                 // Team of 4 agents
      specializedTools: ['react', 'node.js', 'postgresql', 'stripe']
    },
    departmentScope: 'engineering'
  },
  managerContext
);

// Result includes multiple spawned agents
teamResult.spawnedAgents.forEach(agent => {
  console.log(`Team member: ${agent.id} (${agent.type})`);
});
```

### 3. Conditional Spawning

```javascript
// Spawn agents based on workload
const currentWorkload = await getAgentWorkload(parentAgentId);

if (currentWorkload > 80) {  // 80% capacity
  await TASK(
    "Scale team to handle increased workload",
    {
      spawnAgent: true,
      agentType: 'specialist',
      collaborationMode: 'team',
      resourceRequirements: {
        teamSize: 2  // Spawn 2 additional agents
      }
    },
    parentContext
  );
}
```

---

## Task Delegation

### 1. Simple Task Delegation

```bash
# Delegate task through CLI
./claude-flow org task <orgId> "Analyze customer feedback" \
  --delegate \
  --to-department product \
  --priority high
```

### 2. Complex Task Breakdown

```javascript
// Complex project with automatic breakdown
const projectResult = await TASK(
  "Launch new mobile app",
  {
    priority: 'critical',
    spawnAgent: true,
    collaborationMode: 'cross-functional',
    departments: ['engineering', 'product', 'design', 'marketing'],
    approvalRequired: true,
    escalationLevel: 2
  },
  ceoContext
);

// System automatically creates:
// 1. Project manager agent
// 2. Engineering team (3-4 developers)
// 3. Design team (UI/UX specialists)
// 4. Product team (PM, researcher)
// 5. Marketing team (growth specialists)
```

### 3. Delegation with Supervision

```javascript
// Parent agent monitors child agents
const delegation = await setupDelegationCommunication(
  parentAgent,
  childAgent,
  taskId
);

// Automatic reporting every 30 minutes
// Parent receives progress updates
// Escalation if child agent encounters issues
```

---

## Communication Systems

### 1. Create Communication Channels

```javascript
// Create department-wide channel
const engChannel = await communicationSystem.createChannel(
  'engineering-team',
  'broadcast',
  techLeadAgent,
  [dev1, dev2, dev3],
  {
    isPublic: false,
    requiresApproval: true
  }
);

// Create hierarchical reporting channel
const reportingChannel = await communicationSystem.createChannel(
  'executive-reports',
  'hierarchical',
  ceoAgent,
  [cto, cpo, cmo]
);
```

### 2. Send Messages Between Agents

```javascript
// Standard message
await communicationSystem.sendMessage(
  fromAgent,
  toAgent,
  'request',
  {
    subject: 'Code Review Request',
    body: 'Please review the authentication module',
    data: { pullRequest: 'PR-123', priority: 'high' }
  }
);

// Secure message with QuDAG
await communicationSystem.sendMessage(
  fromAgent,
  toAgent,
  'notification',
  {
    subject: 'Confidential Project Update',
    body: 'Classified information...'
  },
  {
    useQuDAG: true,                 // Use quantum-resistant encryption
    anonymityLevel: 'high',         // Anonymous routing
    quantumResistant: true          // Post-quantum cryptography
  }
);
```

### 3. Broadcast to Teams

```bash
# CLI broadcast
./claude-flow org communicate <leaderId> engineering-team \
  "Sprint planning meeting in 10 minutes"

# Programmatic broadcast
await communicationSystem.broadcastMessage(
  managerAgent,
  'team-channel-id',
  {
    subject: 'All Hands Meeting',
    body: 'Quarterly review at 3 PM'
  },
  { priority: 'high' }
);
```

---

## QuDAG Security Integration

### 1. Initialize Quantum-Resistant Network

```bash
# Initialize QuDAG network
./claude-flow org qudag init \
  --node-id secure-company-node \
  --dark-domain mycompany.dark \
  --quantum-resistant \
  --onion-routing
```

### 2. Create Secure Swarms

```bash
# Create secure financial team
./claude-flow org qudag swarm create financial-ops \
  --type hierarchical \
  --agents finance-lead,analyst1,analyst2,compliance-officer \
  --consensus dag-consensus \
  --anonymous \
  --resources
```

### 3. Secure Communication Examples

```javascript
// Financial services secure communication
const financialSwarm = await communicationSystem.createQuDAGSwarm(
  'financial-processing',
  'hierarchical',
  [
    {
      agentId: 'risk-manager',
      capabilities: ['risk-analysis', 'compliance'],
      communicationPreferences: {
        anonymityLevel: 'high',
        routingStrategy: 'onion',
        encryptionRequired: true
      }
    },
    {
      agentId: 'fraud-detector',
      capabilities: ['fraud-detection', 'pattern-analysis'],
      communicationPreferences: {
        anonymityLevel: 'high',
        routingStrategy: 'onion',
        encryptionRequired: true
      }
    }
  ],
  {
    consensusProtocol: 'dag-consensus',
    resourceSharingEnabled: true
  }
);

// Send quantum-resistant message
await communicationSystem.sendMessage(
  riskManager,
  fraudDetector,
  'alert',
  {
    subject: 'Suspicious Transaction Alert',
    body: 'Transaction ID TX-789123 flagged for review',
    data: { amount: 50000, flags: ['high-value', 'foreign'] }
  },
  {
    useQuDAG: true,
    anonymityLevel: 'high',
    quantumResistant: true,
    priority: 1
  }
);
```

---

## Real-World Examples

### 1. Software Development Team

```javascript
// Complete software development workflow
async function createDevTeam() {
  // 1. Create organization
  const company = await orgScaffold.createOrganization('startup', 'TechCorp');
  
  // 2. Add CTO
  const cto = await orgScaffold.addAgentToOrganization(
    company.id,
    'coordinator',
    {
      title: 'Chief Technology Officer',
      permissions: ['spawn-agent', 'delegate-task', 'make-decision']
    },
    'engineering'
  );
  
  // 3. CTO spawns development team
  const devProject = await TASK(
    "Build microservices architecture",
    {
      spawnAgent: true,
      collaborationMode: 'team',
      resourceRequirements: {
        teamSize: 5,
        specializedTools: ['docker', 'kubernetes', 'node.js', 'postgresql']
      }
    },
    { agentId: cto.id, permissions: ['spawn-agent'] }
  );
  
  // 4. Team automatically includes:
  // - Backend Lead
  // - 2x Backend Developers  
  // - DevOps Engineer
  // - QA Engineer
  
  return devProject;
}
```

### 2. Research Organization

```javascript
// Research lab with specialized teams
async function createResearchLab() {
  const lab = await orgScaffold.createOrganization('custom', 'AI Research Lab', {
    departments: [
      {
        id: 'nlp',
        name: 'Natural Language Processing',
        requiredRoles: ['research-lead', 'ml-engineer', 'data-scientist']
      },
      {
        id: 'computer-vision',
        name: 'Computer Vision',
        requiredRoles: ['vision-lead', 'cv-engineer', 'annotation-specialist']
      }
    ]
  });
  
  // Research director spawns specialized teams
  const nlpProject = await TASK(
    "Develop next-generation language model",
    {
      spawnAgent: true,
      agentType: 'researcher',
      departmentScope: 'nlp',
      resourceRequirements: {
        teamSize: 4,
        specializedTools: ['pytorch', 'huggingface', 'wandb']
      }
    },
    directorContext
  );
  
  return nlpProject;
}
```

### 3. Customer Support Organization

```javascript
// Multi-tier support with escalation
async function createSupportOrg() {
  const support = await orgScaffold.createOrganization('enterprise', 'Support Division');
  
  // Support manager spawns tiered support structure
  const supportStructure = await TASK(
    "Handle customer support tickets",
    {
      spawnAgent: true,
      collaborationMode: 'hierarchical',
      resourceRequirements: {
        teamSize: 8,
        specializedTools: ['zendesk', 'slack', 'knowledge-base']
      }
    },
    supportManagerContext
  );
  
  // Creates:
  // - Level 1: General support agents (4 agents)
  // - Level 2: Technical specialists (2 agents)  
  // - Level 3: Senior engineers (1 agent)
  // - Escalation: Manager (1 agent)
  
  return supportStructure;
}
```

---

## CLI Commands Reference

### Organization Management

```bash
# Create organizations
./claude-flow org create <name> --template [startup|enterprise|custom]
./claude-flow org list
./claude-flow org status <orgId>
./claude-flow org delete <orgId>

# Agent management
./claude-flow org add-agent <type> <name> --level <level> --parent <parentId>
./claude-flow org list-agents <orgId>
./claude-flow org remove-agent <agentId>

# Task management
./claude-flow org task <orgId> "<description>" --spawn --department <dept>
./claude-flow org task <orgId> "<description>" --delegate --to <agentId>
./claude-flow org list-tasks <orgId>
./claude-flow org task-status <taskId>
```

### Communication Commands

```bash
# Direct communication
./claude-flow org communicate <fromAgent> <toAgent> "<message>"

# Broadcast
./claude-flow org communicate <fromAgent> <channelId> "<message>" --broadcast

# Create channels
./claude-flow org create-channel <name> --type [direct|broadcast|hierarchical]
```

### QuDAG Security Commands

```bash
# Initialize QuDAG
./claude-flow org qudag init --node-id <id> --dark-domain <domain>

# Secure swarms
./claude-flow org qudag swarm create <swarmId> --type <type> --anonymous
./claude-flow org qudag swarm list
./claude-flow org qudag swarm status <swarmId>

# Secure messaging
./claude-flow org qudag send <from> <to> "<message>" --anonymous --encrypt

# Resource sharing
./claude-flow org qudag resource share --type <type> --amount <amount>
./claude-flow org qudag resource list
./claude-flow org qudag resource buy --type <type> --amount <amount>

# Security tools
./claude-flow org qudag security audit
./claude-flow org qudag security encrypt --file <path>
./claude-flow org qudag status --detailed
```

---

## Advanced Patterns

### 1. Dynamic Scaling

```javascript
// Auto-scaling based on workload
class AutoScalingManager {
  async monitorAndScale(organizationId) {
    const metrics = await this.getOrganizationMetrics(organizationId);
    
    if (metrics.workload > 0.8) {  // 80% capacity
      await TASK(
        "Scale team to handle increased workload",
        {
          spawnAgent: true,
          agentType: 'specialist',
          resourceRequirements: {
            teamSize: Math.ceil(metrics.workload * 2)
          }
        },
        { agentId: metrics.managerId }
      );
    }
  }
}
```

### 2. Cross-Organizational Collaboration

```javascript
// Secure collaboration between companies
async function setupPartnership() {
  // Company A creates secure swarm
  const partnershipSwarm = await communicationSystem.createQuDAGSwarm(
    'partnership-alpha-beta',
    'mesh',
    [
      { agentId: 'company-a-liaison', capabilities: ['negotiation', 'legal'] },
      { agentId: 'company-b-liaison', capabilities: ['technical', 'integration'] }
    ],
    {
      consensusProtocol: 'voting',
      resourceSharingEnabled: true
    }
  );
  
  // Secure cross-company communication
  await communicationSystem.sendMessage(
    companyALiaison,
    companyBLiaison,
    'proposal',
    {
      subject: 'Joint Development Proposal',
      body: 'Proposing collaboration on AI research project',
      data: { budget: 1000000, timeline: '6 months' }
    },
    {
      useQuDAG: true,
      anonymityLevel: 'medium',
      quantumResistant: true
    }
  );
}
```

### 3. Fault Tolerance and Recovery

```javascript
// Agent failure recovery
class FailureRecoveryManager {
  async handleAgentFailure(failedAgentId) {
    const agent = await this.getAgentDetails(failedAgentId);
    
    if (agent.hasChildren) {
      // Reassign children to backup agent
      await this.reassignChildren(agent.children, agent.backupAgent);
    }
    
    if (agent.hasCriticalTasks) {
      // Spawn replacement agent
      const replacement = await TASK(
        `Replace failed ${agent.role}`,
        {
          spawnAgent: true,
          agentType: agent.type,
          resourceRequirements: agent.capabilities,
          urgent: true
        },
        { agentId: agent.parentId }
      );
      
      // Transfer tasks to replacement
      await this.transferTasks(failedAgentId, replacement.agentId);
    }
  }
}
```

---

## Best Practices

### 1. Organizational Design

**Keep Hierarchies Shallow**
```javascript
// Good: 3 levels maximum for startups
CEO → Department Lead → Specialist

// Avoid: Deep hierarchies (5+ levels)
CEO → VP → Director → Manager → Lead → Specialist
```

**Optimal Span of Control**
```javascript
// Recommended: 3-7 direct reports per manager
const optimalSpan = {
  executive: 3-5,    // CEO managing department heads
  manager: 4-7,      // Department lead managing specialists
  specialist: 1-3    // Senior developer managing juniors
};
```

### 2. Task Delegation

**Break Down Complex Tasks**
```javascript
// Good: Specific, actionable tasks
await TASK("Implement user authentication API endpoints", options);
await TASK("Design database schema for user management", options);
await TASK("Create frontend login interface", options);

// Avoid: Overly broad tasks
await TASK("Build the entire application", options);
```

**Use Appropriate Agent Types**
```javascript
// Match agent type to task requirements
const taskAgentMapping = {
  'research': 'researcher',
  'coding': 'developer', 
  'testing': 'tester',
  'review': 'reviewer',
  'coordination': 'coordinator'
};
```

### 3. Communication Patterns

**Choose Right Channel Type**
```javascript
// Direct: One-on-one communication
await createChannel('peer-review', 'direct', dev1, [dev2]);

// Broadcast: Team announcements
await createChannel('team-updates', 'broadcast', manager, team);

// Hierarchical: Reporting structure
await createChannel('status-reports', 'hierarchical', director, managers);
```

### 4. Security Considerations

**Use QuDAG for Sensitive Data**
```javascript
// Financial, legal, or confidential communications
if (messageType === 'confidential') {
  options.useQuDAG = true;
  options.anonymityLevel = 'high';
  options.quantumResistant = true;
}
```

**Implement Proper Access Controls**
```javascript
const permissions = {
  executive: ['spawn-agent', 'delegate-task', 'make-decision', 'access-all'],
  manager: ['spawn-agent', 'delegate-task', 'access-department'],
  specialist: ['execute-task', 'report-status', 'access-own']
};
```

---

## Troubleshooting

### Common Issues

#### 1. Agent Spawning Failures

**Problem**: Agent spawning fails with permission errors
```bash
Error: Agent does not have spawn-agent permission
```

**Solution**: Grant spawning permissions to parent agent
```bash
./claude-flow org update-agent <agentId> --add-permission spawn-agent
```

#### 2. Communication Channel Issues

**Problem**: Messages not reaching target agents
```bash
Error: Agent not subscribed to channel
```

**Solution**: Check channel subscriptions
```bash
./claude-flow org list-channels <orgId>
./claude-flow org subscribe-agent <agentId> <channelId>
```

#### 3. QuDAG Network Issues

**Problem**: Quantum-resistant messages failing
```bash
Error: QuDAG network not initialized
```

**Solution**: Initialize QuDAG network
```bash
./claude-flow org qudag init --node-id company-node
./claude-flow org qudag status
```

#### 4. Task Delegation Failures

**Problem**: Tasks not being delegated properly
```bash
Error: No suitable agent found for task
```

**Solution**: Check agent capabilities and availability
```javascript
// Verify agent has required capabilities
const agent = await getAgentDetails(agentId);
console.log('Agent capabilities:', agent.capabilities);

// Check agent workload
const workload = await getAgentWorkload(agentId);
if (workload > 0.9) {
  console.log('Agent overloaded, spawning additional agent');
}
```

### Performance Optimization

#### 1. Memory Management
```javascript
// Regular cleanup of completed tasks
setInterval(async () => {
  await cleanupCompletedTasks();
  await archiveOldMessages();
}, 24 * 60 * 60 * 1000); // Daily
```

#### 2. Communication Optimization
```javascript
// Batch messages when possible
const messageBatch = [];
for (const agent of teamAgents) {
  messageBatch.push({
    to: agent,
    content: statusUpdate
  });
}
await sendBatchMessages(messageBatch);
```

#### 3. Resource Monitoring
```bash
# Monitor system resources
./claude-flow org status <orgId> --detailed
./claude-flow org qudag status --detailed

# Check agent performance
./claude-flow org agent-metrics <agentId>
```

---

## Getting Help

### Documentation Resources
- **API Reference**: Complete TypeScript interfaces and method documentation
- **Examples Repository**: `/examples/hierarchical-agents/` directory
- **CLI Help**: `./claude-flow org --help` for command reference

### Support Channels
- **GitHub Issues**: Report bugs and feature requests
- **Community Forum**: Discuss implementation patterns
- **Documentation**: Comprehensive guides and tutorials

### Debugging Tools
```bash
# Enable debug logging
export CLAUDE_FLOW_DEBUG=true

# Verbose output
./claude-flow org status <orgId> --verbose

# System health check
./claude-flow org health-check
```

---

This comprehensive user guide covers everything needed to effectively use the hierarchical agent system in Claude-Flow. Start with the Quick Start section for immediate usage, then dive deeper into specific areas as needed for your use case.