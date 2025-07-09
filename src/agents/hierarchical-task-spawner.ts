/**
 * Hierarchical Task Spawner - Enhanced TASK() function with agent spawning capabilities
 * Allows agents to create sub-agents and delegate tasks in organizational structures
 */

import { EventEmitter } from 'node:events';
import { HierarchicalAgentSystem, AgentSpawnRequest, TaskDelegation } from './hierarchical-agent-system.js';
import { TaskCoordinator } from '../task/coordination.js';
import { AgentState, AgentId, AgentType, TaskDefinition, TaskType, TaskPriority, AgentCapabilities } from '../swarm/types.js';
import { generateId } from '../utils/helpers.js';

export interface EnhancedTaskOptions {
  // Standard task options
  priority?: 'high' | 'medium' | 'low';
  assignedAgent?: string;
  dependencies?: string[];
  timeout?: number;
  
  // Hierarchical options
  spawnAgent?: boolean;
  agentType?: AgentType;
  agentRole?: string;
  delegateToChild?: boolean;
  requiresSpecialist?: boolean;
  
  // Organizational options
  departmentScope?: string;
  approvalRequired?: boolean;
  escalationLevel?: number;
  collaborationMode?: 'individual' | 'team' | 'cross-functional';
  
  // Communication options
  reportingFrequency?: number;
  statusUpdates?: boolean;
  stakeholderNotifications?: string[];
  
  // Resource options
  resourceRequirements?: {
    cpu?: number;
    memory?: number;
    specializedTools?: string[];
    teamSize?: number;
  };
}

export interface TaskResult {
  taskId: string;
  agentId?: string;
  spawnedAgents?: AgentId[];
  status: 'pending' | 'in_progress' | 'completed' | 'failed' | 'delegated';
  result?: any;
  delegations?: TaskDelegation[];
  organizationalImpact?: OrganizationalImpact;
}

export interface OrganizationalImpact {
  newAgents: number;
  hierarchyChanges: string[];
  resourceUtilization: number;
  teamEfficiency: number;
  communicationVolume: number;
}

export interface AgentTaskContext {
  agentId: string;
  hierarchyId?: string;
  parentAgent?: string;
  children: string[];
  role: string;
  permissions: string[];
  currentTasks: string[];
  availableResources: any;
}

export class HierarchicalTaskSpawner extends EventEmitter {
  private hierarchicalSystem: HierarchicalAgentSystem;
  private taskCoordinator: TaskCoordinator;
  private activeContexts = new Map<string, AgentTaskContext>();
  private taskExecutions = new Map<string, TaskExecution>();
  private agentCapabilities = new Map<string, AgentCapabilities>();

  constructor(
    hierarchicalSystem: HierarchicalAgentSystem,
    taskCoordinator: TaskCoordinator
  ) {
    super();
    this.hierarchicalSystem = hierarchicalSystem;
    this.taskCoordinator = taskCoordinator;
    this.setupEventHandlers();
  }

  /**
   * Enhanced TASK() function that can spawn agents and delegate
   */
  async TASK(
    description: string,
    options: EnhancedTaskOptions = {},
    context?: AgentTaskContext
  ): Promise<TaskResult> {
    const taskId = generateId('task');
    
    // Create base task definition
    const taskDefinition: TaskDefinition = {
      id: { id: taskId, swarmId: 'hierarchy', sequence: 1, priority: this.getPriorityScore(options.priority) },
      type: this.determineTaskType(description, options) as TaskType,
      name: description.substring(0, 50) + (description.length > 50 ? '...' : ''),
      description,
      requirements: {
        capabilities: this.extractCapabilities(description, options),
        tools: options.resourceRequirements?.specializedTools || [],
        permissions: []
      },
      constraints: {
        dependencies: (options.dependencies || []).map(dep => ({ id: dep, swarmId: 'hierarchy', sequence: 1, priority: 1 })),
        dependents: [],
        conflicts: []
      },
      priority: this.mapPriorityToTaskPriority(options.priority),
      input: { description, options, context },
      instructions: description,
      context: {
        createdBy: context?.agentId || 'system',
        hierarchical: true,
        organizationalScope: options.departmentScope,
        ...options
      },
      status: 'created',
      createdAt: new Date(),
      updatedAt: new Date(),
      attempts: [],
      statusHistory: []
    };

    // Determine execution strategy
    const executionStrategy = await this.determineExecutionStrategy(taskDefinition, options, context);

    let result: TaskResult;

    switch (executionStrategy.type) {
      case 'spawn-and-delegate':
        result = await this.spawnAgentAndDelegate(taskDefinition, options, context);
        break;
      case 'delegate-to-existing':
        result = await this.delegateToExistingAgent(taskDefinition, options, context);
        break;
      case 'self-execute':
        result = await this.selfExecute(taskDefinition, options, context);
        break;
      case 'team-execution':
        result = await this.executeWithTeam(taskDefinition, options, context);
        break;
      default:
        result = await this.defaultExecution(taskDefinition, options, context);
    }

    // Track organizational impact
    result.organizationalImpact = await this.calculateOrganizationalImpact(result);

    // Store execution record
    this.taskExecutions.set(taskId, {
      id: taskId,
      task: taskDefinition,
      result,
      startTime: new Date(),
      context,
      strategy: executionStrategy
    });

    this.emit('task:executed', result);

    return result;
  }

