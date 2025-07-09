/**
 * Hierarchical Agent System - Enables TASK() agents to spawn their own TASK() agents
 * Creates org-like scaffolds with parent-child relationships and communication
 */

import { EventEmitter } from 'node:events';
import { AgentState, AgentId, AgentType, TaskDefinition, AgentCapabilities } from '../swarm/types.js';
import { TaskCoordinator } from '../task/coordination.js';
import { AgentManager } from './agent-manager.js';
import { DistributedMemorySystem } from '../memory/distributed-memory.js';
import { generateId } from '../utils/helpers.js';

export interface HierarchicalAgentConfig {
  maxDepth: number;
  maxChildrenPerAgent: number;
  communicationProtocol: 'event-driven' | 'message-queue' | 'shared-memory';
  organizationStructure: 'flat' | 'tree' | 'matrix' | 'network';
  delegationRules: DelegationRule[];
  escalationPolicy: EscalationPolicy;
}

export interface AgentHierarchy {
  id: string;
  rootAgent: AgentId;
  nodes: Map<string, HierarchyNode>;
  relationships: Map<string, string[]>; // parentId -> childIds[]
  communicationGraph: Map<string, string[]>; // agentId -> connectedAgentIds[]
  depth: number;
  totalAgents: number;
}

export interface HierarchyNode {
  agent: AgentState;
  parent?: AgentId;
  children: AgentId[];
  siblings: AgentId[];
  level: number;
  role: OrganizationalRole;
  responsibilities: string[];
  capabilities: AgentCapabilities;
  delegates: AgentId[];
  reports: AgentId[];
}

export interface OrganizationalRole {
  title: string;
  type: 'manager' | 'coordinator' | 'specialist' | 'support' | 'executive';
  level: number;
  permissions: Permission[];
  canSpawnAgents: boolean;
  maxSubordinates: number;
  reportingFrequency: number;
  decisionAuthority: string[];
}

export interface Permission {
  action: string;
  resource: string;
  conditions?: string[];
  constraints?: Record<string, any>;
}

export interface DelegationRule {
  condition: string;
  targetRole: string;
  taskTypes: string[];
  priority: number;
  autoDelegate: boolean;
}

export interface EscalationPolicy {
  levels: EscalationLevel[];
  timeouts: Map<number, number>;
  autoEscalate: boolean;
  notificationChannels: string[];
}

export interface EscalationLevel {
  level: number;
  targetRole: string;
  conditions: string[];
  actions: string[];
}

export interface AgentSpawnRequest {
  requesterId: string;
  agentType: AgentType;
  role: OrganizationalRole;
  purpose: string;
  capabilities: AgentCapabilities;
  lifetime: 'permanent' | 'temporary' | 'task-bound';
  priority: number;
  resources: ResourceRequirements;
  constraints?: SpawnConstraints;
}

export interface ResourceRequirements {
  cpu: number;
  memory: number;
  disk: number;
  network: number;
  specialResources?: string[];
  specializedTools?: string[];
}

export interface SpawnConstraints {
  maxRuntime?: number;
  allowedActions?: string[];
  restrictedResources?: string[];
  communicationLimits?: CommunicationLimits;
}

export interface CommunicationLimits {
  maxMessagesPerSecond: number;
  allowedTargets?: string[];
  blockedTargets?: string[];
  messageTypes?: string[];
}

export interface InterAgentMessage {
  id: string;
  from: AgentId;
  to: AgentId;
  type: 'request' | 'response' | 'notification' | 'delegation' | 'report';
  content: any;
  priority: number;
  timestamp: Date;
  expiresAt?: Date;
  requiresResponse?: boolean;
  correlationId?: string;
}

export interface TaskDelegation {
  id: string;
  delegator: AgentId;
  delegate: AgentId;
  task: TaskDefinition;
  authority: DelegationAuthority;
  constraints: DelegationConstraints;
  deadline?: Date;
  callbacks: DelegationCallbacks;
}

export interface DelegationAuthority {
  canSubDelegate: boolean;
  resourceLimits: ResourceRequirements;
  decisionScope: string[];
  escalationRights: boolean;
}

export interface DelegationConstraints {
  mustReportEvery: number;
  cannotModify: string[];
  requiredApprovals: string[];
  maxSubTasks: number;
}

