// Internal module type definitions for missing exports and interfaces
// This file provides type definitions for internal modules that are missing or incomplete

// Task engine types
declare module '../task/engine.js' {
  export interface Task {
    id: string;
    name: string;
    description?: string;
    status: 'pending' | 'in_progress' | 'completed' | 'failed' | 'cancelled';
    priority: 'low' | 'medium' | 'high' | 'urgent';
    dependencies: string[];
    assignedAgent?: string;
    metadata?: {
      createdAt: Date;
      updatedAt: Date;
      retryCount?: number;
      estimatedDuration?: number;
      actualDuration?: number;
      tags?: string[];
      category?: string;
      complexity?: 'simple' | 'moderate' | 'complex';
      [key: string]: any;
    };
    data?: any;
    result?: any;
    error?: Error;
    progress?: number;
    subtasks?: Task[];
    parentTaskId?: string;
    workflowId?: string;
    agentId?: string;
    createdBy?: string;
    updatedBy?: string;
    version?: number;
    executionContext?: any;
    timeout?: number;
    retryPolicy?: {
      maxRetries: number;
      retryDelay: number;
      backoffStrategy: 'linear' | 'exponential';
    };
  }

  export interface TaskDependency {
    id: string;
    taskId: string;
    dependsOnTaskId: string;
    type: 'sequential' | 'parallel' | 'conditional';
    condition?: (task: Task) => boolean;
    metadata?: any;
  }

  export interface WorkflowTask extends Task {
    dependencies: TaskDependency[];
    workflow?: {
      id: string;
      name: string;
      version: string;
      steps: Task[];
      metadata?: any;
    };
  }

  export interface TaskExecutionContext {
    taskId: string;
    agentId?: string;
    workflowId?: string;
    sessionId?: string;
    userId?: string;
    environment: 'development' | 'staging' | 'production';
    resources?: {
      memory: number;
      cpu: number;
      disk: number;
      network: number;
    };
    permissions?: string[];
    variables?: Record<string, any>;
    startTime: Date;
    endTime?: Date;
    logs?: LogEntry[];
  }

  export interface LogEntry {
    timestamp: Date;
    level: 'debug' | 'info' | 'warn' | 'error';
    message: string;
    data?: any;
    source?: string;
  }

  export interface TaskEngineConfig {
    maxConcurrentTasks: number;
    taskTimeout: number;
    retryPolicy: {
      maxRetries: number;
      retryDelay: number;
      backoffStrategy: 'linear' | 'exponential';
    };
    persistence: {
      enabled: boolean;
      adapter: 'memory' | 'file' | 'database';
      config?: any;
    };
    logging: {
      enabled: boolean;
      level: 'debug' | 'info' | 'warn' | 'error';
      destination: 'console' | 'file' | 'remote';
      config?: any;
    };
    monitoring: {
      enabled: boolean;
      metricsInterval: number;
      healthCheckInterval: number;
    };
    security: {
      enabled: boolean;
      authentication: boolean;
      authorization: boolean;
      encryption: boolean;
    };
  }

  export class TaskEngine {
    constructor(config?: Partial<TaskEngineConfig>);
    
    // Task management
    createTask(task: Partial<Task>): Promise<Task>;
    getTask(id: string): Promise<Task | null>;
    updateTask(id: string, updates: Partial<Task>): Promise<Task>;
    deleteTask(id: string): Promise<boolean>;
    listTasks(filter?: Partial<Task>): Promise<Task[]>;
    
    // Task execution
    executeTask(id: string, context?: Partial<TaskExecutionContext>): Promise<any>;
    cancelTask(id: string): Promise<boolean>;
    pauseTask(id: string): Promise<boolean>;
    resumeTask(id: string): Promise<boolean>;
    retryTask(id: string): Promise<any>;
    
    // Workflow management
    createWorkflow(tasks: Task[], dependencies: TaskDependency[]): Promise<string>;
    executeWorkflow(workflowId: string): Promise<any>;
    cancelWorkflow(workflowId: string): Promise<boolean>;
    pauseWorkflow(workflowId: string): Promise<boolean>;
    resumeWorkflow(workflowId: string): Promise<boolean>;
    