  /**
   * Allow an agent to spawn a specialized sub-agent for a task
   */
  async spawnSpecializedAgent(
    parentAgentId: string,
    taskType: string,
    capabilities: string[],
    purpose: string
  ): Promise<AgentId> {
    const spawnRequest: AgentSpawnRequest = {
      requesterId: parentAgentId,
      agentType: this.mapTaskTypeToAgentType(taskType),
      role: {
        title: `${taskType} Specialist`,
        type: 'specialist',
        level: await this.getAgentLevel(parentAgentId) + 1,
        permissions: this.generateSpecialistPermissions(capabilities),
        canSpawnAgents: false,
        maxSubordinates: 0,
        reportingFrequency: 3600000, // 1 hour
        decisionAuthority: ['tactical']
      },
      purpose,
      capabilities: this.buildAgentCapabilities(capabilities),
      lifetime: 'task-bound',
      priority: 5,
      resources: {
        cpu: 1,
        memory: 512,
        disk: 100,
        network: 10,
        specializedTools: capabilities
      }
    };

    return await this.hierarchicalSystem.handleAgentSpawnRequest(spawnRequest);
  }

  /**
   * Create a task delegation with hierarchical oversight
   */
  async delegateTask(
    delegatorId: string,
    taskDefinition: TaskDefinition,
    targetAgent?: string
  ): Promise<TaskDelegation> {
    // Find or create appropriate agent for delegation
    const delegateAgent = targetAgent 
      ? { id: targetAgent, swarmId: 'hierarchy', type: 'specialist' as AgentType, instance: 1 }
      : await this.findBestDelegateAgent(delegatorId, taskDefinition);

    const delegation: TaskDelegation = {
      id: generateId('delegation'),
      delegator: { id: delegatorId, swarmId: 'hierarchy', type: 'coordinator', instance: 1 },
      delegate: delegateAgent,
      task: taskDefinition,
      authority: {
        canSubDelegate: await this.canAgentSubDelegate(delegateAgent.id),
        resourceLimits: await this.getAgentResourceLimits(delegateAgent.id),
        decisionScope: await this.getAgentDecisionScope(delegateAgent.id),
        escalationRights: true
      },
      constraints: {
        mustReportEvery: 1800000, // 30 minutes
        cannotModify: ['priority', 'deadline'],
        requiredApprovals: [],
        maxSubTasks: 5
      },
      callbacks: {
        onProgress: (progress) => this.handleDelegationProgress(delegation.id, progress),
        onComplete: (result) => this.handleDelegationComplete(delegation.id, result),
        onError: (error) => this.handleDelegationError(delegation.id, error),
        onEscalation: (reason) => this.handleDelegationEscalation(delegation.id, reason)
      }
    };

    await this.hierarchicalSystem.delegateTask(delegation);
    return delegation;
  }

  /**
   * Create an organizational team for complex tasks
   */
  async createTaskTeam(
    coordinatorId: string,
    taskDefinition: TaskDefinition,
    teamComposition: TeamComposition
  ): Promise<TaskTeam> {
    const teamId = generateId('team');
    const team: TaskTeam = {
      id: teamId,
      coordinator: { id: coordinatorId, swarmId: 'hierarchy', type: 'coordinator', instance: 1 },
      members: [],
      task: taskDefinition,
      structure: teamComposition.structure,
      communicationChannels: [],
      sharedResources: new Map(),
      decisionMaking: teamComposition.decisionMaking
    };

    // Spawn team members based on composition
    for (const role of teamComposition.roles) {
      const agentId = await this.spawnSpecializedAgent(
        coordinatorId,
        role.type,
        role.capabilities,
        `Team member for ${taskDefinition.description}`
      );

      team.members.push({
        agentId,
        role: role.title,
        responsibilities: role.responsibilities,
        authority: role.authority
      });

      // Setup communication channels
      await this.setupTeamCommunication(coordinatorId, agentId.id, teamId);
    }

    // Initialize team coordination
    await this.initializeTeamCoordination(team);

    return team;
  }

