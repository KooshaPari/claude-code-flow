/**
 * Agent Lifecycle Manager
 * Manages the entire lifecycle of hierarchical agents from spawn to termination
 */

import { EventEmitter } from 'node:events';
import { AgentState, AgentId, AgentType } from '../swarm/types.js';
import { HierarchicalAgentSystem } from './hierarchical-agent-system.js';
import { InterAgentCommunicationSystem } from '../communication/inter-agent-communication.js';
import { DistributedMemorySystem } from '../memory/distributed-memory.js';
import { generateId } from '../utils/helpers.js';

export interface AgentLifecycleState {
  agentId: string;
  state: LifecycleState;
  parentAgent?: string;
  childAgents: string[];
  createdAt: Date;
  lastStateChange: Date;
  totalUptime: number;
  tasksCompleted: number;
  performance: AgentLifecycleMetrics;
  scheduledEvents: ScheduledEvent[];
  dependencies: AgentDependency[];
  resources: AllocatedResources;
}

export type LifecycleState = 
  | 'spawning'        // Agent is being created
  | 'initializing'    // Agent is starting up
  | 'training'        // Agent is learning initial role
  | 'active'          // Agent is operational
  | 'idle'            // Agent is waiting for tasks
  | 'busy'            // Agent is executing tasks
  | 'scaling'         // Agent is spawning children
  | 'delegating'      // Agent is delegating tasks
  | 'reporting'       // Agent is providing status updates
  | 'maintenance'     // Agent is being updated/maintained
  | 'paused'          // Agent is temporarily suspended
  | 'retiring'        // Agent is preparing for termination
  | 'terminated'      // Agent has been shut down
  | 'error';          // Agent is in error state

export interface LifecycleTransition {
  from: LifecycleState;
  to: LifecycleState;
  trigger: TransitionTrigger;
  conditions: string[];
  actions: LifecycleAction[];
  timeout?: number;
  canRevert?: boolean;
}

export type TransitionTrigger = 
  | 'task-assigned'
  | 'task-completed'
  | 'spawn-request'
  | 'delegation-complete'
  | 'performance-threshold'
  | 'resource-limit'
  | 'scheduled-event'
  | 'parent-termination'
  | 'manual-intervention'
  | 'error-detected'
  | 'timeout'
  | 'system-shutdown';

export interface LifecycleAction {
  type: 'notify' | 'cleanup' | 'migrate' | 'backup' | 'delegate' | 'escalate';
  target?: string;
  parameters: Record<string, any>;
  priority: number;
  timeout?: number;
}

export interface ScheduledEvent {
  id: string;
  agentId: string;
  type: 'maintenance' | 'report' | 'cleanup' | 'performance-review' | 'retirement';
  scheduledAt: Date;
  parameters: Record<string, any>;
  recurring?: {
    interval: number;
    maxOccurrences?: number;
  };
  completed: boolean;
}

export interface AgentDependency {
  dependentAgent: string;
  dependencyType: 'parent' | 'child' | 'sibling' | 'service' | 'resource';
  critical: boolean;
  healthCheck: () => Promise<boolean>;
  onFailure: 'escalate' | 'retry' | 'failover' | 'terminate';
}

export interface AllocatedResources {
  cpu: number;
  memory: number;
  storage: number;
  networkBandwidth: number;
  specializedTools: string[];
  sharedResources: Map<string, any>;
  reservations: ResourceReservation[];
}

export interface ResourceReservation {
  resourceType: string;
  amount: number;
  reservedUntil: Date;
  priority: number;
  transferable: boolean;
}

export interface AgentLifecycleMetrics {
  stateTransitions: Map<string, number>;
  averageStateTime: Map<LifecycleState, number>;
  performanceScore: number;
  reliabilityScore: number;
  efficiencyScore: number;
  lastPerformanceReview: Date;
  issues: LifecycleIssue[];
  achievements: LifecycleAchievement[];
}