    // Dependencies
    addDependency(taskId: string, dependsOnTaskId: string, type?: TaskDependency['type']): Promise<TaskDependency>;
    removeDependency(dependencyId: string): Promise<boolean>;
    getDependencies(taskId: string): Promise<TaskDependency[]>;
    
    // Monitoring
    getMetrics(): Promise<TaskEngineMetrics>;
    getStatus(): Promise<TaskEngineStatus>;
    getHealth(): Promise<TaskEngineHealth>;
    
    // Event handling
    on(event: string, listener: (...args: any[]) => void): void;
    off(event: string, listener: (...args: any[]) => void): void;
    emit(event: string, ...args: any[]): boolean;
    
    // Lifecycle
    start(): Promise<void>;
    stop(): Promise<void>;
    restart(): Promise<void>;
    reset(): Promise<void>;
  }

  export interface TaskEngineMetrics {
    totalTasks: number;
    pendingTasks: number;
    inProgressTasks: number;
    completedTasks: number;
    failedTasks: number;
    cancelledTasks: number;
    averageExecutionTime: number;
    taskThroughput: number;
    errorRate: number;
    resourceUsage: {
      memory: number;
      cpu: number;
      disk: number;
      network: number;
    };
    timestamp: Date;
  }

  export interface TaskEngineStatus {
    isRunning: boolean;
    startTime: Date;
    uptime: number;
    activeTasks: Task[];
    queuedTasks: Task[];
    completedTasks: number;
    failedTasks: number;
    lastError?: Error;
    lastHeartbeat: Date;
  }

  export interface TaskEngineHealth {
    status: 'healthy' | 'degraded' | 'unhealthy';
    checks: Array<{
      name: string;
      status: 'pass' | 'fail' | 'warn';
      message?: string;
      timestamp: Date;
    }>;
    details?: any;
  }

  export const TaskEngine: {
    new (config?: Partial<TaskEngineConfig>): TaskEngine;
  };
}

// Task coordination types
declare module '../task/coordination.js' {
  export interface TaskMetrics {
    executionTime: number;
    memoryUsage: number;
    cpuUsage: number;
    networkUsage: number;
    diskUsage: number;
    errorCount: number;
    warningCount: number;
    successCount: number;
    retryCount: number;
    timestamp: Date;
  }

  export interface TaskLog {
    id: string;
    taskId: string;
    timestamp: Date;
    level: 'debug' | 'info' | 'warn' | 'error';
    message: string;
    data?: any;
    source: string;
    agentId?: string;
    workflowId?: string;
    sessionId?: string;
  }

  export interface CoordinationContext {
    sessionId: string;
    workflowId?: string;
    userId?: string;
    agentId?: string;
    environment: 'development' | 'staging' | 'production';
    permissions: string[];
    resources: {
      memory: number;
      cpu: number;
      disk: number;
      network: number;
    };
    configuration: Record<string, any>;
    startTime: Date;
    timeout?: number;
    priority: 'low' | 'medium' | 'high' | 'urgent';
  }

  export interface TaskCoordinatorConfig {
    maxConcurrentTasks: number;
    taskTimeout: number;
    retryPolicy: {
      maxRetries: number;
      retryDelay: number;
      backoffStrategy: 'linear' | 'exponential';
    };
    loadBalancing: {
      enabled: boolean;
      strategy: 'round_robin' | 'least_connections' | 'weighted' | 'random';
      weights?: Record<string, number>;
    };
    failover: {
      enabled: boolean;
      strategy: 'immediate' | 'graceful' | 'manual';
      healthCheckInterval: number;
    };
    monitoring: {
      enabled: boolean;
      metricsInterval: number;
      logLevel: 'debug' | 'info' | 'warn' | 'error';
    };
    persistence: {
      enabled: boolean;
      adapter: 'memory' | 'file' | 'database';
      config?: any;
    };
  }

  export class TaskCoordinator {
    constructor(config?: Partial<TaskCoordinatorConfig>);
    
    // Task coordination
    assignTask(taskId: string, agentId: string, taskData?: any): Promise<boolean>;
    unassignTask(taskId: string): Promise<boolean>;
    reassignTask(taskId: string, newAgentId: string): Promise<boolean>;
    balanceTasks(): Promise<void>;
    
