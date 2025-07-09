/**
 * Optimized Research Strategy Implementation
 * Provides intelligent research capabilities with parallel processing,
 * semantic clustering, caching, and progressive refinement
 */

import { BaseStrategy, DecompositionResult, StrategyMetrics, AgentAllocation } from './base.js';
import { Logger } from '../../core/logger.js';
import { generateId } from '../../utils/helpers.js';
import {
  SwarmObjective, TaskDefinition, TaskId, TaskType, TaskPriority,
  SwarmConfig, SWARM_CONSTANTS, AgentState
} from '../types.js';

// Research-specific interfaces
interface ResearchQuery {
  id: string;
  query: string;
  keywords: string[];
  domains: string[];
  priority: number;
  timestamp: Date;
  sources?: string[];
  filters?: ResearchFilters;
}

interface ResearchFilters {
  dateRange?: { start: Date; end: Date };
  sourceTypes?: ('academic' | 'news' | 'blog' | 'documentation' | 'forum')[];
  languages?: string[];
  credibilityThreshold?: number;
  maxResults?: number;
}

interface ResearchResult {
  id: string;
  queryId: string;
  url: string;
  title: string;
  content: string;
  summary: string;
  credibilityScore: number;
  relevanceScore: number;
  sourceType: string;
  publishedDate?: Date;
  extractedAt: Date;
  metadata: Record<string, any>;
  semanticVector?: number[];
}

interface ResearchCluster {
  id: string;
  topic: string;
  results: ResearchResult[];
  centroid: number[];
  coherenceScore: number;
  keywords: string[];
  summary: string;
}

interface CacheEntry {
  key: string;
  data: any;
  timestamp: Date;
  ttl: number;
  accessCount: number;
  lastAccessed: Date;
}

interface ConnectionPool {
  active: number;
  idle: number;
  max: number;
  timeout: number;
  connections: Map<string, any>;
}

interface RateLimiter {
  requests: number;
  windowStart: Date;
  windowSize: number;
  maxRequests: number;
  backoffMultiplier: number;
}

export class ResearchStrategy extends BaseStrategy {
  private logger: Logger;
  private researchCache: Map<string, CacheEntry> = new Map();
  private connectionPool: ConnectionPool;
  private rateLimiters: Map<string, RateLimiter> = new Map();
  private semanticModel: any; // Placeholder for semantic analysis
  private researchQueries: Map<string, ResearchQuery> = new Map();
  private researchResults: Map<string, ResearchResult> = new Map();
  private researchClusters: Map<string, ResearchCluster> = new Map();

  // Research-specific metrics extending base metrics
  private researchMetrics = {
    queriesExecuted: 0,
    resultsCollected: 0,
    cacheHits: 0,
    cacheMisses: 0,
    averageResponseTime: 0,
    credibilityScores: [] as number[],
    clusteringAccuracy: 0,
    parallelEfficiency: 0
  };