export interface LifecycleIssue {
  id: string;
  type: 'performance' | 'resource' | 'communication' | 'compliance';
  severity: 'low' | 'medium' | 'high' | 'critical';
  description: string;
  detectedAt: Date;
  resolvedAt?: Date;
  impact: string;
  mitigation?: string;
}

export interface LifecycleAchievement {
  id: string;
  type: 'performance' | 'innovation' | 'collaboration' | 'efficiency';
  description: string;
  achievedAt: Date;
  value: number;
  recognition: string;
}

export interface LifecyclePolicy {
  name: string;
  agentPattern: string; // regex pattern for agent types/roles
  maxUptime?: number;
  maxIdleTime?: number;
  performanceThresholds: {
    minimum: number;
    target: number;
    excellent: number;
  };
  autoRetirement: {
    enabled: boolean;
    conditions: string[];
    gracePeriod: number;
  };
  maintenanceSchedule: {
    frequency: number;
    duration: number;
    allowDeferral: boolean;
  };
  successorPlanning: {
    enabled: boolean;
    trainingPeriod: number;
    overlapPeriod: number;
  };
}

export class AgentLifecycleManager extends EventEmitter {
  private lifecycleStates = new Map<string, AgentLifecycleState>();
  private lifecycleTransitions: LifecycleTransition[] = [];
  private scheduledEvents = new Map<string, ScheduledEvent>();
  private lifecyclePolicies = new Map<string, LifecyclePolicy>();
  private hierarchicalSystem: HierarchicalAgentSystem;
  private communicationSystem: InterAgentCommunicationSystem;
  private memory: DistributedMemorySystem;

  constructor(
    hierarchicalSystem: HierarchicalAgentSystem,
    communicationSystem: InterAgentCommunicationSystem,
    memory: DistributedMemorySystem
  ) {
    super();
    this.hierarchicalSystem = hierarchicalSystem;
    this.communicationSystem = communicationSystem;
    this.memory = memory;
    
    this.initializeTransitions();
    this.initializeDefaultPolicies();
    this.startLifecycleMonitoring();
  }

  /**
   * Register a new agent in the lifecycle management system
   */
  async registerAgent(
    agentId: string,
    agentType: AgentType,
    parentAgent?: string,
    initialResources?: Partial<AllocatedResources>
  ): Promise<void> {
    const lifecycleState: AgentLifecycleState = {
      agentId,
      state: 'spawning',
      parentAgent,
      childAgents: [],
      createdAt: new Date(),
      lastStateChange: new Date(),
      totalUptime: 0,
      tasksCompleted: 0,
      performance: {
        stateTransitions: new Map(),
        averageStateTime: new Map(),
        performanceScore: 0.5,
        reliabilityScore: 1.0,
        efficiencyScore: 0.5,
        lastPerformanceReview: new Date(),
        issues: [],
        achievements: []
      },
      scheduledEvents: [],
      dependencies: [],
      resources: {
        cpu: 1,
        memory: 512,
        storage: 100,
        networkBandwidth: 10,
        specializedTools: [],
        sharedResources: new Map(),
        reservations: [],
        ...initialResources
      }
    };

    this.lifecycleStates.set(agentId, lifecycleState);

    // Schedule initial events
    await this.scheduleInitialEvents(agentId, agentType);

    // Apply lifecycle policies
    await this.applyLifecyclePolicies(agentId, agentType);

    // Store in memory
    await this.memory.store(`lifecycle:${agentId}`, lifecycleState, {
      type: 'state',
      tags: ['lifecycle', 'agent', agentType],
      partition: 'lifecycle'
    });

    this.emit('agent:registered', { agentId, state: lifecycleState });
  }