  // Private helper methods

  private async determineExecutionStrategy(
    task: TaskDefinition,
    options: EnhancedTaskOptions,
    context?: AgentTaskContext
  ): Promise<ExecutionStrategy> {
    const complexity = await this.analyzeTaskComplexity(task);
    const agentCapabilities = context ? await this.getAgentCapabilities(context.agentId) : null;
    
    if (options.spawnAgent || complexity.requiresSpecialization) {
      return { type: 'spawn-and-delegate', reason: 'specialized-skills-required' };
    }
    
    if (options.delegateToChild && context?.children && context.children.length > 0) {
      return { type: 'delegate-to-existing', reason: 'existing-subordinates-available' };
    }
    
    if (complexity.requiresTeam || options.collaborationMode === 'team') {
      return { type: 'team-execution', reason: 'collaborative-effort-needed' };
    }
    
    if (agentCapabilities && this.canAgentHandleTask(agentCapabilities, task)) {
      return { type: 'self-execute', reason: 'agent-capable' };
    }
    
    return { type: 'spawn-and-delegate', reason: 'default-strategy' };
  }

  private async spawnAgentAndDelegate(
    task: TaskDefinition,
    options: EnhancedTaskOptions,
    context?: AgentTaskContext
  ): Promise<TaskResult> {
    const parentAgentId = context?.agentId || 'system';
    
    // Determine required capabilities
    const requiredCapabilities = await this.analyzeRequiredCapabilities(task);
    
    // Spawn specialized agent
    const newAgent = await this.spawnSpecializedAgent(
      parentAgentId,
      task.type,
      requiredCapabilities,
      task.description
    );

    // Delegate task to new agent
    const delegation = await this.delegateTask(parentAgentId, task, newAgent.id);

    return {
      taskId: task.id.id,
      agentId: newAgent.id,
      spawnedAgents: [newAgent],
      status: 'delegated',
      delegations: [delegation]
    };
  }

  private async delegateToExistingAgent(
    task: TaskDefinition,
    options: EnhancedTaskOptions,
    context?: AgentTaskContext
  ): Promise<TaskResult> {
    const parentAgentId = context?.agentId || 'system';
    const bestChild = await this.findBestChildAgent(parentAgentId, task);
    
    if (!bestChild) {
      // Fall back to spawning new agent
      return this.spawnAgentAndDelegate(task, options, context);
    }

    const delegation = await this.delegateTask(parentAgentId, task, bestChild);

    return {
      taskId: task.id.id,
      agentId: bestChild,
      spawnedAgents: [],
      status: 'delegated',
      delegations: [delegation]
    };
  }

  private async selfExecute(
    task: TaskDefinition,
    options: EnhancedTaskOptions,
    context?: AgentTaskContext
  ): Promise<TaskResult> {
    // Create task in coordinator first, then execute
    const createdTask = await this.taskCoordinator.createTask({
      id: task.id.id,
      type: task.type,
      description: task.description,
      priority: task.priority,
      dependencies: task.constraints.dependencies.map(d => d.id),
      assignedAgent: context?.agentId,
      status: 'pending',
      input: task.requirements,
      createdAt: new Date(),
      metadata: task.context
    });

    const executionResult = await this.taskCoordinator.executeTask(task.id.id);

    return {
      taskId: task.id.id,
      agentId: context?.agentId,
      spawnedAgents: [],
      status: executionResult.success ? 'completed' : 'failed',
      result: executionResult.data
    };
  }

  private async executeWithTeam(
    task: TaskDefinition,
    options: EnhancedTaskOptions,
    context?: AgentTaskContext
  ): Promise<TaskResult> {
    const coordinatorId = context?.agentId || 'system';
    
    // Determine team composition
    const teamComposition = await this.designTeamComposition(task, options);
    
    // Create task team
    const team = await this.createTaskTeam(coordinatorId, task, teamComposition);

    // Execute task with team
    const teamExecution = await this.executeTeamTask(team);

    return {
      taskId: task.id.id,
      agentId: coordinatorId,
      spawnedAgents: team.members.map(m => m.agentId),
      status: teamExecution.success ? 'completed' : 'failed',
      result: teamExecution.result,
      organizationalImpact: {
        newAgents: team.members.length,
        hierarchyChanges: [`Created team ${team.id}`],
        resourceUtilization: teamExecution.resourceUsage,
        teamEfficiency: teamExecution.efficiency,
        communicationVolume: teamExecution.messageCount
      }
    };
  }