export interface DelegationCallbacks {
  onProgress?: (progress: number) => void;
  onComplete?: (result: any) => void;
  onError?: (error: Error) => void;
  onEscalation?: (reason: string) => void;
}

export class HierarchicalAgentSystem extends EventEmitter {
  private config: HierarchicalAgentConfig;
  private hierarchies = new Map<string, AgentHierarchy>();
  private agentManager: AgentManager;
  private taskCoordinator: TaskCoordinator;
  private memory: DistributedMemorySystem;
  private messageQueue = new Map<string, InterAgentMessage[]>();
  private delegations = new Map<string, TaskDelegation>();
  private organizationalChart = new Map<string, OrganizationalRole>();

  constructor(
    config: HierarchicalAgentConfig,
    agentManager: AgentManager,
    taskCoordinator: TaskCoordinator,
    memory: DistributedMemorySystem
  ) {
    super();
    this.config = config;
    this.agentManager = agentManager;
    this.taskCoordinator = taskCoordinator;
    this.memory = memory;

    this.setupEventHandlers();
    this.initializeOrganizationalStructures();
  }

  /**
   * Enable an agent to spawn child agents
   */
  async enableAgentSpawning(agentId: string, permissions: Permission[]): Promise<void> {
    const agent = await this.agentManager.getAgent(agentId);
    if (!agent) {
      throw new Error(`Agent ${agentId} not found`);
    }

    // Add spawning capabilities to agent
    agent.capabilities = {
      ...agent.capabilities,
      agentSpawning: true,
      hierarchicalManagement: true,
      delegation: true
    };

    // Store spawning permissions
    await this.memory.store(`agent:${agentId}:permissions`, permissions, {
      type: 'configuration',
      tags: ['agent-permissions', 'spawning'],
      partition: 'security'
    });

    this.emit('agent:spawning-enabled', { agentId, permissions });
  }

  /**
   * Handle agent spawn request from another agent
   */
  async handleAgentSpawnRequest(request: AgentSpawnRequest): Promise<AgentId> {
    // Validate spawning permissions
    await this.validateSpawnRequest(request);

    // Check resource availability
    await this.checkResourceAvailability(request.resources);

    // Create new agent with hierarchical context
    const newAgentId = await this.spawnChildAgent(request);

    // Establish parent-child relationship
    await this.establishHierarchicalRelationship(request.requesterId, newAgentId);

    // Configure communication channels
    await this.setupCommunicationChannels(request.requesterId, newAgentId);

    // Initialize organizational role
    await this.assignOrganizationalRole(newAgentId, request.role);

    this.emit('agent:spawned', { parent: request.requesterId, child: newAgentId, request });

    return { id: newAgentId, swarmId: 'hierarchy', type: request.agentType, instance: 1 };
  }

  /**
   * Create an organizational hierarchy
   */
  async createOrganizationalHierarchy(structure: {
    name: string;
    rootAgent: AgentId;
    orgChart: OrganizationalRole[];
    maxDepth: number;
  }): Promise<AgentHierarchy> {
    const hierarchyId = generateId('hierarchy');
    
    const hierarchy: AgentHierarchy = {
      id: hierarchyId,
      rootAgent: structure.rootAgent,
      nodes: new Map(),
      relationships: new Map(),
      communicationGraph: new Map(),
      depth: 0,
      totalAgents: 1
    };

    // Create root node
    const rootAgent = await this.agentManager.getAgent(structure.rootAgent.id);
    if (!rootAgent) {
      throw new Error(`Root agent ${structure.rootAgent.id} not found`);
    }

    const rootNode: HierarchyNode = {
      agent: rootAgent,
      children: [],
      siblings: [],
      level: 0,
      role: structure.orgChart[0] || this.createDefaultRole('ceo'),
      responsibilities: ['strategic-planning', 'resource-allocation', 'team-management'],
      capabilities: rootAgent.capabilities,
      delegates: [],
      reports: []
    };

    hierarchy.nodes.set(structure.rootAgent.id, rootNode);
    this.hierarchies.set(hierarchyId, hierarchy);

    // Store hierarchy in memory
    await this.memory.store(`hierarchy:${hierarchyId}`, hierarchy, {
      type: 'state',
      tags: ['hierarchy', 'organization'],
      partition: 'coordination'
    });

    this.emit('hierarchy:created', { hierarchyId, hierarchy });

    return hierarchy;
  }