  /**
   * Transition an agent to a new lifecycle state
   */
  async transitionAgent(
    agentId: string,
    newState: LifecycleState,
    trigger: TransitionTrigger,
    context?: Record<string, any>
  ): Promise<boolean> {
    const currentState = this.lifecycleStates.get(agentId);
    if (!currentState) {
      throw new Error(`Agent ${agentId} not found in lifecycle management`);
    }

    const transition = this.findValidTransition(currentState.state, newState, trigger);
    if (!transition) {
      console.warn(`Invalid transition: ${currentState.state} -> ${newState} (trigger: ${trigger})`);
      return false;
    }

    // Check transition conditions
    const conditionsMet = await this.checkTransitionConditions(agentId, transition, context);
    if (!conditionsMet) {
      console.warn(`Transition conditions not met for agent ${agentId}`);
      return false;
    }

    // Execute pre-transition actions
    await this.executeTransitionActions(agentId, transition, 'pre', context);

    // Update state
    const oldState = currentState.state;
    currentState.state = newState;
    currentState.lastStateChange = new Date();

    // Update metrics
    this.updateStateMetrics(currentState, oldState, newState);

    // Execute post-transition actions
    await this.executeTransitionActions(agentId, transition, 'post', context);

    // Notify interested parties
    await this.notifyStateChange(agentId, oldState, newState, trigger);

    // Update memory
    await this.memory.store(`lifecycle:${agentId}`, currentState, {
      type: 'state',
      tags: ['lifecycle', 'agent', newState],
      partition: 'lifecycle'
    });

    this.emit('agent:state-changed', { agentId, from: oldState, to: newState, trigger });

    return true;
  }

  /**
   * Handle agent termination with proper cleanup
   */
  async terminateAgent(
    agentId: string,
    reason: 'planned' | 'error' | 'resource-limit' | 'parent-terminated' | 'manual',
    graceful: boolean = true
  ): Promise<void> {
    const agentState = this.lifecycleStates.get(agentId);
    if (!agentState) {
      console.warn(`Agent ${agentId} not found for termination`);
      return;
    }

    // Transition to retiring state first if graceful
    if (graceful && agentState.state !== 'retiring') {
      await this.transitionAgent(agentId, 'retiring', 'manual-intervention', { reason });
    }

    // Handle child agents
    if (agentState.childAgents.length > 0) {
      await this.handleChildAgentTermination(agentId, agentState.childAgents);
    }

    // Transfer ongoing tasks
    await this.transferActiveTasks(agentId);

    // Clean up resources
    await this.cleanupAgentResources(agentId);

    // Notify dependencies
    await this.notifyDependentAgents(agentId);

    // Final state transition
    await this.transitionAgent(agentId, 'terminated', 'manual-intervention', { reason });

    // Archive lifecycle data
    await this.archiveAgentLifecycle(agentId);

    this.emit('agent:terminated', { agentId, reason, graceful });
  }

  /**
   * Schedule maintenance for an agent
   */
  async scheduleMaintenance(
    agentId: string,
    maintenanceType: 'performance-review' | 'resource-optimization' | 'update' | 'cleanup',
    scheduledAt?: Date,
    parameters?: Record<string, any>
  ): Promise<string> {
    const eventId = generateId('maintenance');
    const event: ScheduledEvent = {
      id: eventId,
      agentId,
      type: 'maintenance',
      scheduledAt: scheduledAt || new Date(Date.now() + 60000), // Default: 1 minute from now
      parameters: { maintenanceType, ...parameters },
      completed: false
    };

    this.scheduledEvents.set(eventId, event);

    const agentState = this.lifecycleStates.get(agentId);
    if (agentState) {
      agentState.scheduledEvents.push(event);
    }

    this.emit('maintenance:scheduled', { agentId, eventId, event });

    return eventId;
  }

  /**
   * Get agent lifecycle status
   */
  getAgentLifecycleStatus(agentId: string): AgentLifecycleStatus | null {
    const state = this.lifecycleStates.get(agentId);
    if (!state) return null;

    return {
      agentId,
      currentState: state.state,
      uptime: Date.now() - state.createdAt.getTime(),
      timeSinceLastStateChange: Date.now() - state.lastStateChange.getTime(),
      performance: state.performance,
      resources: state.resources,
      scheduledEvents: state.scheduledEvents.filter(e => !e.completed),
      health: this.calculateAgentHealth(state),
      nextScheduledEvent: this.getNextScheduledEvent(agentId),
      recommendations: this.generateLifecycleRecommendations(state)
    };
  }