  private async defaultExecution(
    task: TaskDefinition,
    options: EnhancedTaskOptions,
    context?: AgentTaskContext
  ): Promise<TaskResult> {
    // Default to task coordinator execution
    const executionResult = await this.taskCoordinator.createTask({
      id: task.id.id,
      type: task.type,
      description: task.description,
      priority: task.priority,
      dependencies: task.constraints.dependencies.map(d => d.id),
      assignedAgent: options.assignedAgent,
      status: 'pending',
      input: task.requirements,
      createdAt: new Date(),
      metadata: task.context
    });

    return {
      taskId: task.id.id,
      status: 'pending',
      result: executionResult
    };
  }

  // Helper methods for task analysis and execution

  private getPriorityScore(priority?: 'high' | 'medium' | 'low'): number {
    const scores = { high: 8, medium: 5, low: 2 };
    return scores[priority || 'medium'];
  }


  private buildRequirements(description: string, options: EnhancedTaskOptions): any {
    return {
      description,
      resourceRequirements: options.resourceRequirements,
      collaborationMode: options.collaborationMode,
      departmentScope: options.departmentScope
    };
  }

  private buildConstraints(options: EnhancedTaskOptions): any {
    return {
      timeout: options.timeout,
      approvalRequired: options.approvalRequired,
      escalationLevel: options.escalationLevel,
      reportingFrequency: options.reportingFrequency
    };
  }

  private setupEventHandlers(): void {
    this.hierarchicalSystem.on('agent:spawned', (data) => {
      this.handleAgentSpawned(data);
    });

    this.hierarchicalSystem.on('task:delegated', (data) => {
      this.handleTaskDelegated(data);
    });
  }

  private handleAgentSpawned(data: any): void {
    this.emit('agent:spawned', data);
  }

  private handleTaskDelegated(data: any): void {
    this.emit('task:delegated', data);
  }

  private handleDelegationProgress(delegationId: string, progress: number): void {
    this.emit('delegation:progress', { delegationId, progress });
  }

  private handleDelegationComplete(delegationId: string, result: any): void {
    this.emit('delegation:complete', { delegationId, result });
  }

  private handleDelegationError(delegationId: string, error: Error): void {
    this.emit('delegation:error', { delegationId, error });
  }

  private handleDelegationEscalation(delegationId: string, reason: string): void {
    this.emit('delegation:escalation', { delegationId, reason });
  }

  // Placeholder implementations for complex methods
  private async analyzeTaskComplexity(task: TaskDefinition): Promise<any> {
    return { requiresSpecialization: false, requiresTeam: false, complexity: 'medium' };
  }

  private async getAgentCapabilities(agentId: string): Promise<any> {
    return this.agentCapabilities.get(agentId) || {};
  }

  private canAgentHandleTask(capabilities: any, task: TaskDefinition): boolean {
    return true; // Simplified implementation
  }

  private async analyzeRequiredCapabilities(task: TaskDefinition): Promise<string[]> {
    return ['general']; // Simplified implementation
  }

  private async findBestChildAgent(parentId: string, task: TaskDefinition): Promise<string | null> {
    return null; // Simplified implementation
  }

  private async findBestDelegateAgent(delegatorId: string, task: TaskDefinition): Promise<AgentId> {
    return { id: 'default-agent', swarmId: 'hierarchy', type: 'specialist', instance: 1 };
  }

  private async calculateOrganizationalImpact(result: TaskResult): Promise<OrganizationalImpact> {
    return {
      newAgents: result.spawnedAgents?.length || 0,
      hierarchyChanges: [],
      resourceUtilization: 0.5,
      teamEfficiency: 0.8,
      communicationVolume: 10
    };
  }

  private mapTaskTypeToAgentType(taskType: string): AgentType {
    const mapping: Record<string, AgentType> = {
      research: 'researcher',
      development: 'developer',
      coordination: 'coordinator',
      analysis: 'analyzer',
      review: 'reviewer',
      testing: 'tester',
      documentation: 'documenter'
    };
    return mapping[taskType] || 'specialist';
  }

  private async getAgentLevel(agentId: string): Promise<number> {
    return 1; // Simplified implementation
  }

  private generateSpecialistPermissions(capabilities: string[]): any[] {
    return capabilities.map(cap => ({ action: 'use', resource: cap }));
  }