    // Agent management
    registerAgent(agentId: string, capabilities: string[]): Promise<boolean>;
    unregisterAgent(agentId: string): Promise<boolean>;
    getAgentStatus(agentId: string): Promise<AgentStatus>;
    listAgents(): Promise<AgentInfo[]>;
    
    // Task lifecycle
    onTaskCreated(callback: (task: Task) => void): void;
    onTaskStarted(callback: (task: Task) => void): void;
    onTaskCompleted(callback: (task: Task, result: any) => void): void;
    onTaskFailed(callback: (task: Task, error: Error) => void): void;
    onTaskCancelled(callback: (task: Task) => void): void;
    
    // Coordination strategies
    scheduleTask(task: Task, context: CoordinationContext): Promise<string>;
    coordinateWorkflow(workflow: WorkflowDefinition): Promise<string>;
    optimizeTaskDistribution(): Promise<void>;
    
    // Monitoring and metrics
    getMetrics(): Promise<CoordinationMetrics>;
    getTaskMetrics(taskId: string): Promise<TaskMetrics>;
    getLogs(filter?: LogFilter): Promise<TaskLog[]>;
    
    // Configuration
    updateConfig(config: Partial<TaskCoordinatorConfig>): Promise<void>;
    getConfig(): TaskCoordinatorConfig;
    
    // Health and status
    getHealth(): Promise<CoordinatorHealth>;
    getStatus(): Promise<CoordinatorStatus>;
    
    // Lifecycle
    start(): Promise<void>;
    stop(): Promise<void>;
    restart(): Promise<void>;
    
    // Properties
    tasks: Map<string, Task>;
    executions: Map<string, TaskExecution>;
  }

  export interface AgentStatus {
    id: string;
    isActive: boolean;
    capabilities: string[];
    currentTasks: string[];
    maxConcurrentTasks: number;
    loadLevel: number;
    lastHeartbeat: Date;
    health: 'healthy' | 'degraded' | 'unhealthy';
    metadata?: any;
  }

  export interface AgentInfo {
    id: string;
    name: string;
    type: string;
    capabilities: string[];
    status: AgentStatus;
    registeredAt: Date;
    lastActive: Date;
    totalTasksCompleted: number;
    averageTaskTime: number;
    successRate: number;
  }

  export interface WorkflowDefinition {
    id: string;
    name: string;
    description?: string;
    version: string;
    tasks: Task[];
    dependencies: TaskDependency[];
    configuration: Record<string, any>;
    triggers: WorkflowTrigger[];
    schedule?: WorkflowSchedule;
    metadata?: any;
  }

  export interface WorkflowTrigger {
    type: 'manual' | 'scheduled' | 'event' | 'webhook';
    configuration: any;
    enabled: boolean;
  }

  export interface WorkflowSchedule {
    type: 'cron' | 'interval' | 'once';
    expression: string;
    timezone?: string;
    enabled: boolean;
  }

  export interface CoordinationMetrics {
    totalTasksCoordinated: number;
    activeCoordinations: number;
    completedCoordinations: number;
    failedCoordinations: number;
    averageCoordinationTime: number;
    agentUtilization: Record<string, number>;
    taskDistribution: Record<string, number>;
    errorRate: number;
    throughput: number;
    timestamp: Date;
  }

  export interface TaskExecution {
    id: string;
    taskId: string;
    agentId: string;
    status: 'pending' | 'running' | 'completed' | 'failed' | 'cancelled';
    startTime: Date;
    endTime?: Date;
    duration?: number;
    progress: number;
    result?: any;
    error?: Error;
    metrics: TaskMetrics;
    logs: TaskLog[];
    context: CoordinationContext;
  }

  export interface LogFilter {
    taskId?: string;
    agentId?: string;
    level?: 'debug' | 'info' | 'warn' | 'error';
    startTime?: Date;
    endTime?: Date;
    limit?: number;
    offset?: number;
    source?: string;
  }