  /**
   * Enable inter-agent communication
   */
  async sendMessage(message: InterAgentMessage): Promise<void> {
    // Validate communication permissions
    await this.validateCommunication(message);

    // Route message through hierarchy
    await this.routeMessage(message);

    // Store message for persistence
    await this.storeMessage(message);

    // Deliver message to target agent
    await this.deliverMessage(message);

    this.emit('message:sent', message);
  }

  /**
   * Delegate a task to a subordinate agent
   */
  async delegateTask(delegation: TaskDelegation): Promise<void> {
    // Validate delegation authority
    await this.validateDelegationAuthority(delegation);

    // Create delegation record
    this.delegations.set(delegation.id, delegation);

    // Assign task to delegate
    await this.taskCoordinator.assignTask(delegation.task.id.id, delegation.delegate.id);

    // Setup progress monitoring
    await this.setupDelegationMonitoring(delegation);

    // Store delegation in memory
    await this.memory.store(`delegation:${delegation.id}`, delegation, {
      type: 'state',
      tags: ['delegation', 'task-management'],
      partition: 'coordination'
    });

    this.emit('task:delegated', delegation);
  }

  /**
   * Get organizational chart for a hierarchy
   */
  async getOrganizationalChart(hierarchyId: string): Promise<OrganizationalChart> {
    const hierarchy = this.hierarchies.get(hierarchyId);
    if (!hierarchy) {
      throw new Error(`Hierarchy ${hierarchyId} not found`);
    }

    const chart: OrganizationalChart = {
      hierarchyId,
      structure: this.config.organizationStructure,
      levels: new Map(),
      relationships: new Map(),
      communicationPaths: new Map(),
      roles: new Map()
    };

    // Build organizational chart from hierarchy
    for (const [agentId, node] of hierarchy.nodes) {
      chart.roles.set(agentId, node.role);
      
      if (!chart.levels.has(node.level)) {
        chart.levels.set(node.level, []);
      }
      chart.levels.get(node.level)!.push(agentId);

      chart.relationships.set(agentId, {
        parent: node.parent?.id,
        children: node.children.map(child => child.id),
        siblings: node.siblings.map(sibling => sibling.id)
      });
    }

    return chart;
  }

  // Private helper methods

  private async validateSpawnRequest(request: AgentSpawnRequest): Promise<void> {
    const requesterPermissions = await this.memory.retrieve(`agent:${request.requesterId}:permissions`);
    if (!requesterPermissions) {
      throw new Error(`Agent ${request.requesterId} does not have spawning permissions`);
    }

    const permissions = requesterPermissions.value as Permission[];
    const canSpawn = permissions.some(p => p.action === 'spawn-agent');
    if (!canSpawn) {
      throw new Error(`Agent ${request.requesterId} is not authorized to spawn agents`);
    }

    // Check hierarchy depth limits
    const currentDepth = await this.getAgentDepth(request.requesterId);
    if (currentDepth >= this.config.maxDepth) {
      throw new Error(`Maximum hierarchy depth (${this.config.maxDepth}) exceeded`);
    }

    // Check child count limits
    const childCount = await this.getChildCount(request.requesterId);
    if (childCount >= this.config.maxChildrenPerAgent) {
      throw new Error(`Maximum children per agent (${this.config.maxChildrenPerAgent}) exceeded`);
    }
  }

  private async checkResourceAvailability(requirements: ResourceRequirements): Promise<void> {
    // Implementation would check system resources
    // This is a placeholder for resource validation
  }

