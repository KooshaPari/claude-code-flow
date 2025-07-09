/**
 * Simple Agent Spawning Example
 * Demonstrates how a TASK() agent can spawn its own sub-agents
 */

import { HierarchicalTaskSpawner } from '../../../src/agents/hierarchical-task-spawner.js';
import { HierarchicalAgentSystem } from '../../../src/agents/hierarchical-agent-system.js';

async function basicAgentSpawningExample() {
  console.log('🚀 Basic Agent Spawning Example');
  console.log('===============================\n');

  // Initialize the hierarchical task spawner
  const taskSpawner = new HierarchicalTaskSpawner(hierarchicalSystem, taskCoordinator);

  // Example 1: Parent agent spawns a specialized researcher
  console.log('📋 Example 1: Spawning a Research Agent');
  console.log('--------------------------------------');

  // Parent agent context
  const parentContext = {
    agentId: 'parent-agent-001',
    hierarchyId: 'example-hierarchy',
    role: 'coordinator',
    permissions: ['spawn-agent', 'delegate-task', 'access-memory'],
    currentTasks: [],
    children: [],
    availableResources: {
      cpu: 4,
      memory: 8192,
      specializedTools: ['research', 'analysis']
    }
  };

  // Execute task that requires specialized research
  const researchTaskResult = await taskSpawner.TASK(
    "Research emerging AI trends in healthcare for the next 6 months",
    {
      priority: 'high',
      spawnAgent: true,
      agentType: 'researcher',
      agentRole: 'AI Research Specialist',
      requiresSpecialist: true,
      resourceRequirements: {
        specializedTools: ['web-search', 'data-analysis', 'report-generation'],
        teamSize: 1
      },
      reportingFrequency: 1800000, // 30 minutes
      statusUpdates: true
    },
    parentContext
  );

  console.log(`✅ Research task initiated: ${researchTaskResult.taskId}`);
  console.log(`🤖 Spawned agent: ${researchTaskResult.agentId}`);
  console.log(`📊 New agents created: ${researchTaskResult.spawnedAgents?.length || 0}`);
  
  if (researchTaskResult.spawnedAgents) {
    researchTaskResult.spawnedAgents.forEach((agent, index) => {
      console.log(`   ${index + 1}. ${agent.id} (${agent.type})`);
    });
  }

  console.log('\n');

  // Example 2: Parent agent spawns multiple specialized agents for complex task
  console.log('📋 Example 2: Spawning Multiple Agents for Complex Task');
  console.log('-----------------------------------------------------');

  const complexTaskResult = await taskSpawner.TASK(
    "Develop a complete user authentication system with modern security practices",
    {
      priority: 'high',
      spawnAgent: true,
      agentType: 'developer',
      collaborationMode: 'team',
      resourceRequirements: {
        specializedTools: ['node.js', 'react', 'postgresql', 'security-tools'],
        teamSize: 3
      },
      departmentScope: 'engineering',
      approvalRequired: false,
      statusUpdates: true
    },
    parentContext
  );

  console.log(`✅ Development task initiated: ${complexTaskResult.taskId}`);
  console.log(`🤖 Lead agent: ${complexTaskResult.agentId}`);
  console.log(`👥 Team size: ${complexTaskResult.spawnedAgents?.length || 0} agents`);

  if (complexTaskResult.organizationalImpact) {
    console.log('\n📊 Organizational Impact:');
    console.log(`   New agents: ${complexTaskResult.organizationalImpact.newAgents}`);
    console.log(`   Resource utilization: ${(complexTaskResult.organizationalImpact.resourceUtilization * 100).toFixed(1)}%`);
    console.log(`   Team efficiency: ${(complexTaskResult.organizationalImpact.teamEfficiency * 100).toFixed(1)}%`);
  }

  console.log('\n');

  // Example 3: Specialized agent spawns its own sub-agents
  console.log('📋 Example 3: Sub-Agent Spawning Sub-Agents');
  console.log('------------------------------------------');

  // Simulate the spawned research agent spawning its own assistants
  const subAgentContext = {
    agentId: researchTaskResult.agentId || 'research-agent-001',
    hierarchyId: 'example-hierarchy',
    parentAgent: parentContext.agentId,
    role: 'researcher',
    permissions: ['spawn-agent', 'research', 'analyze'],
    currentTasks: [researchTaskResult.taskId],
    children: [],
    availableResources: {
      cpu: 2,
      memory: 4096,
      specializedTools: ['research', 'analysis']
    }
  };

  const subResearchResult = await taskSpawner.TASK(
    "Gather and analyze healthcare AI patent filings from the last 12 months",
    {
      priority: 'medium',
      spawnAgent: true,
      agentType: 'analyst',
      agentRole: 'Patent Research Assistant',
      requiresSpecialist: true,
      resourceRequirements: {
        specializedTools: ['patent-search', 'legal-analysis', 'data-visualization']
      }
    },
    subAgentContext
  );

  console.log(`✅ Sub-research task initiated: ${subResearchResult.taskId}`);
  console.log(`🤖 Assistant agent: ${subResearchResult.agentId}`);
  console.log(`🌳 Hierarchy depth: 3 levels (Parent -> Researcher -> Assistant)`);

  console.log('\n');

  // Example 4: Dynamic agent spawning based on workload
  console.log('📋 Example 4: Dynamic Agent Spawning');
  console.log('-----------------------------------');

  // Simulate high workload scenario
  const currentWorkload = 15; // Simulated high workload
  const workloadThreshold = 10;

  if (currentWorkload > workloadThreshold) {
    console.log(`⚠️  High workload detected: ${currentWorkload} tasks (threshold: ${workloadThreshold})`);
    
    const scalingResult = await taskSpawner.TASK(
      "Scale team to handle increased workload",
      {
        priority: 'high',
        spawnAgent: true,
        agentType: 'specialist',
        collaborationMode: 'team',
        resourceRequirements: {
          teamSize: 2,
          specializedTools: ['load-balancing', 'task-distribution']
        }
      },
      parentContext
    );

    console.log(`✅ Scaling completed: ${scalingResult.taskId}`);
    console.log(`📈 Added ${scalingResult.spawnedAgents?.length || 0} new agents`);
  }

  console.log('\n');

  // Example 5: Agent spawning with specific constraints
  console.log('📋 Example 5: Constrained Agent Spawning');
  console.log('---------------------------------------');

  const constrainedTaskResult = await taskSpawner.TASK(
    "Perform security audit of existing codebase",
    {
      priority: 'high',
      spawnAgent: true,
      agentType: 'reviewer',
      agentRole: 'Security Auditor',
      requiresSpecialist: true,
      approvalRequired: true,
      escalationLevel: 2,
      resourceRequirements: {
        specializedTools: ['security-scanner', 'code-analysis', 'vulnerability-detection'],
        teamSize: 1
      },
      // Constraints for security-sensitive task
      timeout: 7200000, // 2 hours
      departmentScope: 'security',
      stakeholderNotifications: ['security-lead', 'cto']
    },
    parentContext
  );

  console.log(`✅ Security audit initiated: ${constrainedTaskResult.taskId}`);
  console.log(`🔒 Security specialist assigned: ${constrainedTaskResult.agentId}`);
  console.log(`⏰ Time limit: 2 hours`);
  console.log(`📬 Stakeholders notified: security-lead, cto`);

  console.log('\n🎉 Basic agent spawning examples completed!');
  console.log('\nKey takeaways:');
  console.log('• Agents can spawn specialized sub-agents for complex tasks');
  console.log('• Sub-agents can spawn their own assistants, creating deep hierarchies');
  console.log('• Spawning can be dynamic based on workload and conditions');
  console.log('• Constraints and approvals can control agent spawning');
  console.log('• Each spawned agent inherits appropriate permissions and resources');
}

// Run the example
async function runExample() {
  try {
    await basicAgentSpawningExample();
  } catch (error) {
    console.error('❌ Example failed:', error);
  }
}

// Export for use in other examples
export { basicAgentSpawningExample, runExample };

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  runExample();
}