  /**
   * Get lifecycle metrics for all agents
   */
  getLifecycleMetrics(): LifecycleSystemMetrics {
    const states = Array.from(this.lifecycleStates.values());
    
    return {
      totalAgents: states.length,
      agentsByState: this.groupAgentsByState(states),
      averageUptime: this.calculateAverageUptime(states),
      averagePerformance: this.calculateAveragePerformance(states),
      totalStateTransitions: this.calculateTotalTransitions(states),
      activeIssues: this.getActiveIssues(states),
      scheduledMaintenanceEvents: this.scheduledEvents.size,
      resourceUtilization: this.calculateResourceUtilization(states)
    };
  }

  // Private helper methods

  private initializeTransitions(): void {
    this.lifecycleTransitions = [
      // Initialization flow
      { from: 'spawning', to: 'initializing', trigger: 'spawn-request', conditions: [], actions: [] },
      { from: 'initializing', to: 'training', trigger: 'system-shutdown', conditions: [], actions: [] },
      { from: 'training', to: 'active', trigger: 'performance-threshold', conditions: [], actions: [] },
      
      // Normal operation
      { from: 'active', to: 'idle', trigger: 'task-completed', conditions: [], actions: [] },
      { from: 'idle', to: 'busy', trigger: 'task-assigned', conditions: [], actions: [] },
      { from: 'busy', to: 'active', trigger: 'task-completed', conditions: [], actions: [] },
      { from: 'active', to: 'scaling', trigger: 'spawn-request', conditions: [], actions: [] },
      { from: 'scaling', to: 'active', trigger: 'spawn-request', conditions: [], actions: [] },
      { from: 'active', to: 'delegating', trigger: 'task-assigned', conditions: [], actions: [] },
      { from: 'delegating', to: 'active', trigger: 'delegation-complete', conditions: [], actions: [] },
      
      // Maintenance and reporting
      { from: 'active', to: 'maintenance', trigger: 'scheduled-event', conditions: [], actions: [] },
      { from: 'maintenance', to: 'active', trigger: 'scheduled-event', conditions: [], actions: [] },
      { from: 'active', to: 'reporting', trigger: 'scheduled-event', conditions: [], actions: [] },
      { from: 'reporting', to: 'active', trigger: 'scheduled-event', conditions: [], actions: [] },
      
      // Error and recovery
      { from: 'active', to: 'error', trigger: 'error-detected', conditions: [], actions: [] },
      { from: 'error', to: 'active', trigger: 'manual-intervention', conditions: [], actions: [] },
      { from: 'active', to: 'paused', trigger: 'manual-intervention', conditions: [], actions: [] },
      { from: 'paused', to: 'active', trigger: 'manual-intervention', conditions: [], actions: [] },
      
      // Termination
      { from: 'active', to: 'retiring', trigger: 'manual-intervention', conditions: [], actions: [] },
      { from: 'retiring', to: 'terminated', trigger: 'manual-intervention', conditions: [], actions: [] },
      { from: 'error', to: 'terminated', trigger: 'manual-intervention', conditions: [], actions: [] }
    ];
  }

  private initializeDefaultPolicies(): void {
    const defaultPolicy: LifecyclePolicy = {
      name: 'default',
      agentPattern: '.*',
      maxUptime: 7 * 24 * 60 * 60 * 1000, // 7 days
      maxIdleTime: 60 * 60 * 1000, // 1 hour
      performanceThresholds: {
        minimum: 0.3,
        target: 0.7,
        excellent: 0.9
      },
      autoRetirement: {
        enabled: true,
        conditions: ['performance < 0.3', 'uptime > maxUptime'],
        gracePeriod: 60 * 60 * 1000 // 1 hour
      },
      maintenanceSchedule: {
        frequency: 24 * 60 * 60 * 1000, // Daily
        duration: 5 * 60 * 1000, // 5 minutes
        allowDeferral: true
      },
      successorPlanning: {
        enabled: false,
        trainingPeriod: 30 * 60 * 1000, // 30 minutes
        overlapPeriod: 15 * 60 * 1000 // 15 minutes
      }
    };

    this.lifecyclePolicies.set('default', defaultPolicy);
  }

