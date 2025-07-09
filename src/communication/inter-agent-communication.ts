/**
 * Inter-Agent Communication System
 * Enables structured communication between agents in hierarchical organizations
 */

import { EventEmitter } from 'node:events';
import { AgentId, TaskDefinition } from '../swarm/types.js';
import { DistributedMemorySystem } from '../memory/distributed-memory.js';
import { generateId } from '../utils/helpers.js';
import { QuDAGCommunicationSystem, QuDAGConfig } from './qudag-integration.js';

export interface CommunicationChannel {
  id: string;
  name: string;
  type: 'direct' | 'broadcast' | 'hierarchical' | 'peer-to-peer' | 'multicast';
  participants: AgentId[];
  permissions: ChannelPermissions;
  messageHistory: AgentMessage[];
  encryption: boolean;
  retention: RetentionPolicy;
  moderators: AgentId[];
  metadata: Record<string, any>;
}

export interface ChannelPermissions {
  canSend: AgentId[];
  canReceive: AgentId[];
  canModerate: AgentId[];
  canInvite: AgentId[];
  isPublic: boolean;
  requiresApproval: boolean;
}

export interface RetentionPolicy {
  duration: number; // milliseconds
  maxMessages: number;
  archiveAfter: number;
  deleteAfter: number;
  compressionEnabled: boolean;
}

export interface AgentMessage {
  id: string;
  channel: string;
  from: AgentId;
  to: AgentId | AgentId[]; // Can be single agent or multiple
  type: MessageType;
  content: MessageContent;
  metadata: MessageMetadata;
  timestamp: Date;
  expiresAt?: Date;
  priority: MessagePriority;
  requiresResponse: boolean;
  parentMessageId?: string;
  threadId?: string;
  attachments?: MessageAttachment[];
}

export type MessageType = 
  | 'request'           // Request for action or information
  | 'response'          // Response to a request
  | 'notification'      // General notification
  | 'delegation'        // Task delegation
  | 'report'           // Status or progress report
  | 'escalation'       // Issue escalation
  | 'coordination'     // Coordination message
  | 'broadcast'        // General broadcast
  | 'system'           // System-generated message
  | 'emergency'        // Emergency/urgent message
  | 'qudag-message';   // QuDAG protocol message

export interface MessageContent {
  subject: string;
  body: string;
  data?: any;
  format: 'text' | 'json' | 'xml' | 'markdown' | 'structured';
  language?: string;
  encoding?: string;
}

export interface MessageMetadata {
  correlationId?: string;
  conversationId?: string;
  requestId?: string;
  tags: string[];
  departmentScope?: string;
  securityLevel: 'public' | 'internal' | 'confidential' | 'secret';
  origin: 'human' | 'agent' | 'system';
  context?: CommunicationContext;
}

export interface CommunicationContext {
  taskId?: string;
  projectId?: string;
  workflowId?: string;
  hierarchyLevel: number;
  organizationalUnit: string;
  urgency: 'low' | 'normal' | 'high' | 'critical';
}

export type MessagePriority = 1 | 2 | 3 | 4 | 5; // 1 = highest, 5 = lowest

export interface MessageAttachment {
  id: string;
  filename: string;
  contentType: string;
  size: number;
  data: Buffer | string;
  checksum: string;
}

export interface CommunicationProtocol {
  name: string;
  version: string;
  features: ProtocolFeature[];
  messageFormat: string;
  encryption: EncryptionConfig;
  routing: RoutingConfig;
  reliability: ReliabilityConfig;
}

export interface ProtocolFeature {
  name: string;
  enabled: boolean;
  configuration: Record<string, any>;
}

export interface EncryptionConfig {
  enabled: boolean;
  algorithm: string;
  keyLength: number;
  keyRotationInterval: number;
}

export interface RoutingConfig {
  strategy: 'direct' | 'hierarchical' | 'mesh' | 'hub-spoke';
  maxHops: number;
  loadBalancing: boolean;
  redundancy: boolean;
}

export interface ReliabilityConfig {
  acknowledgments: boolean;
  retryAttempts: number;
  retryDelay: number;
  duplicateDetection: boolean;
  messageOrdering: boolean;
}

export interface MessageRoute {
  id: string;
  from: AgentId;
  to: AgentId;
  path: AgentId[];
  cost: number;
  latency: number;
  reliability: number;
  lastUpdated: Date;
}

