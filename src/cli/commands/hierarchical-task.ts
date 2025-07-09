/**
 * CLI Commands for Hierarchical Task Management
 * Provides CLI interface for org-like agent scaffolding and task delegation
 */

import type { Command, CommandContext } from '../cli-core.js';
import chalk from 'chalk';
import { HierarchicalAgentSystem } from '../../agents/hierarchical-agent-system.js';
import { HierarchicalTaskSpawner } from '../../agents/hierarchical-task-spawner.js';
import { OrganizationalScaffold } from '../../organization/org-scaffold.js';
import { InterAgentCommunicationSystem } from '../../communication/inter-agent-communication.js';
import { AgentManager } from '../../agents/agent-manager.js';
import { TaskCoordinator } from '../../task/coordination.js';
import { DistributedMemorySystem } from '../../memory/distributed-memory.js';
import { generateId } from '../../utils/helpers.js';

interface CLIContext {
  hierarchicalSystem?: HierarchicalAgentSystem;
  taskSpawner?: HierarchicalTaskSpawner;
  orgScaffold?: OrganizationalScaffold;
  communicationSystem?: InterAgentCommunicationSystem;
  agentManager?: AgentManager;
  taskCoordinator?: TaskCoordinator;
  memorySystem?: DistributedMemorySystem;
}

let cliContext: CLIContext = {};

export function createHierarchicalTaskCommand(): Command {
  return {
    name: 'org',
    description: 'Organizational agent management and hierarchical task execution',
    subcommands: [
      {
        name: 'create',
        description: 'Create a new organizational structure',
        options: [
          { name: 'template', short: 't', description: 'Organization template (startup, enterprise, team)', type: 'string', default: 'startup' },
          { name: 'size', short: 's', description: 'Initial organization size', type: 'string', default: '5' },
          { name: 'max-levels', description: 'Maximum hierarchy levels', type: 'string', default: '3' },
          { name: 'departments', description: 'Comma-separated list of departments', type: 'string' }
        ],
        action: async (ctx: CommandContext) => {
          await initializeHierarchicalSystems();
          const name = ctx.args[0];
          const options = ctx.flags;
          if (!name) {
            console.error(chalk.red('❌ Organization name is required'));
            return;
          }
          try {
            console.log(chalk.cyan.bold(`🏢 Creating organization: ${name}`));
            
            const customizations = {
              structure: {
                maxLevels: parseInt(options['max-levels'] as string || '3'),
                spanOfControl: { min: 2, max: 8, default: 5 },
                hierarchyType: 'flat' as const,
                flexibilityLevel: 'adaptive' as const,
                autonomyDistribution: []
              }
            };

            if (options.departments) {
              const deptNames = (options.departments as string).split(',');
              console.log(chalk.blue(`📊 Departments: ${deptNames.join(', ')}`));
            }

            const organization = await cliContext.orgScaffold!.createOrganization(
              options.template as string || 'startup',
              name,
              customizations
            );

            console.log(chalk.green.bold('✅ Organization created successfully!'));
            console.log(chalk.white(`📋 Organization ID: ${organization.id}`));
            console.log(chalk.white(`🎯 Template: ${options.template}`));
            console.log(chalk.white(`👥 Initial agents: ${organization.performanceMetrics.totalAgents}`));
            console.log(chalk.white(`🏗️  Status: ${organization.status}`));

            // Display organization structure
            await displayOrganizationStructure(organization.id);

          } catch (error) {
            console.error(chalk.red.bold('❌ Failed to create organization:'));
            console.error(chalk.red(error instanceof Error ? error.message : String(error)));
          }
        }
      },
      {
        name: 'list',
        description: 'List all organizations',
        options: [
          { name: 'verbose', short: 'v', description: 'Show detailed information', type: 'boolean' }
        ],
        action: async (ctx: CommandContext) => {
          try {
            console.log(chalk.cyan.bold('🏢 Active Organizations:'));
            
            // This would list actual organizations in a real implementation
            console.log(chalk.gray('No organizations found. Create one with: claude-flow org create <name>'));
            
            if (ctx.flags.verbose) {
              console.log(chalk.blue('\nExample commands:'));
              console.log('  claude-flow org create "TechStartup" --template startup');
              console.log('  claude-flow org create "Enterprise" --template enterprise --departments "eng,product,sales"');
            }
          } catch (error) {
            console.error(chalk.red.bold('❌ Failed to list organizations:'));
            console.error(chalk.red(error instanceof Error ? error.message : String(error)));
          }
        }
      },
      {
        name: 'status',
        description: 'Show organization status and metrics',
        options: [
          { name: 'metrics', short: 'm', description: 'Show detailed metrics', type: 'boolean' },
          { name: 'communication', short: 'c', description: 'Show communication patterns', type: 'boolean' }
        ],
        action: async (ctx: CommandContext) => {
          try {
            const orgId = ctx.args[0];
            if (!orgId) {
              console.error(chalk.red('❌ Organization ID is required'));
              return;
            }
            
            console.log(chalk.cyan.bold(`📊 Organization Status: ${orgId}`));
            console.log(chalk.green('✅ Status display would be implemented here'));

          } catch (error) {
            console.error(chalk.red.bold('❌ Failed to get organization status:'));
            console.error(chalk.red(error instanceof Error ? error.message : String(error)));
          }
        }
      },
      {
        name: 'add-agent',
        description: 'Add an agent to an organization',
        options: [
          { name: 'type', short: 't', description: 'Agent type', type: 'string', default: 'specialist' },
          { name: 'role', short: 'r', description: 'Organizational role', type: 'string', default: 'Team Member' }
        ],
        action: async (ctx: CommandContext) => {
          const orgId = ctx.args[0];
          if (!orgId) {
            console.error(chalk.red('❌ Organization ID is required'));
            return;
          }
          console.log(chalk.cyan.bold(`👤 Adding agent to organization: ${orgId}`));
          console.log(chalk.green('✅ Agent addition would be implemented here'));
        }
      },
      {
        name: 'task',
        description: 'Create and delegate tasks within the organization',
        options: [
          { name: 'priority', short: 'p', description: 'Task priority (1-5)', type: 'string', default: '3' }
        ],
        action: async (ctx: CommandContext) => {
          const orgId = ctx.args[0];
          const description = ctx.args[1];
          if (!orgId || !description) {
            console.error(chalk.red('❌ Organization ID and task description are required'));
            return;
          }
          console.log(chalk.cyan.bold(`📋 Creating task in organization: ${orgId}`));
          console.log(chalk.white(`Description: ${description}`));
          console.log(chalk.green('✅ Task creation would be implemented here'));
        }
      },
      {
        name: 'examples',
        description: 'Show example commands and usage patterns',
        action: () => {
          console.log(chalk.cyan.bold('🚀 Hierarchical Task Management Examples'));
          console.log(chalk.blue.bold('\n📚 Basic Organization Management:'));
          console.log(chalk.white('  • Create a startup org: claude-flow org create "MyStartup" --template startup'));
          console.log(chalk.white('  • List organizations: claude-flow org list --verbose'));
          console.log(chalk.white('  • Check org status: claude-flow org status org-12345 --metrics'));
        }
      }
    ]
  };
}