  private startLifecycleMonitoring(): void {
    // Monitor agent states and execute scheduled events
    setInterval(() => {
      this.processScheduledEvents();
      this.checkAgentHealth();
      this.enforceLifecyclePolicies();
    }, 60000); // Every minute
  }

  private async scheduleInitialEvents(agentId: string, agentType: AgentType): Promise<void> {
    // Schedule initial performance review
    await this.scheduleMaintenance(agentId, 'performance-review', 
      new Date(Date.now() + 24 * 60 * 60 * 1000)); // 24 hours from now
  }

  private async applyLifecyclePolicies(agentId: string, agentType: AgentType): Promise<void> {
    // Apply matching policies to the agent
    for (const [name, policy] of this.lifecyclePolicies) {
      if (new RegExp(policy.agentPattern).test(agentType)) {
        // Apply policy settings
        console.log(`Applying lifecycle policy ${name} to agent ${agentId}`);
      }
    }
  }

  private findValidTransition(
    from: LifecycleState,
    to: LifecycleState,
    trigger: TransitionTrigger
  ): LifecycleTransition | null {
    return this.lifecycleTransitions.find(t => 
      t.from === from && t.to === to && t.trigger === trigger
    ) || null;
  }

  private async checkTransitionConditions(
    agentId: string,
    transition: LifecycleTransition,
    context?: Record<string, any>
  ): Promise<boolean> {
    // Check all conditions for the transition
    return true; // Simplified implementation
  }

  private async executeTransitionActions(
    agentId: string,
    transition: LifecycleTransition,
    phase: 'pre' | 'post',
    context?: Record<string, any>
  ): Promise<void> {
    // Execute transition actions
    for (const action of transition.actions) {
      await this.executeLifecycleAction(agentId, action, context);
    }
  }

  private async executeLifecycleAction(
    agentId: string,
    action: LifecycleAction,
    context?: Record<string, any>
  ): Promise<void> {
    switch (action.type) {
      case 'notify':
        await this.sendNotification(agentId, action.target, action.parameters);
        break;
      case 'cleanup':
        await this.performCleanup(agentId, action.parameters);
        break;
      case 'migrate':
        await this.migrateAgent(agentId, action.parameters);
        break;
      case 'backup':
        await this.backupAgentState(agentId, action.parameters);
        break;
      case 'delegate':
        await this.delegateTasks(agentId, action.parameters);
        break;
      case 'escalate':
        await this.escalateIssue(agentId, action.parameters);
        break;
    }
  }

  private updateStateMetrics(
    agentState: AgentLifecycleState,
    oldState: LifecycleState,
    newState: LifecycleState
  ): void {
    // Update transition count
    const transitionKey = `${oldState}->${newState}`;
    const currentCount = agentState.performance.stateTransitions.get(transitionKey) || 0;
    agentState.performance.stateTransitions.set(transitionKey, currentCount + 1);

    // Update average time in state
    const timeInState = Date.now() - agentState.lastStateChange.getTime();
    const currentAverage = agentState.performance.averageStateTime.get(oldState) || 0;
    const newAverage = (currentAverage + timeInState) / 2;
    agentState.performance.averageStateTime.set(oldState, newAverage);
  }

  private async notifyStateChange(
    agentId: string,
    oldState: LifecycleState,
    newState: LifecycleState,
    trigger: TransitionTrigger
  ): Promise<void> {
    // Notify parent and children about state change
    const agentState = this.lifecycleStates.get(agentId);
    if (!agentState) return;

    if (agentState.parentAgent) {
      await this.communicationSystem.sendMessage(
        { id: agentId, swarmId: 'lifecycle', type: 'specialist', instance: 1 },
        { id: agentState.parentAgent, swarmId: 'lifecycle', type: 'coordinator', instance: 1 },
        'notification',
        {
          subject: 'Agent State Change',
          body: `Agent ${agentId} transitioned from ${oldState} to ${newState}`,
          data: { agentId, oldState, newState, trigger },
          format: 'structured'
        }
      );
    }
  }