  constructor(config: Partial<SwarmConfig> = {}) {
    const fullConfig: SwarmConfig = {
      name: config.name || 'ResearchSwarm',
      description: config.description || 'Research-focused swarm configuration',
      version: config.version || '1.0.0',
      mode: config.mode || 'distributed',
      strategy: config.strategy || 'research',
      coordinationStrategy: config.coordinationStrategy || {
        name: 'consensus',
        description: 'Consensus-based coordination strategy',
        agentSelection: 'capability-based',
        taskScheduling: 'priority',
        loadBalancing: 'work-stealing',
        faultTolerance: 'retry',
        communication: 'direct'
      },
      maxAgents: config.maxAgents || 5,
      maxTasks: config.maxTasks || 100,
      maxDuration: config.maxDuration || 3600000,
      taskTimeoutMinutes: config.taskTimeoutMinutes || 30,
      resourceLimits: config.resourceLimits || {},
      qualityThreshold: config.qualityThreshold || 0.8,
      reviewRequired: config.reviewRequired || false,
      testingRequired: config.testingRequired || false,
      monitoring: config.monitoring || {
        metricsEnabled: true,
        loggingEnabled: true,
        tracingEnabled: false,
        metricsInterval: 5000,
        heartbeatInterval: 10000,
        healthCheckInterval: 30000,
        retentionPeriod: 86400000,
        maxLogSize: 10485760,
        maxMetricPoints: 1000,
        alertingEnabled: false,
        alertThresholds: {},
        exportEnabled: false,
        exportFormat: 'json',
        exportDestination: 'console'
      },
      memory: config.memory || {
        namespace: 'research',
        partitions: [],
        permissions: {
          read: 'swarm',
          write: 'swarm',
          delete: 'team',
          share: 'swarm'
        },
        persistent: false,
        backupEnabled: false,
        distributed: false,
        consistency: 'eventual',
        cacheEnabled: true,
        compressionEnabled: false
      },
      security: config.security || {
        authenticationRequired: false,
        authorizationRequired: false,
        encryptionEnabled: false,
        defaultPermissions: ['read', 'write'],
        adminRoles: ['admin'],
        auditEnabled: false,
        auditLevel: 'info',
        inputValidation: true,
        outputSanitization: true
      },
      performance: config.performance || {
        maxConcurrency: 5,
        defaultTimeout: 30000,
        cacheEnabled: true,
        cacheSize: 1000,
        cacheTtl: 300000,
        optimizationEnabled: true,
        adaptiveScheduling: true,
        predictiveLoading: false,
        resourcePooling: true,
        connectionPooling: true,
        memoryPooling: true
      },
      ...config
    };
    super(fullConfig);
    
    this.logger = new Logger(
      { level: 'info', format: 'text', destination: 'console' },
      { component: 'ResearchStrategy' }
    );

    // Initialize connection pool
    this.connectionPool = {
      active: 0,
      idle: 0,
      max: config.performance?.maxConcurrency || 10,
      timeout: 30000,
      connections: new Map()
    };

    this.logger.info('ResearchStrategy initialized with optimizations', {
      maxConcurrency: this.connectionPool.max,
      cacheEnabled: config.performance?.cacheEnabled !== false
    });
  }