export interface CommunicationMetrics {
  totalMessages: number;
  messagesByType: Map<MessageType, number>;
  messagesByChannel: Map<string, number>;
  averageLatency: number;
  deliveryRate: number;
  errorRate: number;
  throughput: number;
  activeChannels: number;
  activeAgents: number;
  networkLoad: number;
}

export class InterAgentCommunicationSystem extends EventEmitter {
  private channels = new Map<string, CommunicationChannel>();
  private messages = new Map<string, AgentMessage>();
  private routes = new Map<string, MessageRoute>();
  private protocols = new Map<string, CommunicationProtocol>();
  private memory: DistributedMemorySystem;
  private messageQueue = new Map<string, AgentMessage[]>(); // agentId -> pending messages
  private subscriptions = new Map<string, Set<string>>(); // agentId -> channelIds
  private metrics: CommunicationMetrics;
  private quadgSystem?: QuDAGCommunicationSystem;

  constructor(memory: DistributedMemorySystem, quadgConfig?: QuDAGConfig) {
    super();
    this.memory = memory;
    this.metrics = this.initializeMetrics();
    this.setupDefaultProtocols();
    this.startMessageProcessing();
    
    // Initialize QuDAG integration if configured
    if (quadgConfig) {
      this.initializeQuDAGIntegration(quadgConfig);
    }
  }

  /**
   * Create a communication channel
   */
  async createChannel(
    name: string,
    type: CommunicationChannel['type'],
    creator: AgentId,
    participants: AgentId[] = [],
    permissions?: Partial<ChannelPermissions>
  ): Promise<CommunicationChannel> {
    const channelId = generateId('channel');
    
    const channel: CommunicationChannel = {
      id: channelId,
      name,
      type,
      participants: [creator, ...participants],
      permissions: {
        canSend: [creator, ...participants],
        canReceive: [creator, ...participants],
        canModerate: [creator],
        canInvite: [creator],
        isPublic: false,
        requiresApproval: false,
        ...permissions
      },
      messageHistory: [],
      encryption: true,
      retention: {
        duration: 7 * 24 * 60 * 60 * 1000, // 7 days
        maxMessages: 10000,
        archiveAfter: 30 * 24 * 60 * 60 * 1000, // 30 days
        deleteAfter: 90 * 24 * 60 * 60 * 1000, // 90 days
        compressionEnabled: true
      },
      moderators: [creator],
      metadata: {
        createdAt: new Date(),
        createdBy: creator.id
      }
    };

    this.channels.set(channelId, channel);

    // Subscribe participants to channel
    for (const participant of channel.participants) {
      await this.subscribeAgentToChannel(participant.id, channelId);
    }

    // Store channel in memory
    await this.memory.store(`channel:${channelId}`, channel, {
      type: 'communication',
      tags: ['channel', type],
      partition: 'communication'
    });

    this.emit('channel:created', { channel, creator });

    return channel;
  }

  /**
   * Send a message between agents
   */
  async sendMessage(
    from: AgentId,
    to: AgentId | AgentId[],
    type: MessageType,
    content: MessageContent,
    options: {
      channelId?: string;
      priority?: MessagePriority;
      requiresResponse?: boolean;
      expiresAt?: Date;
      context?: CommunicationContext;
      attachments?: MessageAttachment[];
      useQuDAG?: boolean;
      anonymityLevel?: 'none' | 'low' | 'medium' | 'high';
      quantumResistant?: boolean;
    } = {}
  ): Promise<AgentMessage> {
    const messageId = generateId('message');
    
    const message: AgentMessage = {
      id: messageId,
      channel: options.channelId || 'direct',
      from,
      to,
      type,
      content,
      metadata: {
        tags: [],
        securityLevel: 'internal',
        origin: 'agent',
        context: options.context,
        ...this.generateMessageMetadata(type, content, options.context)
      },
      timestamp: new Date(),
      expiresAt: options.expiresAt,
      priority: options.priority || 3,
      requiresResponse: options.requiresResponse || false,
      attachments: options.attachments
    };

    // Validate message
    await this.validateMessage(message);

    // Use QuDAG for enhanced security and anonymity if available and requested
    if (this.quadgSystem && (options.useQuDAG || options.anonymityLevel || options.quantumResistant)) {
      const recipients = Array.isArray(to) ? to : [to];
      for (const recipient of recipients) {
        await this.quadgSystem.sendMessage(from, recipient, content, {
          anonymityLevel: options.anonymityLevel,
          encryption: options.quantumResistant || true,
          ttl: options.expiresAt ? Math.floor((options.expiresAt.getTime() - Date.now()) / 1000) : undefined
        });
      }
    } else {
      // Route message through traditional system
      await this.routeMessage(message);
    }

    // Store message
    this.messages.set(messageId, message);
    await this.storeMessage(message);

    // Update metrics
    this.updateMetrics(message);

    this.emit('message:sent', message);

    return message;
  }