// Helper functions

async function initializeHierarchicalSystems(): Promise<void> {
  try {
    // Initialize logger and event bus
    const { Logger } = await import('../../core/logger.js');
    const { EventBus } = await import('../../core/event-bus.js');
    const logger = new Logger({ 
      level: 'info', 
      format: 'json', 
      destination: 'console',
      filePath: './logs/hierarchical.log',
      maxFileSize: 10 * 1024 * 1024,
      maxFiles: 5
    });
    const eventBus = EventBus.getInstance();

    // Initialize memory system
    cliContext.memorySystem = new DistributedMemorySystem(
      { namespace: 'hierarchical-cli' },
      logger,
      eventBus
    );
    await cliContext.memorySystem.initialize();

    // Initialize agent manager
    cliContext.agentManager = new AgentManager(
      { maxAgents: 100, defaultTimeout: 60000, heartbeatInterval: 30000, healthCheckInterval: 60000, autoRestart: true, resourceLimits: { memory: 1024, cpu: 2, disk: 1000 }, agentDefaults: { autonomyLevel: 0.7, learningEnabled: true, adaptationEnabled: true }, environmentDefaults: { runtime: 'claude', workingDirectory: './agents', tempDirectory: './tmp', logDirectory: './logs' } },
      logger,
      eventBus,
      cliContext.memorySystem
    );

    // Initialize task engine first
    const { TaskEngine } = await import('../../task/engine.js');
    const taskEngine = new TaskEngine();
    
    // Initialize task coordinator
    cliContext.taskCoordinator = new TaskCoordinator(taskEngine, cliContext.memorySystem);

    // Initialize communication system
    cliContext.communicationSystem = new InterAgentCommunicationSystem(cliContext.memorySystem);

    // Initialize hierarchical system
    const hierarchicalConfig = {
      maxDepth: 5,
      maxChildrenPerAgent: 10,
      communicationProtocol: 'event-driven' as const,
      organizationStructure: 'flat' as const,
      delegationRules: [],
      escalationPolicy: {
        levels: [],
        timeouts: new Map(),
        autoEscalate: false,
        notificationChannels: []
      }
    };
    
    cliContext.hierarchicalSystem = new HierarchicalAgentSystem(
      hierarchicalConfig,
      cliContext.agentManager,
      cliContext.taskCoordinator,
      cliContext.memorySystem
    );

    // Initialize task spawner
    cliContext.taskSpawner = new HierarchicalTaskSpawner(
      cliContext.hierarchicalSystem,
      cliContext.taskCoordinator
    );

    // Initialize organizational scaffold
    cliContext.orgScaffold = new OrganizationalScaffold(
      cliContext.hierarchicalSystem,
      cliContext.taskSpawner,
      cliContext.communicationSystem,
      cliContext.agentManager,
      cliContext.taskCoordinator,
      cliContext.memorySystem
    );

    console.log(chalk.green('✅ Hierarchical systems initialized'));
  } catch (error) {
    console.error(chalk.red('❌ Failed to initialize systems:'), error);
    throw error;
  }
}

async function displayOrganizationStructure(orgId: string): Promise<void> {
  console.log(chalk.blue.bold('\n🏗️  Organization Structure:'));
  console.log(chalk.white('  📊 CEO/Founder'));
  console.log(chalk.white('    ├── 🛠️  CTO (Technical Lead)'));
  console.log(chalk.white('    │   ├── 💻 Senior Developer'));
  console.log(chalk.white('    │   └── 🔬 Researcher'));
  console.log(chalk.white('    └── 📈 COO (Operations Lead)'));
  console.log(chalk.white('        ├── 📊 Analyst'));
  console.log(chalk.white('        └── 🎨 Designer'));
  
  console.log(chalk.blue.bold('\n💬 Communication Channels:'));
  console.log(chalk.white('  • daily-standup (all agents)'));
  console.log(chalk.white('  • eng-team (engineering department)'));
  console.log(chalk.white('  • product-team (product department)'));
  console.log(chalk.white('  • leadership (leads and above)'));
}

export { initializeHierarchicalSystems };