  private async spawnChildAgent(request: AgentSpawnRequest): Promise<string> {
    const agentTemplate = {
      name: `${request.agentType}-child-${generateId('agent')}`,
      type: request.agentType,
      capabilities: request.capabilities,
      config: {
        autonomyLevel: 0.7,
        learningEnabled: true,
        adaptationEnabled: true,
        maxTasksPerHour: 10,
        maxConcurrentTasks: 3,
        timeoutThreshold: 60000,
        reportingInterval: 30000,
        heartbeatInterval: 10000,
        permissions: [],
        trustedAgents: [{ id: request.requesterId, swarmId: 'hierarchy', type: 'coordinator' as AgentType, instance: 1 }],
        expertise: {},
        preferences: { parentAgent: request.requesterId }
      },
      environment: {
        runtime: 'claude' as const,
        version: '1.0.0',
        workingDirectory: './agents',
        tempDirectory: './tmp',
        logDirectory: './logs',
        apiEndpoints: {},
        credentials: {},
        availableTools: ['task-management', 'communication', 'delegation'],
        toolConfigs: {}
      }
    };

    return await this.agentManager.createAgent('default', {
      name: agentTemplate.name,
      config: agentTemplate.config,
      environment: agentTemplate.environment
    });
  }

  async establishHierarchicalRelationship(parentId: string, childId: string): Promise<void> {
    // Find the hierarchy containing the parent
    let targetHierarchy: AgentHierarchy | undefined;
    for (const hierarchy of this.hierarchies.values()) {
      if (hierarchy.nodes.has(parentId)) {
        targetHierarchy = hierarchy;
        break;
      }
    }

    if (!targetHierarchy) {
      throw new Error(`Parent agent ${parentId} not found in any hierarchy`);
    }

    // Get parent node
    const parentNode = targetHierarchy.nodes.get(parentId)!;
    const childAgent = await this.agentManager.getAgent(childId);
    if (!childAgent) {
      throw new Error(`Child agent ${childId} not found`);
    }

    // Create child node
    const childNode: HierarchyNode = {
      agent: childAgent,
      parent: { id: parentId, swarmId: 'hierarchy', type: parentNode.agent.type, instance: 1 },
      children: [],
      siblings: parentNode.children.filter(sibling => sibling.id !== childId),
      level: parentNode.level + 1,
      role: this.createDefaultRole('agent'),
      responsibilities: [],
      capabilities: childAgent.capabilities,
      delegates: [],
      reports: [{ id: parentId, swarmId: 'hierarchy', type: parentNode.agent.type, instance: 1 }]
    };

    // Add child to hierarchy
    targetHierarchy.nodes.set(childId, childNode);
    parentNode.children.push({ id: childId, swarmId: 'hierarchy', type: childAgent.type, instance: 1 });

    // Update siblings
    for (const siblingId of parentNode.children) {
      if (siblingId.id !== childId) {
        const siblingNode = targetHierarchy.nodes.get(siblingId.id);
        if (siblingNode) {
          siblingNode.siblings.push({ id: childId, swarmId: 'hierarchy', type: childAgent.type, instance: 1 });
          childNode.siblings.push(siblingId);
        }
      }
    }

    // Update hierarchy metadata
    targetHierarchy.totalAgents++;
    targetHierarchy.depth = Math.max(targetHierarchy.depth, childNode.level);

    // Store updated hierarchy
    await this.memory.store(`hierarchy:${targetHierarchy.id}`, targetHierarchy, {
      type: 'state',
      tags: ['hierarchy', 'organization'],
      partition: 'coordination'
    });
  }

  private async setupCommunicationChannels(parentId: string, childId: string): Promise<void> {
    // Initialize message queues
    if (!this.messageQueue.has(parentId)) {
      this.messageQueue.set(parentId, []);
    }
    if (!this.messageQueue.has(childId)) {
      this.messageQueue.set(childId, []);
    }

    // Store communication configuration
    await this.memory.store(`communication:${parentId}:${childId}`, {
      protocol: this.config.communicationProtocol,
      established: new Date(),
      permissions: ['send', 'receive', 'delegate', 'report']
    }, {
      type: 'configuration',
      tags: ['communication', 'hierarchy'],
      partition: 'coordination'
    });
  }

  private async assignOrganizationalRole(agentId: string, role: OrganizationalRole): Promise<void> {
    this.organizationalChart.set(agentId, role);

    await this.memory.store(`agent:${agentId}:role`, role, {
      type: 'configuration',
      tags: ['organizational-role', 'hierarchy'],
      partition: 'coordination'
    });
  }