  async decomposeObjective(objective: SwarmObjective): Promise<DecompositionResult> {
    this.logger.info('Decomposing research objective', {
      objectiveId: objective.id,
      description: objective.description
    });

    const tasks: TaskDefinition[] = [];
    const dependencies = new Map<string, string[]>();
    
    // Extract research parameters from objective
    const researchParams = this.extractResearchParameters(objective.description);
    
    // Create research query planning task
    const queryPlanningTask = this.createResearchTask(
      'query-planning',
      'research-planning',
      'Research Query Planning',
      `Analyze the research objective and create optimized search queries:

${objective.description}

Create a comprehensive research plan that includes:
1. Primary and secondary research questions
2. Key search terms and synonyms
3. Relevant domains and sources to explore
4. Research methodology and approach
5. Quality criteria for evaluating sources

Focus on creating queries that will yield high-quality, credible results.`,
      {
        priority: 'high' as TaskPriority,
        estimatedDuration: 5 * 60 * 1000, // 5 minutes
        requiredCapabilities: ['research', 'analysis'],
        researchParams
      }
    );
    tasks.push(queryPlanningTask);

    // Create parallel web search tasks
    const webSearchTask = this.createResearchTask(
      'web-search',
      'web-search',
      'Parallel Web Search Execution',
      `Execute parallel web searches based on the research plan:

${objective.description}

Perform comprehensive web searches using:
1. Multiple search engines and sources
2. Parallel query execution for efficiency
3. Intelligent source ranking and filtering
4. Real-time credibility assessment
5. Deduplication of results

Collect diverse, high-quality sources relevant to the research objective.`,
      {
        priority: 'high' as TaskPriority,
        estimatedDuration: 10 * 60 * 1000, // 10 minutes
        requiredCapabilities: ['web-search', 'research'],
        dependencies: [queryPlanningTask.id.id],
        researchParams
      }
    );
    tasks.push(webSearchTask);
    dependencies.set(webSearchTask.id.id, [queryPlanningTask.id.id]);

    // Create data extraction and processing task
    const dataExtractionTask = this.createResearchTask(
      'data-extraction',
      'data-processing',
      'Parallel Data Extraction',
      `Extract and process data from collected sources:

${objective.description}

Process the collected sources by:
1. Extracting key information and insights
2. Performing semantic analysis and clustering
3. Identifying patterns and relationships
4. Assessing information quality and reliability
5. Creating structured summaries

Use parallel processing for efficient data extraction.`,
      {
        priority: 'high' as TaskPriority,
        estimatedDuration: 8 * 60 * 1000, // 8 minutes
        requiredCapabilities: ['analysis', 'research'],
        dependencies: [webSearchTask.id.id],
        researchParams
      }
    );
    tasks.push(dataExtractionTask);
    dependencies.set(dataExtractionTask.id.id, [webSearchTask.id.id]);

    // Create semantic clustering task
    const clusteringTask = this.createResearchTask(
      'semantic-clustering',
      'analysis',
      'Semantic Clustering and Analysis',
      `Perform semantic clustering of research findings:

${objective.description}

Analyze the extracted data by:
1. Grouping related information using semantic similarity
2. Identifying key themes and topics
3. Creating coherent clusters of information
4. Generating cluster summaries and insights
5. Mapping relationships between clusters

Provide a structured analysis of the research findings.`,
      {
        priority: 'medium' as TaskPriority,
        estimatedDuration: 6 * 60 * 1000, // 6 minutes
        requiredCapabilities: ['analysis', 'research'],
        dependencies: [dataExtractionTask.id.id],
        researchParams
      }
    );
    tasks.push(clusteringTask);
    dependencies.set(clusteringTask.id.id, [dataExtractionTask.id.id]);

    // Create synthesis and reporting task
    const synthesisTask = this.createResearchTask(
      'synthesis-reporting',
      'documentation',
      'Research Synthesis and Reporting',
      `Synthesize research findings into comprehensive report:

${objective.description}

Create a comprehensive research report that includes:
1. Executive summary of key findings
2. Detailed analysis of each research cluster
3. Insights and recommendations
4. Source credibility assessment
5. Methodology and limitations
6. References and citations

Ensure the report is well-structured and actionable.`,
      {
        priority: 'medium' as TaskPriority,
        estimatedDuration: 7 * 60 * 1000, // 7 minutes
        requiredCapabilities: ['documentation', 'analysis'],
        dependencies: [clusteringTask.id.id],
        researchParams
      }
    );
    tasks.push(synthesisTask);
    dependencies.set(synthesisTask.id.id, [clusteringTask.id.id]);

    const totalDuration = tasks.reduce((sum, task) => 
      sum + (task.constraints.timeoutAfter || 0), 0
    );

    this.logger.info('Research objective decomposed', {
      objectiveId: objective.id,
      taskCount: tasks.length,
      estimatedDuration: totalDuration,
      parallelTasks: tasks.filter(t => !dependencies.has(t.id.id)).length
    });

    return {
      tasks,
      dependencies,
      estimatedDuration: totalDuration,
      recommendedStrategy: 'research',
      complexity: Math.min(tasks.length * 0.2, 1.0), // Normalize complexity 0-1
      batchGroups: [
        {
          id: 'research-planning',
          tasks: tasks.filter(t => t.type === 'research-planning'),
          canRunInParallel: false,
          estimatedDuration: 5 * 60 * 1000,
          requiredResources: { cpu: 1, memory: 100 }
        },
        {
          id: 'parallel-search',
          tasks: tasks.filter(t => t.type === 'web-search'),
          canRunInParallel: true,
          estimatedDuration: 10 * 60 * 1000,
          requiredResources: { cpu: 2, memory: 200 }
        },
        {
          id: 'data-processing',
          tasks: tasks.filter(t => t.type === 'data-processing'),
          canRunInParallel: true,
          estimatedDuration: 8 * 60 * 1000,
          requiredResources: { cpu: 1, memory: 150 }
        },
        {
          id: 'analysis',
          tasks: tasks.filter(t => t.type === 'analysis'),
          canRunInParallel: false,
          estimatedDuration: 7 * 60 * 1000,
          requiredResources: { cpu: 1, memory: 100 }
        }
      ]
    };
  }

  // Research-specific optimizations for task execution
  async optimizeTaskExecution(task: TaskDefinition, agent: any): Promise<any> {
    const startTime = Date.now();
    
    try {
      // Apply research-specific optimizations based on task type
      switch (task.type) {
        case 'web-search':
          return await this.executeOptimizedWebSearch(task, agent);
        case 'data-processing':
          return await this.executeOptimizedDataExtraction(task, agent);
        case 'analysis':
          return await this.executeOptimizedClustering(task, agent);
        default:
          return await this.executeGenericResearchTask(task, agent);
      }
    } finally {
      const duration = Date.now() - startTime;
      this.updateTaskMetrics(task.type, duration);
    }
  }