  export interface CoordinatorHealth {
    status: 'healthy' | 'degraded' | 'unhealthy';
    agents: number;
    activeTasks: number;
    queuedTasks: number;
    errors: number;
    warnings: number;
    uptime: number;
    checks: Array<{
      name: string;
      status: 'pass' | 'fail' | 'warn';
      message?: string;
      timestamp: Date;
    }>;
  }

  export interface CoordinatorStatus {
    isActive: boolean;
    startTime: Date;
    uptime: number;
    registeredAgents: number;
    activeTasks: number;
    queuedTasks: number;
    completedTasks: number;
    failedTasks: number;
    totalTasksProcessed: number;
    averageTaskTime: number;
    lastError?: Error;
  }

  export const TaskCoordinator: {
    new (config?: Partial<TaskCoordinatorConfig>): TaskCoordinator;
  };
}

// Utils types
declare module '../utils/types.js' {
  export interface TaskMetrics {
    executionTime: number;
    memoryUsage: number;
    cpuUsage: number;
    networkUsage: number;
    diskUsage: number;
    errorCount: number;
    warningCount: number;
    successCount: number;
    retryCount: number;
    timestamp: Date;
  }

  export interface TaskLog {
    id: string;
    taskId: string;
    timestamp: Date;
    level: 'debug' | 'info' | 'warn' | 'error';
    message: string;
    data?: any;
    source: string;
    agentId?: string;
    workflowId?: string;
    sessionId?: string;
  }

  export interface AgentProfile {
    id: string;
    name: string;
    type: 'researcher' | 'coder' | 'analyst' | 'coordinator' | 'specialist' | 'tester' | 'reviewer' | 'documenter' | 'monitor';
    capabilities: string[];
    configuration: AgentConfiguration;
    status: AgentStatus;
    metadata: AgentMetadata;
    createdAt: Date;
    updatedAt: Date;
    version: string;
  }

  export interface AgentConfiguration {
    maxConcurrentTasks: number;
    timeout: number;
    retryPolicy: {
      maxRetries: number;
      retryDelay: number;
      backoffStrategy: 'linear' | 'exponential';
    };
    resources: {
      memory: number;
      cpu: number;
      disk: number;
      network: number;
    };
    permissions: string[];
    environment: Record<string, any>;
    tools: string[];
    models: string[];
    endpoints: Record<string, string>;
  }

  export interface AgentStatus {
    isActive: boolean;
    health: 'healthy' | 'degraded' | 'unhealthy';
    loadLevel: number;
    currentTasks: number;
    maxTasks: number;
    lastHeartbeat: Date;
    lastError?: Error;
    uptime: number;
    totalTasksCompleted: number;
    averageTaskTime: number;
    successRate: number;
  }

  export interface AgentMetadata {
    description?: string;
    tags: string[];
    owner?: string;
    team?: string;
    department?: string;
    costCenter?: string;
    priority: 'low' | 'medium' | 'high' | 'critical';
    slaLevel: string;
    documentation?: string;
    supportContact?: string;
    maintenanceWindow?: string;
    deprecationDate?: Date;
  }

  export interface TaskType {
    id: string;
    name: string;
    description?: string;
    category: string;
    complexity: 'simple' | 'moderate' | 'complex';
    estimatedDuration: number;
    requiredCapabilities: string[];
    optionalCapabilities: string[];
    requiredResources: {
      memory: number;
      cpu: number;
      disk: number;
      network: number;
    };
    schema: {
      input: any;
      output: any;
      configuration: any;
    };
    validation: {
      input: (data: any) => boolean;
      output: (data: any) => boolean;
      configuration: (data: any) => boolean;
    };
    defaultConfiguration: any;
    metadata: {
      version: string;
      author: string;
      license: string;
      documentation: string;
      examples: any[];
      tags: string[];
    };
  }

  export interface CoordinationStrategy {
    id: string;
    name: string;
    description?: string;
    type: 'sequential' | 'parallel' | 'pipeline' | 'tree' | 'graph' | 'adaptive';
    configuration: any;
    rules: CoordinationRule[];
    metrics: StrategyMetrics;
    isActive: boolean;
    priority: number;
    createdAt: Date;
    updatedAt: Date;
  }