  // Additional private helper methods (simplified implementations)
  private async handleChildAgentTermination(agentId: string, childAgents: string[]): Promise<void> {
    for (const childId of childAgents) {
      await this.terminateAgent(childId, 'parent-terminated', true);
    }
  }

  private async transferActiveTasks(agentId: string): Promise<void> {
    // Transfer tasks to other agents or back to parent
  }

  private async cleanupAgentResources(agentId: string): Promise<void> {
    // Clean up allocated resources
  }

  private async notifyDependentAgents(agentId: string): Promise<void> {
    // Notify agents that depend on this agent
  }

  private async archiveAgentLifecycle(agentId: string): Promise<void> {
    const state = this.lifecycleStates.get(agentId);
    if (state) {
      await this.memory.store(`lifecycle:archive:${agentId}`, state, {
        type: 'logs',
        tags: ['lifecycle', 'archived'],
        partition: 'archives'
      });
      this.lifecycleStates.delete(agentId);
    }
  }

  private processScheduledEvents(): void {
    const now = new Date();
    for (const [eventId, event] of this.scheduledEvents) {
      if (!event.completed && event.scheduledAt <= now) {
        this.executeScheduledEvent(event);
      }
    }
  }

  private async executeScheduledEvent(event: ScheduledEvent): Promise<void> {
    try {
      switch (event.type) {
        case 'maintenance':
          await this.performMaintenance(event.agentId, event.parameters);
          break;
        case 'performance-review':
          await this.performPerformanceReview(event.agentId);
          break;
        case 'cleanup':
          await this.performCleanup(event.agentId, event.parameters);
          break;
        case 'retirement':
          await this.terminateAgent(event.agentId, 'planned', true);
          break;
      }
      
      event.completed = true;
      this.emit('scheduled-event:completed', event);
    } catch (error) {
      console.error(`Failed to execute scheduled event ${event.id}:`, error);
    }
  }

  private checkAgentHealth(): void {
    for (const [agentId, state] of this.lifecycleStates) {
      const health = this.calculateAgentHealth(state);
      if (health < 0.3) {
        this.emit('agent:health-warning', { agentId, health, state: state.state });
      }
    }
  }

  private enforceLifecyclePolicies(): void {
    for (const [agentId, state] of this.lifecycleStates) {
      // Apply auto-retirement and other policy enforcements
      this.checkAutoRetirement(agentId, state);
    }
  }

  private checkAutoRetirement(agentId: string, state: AgentLifecycleState): void {
    // Check if agent should be auto-retired based on policies
    const uptime = Date.now() - state.createdAt.getTime();
    const policy = this.lifecyclePolicies.get('default');
    
    if (policy?.autoRetirement.enabled && policy.maxUptime && uptime > policy.maxUptime) {
      this.terminateAgent(agentId, 'planned', true);
    }
  }

  private calculateAgentHealth(state: AgentLifecycleState): number {
    // Calculate overall health score based on performance and state
    let health = state.performance.performanceScore;
    
    // Reduce health for error states
    if (state.state === 'error') health *= 0.5;
    if (state.state === 'paused') health *= 0.8;
    
    // Factor in issues
    const criticalIssues = state.performance.issues.filter(i => i.severity === 'critical').length;
    health -= criticalIssues * 0.2;
    
    return Math.max(0, Math.min(1, health));
  }

  private getNextScheduledEvent(agentId: string): ScheduledEvent | null {
    const agentEvents = Array.from(this.scheduledEvents.values())
      .filter(e => e.agentId === agentId && !e.completed)
      .sort((a, b) => a.scheduledAt.getTime() - b.scheduledAt.getTime());
    
    return agentEvents[0] || null;
  }