  private async executeOptimizedWebSearch(task: TaskDefinition, agent: any): Promise<any> {
    this.logger.info('Executing optimized web search', { taskId: task.id.id });

    // Check cache first
    const cacheKey = this.generateCacheKey('web-search', task.description);
    const cached = this.getFromCache(cacheKey);
    if (cached) {
      this.metrics.cacheHits++;
      return cached;
    }

    // Execute parallel web searches with rate limiting
    const queries = this.generateSearchQueries(task.description);
    const searchPromises = queries.map(query => 
      this.executeRateLimitedSearch(query, agent)
    );

    const results = await Promise.allSettled(searchPromises);
    const successfulResults = results
      .filter(r => r.status === 'fulfilled')
      .map(r => (r as PromiseFulfilledResult<any>).value)
      .flat();

    // Rank and filter results by credibility
    const rankedResults = await this.rankResultsByCredibility(successfulResults);
    
    // Cache results
    this.setCache(cacheKey, rankedResults, 3600000); // 1 hour TTL
    this.metrics.cacheMisses++;

    return {
      results: rankedResults,
      totalFound: successfulResults.length,
      queriesExecuted: queries.length,
      credibilityScores: rankedResults.map(r => r.credibilityScore)
    };
  }

  private async executeOptimizedDataExtraction(task: TaskDefinition, agent: any): Promise<any> {
    this.logger.info('Executing optimized data extraction', { taskId: task.id.id });

    // Get connection from pool
    const connection = await this.getPooledConnection();
    
    try {
      // Parallel data extraction with deduplication
      const extractionPromises = this.createParallelExtractionTasks(task, agent);
      const extractedData = await Promise.all(extractionPromises);
      
      // Deduplicate results
      const deduplicatedData = this.deduplicateResults(extractedData.flat());
      
      return {
        extractedData: deduplicatedData,
        totalExtracted: extractedData.flat().length,
        uniqueResults: deduplicatedData.length,
        deduplicationRate: 1 - (deduplicatedData.length / extractedData.flat().length)
      };
    } finally {
      this.releasePooledConnection(connection);
    }
  }

  private async executeOptimizedClustering(task: TaskDefinition, agent: any): Promise<any> {
    this.logger.info('Executing optimized semantic clustering', { taskId: task.id.id });

    // Implement semantic clustering with caching
    const data = task.input?.extractedData || [];
    const cacheKey = this.generateCacheKey('clustering', JSON.stringify(data));
    
    const cached = this.getFromCache(cacheKey);
    if (cached) {
      return cached;
    }

    // Perform semantic clustering
    const clusters = await this.performSemanticClustering(data);
    
    // Cache clustering results
    this.setCache(cacheKey, clusters, 7200000); // 2 hours TTL

    return {
      clusters,
      clusterCount: clusters.length,
      averageClusterSize: clusters.reduce((sum, c) => sum + c.results.length, 0) / clusters.length,
      coherenceScore: clusters.reduce((sum, c) => sum + c.coherenceScore, 0) / clusters.length
    };
  }

  private async executeGenericResearchTask(task: TaskDefinition, agent: any): Promise<any> {
    this.logger.info('Executing generic research task', { taskId: task.id.id });

    // Apply general research optimizations
    return {
      status: 'completed',
      optimizations: ['caching', 'rate-limiting', 'connection-pooling'],
      executionTime: Date.now()
    };
  }

  // Helper methods for research optimizations

  private extractResearchParameters(description: string): any {
    return {
      domains: this.extractDomains(description),
      keywords: this.extractKeywords(description),
      timeframe: this.extractTimeframe(description),
      sourceTypes: this.extractSourceTypes(description)
    };
  }

  private extractDomains(description: string): string[] {
    // Extract relevant domains from description
    const domains = [];
    if (description.includes('academic') || description.includes('research')) domains.push('academic');
    if (description.includes('news') || description.includes('current')) domains.push('news');
    if (description.includes('technical') || description.includes('documentation')) domains.push('technical');
    return domains.length > 0 ? domains : ['general'];
  }