  export interface CoordinationRule {
    id: string;
    condition: string;
    action: string;
    priority: number;
    enabled: boolean;
    metadata?: any;
  }

  export interface StrategyMetrics {
    totalExecutions: number;
    successfulExecutions: number;
    failedExecutions: number;
    averageExecutionTime: number;
    averageTaskCount: number;
    efficiency: number;
    lastExecuted: Date;
    lastResult: 'success' | 'failure' | 'partial';
  }

  export interface DecompositionResult {
    originalTask: Task;
    subtasks: Task[];
    dependencies: TaskDependency[];
    estimatedTime: number;
    confidence: number;
    strategy: string;
    metadata: {
      decompositionMethod: string;
      analysisTime: number;
      complexityScore: number;
      riskLevel: 'low' | 'medium' | 'high';
      recommendations: string[];
    };
  }

  export interface ExecutionPlan {
    id: string;
    workflowId: string;
    tasks: Task[];
    dependencies: TaskDependency[];
    schedule: ExecutionSchedule;
    resources: ResourceAllocation;
    constraints: ExecutionConstraint[];
    contingencies: ContingencyPlan[];
    metrics: PlanMetrics;
    status: 'draft' | 'approved' | 'executing' | 'completed' | 'failed' | 'cancelled';
    createdAt: Date;
    updatedAt: Date;
  }

  export interface ExecutionSchedule {
    startTime: Date;
    endTime: Date;
    estimatedDuration: number;
    phases: ExecutionPhase[];
    milestones: Milestone[];
    criticalPath: string[];
  }

  export interface ExecutionPhase {
    id: string;
    name: string;
    tasks: string[];
    startTime: Date;
    endTime: Date;
    dependencies: string[];
    status: 'pending' | 'running' | 'completed' | 'failed';
  }

  export interface Milestone {
    id: string;
    name: string;
    description?: string;
    targetDate: Date;
    criteria: string[];
    dependencies: string[];
    status: 'pending' | 'achieved' | 'missed';
  }

  export interface ResourceAllocation {
    agents: Array<{
      agentId: string;
      tasks: string[];
      allocation: number;
      startTime: Date;
      endTime: Date;
    }>;
    compute: {
      memory: number;
      cpu: number;
      disk: number;
      network: number;
    };
    external: Array<{
      type: string;
      identifier: string;
      allocation: any;
    }>;
  }

  export interface ExecutionConstraint {
    type: 'time' | 'resource' | 'dependency' | 'policy' | 'security';
    description: string;
    value: any;
    enforced: boolean;
    priority: 'low' | 'medium' | 'high' | 'critical';
  }

  export interface ContingencyPlan {
    trigger: string;
    condition: string;
    actions: ContingencyAction[];
    priority: number;
    activated: boolean;
  }

  export interface ContingencyAction {
    type: 'retry' | 'reschedule' | 'reassign' | 'escalate' | 'abort';
    configuration: any;
    timeout: number;
  }

  export interface PlanMetrics {
    totalTasks: number;
    estimatedDuration: number;
    requiredResources: ResourceAllocation['compute'];
    riskScore: number;
    complexityScore: number;
    confidence: number;
    costEstimate: number;
  }

  export interface WorkflowInstance {
    id: string;
    definitionId: string;
    status: 'pending' | 'running' | 'completed' | 'failed' | 'cancelled' | 'paused';
    progress: number;
    startTime: Date;
    endTime?: Date;
    duration?: number;
    currentPhase: string;
    completedTasks: number;
    totalTasks: number;
    failedTasks: number;
    context: WorkflowContext;
    result?: any;
    error?: Error;
    logs: TaskLog[];
    metrics: WorkflowMetrics;
  }

  export interface WorkflowContext {
    userId: string;
    sessionId: string;
    environment: 'development' | 'staging' | 'production';
    variables: Record<string, any>;
    permissions: string[];
    configuration: Record<string, any>;
  }

  export interface WorkflowMetrics {
    executionTime: number;
    taskCount: number;
    successRate: number;
    errorRate: number;
    throughput: number;
    resourceUtilization: ResourceAllocation['compute'];
    costActual: number;
    efficiency: number;
  }