  /**
   * Broadcast a message to multiple agents or a channel
   */
  async broadcastMessage(
    from: AgentId,
    channelId: string,
    content: MessageContent,
    options: {
      priority?: MessagePriority;
      context?: CommunicationContext;
      excludeAgents?: AgentId[];
    } = {}
  ): Promise<AgentMessage> {
    const channel = this.channels.get(channelId);
    if (!channel) {
      throw new Error(`Channel ${channelId} not found`);
    }

    // Check broadcast permissions
    if (!channel.permissions.canSend.some(agent => agent.id === from.id)) {
      throw new Error(`Agent ${from.id} does not have send permissions for channel ${channelId}`);
    }

    const recipients = channel.participants.filter(agent => 
      !options.excludeAgents?.some(excluded => excluded.id === agent.id)
    );

    return await this.sendMessage(from, recipients, 'broadcast', content, {
      channelId,
      priority: options.priority,
      context: options.context
    });
  }

  /**
   * Subscribe an agent to a channel
   */
  async subscribeAgentToChannel(agentId: string, channelId: string): Promise<void> {
    if (!this.subscriptions.has(agentId)) {
      this.subscriptions.set(agentId, new Set());
    }
    
    this.subscriptions.get(agentId)!.add(channelId);
    
    // Add to message queue if not exists
    if (!this.messageQueue.has(agentId)) {
      this.messageQueue.set(agentId, []);
    }

    this.emit('agent:subscribed', { agentId, channelId });
  }

  /**
   * Get pending messages for an agent
   */
  async getAgentMessages(
    agentId: string,
    filters: {
      channelId?: string;
      type?: MessageType;
      unreadOnly?: boolean;
      since?: Date;
      limit?: number;
    } = {}
  ): Promise<AgentMessage[]> {
    const agentQueue = this.messageQueue.get(agentId) || [];
    
    let filteredMessages = agentQueue;

    if (filters.channelId) {
      filteredMessages = filteredMessages.filter(msg => msg.channel === filters.channelId);
    }

    if (filters.type) {
      filteredMessages = filteredMessages.filter(msg => msg.type === filters.type);
    }

    if (filters.since) {
      filteredMessages = filteredMessages.filter(msg => msg.timestamp >= filters.since!);
    }

    if (filters.limit) {
      filteredMessages = filteredMessages.slice(0, filters.limit);
    }

    return filteredMessages;
  }

  /**
   * Create a hierarchical communication structure
   */
  async createHierarchicalCommunication(
    hierarchyId: string,
    rootAgent: AgentId,
    structure: HierarchyNode[]
  ): Promise<Map<string, string>> {
    const channelMap = new Map<string, string>(); // agentId -> channelId

    // Create channels for each level and relationship
    for (const node of structure) {
      // Create upward communication channel (to parent)
      if (node.parent) {
        const upwardChannelId = await this.createChannel(
          `upward-${node.agent.id.id}`,
          'hierarchical',
          node.agent.id,
          [node.parent],
          {
            canSend: [node.agent.id],
            canReceive: [node.parent],
            isPublic: false
          }
        );
        channelMap.set(`${node.agent.id.id}-upward`, upwardChannelId.id);
      }

      // Create downward communication channel (to children)
      if (node.children.length > 0) {
        const downwardChannelId = await this.createChannel(
          `downward-${node.agent.id.id}`,
          'hierarchical',
          node.agent.id,
          node.children,
          {
            canSend: [node.agent.id],
            canReceive: node.children,
            isPublic: false
          }
        );
        channelMap.set(`${node.agent.id.id}-downward`, downwardChannelId.id);
      }

      // Create peer communication channel (with siblings)
      if (node.siblings.length > 0) {
        const peerChannelId = await this.createChannel(
          `peer-${node.agent.id.id}`,
          'peer-to-peer',
          node.agent.id,
          node.siblings,
          {
            canSend: [node.agent.id, ...node.siblings],
            canReceive: [node.agent.id, ...node.siblings],
            isPublic: false
          }
        );
        channelMap.set(`${node.agent.id.id}-peer`, peerChannelId.id);
      }
    }

    // Store hierarchy communication structure
    await this.memory.store(`hierarchy:${hierarchyId}:communication`, {
      hierarchyId,
      channelMap: Object.fromEntries(channelMap),
      structure: structure.map(node => ({
        agentId: node.agent.id.id,
        level: node.level,
        role: node.role
      }))
    }, {
      type: 'communication',
      tags: ['hierarchy', 'structure'],
      partition: 'communication'
    });

    return channelMap;
  }