  private extractKeywords(description: string): string[] {
    // Simple keyword extraction - in production, use NLP
    return description
      .toLowerCase()
      .split(/\s+/)
      .filter(word => word.length > 3)
      .slice(0, 10);
  }

  private extractTimeframe(description: string): any {
    // Extract time-related constraints
    const now = new Date();
    return {
      start: new Date(now.getTime() - 365 * 24 * 60 * 60 * 1000), // 1 year ago
      end: now
    };
  }

  private extractSourceTypes(description: string): string[] {
    return ['academic', 'news', 'documentation', 'blog'];
  }

  private generateSearchQueries(description: string): ResearchQuery[] {
    const baseQuery = description.substring(0, 100);
    const keywords = this.extractKeywords(description);
    
    return [
      {
        id: generateId('query'),
        query: baseQuery,
        keywords: keywords.slice(0, 5),
        domains: ['general'],
        priority: 1,
        timestamp: new Date()
      },
      {
        id: generateId('query'),
        query: `${baseQuery} research study`,
        keywords: [...keywords.slice(0, 3), 'research', 'study'],
        domains: ['academic'],
        priority: 2,
        timestamp: new Date()
      },
      {
        id: generateId('query'),
        query: `${baseQuery} best practices`,
        keywords: [...keywords.slice(0, 3), 'best', 'practices'],
        domains: ['technical'],
        priority: 2,
        timestamp: new Date()
      }
    ];
  }

  private async executeRateLimitedSearch(query: ResearchQuery, agent: any): Promise<ResearchResult[]> {
    const domain = query.domains[0] || 'general';
    
    // Check rate limits
    if (!this.checkRateLimit(domain)) {
      await this.waitForRateLimit(domain);
    }

    // Simulate web search with retry logic
    let attempts = 0;
    const maxAttempts = 3;
    
    while (attempts < maxAttempts) {
      try {
        // Simulate search execution
        const results = await this.simulateWebSearch(query);
        this.updateRateLimit(domain);
        return results;
      } catch (error) {
        attempts++;
        if (attempts >= maxAttempts) throw error;
        await this.exponentialBackoff(attempts);
      }
    }
    
    return [];
  }

  private async simulateWebSearch(query: ResearchQuery): Promise<ResearchResult[]> {
    // Simulate web search results
    const resultCount = Math.floor(Math.random() * 10) + 5;
    const results: ResearchResult[] = [];
    
    for (let i = 0; i < resultCount; i++) {
      results.push({
        id: generateId('result'),
        queryId: query.id,
        url: `https://example.com/result-${i}`,
        title: `Research Result ${i} for ${query.query}`,
        content: `Content for ${query.query} - result ${i}`,
        summary: `Summary of result ${i}`,
        credibilityScore: Math.random() * 0.4 + 0.6, // 0.6-1.0
        relevanceScore: Math.random() * 0.3 + 0.7, // 0.7-1.0
        sourceType: query.domains[0] || 'general',
        extractedAt: new Date(),
        metadata: { queryKeywords: query.keywords }
      });
    }
    
    return results;
  }

  private async rankResultsByCredibility(results: ResearchResult[]): Promise<ResearchResult[]> {
    // Sort by combined credibility and relevance score
    return results.sort((a, b) => {
      const scoreA = (a.credibilityScore * 0.6) + (a.relevanceScore * 0.4);
      const scoreB = (b.credibilityScore * 0.6) + (b.relevanceScore * 0.4);
      return scoreB - scoreA;
    });
  }

  private createParallelExtractionTasks(task: TaskDefinition, agent: any): Promise<any>[] {
    // Create parallel extraction tasks
    const results = task.input?.results || [];
    const batchSize = Math.ceil(results.length / this.connectionPool.max);
    const batches = [];
    
    for (let i = 0; i < results.length; i += batchSize) {
      const batch = results.slice(i, i + batchSize);
      batches.push(this.extractDataFromBatch(batch));
    }
    
    return batches;
  }

  private async extractDataFromBatch(batch: ResearchResult[]): Promise<any[]> {
    // Simulate parallel data extraction
    return batch.map(result => ({
      id: result.id,
      extractedData: `Extracted data from ${result.title}`,
      insights: [`Insight 1 from ${result.title}`, `Insight 2 from ${result.title}`],
      metadata: result.metadata
    }));
  }