  private buildAgentCapabilities(capabilities: string[]): any {
    return {
      codeGeneration: capabilities.includes('coding'),
      research: capabilities.includes('research'),
      analysis: capabilities.includes('analysis'),
      testing: capabilities.includes('testing'),
      documentation: capabilities.includes('documentation'),
      specializations: capabilities
    };
  }

  // Additional placeholder methods for team and organizational features
  private async canAgentSubDelegate(agentId: string): Promise<boolean> { return false; }
  private async getAgentResourceLimits(agentId: string): Promise<any> { return {}; }
  private async getAgentDecisionScope(agentId: string): Promise<string[]> { return []; }


  private mapPriorityToTaskPriority(priority?: string): TaskPriority {
    switch (priority) {
      case 'high': return 'high';
      case 'medium': return 'normal';
      case 'low': return 'low';
      default: return 'normal';
    }
  }

  private extractCapabilities(description: string, options: EnhancedTaskOptions): string[] {
    const capabilities: string[] = [];
    
    // Extract from resource requirements
    if (options.resourceRequirements?.specializedTools) {
      capabilities.push(...options.resourceRequirements.specializedTools);
    }
    
    // Extract from description keywords
    const keywords = description.toLowerCase();
    if (keywords.includes('code') || keywords.includes('develop')) {
      capabilities.push('codeGeneration');
    }
    if (keywords.includes('test')) {
      capabilities.push('testing');
    }
    if (keywords.includes('research')) {
      capabilities.push('research');
    }
    if (keywords.includes('analyze') || keywords.includes('analysis')) {
      capabilities.push('analysis');
    }
    if (keywords.includes('review')) {
      capabilities.push('codeReview');
    }
    
    return capabilities.length > 0 ? capabilities : ['general'];
  }

  private determineTaskType(description: string, options: EnhancedTaskOptions): TaskType {
    const keywords = description.toLowerCase();
    
    if (keywords.includes('research')) return 'research';
    if (keywords.includes('develop') || keywords.includes('code') || keywords.includes('implement')) return 'coding';
    if (keywords.includes('test')) return 'testing';
    if (keywords.includes('analyze') || keywords.includes('analysis')) return 'analysis';
    if (keywords.includes('review')) return 'review';
    if (keywords.includes('document')) return 'documentation';
    if (keywords.includes('deploy')) return 'deployment';
    if (keywords.includes('monitor')) return 'monitoring';
    if (keywords.includes('coordinate')) return 'coordination';
    
    // Use agent type as fallback
    if (options.agentType) {
      switch (options.agentType) {
        case 'researcher': return 'research';
        case 'developer': return 'coding';
        case 'tester': return 'testing';
        case 'analyzer': return 'analysis';
        case 'reviewer': return 'review';
        case 'documenter': return 'documentation';
        case 'coordinator': return 'coordination';
        default: return 'custom';
      }
    }
    
    return 'custom';
  }
  private async designTeamComposition(task: TaskDefinition, options: EnhancedTaskOptions): Promise<TeamComposition> { 
    return { structure: 'hierarchical', roles: [], decisionMaking: 'consensus' }; 
  }
  private async setupTeamCommunication(coordinatorId: string, memberId: string, teamId: string): Promise<void> {}
  private async initializeTeamCoordination(team: TaskTeam): Promise<void> {}
  private async executeTeamTask(team: TaskTeam): Promise<any> { 
    return { success: true, result: {}, resourceUsage: 0.5, efficiency: 0.8, messageCount: 25 }; 
  }
}

// Additional interfaces for team management
interface TaskExecution {
  id: string;
  task: TaskDefinition;
  result: TaskResult;
  startTime: Date;
  context?: AgentTaskContext;
  strategy: ExecutionStrategy;
}

interface ExecutionStrategy {
  type: 'spawn-and-delegate' | 'delegate-to-existing' | 'self-execute' | 'team-execution' | 'default';
  reason: string;
}

interface TeamComposition {
  structure: 'hierarchical' | 'flat' | 'matrix';
  roles: TeamRole[];
  decisionMaking: 'consensus' | 'majority' | 'authoritarian';
}

interface TeamRole {
  title: string;
  type: string;
  capabilities: string[];
  responsibilities: string[];
  authority: string[];
}

interface TaskTeam {
  id: string;
  coordinator: AgentId;
  members: TeamMember[];
  task: TaskDefinition;
  structure: string;
  communicationChannels: string[];
  sharedResources: Map<string, any>;
  decisionMaking: string;
}

interface TeamMember {
  agentId: AgentId;
  role: string;
  responsibilities: string[];
  authority: string[];
}