  /**
   * Enable delegation communication patterns
   */
  async setupDelegationCommunication(
    delegator: AgentId,
    delegate: AgentId,
    taskId: string
  ): Promise<DelegationCommunicationContext> {
    // Create dedicated delegation channel
    const delegationChannel = await this.createChannel(
      `delegation-${taskId}`,
      'direct',
      delegator,
      [delegate],
      {
        canSend: [delegator, delegate],
        canReceive: [delegator, delegate],
        isPublic: false,
        requiresApproval: false
      }
    );

    // Setup automated reporting
    const reportingInterval = 30 * 60 * 1000; // 30 minutes
    const reportingTimer = setInterval(async () => {
      await this.sendMessage(delegate, delegator, 'report', {
        subject: `Progress Report: Task ${taskId}`,
        body: 'Automated progress report',
        data: { taskId, timestamp: new Date() },
        format: 'structured'
      }, {
        channelId: delegationChannel.id,
        priority: 2,
        context: {
          taskId,
          hierarchyLevel: 1,
          organizationalUnit: 'delegation',
          urgency: 'normal'
        }
      });
    }, reportingInterval);

    const context: DelegationCommunicationContext = {
      taskId,
      delegator,
      delegate,
      channelId: delegationChannel.id,
      reportingInterval,
      reportingTimer,
      escalationPath: [delegator],
      communicationRules: {
        mustReportEvery: reportingInterval,
        canEscalate: true,
        requiresApproval: false,
        allowDirectCommunication: true
      }
    };

    return context;
  }

  /**
   * Get communication metrics
   */
  getMetrics(): CommunicationMetrics {
    return { ...this.metrics };
  }

  /**
   * Initialize QuDAG integration for quantum-resistant communication
   */
  private async initializeQuDAGIntegration(config: QuDAGConfig): Promise<void> {
    try {
      this.quadgSystem = new QuDAGCommunicationSystem(config, this, this.memory);
      
      // Setup event forwarding from QuDAG to traditional system
      this.quadgSystem.on('message-received', async (data: any) => {
        await this.handleQuDAGMessage(data);
      });

      this.quadgSystem.on('swarm-created', (data: any) => {
        this.emit('qudag:swarm-created', data);
      });

      console.log('QuDAG integration initialized successfully');
    } catch (error) {
      console.error('Failed to initialize QuDAG integration:', error);
    }
  }

  /**
   * Handle messages received from QuDAG network
   */
  async handleQuDAGMessage(data: { message: any; content: any }): Promise<void> {
    const { message, content } = data;
    
    // Convert QuDAG message to internal format
    const internalMessage: AgentMessage = {
      id: message.id,
      channel: 'qudag',
      from: message.from,
      to: message.to,
      type: content.type || 'notification',
      content: {
        subject: content.subject || 'QuDAG Message',
        body: content.body || JSON.stringify(content),
        data: content,
        format: 'structured'
      },
      metadata: {
        tags: ['qudag', 'quantum-resistant'],
        securityLevel: 'confidential',
        origin: 'agent'
      },
      timestamp: message.payload.timestamp,
      priority: 2,
      requiresResponse: false
    };

    // Process through traditional system
    this.messages.set(internalMessage.id, internalMessage);
    await this.storeMessage(internalMessage);
    this.updateMetrics(internalMessage);

    this.emit('message:received', { message: internalMessage, recipient: message.to });
  }