  private deduplicateResults(results: any[]): any[] {
    const seen = new Set();
    return results.filter(result => {
      const key = result.extractedData || result.id;
      if (seen.has(key)) return false;
      seen.add(key);
      return true;
    });
  }

  private async performSemanticClustering(data: any[]): Promise<ResearchCluster[]> {
    // Simulate semantic clustering
    const clusterCount = Math.min(Math.ceil(data.length / 5), 10);
    const clusters: ResearchCluster[] = [];
    
    for (let i = 0; i < clusterCount; i++) {
      const clusterData = data.slice(i * 5, (i + 1) * 5);
      clusters.push({
        id: generateId('cluster'),
        topic: `Research Topic ${i + 1}`,
        results: clusterData,
        centroid: Array(10).fill(0).map(() => Math.random()),
        coherenceScore: Math.random() * 0.3 + 0.7,
        keywords: [`keyword${i}1`, `keyword${i}2`],
        summary: `Summary of cluster ${i + 1}`
      });
    }
    
    return clusters;
  }

  // Connection pooling methods
  private async getPooledConnection(): Promise<any> {
    if (this.connectionPool.active >= this.connectionPool.max) {
      await this.waitForConnection();
    }
    
    this.connectionPool.active++;
    return { id: generateId('connection'), timestamp: new Date() };
  }

  private releasePooledConnection(connection: any): void {
    this.connectionPool.active--;
    this.connectionPool.idle++;
  }

  private async waitForConnection(): Promise<void> {
    return new Promise(resolve => {
      const checkConnection = () => {
        if (this.connectionPool.active < this.connectionPool.max) {
          resolve();
        } else {
          setTimeout(checkConnection, 100);
        }
      };
      checkConnection();
    });
  }

  // Rate limiting methods
  private checkRateLimit(domain: string): boolean {
    const limiter = this.rateLimiters.get(domain);
    if (!limiter) {
      this.rateLimiters.set(domain, {
        requests: 0,
        windowStart: new Date(),
        windowSize: 60000, // 1 minute
        maxRequests: 10,
        backoffMultiplier: 1
      });
      return true;
    }

    const now = new Date();
    if (now.getTime() - limiter.windowStart.getTime() > limiter.windowSize) {
      limiter.requests = 0;
      limiter.windowStart = now;
    }

    return limiter.requests < limiter.maxRequests;
  }

  private updateRateLimit(domain: string): void {
    const limiter = this.rateLimiters.get(domain);
    if (limiter) {
      limiter.requests++;
    }
  }

  private async waitForRateLimit(domain: string): Promise<void> {
    const limiter = this.rateLimiters.get(domain);
    if (!limiter) return;

    const waitTime = limiter.windowSize * limiter.backoffMultiplier;
    await new Promise(resolve => setTimeout(resolve, waitTime));
  }

  private async exponentialBackoff(attempt: number): Promise<void> {
    const delay = Math.pow(2, attempt) * 1000;
    await new Promise(resolve => setTimeout(resolve, delay));
  }

  // Caching methods
  private generateCacheKey(type: string, data: string): string {
    return `${type}:${Buffer.from(data).toString('base64').substring(0, 32)}`;
  }

  private getFromCache(key: string): any | null {
    const entry = this.researchCache.get(key);
    if (!entry) return null;

    const now = new Date();
    if (now.getTime() - entry.timestamp.getTime() > entry.ttl) {
      this.researchCache.delete(key);
      return null;
    }

    entry.accessCount++;
    entry.lastAccessed = now;
    return entry.data;
  }

  private setCache(key: string, data: any, ttl: number): void {
    this.researchCache.set(key, {
      key,
      data,
      timestamp: new Date(),
      ttl,
      accessCount: 0,
      lastAccessed: new Date()
    });

    // Cleanup old entries if cache is too large
    if (this.researchCache.size > 1000) {
      this.cleanupCache();
    }
  }

  private cleanupCache(): void {
    const entries = Array.from(this.researchCache.entries());
    entries.sort((a, b) => a[1].lastAccessed.getTime() - b[1].lastAccessed.getTime());
    
    // Remove oldest 20% of entries
    const toRemove = Math.floor(entries.length * 0.2);
    for (let i = 0; i < toRemove; i++) {
      this.researchCache.delete(entries[i][0]);
    }
  }