  private generateLifecycleRecommendations(state: AgentLifecycleState): string[] {
    const recommendations: string[] = [];
    
    if (state.performance.performanceScore < 0.5) {
      recommendations.push('Consider performance optimization or retraining');
    }
    
    if (state.performance.issues.length > 0) {
      recommendations.push(`Address ${state.performance.issues.length} open issues`);
    }
    
    const uptime = Date.now() - state.createdAt.getTime();
    if (uptime > 5 * 24 * 60 * 60 * 1000) { // 5 days
      recommendations.push('Agent has been running for an extended period, consider planned maintenance');
    }
    
    return recommendations;
  }

  // Placeholder implementations for complex operations
  private async performMaintenance(agentId: string, parameters: Record<string, any>): Promise<void> {}
  private async performPerformanceReview(agentId: string): Promise<void> {}
  private async performCleanup(agentId: string, parameters: Record<string, any>): Promise<void> {}
  private async sendNotification(agentId: string, target?: string, parameters?: Record<string, any>): Promise<void> {}
  private async migrateAgent(agentId: string, parameters: Record<string, any>): Promise<void> {}
  private async backupAgentState(agentId: string, parameters: Record<string, any>): Promise<void> {}
  private async delegateTasks(agentId: string, parameters: Record<string, any>): Promise<void> {}
  private async escalateIssue(agentId: string, parameters: Record<string, any>): Promise<void> {}

  // Metrics calculation methods
  private groupAgentsByState(states: AgentLifecycleState[]): Map<LifecycleState, number> {
    const groups = new Map<LifecycleState, number>();
    for (const state of states) {
      groups.set(state.state, (groups.get(state.state) || 0) + 1);
    }
    return groups;
  }

  private calculateAverageUptime(states: AgentLifecycleState[]): number {
    if (states.length === 0) return 0;
    const totalUptime = states.reduce((sum, state) => 
      sum + (Date.now() - state.createdAt.getTime()), 0);
    return totalUptime / states.length;
  }

  private calculateAveragePerformance(states: AgentLifecycleState[]): number {
    if (states.length === 0) return 0;
    const totalPerformance = states.reduce((sum, state) => 
      sum + state.performance.performanceScore, 0);
    return totalPerformance / states.length;
  }

  private calculateTotalTransitions(states: AgentLifecycleState[]): number {
    return states.reduce((sum, state) => {
      const transitions = Array.from(state.performance.stateTransitions.values());
      return sum + transitions.reduce((s, t) => s + t, 0);
    }, 0);
  }

  private getActiveIssues(states: AgentLifecycleState[]): LifecycleIssue[] {
    return states.flatMap(state => 
      state.performance.issues.filter(issue => !issue.resolvedAt)
    );
  }

  private calculateResourceUtilization(states: AgentLifecycleState[]): number {
    // Calculate overall resource utilization across all agents
    const totalResources = states.reduce((sum, state) => ({
      cpu: sum.cpu + state.resources.cpu,
      memory: sum.memory + state.resources.memory,
      storage: sum.storage + state.resources.storage
    }), { cpu: 0, memory: 0, storage: 0 });

    // Simplified calculation - in practice would compare against system limits
    return Math.min(1, (totalResources.cpu + totalResources.memory + totalResources.storage) / 1000);
  }
}

// Additional interfaces for lifecycle management
export interface AgentLifecycleStatus {
  agentId: string;
  currentState: LifecycleState;
  uptime: number;
  timeSinceLastStateChange: number;
  performance: AgentLifecycleMetrics;
  resources: AllocatedResources;
  scheduledEvents: ScheduledEvent[];
  health: number;
  nextScheduledEvent: ScheduledEvent | null;
  recommendations: string[];
}

export interface LifecycleSystemMetrics {
  totalAgents: number;
  agentsByState: Map<LifecycleState, number>;
  averageUptime: number;
  averagePerformance: number;
  totalStateTransitions: number;
  activeIssues: LifecycleIssue[];
  scheduledMaintenanceEvents: number;
  resourceUtilization: number;
}