  private createDefaultRole(type: 'ceo' | 'manager' | 'agent'): OrganizationalRole {
    const roles = {
      ceo: {
        title: 'Chief Executive Officer',
        type: 'executive' as const,
        level: 0,
        permissions: [
          { action: 'spawn-agent', resource: '*' },
          { action: 'delegate-task', resource: '*' },
          { action: 'access-memory', resource: '*' }
        ],
        canSpawnAgents: true,
        maxSubordinates: 10,
        reportingFrequency: 86400000, // 24 hours
        decisionAuthority: ['strategic', 'operational', 'tactical']
      },
      manager: {
        title: 'Team Manager',
        type: 'manager' as const,
        level: 1,
        permissions: [
          { action: 'spawn-agent', resource: 'team' },
          { action: 'delegate-task', resource: 'department' },
          { action: 'access-memory', resource: 'team' }
        ],
        canSpawnAgents: true,
        maxSubordinates: 5,
        reportingFrequency: 43200000, // 12 hours
        decisionAuthority: ['operational', 'tactical']
      },
      agent: {
        title: 'Team Member',
        type: 'specialist' as const,
        level: 2,
        permissions: [
          { action: 'execute-task', resource: 'assigned' },
          { action: 'access-memory', resource: 'personal' }
        ],
        canSpawnAgents: false,
        maxSubordinates: 0,
        reportingFrequency: 21600000, // 6 hours
        decisionAuthority: ['tactical']
      }
    };

    return roles[type];
  }

  private setupEventHandlers(): void {
    // Handle agent lifecycle events
    this.agentManager.on('agent:terminated', (data) => {
      this.handleAgentTermination(data.agentId);
    });

    // Handle task completion events
    this.taskCoordinator.on('task:completed', (data) => {
      this.handleTaskCompletion(data.taskId);
    });
  }

  private initializeOrganizationalStructures(): void {
    // Initialize default organizational roles and hierarchies
    // This would set up templates for common organizational patterns
  }

  private async getAgentDepth(agentId: string): Promise<number> {
    for (const hierarchy of this.hierarchies.values()) {
      const node = hierarchy.nodes.get(agentId);
      if (node) {
        return node.level;
      }
    }
    return 0;
  }

  private async getChildCount(agentId: string): Promise<number> {
    for (const hierarchy of this.hierarchies.values()) {
      const node = hierarchy.nodes.get(agentId);
      if (node) {
        return node.children.length;
      }
    }
    return 0;
  }

  private async validateCommunication(message: InterAgentMessage): Promise<void> {
    // Validate communication permissions and routing
  }

  private async routeMessage(message: InterAgentMessage): Promise<void> {
    // Route message through organizational hierarchy
  }

  private async storeMessage(message: InterAgentMessage): Promise<void> {
    await this.memory.store(`message:${message.id}`, message, {
      type: 'communication',
      tags: ['inter-agent', 'hierarchy'],
      partition: 'messages'
    });
  }

  private async deliverMessage(message: InterAgentMessage): Promise<void> {
    const targetQueue = this.messageQueue.get(message.to.id);
    if (targetQueue) {
      targetQueue.push(message);
    }
  }

  private async validateDelegationAuthority(delegation: TaskDelegation): Promise<void> {
    // Validate that delegator has authority to delegate this task
  }

  private async setupDelegationMonitoring(delegation: TaskDelegation): Promise<void> {
    // Setup monitoring and callbacks for delegation
  }

  private async handleAgentTermination(agentId: string): Promise<void> {
    // Handle cleanup when an agent in hierarchy terminates
  }

  private async handleTaskCompletion(taskId: string): Promise<void> {
    // Handle task completion in hierarchical context
  }
}

// Additional types for organizational chart
export interface OrganizationalChart {
  hierarchyId: string;
  structure: 'flat' | 'tree' | 'matrix' | 'network';
  levels: Map<number, string[]>;
  relationships: Map<string, { parent?: string; children: string[]; siblings: string[] }>;
  communicationPaths: Map<string, string[]>;
  roles: Map<string, OrganizationalRole>;
}

// Agent capabilities extension for hierarchical features
declare module '../swarm/types.js' {
  interface AgentCapabilities {
    agentSpawning?: boolean;
    hierarchicalManagement?: boolean;
    delegation?: boolean;
    organizationalRole?: string;
  }
}