  private createResearchTask(
    id: string,
    type: TaskType,
    name: string,
    instructions: string,
    options: any = {}
  ): TaskDefinition {
    const taskId: TaskId = {
      id: generateId('task'),
      swarmId: 'research-swarm',
      sequence: 1,
      priority: 1
    };

    return {
      id: taskId,
      type,
      name,
      description: instructions,
      instructions,
      requirements: {
        capabilities: options.requiredCapabilities || ['research'],
        tools: ['WebFetchTool', 'WebSearch'],
        permissions: ['read', 'write']
      },
      constraints: {
        dependencies: options.dependencies || [],
        dependents: [],
        conflicts: [],
        maxRetries: 3,
        timeoutAfter: options.estimatedDuration || 300000
      },
      priority: options.priority || 'medium',
      input: options.researchParams || {},
      context: {},
      examples: [],
      status: 'created',
      createdAt: new Date(),
      updatedAt: new Date(),
      attempts: [],
      statusHistory: [{
        timestamp: new Date(),
        from: 'created',
        to: 'created',
        reason: 'Task created',
        triggeredBy: 'system'
      }]
    };
  }

  protected override updateMetrics(result: DecompositionResult, executionTime: number): void {
    this.metrics.tasksCompleted += result.tasks.length;
    this.metrics.averageExecutionTime = 
      (this.metrics.averageExecutionTime + executionTime) / 2;
    this.metrics.queriesExecuted++;
    this.metrics.averageResponseTime = 
      (this.metrics.averageResponseTime + executionTime) / 2;
  }

  private updateTaskMetrics(taskType: string, duration: number): void {
    this.metrics.queriesExecuted++;
    this.metrics.averageResponseTime = 
      (this.metrics.averageResponseTime + duration) / 2;
  }

  // Public API for metrics
  override getMetrics() {
    return {
      ...this.metrics,
      cacheHitRate: this.metrics.cacheHits / (this.metrics.cacheHits + this.metrics.cacheMisses),
      averageCredibilityScore: this.metrics.credibilityScores.length > 0 
        ? this.metrics.credibilityScores.reduce((a, b) => a + b, 0) / this.metrics.credibilityScores.length 
        : 0,
      connectionPoolUtilization: this.connectionPool.active / this.connectionPool.max,
      cacheSize: this.cache.size
    };
  }

  // Progressive refinement methods
  async refineResearchScope(objective: SwarmObjective, intermediateResults: any[]): Promise<SwarmObjective> {
    this.logger.info('Refining research scope based on intermediate results', {
      objectiveId: objective.id,
      resultsCount: intermediateResults.length
    });

    // Analyze intermediate results to refine scope
    const refinedObjective = { ...objective };
    
    // Update requirements based on findings
    if (intermediateResults.length > 0) {
      const avgCredibility = intermediateResults
        .map(r => r.credibilityScore || 0.5)
        .reduce((a, b) => a + b, 0) / intermediateResults.length;
      
      if (avgCredibility < 0.7) {
        refinedObjective.requirements.qualityThreshold = Math.max(
          refinedObjective.requirements.qualityThreshold,
          0.8
        );
      }
    }

    return refinedObjective;
  }

  // Implementation of abstract methods from BaseStrategy
  async selectAgentForTask(task: TaskDefinition, availableAgents: AgentState[]): Promise<string | null> {
    // Filter agents based on task requirements
    const suitableAgents = availableAgents.filter(agent => {
      // Check if agent has required capabilities
      const requiredCapabilities = task.requirements.capabilities || [];
      const hasRequiredCapabilities = requiredCapabilities.every((cap: string) => 
        agent.capabilities.tools.includes(cap) || 
        agent.capabilities.domains.includes(cap)
      );

      // Check if agent is available
      const isAvailable = agent.status === 'idle' || agent.status === 'busy';
      
      // Check if agent can handle the task type
      const canHandleTaskType = this.canAgentHandleTaskType(agent, task.type);

      return hasRequiredCapabilities && isAvailable && canHandleTaskType;
    });

    if (suitableAgents.length === 0) {
      return null;
    }

    // Score agents based on suitability for research tasks
    const scoredAgents = suitableAgents.map(agent => ({
      agent,
      score: this.scoreAgentForResearchTask(agent, task)
    }));

    // Sort by score descending
    scoredAgents.sort((a, b) => b.score - a.score);

    return scoredAgents[0].agent.id.id;
  }