  /**
   * Create a swarm coordination using QuDAG
   */
  async createQuDAGSwarm(
    swarmId: string,
    coordinationType: 'hierarchical' | 'mesh' | 'hybrid',
    agents: Array<{
      agentId: string;
      capabilities: string[];
      resources: any;
      communicationPreferences: any;
    }>,
    options?: {
      consensusProtocol?: 'dag-consensus' | 'leader-election' | 'voting';
      taskDistributionAlgorithm?: 'round-robin' | 'load-based' | 'capability-based';
      resourceSharingEnabled?: boolean;
    }
  ): Promise<any> {
    if (!this.quadgSystem) {
      throw new Error('QuDAG system not initialized');
    }

    const agentInfos = agents.map(agent => ({
      agentId: agent.agentId,
      nodeId: `node-${agent.agentId}`,
      capabilities: agent.capabilities,
      workload: 0,
      reputation: 1.0,
      resources: {
        cpu: 2,
        memory: 1024,
        storage: 100,
        specializedTools: agent.capabilities,
        ...agent.resources
      },
      communicationPreferences: {
        anonymityLevel: 'medium' as const,
        routingStrategy: 'shortest' as const,
        encryptionRequired: true,
        ...agent.communicationPreferences
      }
    }));

    return await this.quadgSystem.createSwarmCoordination(
      swarmId,
      coordinationType,
      agentInfos,
      options
    );
  }

  /**
   * Get QuDAG network status and metrics
   */
  getQuDAGStatus(): any {
    if (!this.quadgSystem) {
      return { enabled: false, status: 'not initialized' };
    }

    return {
      enabled: true,
      status: 'active',
      metrics: this.quadgSystem.getNetworkMetrics()
    };
  }

  /**
   * Enable quantum-resistant communication for a channel
   */
  async enableQuantumResistantCommunication(channelId: string): Promise<void> {
    const channel = this.channels.get(channelId);
    if (!channel) {
      throw new Error(`Channel ${channelId} not found`);
    }

    if (!this.quadgSystem) {
      throw new Error('QuDAG system not initialized - quantum-resistant communication unavailable');
    }

    // Mark channel as quantum-resistant
    channel.metadata.quantumResistant = true;
    channel.metadata.communicationProtocol = 'QuDAG';
    
    await this.memory.store(`channel:${channelId}`, channel, {
      type: 'communication',
      tags: ['channel', channel.type, 'quantum-resistant'],
      partition: 'communication'
    });

    this.emit('channel:quantum-enabled', { channelId, channel });
  }

  /**
   * Handle messages from external systems (MCP compatibility)
   */
  async handleMessage(
    from: AgentId,
    to: AgentId,
    type: string,
    content: any
  ): Promise<void> {
    // Convert external message to internal format
    const message = await this.sendMessage(from, to, type as MessageType, {
      subject: content.subject || `External ${type}`,
      body: content.body || JSON.stringify(content),
      data: content,
      format: 'structured'
    }, {
      useQuDAG: content.useQuDAG || false,
      anonymityLevel: content.anonymityLevel,
      quantumResistant: content.quantumResistant || false
    });

    this.emit('external:message-handled', { message, originalContent: content });
  }

  // Private helper methods

  private async validateMessage(message: AgentMessage): Promise<void> {
    // Validate message structure and permissions
    if (!message.content.subject || !message.content.body) {
      throw new Error('Message must have subject and body');
    }

    // Check channel permissions if specified
    if (message.channel !== 'direct') {
      const channel = this.channels.get(message.channel);
      if (channel && !channel.permissions.canSend.some(agent => agent.id === message.from.id)) {
        throw new Error(`Agent ${message.from.id} does not have send permissions for channel ${message.channel}`);
      }
    }
  }

  private async routeMessage(message: AgentMessage): Promise<void> {
    const recipients = Array.isArray(message.to) ? message.to : [message.to];
    
    for (const recipient of recipients) {
      const route = await this.findOptimalRoute(message.from, recipient);
      
      // Add message to recipient's queue
      if (!this.messageQueue.has(recipient.id)) {
        this.messageQueue.set(recipient.id, []);
      }
      
      this.messageQueue.get(recipient.id)!.push(message);
      
      // Notify recipient
      this.emit('message:received', { message, recipient });
    }
  }

  private async storeMessage(message: AgentMessage): Promise<void> {
    await this.memory.store(`message:${message.id}`, message, {
      type: 'communication',
      tags: ['message', message.type],
      partition: 'messages'
    });

    // Add to channel history if applicable
    if (message.channel !== 'direct') {
      const channel = this.channels.get(message.channel);
      if (channel) {
        channel.messageHistory.push(message);
        
        // Enforce retention policy
        if (channel.messageHistory.length > channel.retention.maxMessages) {
          channel.messageHistory = channel.messageHistory.slice(-channel.retention.maxMessages);
        }
      }
    }
  }