  // Common utility types
  export type UUID = string;
  export type Timestamp = Date;
  export type JSONValue = string | number | boolean | null | JSONObject | JSONArray;
  export interface JSONObject {
    [key: string]: JSONValue;
  }
  export interface JSONArray extends Array<JSONValue> {}

  export interface BaseEntity {
    id: UUID;
    createdAt: Timestamp;
    updatedAt: Timestamp;
    version: number;
    metadata?: JSONObject;
  }

  export interface PaginationOptions {
    page: number;
    limit: number;
    sortBy?: string;
    sortOrder?: 'asc' | 'desc';
  }

  export interface PaginatedResult<T> {
    data: T[];
    total: number;
    page: number;
    limit: number;
    totalPages: number;
    hasNext: boolean;
    hasPrevious: boolean;
  }

  export interface FilterOptions {
    [key: string]: any;
  }

  export interface SearchOptions {
    query: string;
    fields?: string[];
    fuzzy?: boolean;
    caseSensitive?: boolean;
  }

  export interface ValidationResult {
    isValid: boolean;
    errors: ValidationError[];
    warnings: ValidationWarning[];
  }

  export interface ValidationError {
    field: string;
    message: string;
    code: string;
    value?: any;
  }

  export interface ValidationWarning {
    field: string;
    message: string;
    code: string;
    value?: any;
  }

  export interface ApiResponse<T = any> {
    success: boolean;
    data?: T;
    error?: {
      code: string;
      message: string;
      details?: any;
    };
    metadata?: {
      timestamp: Timestamp;
      requestId: UUID;
      version: string;
      [key: string]: any;
    };
  }

  export interface EventPayload {
    type: string;
    source: string;
    data: any;
    timestamp: Timestamp;
    metadata?: JSONObject;
  }

  export interface Command {
    id: UUID;
    type: string;
    payload: any;
    timestamp: Timestamp;
    userId?: string;
    sessionId?: string;
    metadata?: JSONObject;
  }

  export interface Query {
    id: UUID;
    type: string;
    filters: FilterOptions;
    pagination?: PaginationOptions;
    search?: SearchOptions;
    userId?: string;
    sessionId?: string;
    metadata?: JSONObject;
  }
}

// P-queue types
declare module 'p-queue' {
  export interface QueueAddOptions {
    priority?: number;
    signal?: AbortSignal;
  }

  export interface Options {
    concurrency?: number;
    interval?: number;
    intervalCap?: number;
    carryoverConcurrencyCount?: boolean;
    autoStart?: boolean;
    queueClass?: new () => any;
    timeout?: number;
    throwOnTimeout?: boolean;
  }

  export default class PQueue {
    constructor(options?: Options);
    
    readonly size: number;
    readonly sizeBy: Record<string, number>;
    readonly pending: number;
    readonly isPaused: boolean;
    readonly timeout?: number;
    
    add<T>(fn: () => Promise<T>, options?: QueueAddOptions): Promise<T>;
    addAll<T>(fns: Array<() => Promise<T>>, options?: QueueAddOptions): Promise<T[]>;
    start(): this;
    pause(): void;
    clear(): void;
    onEmpty(): Promise<void>;
    onSizeLessThan(limit: number): Promise<void>;
    onIdle(): Promise<void>;
    
    // Events
    on(event: 'add' | 'next' | 'completed' | 'idle' | 'error', listener: (...args: any[]) => void): this;
    off(event: 'add' | 'next' | 'completed' | 'idle' | 'error', listener: (...args: any[]) => void): this;
  }
}

// Additional Node.js polyfills
declare module 'node:path' {
  export * from 'path';
}

declare module 'node:fs' {
  export * from 'fs';
}

declare module 'node:fs/promises' {
  export * from 'fs/promises';
}

declare module 'node:util' {
  export * from 'util';
}

declare module 'node:crypto' {
  export * from 'crypto';
}

declare module 'node:os' {
  export * from 'os';
}

declare module 'node:process' {
  export * from 'process';
}

declare module 'node:stream' {
  export * from 'stream';
}

declare module 'node:url' {
  export * from 'url';
}

declare module 'node:events' {
  export * from 'events';
}