  async optimizeTaskSchedule(tasks: TaskDefinition[], agents: AgentState[]): Promise<AgentAllocation[]> {
    const allocations: AgentAllocation[] = [];
    
    // Create priority mapping
    const priorityMap = {
      'critical': 5,
      'high': 4,
      'normal': 3,
      'low': 2,
      'background': 1
    };
    
    const tasksByPriority = [...tasks].sort((a, b) => 
      priorityMap[b.priority] - priorityMap[a.priority]
    );
    
    // Create initial allocations for each agent
    for (const agent of agents) {
      if (agent.status === 'idle' || agent.status === 'busy') {
        allocations.push({
          agentId: agent.id.id,
          tasks: [],
          estimatedWorkload: 0,
          capabilities: agent.capabilities.tools
        });
      }
    }

    // Assign tasks to agents using a greedy approach
    for (const task of tasksByPriority) {
      const suitableAllocations = allocations.filter(allocation => {
        const agent = agents.find(a => a.id.id === allocation.agentId);
        return agent && this.canAgentHandleTaskType(agent, task.type);
      });

      if (suitableAllocations.length > 0) {
        // Find allocation with lowest workload
        const bestAllocation = suitableAllocations.reduce((best, current) => 
          current.estimatedWorkload < best.estimatedWorkload ? current : best
        );

        bestAllocation.tasks.push(task.id.id);
        bestAllocation.estimatedWorkload += task.constraints.timeoutAfter || 300000; // 5 minutes default
      }
    }

    return allocations.filter(allocation => allocation.tasks.length > 0);
  }

  private canAgentHandleTaskType(agent: AgentState, taskType: TaskType): boolean {
    const taskTypeToCapabilities: Record<TaskType, string[]> = {
      'research': ['research', 'web-search'],
      'web-search': ['web-search', 'research'],
      'data-processing': ['analysis', 'data-processing'],
      'analysis': ['analysis', 'research'],
      'research-planning': ['research', 'analysis'],
      'documentation': ['documentation', 'analysis'],
      'coding': ['coding', 'development'],
      'testing': ['testing', 'quality-assurance'],
      'review': ['review', 'analysis'],
      'deployment': ['deployment', 'operations'],
      'monitoring': ['monitoring', 'operations'],
      'coordination': ['coordination', 'management'],
      'communication': ['communication', 'coordination'],
      'maintenance': ['maintenance', 'operations'],
      'optimization': ['optimization', 'analysis'],
      'validation': ['validation', 'testing'],
      'integration': ['integration', 'development'],
      'custom': ['general']
    };

    const requiredCapabilities = taskTypeToCapabilities[taskType] || [];
    return requiredCapabilities.some((cap: string) => 
      agent.capabilities.tools.includes(cap) || 
      agent.capabilities.domains.includes(cap)
    );
  }

  private scoreAgentForResearchTask(agent: AgentState, task: TaskDefinition): number {
    let score = 0;

    // Base score from agent capabilities
    if (agent.capabilities.research) score += 30;
    if (agent.capabilities.webSearch) score += 20;
    if (agent.capabilities.analysis) score += 15;

    // Task-specific scoring
    switch (task.type) {
      case 'research':
      case 'research-planning':
        score += agent.capabilities.research ? 25 : 0;
        break;
      case 'web-search':
        score += agent.capabilities.webSearch ? 25 : 0;
        break;
      case 'data-processing':
      case 'analysis':
        score += agent.capabilities.analysis ? 25 : 0;
        break;
    }

    // Performance factors
    score += agent.capabilities.reliability * 20;
    score += agent.capabilities.quality * 15;
    score += agent.capabilities.speed * 10;

    // Workload penalty
    const currentWorkload = agent.metrics.tasksCompleted || 0;
    const maxConcurrent = agent.capabilities.maxConcurrentTasks || 1;
    if (currentWorkload >= maxConcurrent) {
      score -= 50;
    }

    return Math.max(0, score);
  }
}