  private updateMetrics(message: AgentMessage): void {
    this.metrics.totalMessages++;
    
    const typeCount = this.metrics.messagesByType.get(message.type) || 0;
    this.metrics.messagesByType.set(message.type, typeCount + 1);
    
    const channelCount = this.metrics.messagesByChannel.get(message.channel) || 0;
    this.metrics.messagesByChannel.set(message.channel, channelCount + 1);
  }

  private generateMessageMetadata(
    type: MessageType,
    content: MessageContent,
    context?: CommunicationContext
  ): Partial<MessageMetadata> {
    return {
      tags: [type, content.format],
      context,
      correlationId: context?.taskId ? `task-${context.taskId}` : generateId('corr')
    };
  }

  private async findOptimalRoute(from: AgentId, to: AgentId): Promise<MessageRoute> {
    // Simplified routing - in practice would implement graph algorithms
    return {
      id: generateId('route'),
      from,
      to,
      path: [from, to],
      cost: 1,
      latency: 100,
      reliability: 0.99,
      lastUpdated: new Date()
    };
  }

  private initializeMetrics(): CommunicationMetrics {
    return {
      totalMessages: 0,
      messagesByType: new Map(),
      messagesByChannel: new Map(),
      averageLatency: 0,
      deliveryRate: 0.99,
      errorRate: 0.01,
      throughput: 0,
      activeChannels: 0,
      activeAgents: 0,
      networkLoad: 0
    };
  }

  private setupDefaultProtocols(): void {
    const defaultProtocol: CommunicationProtocol = {
      name: 'AgentComm',
      version: '1.0.0',
      features: [
        { name: 'encryption', enabled: true, configuration: {} },
        { name: 'compression', enabled: true, configuration: {} },
        { name: 'acknowledgments', enabled: true, configuration: {} }
      ],
      messageFormat: 'json',
      encryption: {
        enabled: true,
        algorithm: 'AES-256-GCM',
        keyLength: 256,
        keyRotationInterval: 24 * 60 * 60 * 1000 // 24 hours
      },
      routing: {
        strategy: 'hierarchical',
        maxHops: 10,
        loadBalancing: true,
        redundancy: false
      },
      reliability: {
        acknowledgments: true,
        retryAttempts: 3,
        retryDelay: 1000,
        duplicateDetection: true,
        messageOrdering: true
      }
    };

    this.protocols.set('default', defaultProtocol);
  }

  private startMessageProcessing(): void {
    // Start background message processing
    setInterval(() => {
      this.processMessageQueues();
      this.cleanupExpiredMessages();
      this.updateNetworkMetrics();
    }, 5000); // Every 5 seconds
  }

  private processMessageQueues(): void {
    // Process pending messages in queues
    for (const [agentId, messages] of this.messageQueue) {
      // Process priority messages first
      messages.sort((a, b) => a.priority - b.priority);
    }
  }

  private cleanupExpiredMessages(): void {
    const now = new Date();
    
    // Clean up expired messages from queues
    for (const [agentId, messages] of this.messageQueue) {
      const validMessages = messages.filter(msg => 
        !msg.expiresAt || msg.expiresAt > now
      );
      this.messageQueue.set(agentId, validMessages);
    }
  }

  private updateNetworkMetrics(): void {
    this.metrics.activeChannels = this.channels.size;
    this.metrics.activeAgents = this.messageQueue.size;
    // Additional metric calculations would go here
  }
}

// Additional interfaces for delegation communication
export interface DelegationCommunicationContext {
  taskId: string;
  delegator: AgentId;
  delegate: AgentId;
  channelId: string;
  reportingInterval: number;
  reportingTimer: NodeJS.Timer;
  escalationPath: AgentId[];
  communicationRules: DelegationCommunicationRules;
}

export interface DelegationCommunicationRules {
  mustReportEvery: number;
  canEscalate: boolean;
  requiresApproval: boolean;
  allowDirectCommunication: boolean;
}

// Import hierarchy node structure
interface HierarchyNode {
  agent: { id: AgentId };
  parent?: AgentId;
  children: AgentId[];
  siblings: AgentId[];
  level: number;
  role: any;
}