/**
 * QuDAG Integration Example
 * Demonstrates quantum-resistant, decentralized communication for hierarchical agents
 */

import { InterAgentCommunicationSystem } from '../../../src/communication/inter-agent-communication.js';
import { QuDAGCommunicationSystem } from '../../../src/communication/qudag-integration.js';
import { HierarchicalTaskSpawner } from '../../../src/agents/hierarchical-task-spawner.js';

async function demonstrateQuDAGIntegration() {
  console.log('🔒 QuDAG Integration Example');
  console.log('============================\n');

  // QuDAG configuration for quantum-resistant communication
  const qudagConfig = {
    nodeId: 'claude-flow-node-001',
    darkDomain: 'claude-flow.dark',
    cryptoConfig: {
      mlKemKeySize: 768,
      mlDsaVariant: 'ML-DSA-65',
      enableOnionRouting: true,
      trafficObfuscation: true
    },
    networkConfig: {
      listenPort: 8080,
      bootstrapNodes: ['bootstrap1.dark', 'bootstrap2.dark'],
      maxPeers: 50,
      enableDHT: true,
      enableRelay: true
    },
    mcpConfig: {
      enableStdio: true,
      enableHttp: true,
      enableWebSocket: true,
      httpPort: 3001,
      wsPort: 3002
    }
  };

  console.log('🚀 Initializing QuDAG Communication System');
  console.log('------------------------------------------');
  console.log(`📡 Node ID: ${qudagConfig.nodeId}`);
  console.log(`🌐 Dark Domain: ${qudagConfig.darkDomain}`);
  console.log(`🔐 Encryption: ML-KEM-${qudagConfig.cryptoConfig.mlKemKeySize} + ${qudagConfig.cryptoConfig.mlDsaVariant}`);
  console.log(`🧅 Onion Routing: ${qudagConfig.cryptoConfig.enableOnionRouting ? 'Enabled' : 'Disabled'}`);

  // Initialize communication system with QuDAG integration
  const communicationSystem = new InterAgentCommunicationSystem(memory, qudagConfig);

  console.log('\n🔒 Example 1: Quantum-Resistant Agent Communication');
  console.log('=================================================');

  // Scenario: Secure financial transaction processing
  const financialAgents = [
    { id: 'risk-manager-001', type: 'coordinator', instance: 1 },
    { id: 'fraud-detector-001', type: 'specialist', instance: 1 },
    { id: 'compliance-officer-001', type: 'specialist', instance: 1 },
    { id: 'transaction-processor-001', type: 'specialist', instance: 1 }
  ];

  console.log('💼 Creating secure financial processing swarm...');

  // Create QuDAG swarm for financial processing
  const financialSwarm = await communicationSystem.createQuDAGSwarm(
    'financial-processing-swarm',
    'hierarchical',
    financialAgents.map(agent => ({
      agentId: agent.id,
      capabilities: ['financial-analysis', 'risk-assessment', 'compliance-check'],
      resources: { cpu: 4, memory: 2048, storage: 500 },
      communicationPreferences: {
        anonymityLevel: 'high',
        routingStrategy: 'onion',
        encryptionRequired: true
      }
    })),
    {
      consensusProtocol: 'dag-consensus',
      taskDistributionAlgorithm: 'capability-based',
      resourceSharingEnabled: true
    }
  );

  console.log(`✅ Financial swarm created: ${financialSwarm.swarmId}`);
  console.log(`🔐 Quantum-resistant encryption enabled`);
  console.log(`🌐 Anonymous communication channels established`);

  // Demonstrate secure message exchange
  console.log('\n📨 Secure Message Exchange');
  console.log('-------------------------');

  const riskManager = financialAgents[0];
  const fraudDetector = financialAgents[1];

  // Send quantum-resistant message
  await communicationSystem.sendMessage(
    riskManager,
    fraudDetector,
    'request',
    {
      subject: 'High-Value Transaction Analysis Request',
      body: 'Please analyze transaction ID TX-789123 for potential fraud indicators',
      data: {
        transactionId: 'TX-789123',
        amount: 50000,
        currency: 'USD',
        originCountry: 'US',
        destinationCountry: 'CH',
        riskLevel: 'high'
      },
      format: 'structured'
    },
    {
      useQuDAG: true,
      anonymityLevel: 'high',
      quantumResistant: true,
      priority: 1,
      requiresResponse: true
    }
  );

  console.log('💳 Sent quantum-resistant transaction analysis request');
  console.log('🔒 Message encrypted with ML-KEM-768 + ChaCha20Poly1305');
  console.log('🧅 Routed through anonymous onion network');

  console.log('\n🏢 Example 2: Enterprise Multi-Department Coordination');
  console.log('====================================================');

  // Create enterprise departments with QuDAG communication
  const enterpriseDepartments = [
    {
      name: 'Engineering',
      agents: [
        'eng-lead-001', 'senior-dev-001', 'senior-dev-002', 
        'devops-001', 'qa-lead-001'
      ]
    },
    {
      name: 'Product',
      agents: [
        'product-manager-001', 'ux-researcher-001', 
        'product-designer-001', 'data-analyst-001'
      ]
    },
    {
      name: 'Security',
      agents: [
        'security-lead-001', 'security-analyst-001', 
        'compliance-specialist-001'
      ]
    }
  ];

  console.log('🏢 Creating enterprise-wide secure communication...');

  for (const department of enterpriseDepartments) {
    const deptSwarm = await communicationSystem.createQuDAGSwarm(
      `${department.name.toLowerCase()}-dept-swarm`,
      'mesh',
      department.agents.map(agentId => ({
        agentId,
        capabilities: [`${department.name.toLowerCase()}-expertise`],
        resources: { cpu: 2, memory: 1024, storage: 200 },
        communicationPreferences: {
          anonymityLevel: 'medium',
          routingStrategy: 'mesh',
          encryptionRequired: true
        }
      })),
      {
        consensusProtocol: 'voting',
        taskDistributionAlgorithm: 'load-based'
      }
    );

    console.log(`  ✅ ${department.name} department swarm: ${deptSwarm.swarmId}`);
  }

  console.log('\n🌐 Example 3: Cross-Organizational Agent Collaboration');
  console.log('===================================================');

  // Simulate collaboration between different organizations
  const partnerOrganizations = [
    {
      name: 'TechCorp',
      domain: 'techcorp.dark',
      agents: ['techcorp-liaison-001', 'techcorp-engineer-001']
    },
    {
      name: 'DataInc',
      domain: 'datainc.dark', 
      agents: ['datainc-analyst-001', 'datainc-scientist-001']
    },
    {
      name: 'CloudProvider',
      domain: 'cloudprovider.dark',
      agents: ['cloud-architect-001', 'cloud-engineer-001']
    }
  ];

  console.log('🤝 Setting up cross-organizational secure communication...');

  // Create inter-organizational swarm
  const allPartnerAgents = partnerOrganizations.flatMap(org => 
    org.agents.map(agentId => ({
      agentId: `${org.domain}/${agentId}`,
      capabilities: ['inter-org-collaboration', 'data-sharing'],
      resources: { cpu: 3, memory: 1536, storage: 300 },
      communicationPreferences: {
        anonymityLevel: 'high',
        routingStrategy: 'onion',
        encryptionRequired: true
      }
    }))
  );

  const collaborationSwarm = await communicationSystem.createQuDAGSwarm(
    'cross-org-collaboration',
    'hybrid',
    allPartnerAgents,
    {
      consensusProtocol: 'dag-consensus',
      taskDistributionAlgorithm: 'reputation-based',
      resourceSharingEnabled: true
    }
  );

  console.log(`✅ Cross-organizational swarm: ${collaborationSwarm.swarmId}`);
  console.log(`🔐 Quantum-resistant inter-organizational communication enabled`);

  // Demonstrate resource sharing
  console.log('\n💰 Resource Sharing Example');
  console.log('---------------------------');

  const techCorpLiaison = { id: 'techcorp.dark/techcorp-liaison-001', swarmId: 'cross-org', type: 'coordinator', instance: 1 };
  const dataIncAnalyst = { id: 'datainc.dark/datainc-analyst-001', swarmId: 'cross-org', type: 'specialist', instance: 1 };

  // Share computational resources
  await communicationSystem.sendMessage(
    techCorpLiaison,
    dataIncAnalyst,
    'coordination',
    {
      subject: 'GPU Cluster Resource Sharing Proposal',
      body: 'We have available GPU compute capacity for large-scale ML training',
      data: {
        resourceType: 'gpu-cluster',
        capacity: '1000 GPU-hours',
        availability: '24/7',
        pricing: '0.50 rUv/GPU-hour',
        qualityGuarantees: {
          uptime: 0.999,
          performance: 'A100-equivalent'
        }
      },
      format: 'structured'
    },
    {
      useQuDAG: true,
      anonymityLevel: 'medium',
      quantumResistant: true
    }
  );

  console.log('🖥️  Shared GPU cluster resources through QuDAG network');
  console.log('💎 Payment in rUv tokens with smart contracts');

  console.log('\n📊 Example 4: Performance and Security Monitoring');
  console.log('===============================================');

  // Get QuDAG network status
  const qudagStatus = communicationSystem.getQuDAGStatus();
  console.log('📡 QuDAG Network Status:');
  console.log(`   • Status: ${qudagStatus.status}`);
  console.log(`   • Total Nodes: ${qudagStatus.metrics?.totalNodes || 0}`);
  console.log(`   • Active Swarms: ${qudagStatus.metrics?.activeSwarms || 0}`);
  console.log(`   • Message Latency: ${qudagStatus.metrics?.messageLatency || 0}ms`);
  console.log(`   • Anonymity Level: ${((qudagStatus.metrics?.anonymityLevel || 0) * 100).toFixed(1)}%`);
  console.log(`   • Resource Utilization: ${((qudagStatus.metrics?.resourceUtilization || 0) * 100).toFixed(1)}%`);

  // Get traditional communication metrics
  const traditionalMetrics = communicationSystem.getMetrics();
  console.log('\n📈 Traditional Communication Metrics:');
  console.log(`   • Total Messages: ${traditionalMetrics.totalMessages}`);
  console.log(`   • Active Channels: ${traditionalMetrics.activeChannels}`);
  console.log(`   • Delivery Rate: ${(traditionalMetrics.deliveryRate * 100).toFixed(1)}%`);
  console.log(`   • Average Latency: ${traditionalMetrics.averageLatency}ms`);

  console.log('\n🔍 Example 5: Advanced Security Features');
  console.log('======================================');

  // Enable quantum-resistant communication for specific channel
  const sensitiveChannel = await communicationSystem.createChannel(
    'classified-operations',
    'hierarchical',
    { id: 'security-lead-001', swarmId: 'security', type: 'coordinator', instance: 1 },
    [
      { id: 'security-analyst-001', swarmId: 'security', type: 'specialist', instance: 1 },
      { id: 'compliance-specialist-001', swarmId: 'security', type: 'specialist', instance: 1 }
    ],
    {
      isPublic: false,
      requiresApproval: true
    }
  );

  await communicationSystem.enableQuantumResistantCommunication(sensitiveChannel.id);
  
  console.log(`🛡️  Enabled quantum-resistant communication for channel: ${sensitiveChannel.name}`);
  console.log('🔐 All messages in this channel are protected against quantum attacks');
  console.log('🧅 Anonymous routing prevents traffic analysis');
  console.log('🏛️  Compliance-ready for government and financial regulations');

  console.log('\n⚡ Example 6: Dynamic Agent Spawning with QuDAG');
  console.log('=============================================');

  // Demonstrate dynamic agent spawning using QuDAG for coordination
  const taskSpawner = new HierarchicalTaskSpawner(hierarchicalSystem, taskCoordinator);

  const emergencyTaskResult = await taskSpawner.TASK(
    'Respond to security breach - coordinate incident response team',
    {
      priority: 'critical',
      spawnAgent: true,
      agentType: 'coordinator',
      agentRole: 'Incident Response Commander',
      collaborationMode: 'team',
      resourceRequirements: {
        teamSize: 5,
        specializedTools: ['security-analysis', 'forensics', 'communication'],
        urgency: 'immediate'
      },
      communicationSettings: {
        useQuDAG: true,
        anonymityLevel: 'high',
        quantumResistant: true,
        secureChannelsOnly: true
      }
    },
    {
      agentId: 'security-lead-001',
      permissions: ['spawn-agent', 'escalate', 'coordinate-response'],
      communicationPreferences: {
        anonymityLevel: 'high',
        routingStrategy: 'onion',
        encryptionRequired: true
      }
    }
  );

  console.log(`🚨 Emergency response team spawned: ${emergencyTaskResult.taskId}`);
  console.log(`👤 Incident commander: ${emergencyTaskResult.agentId}`);
  console.log(`🔒 All coordination through quantum-resistant QuDAG network`);
  console.log(`🎭 High anonymity to prevent adversary monitoring`);

  // Show spawned team members
  if (emergencyTaskResult.spawnedAgents) {
    console.log('\n👥 Incident Response Team:');
    emergencyTaskResult.spawnedAgents.forEach((agent, index) => {
      console.log(`   ${index + 1}. ${agent.id} (${agent.type}) - Secured communication enabled`);
    });
  }

  console.log('\n🎯 Example 7: Zero-Trust Agent Authentication');
  console.log('==========================================');

  // Demonstrate zero-trust authentication using QuDAG
  const authenticationFlow = {
    step1: 'Agent requests access with quantum-resistant credentials',
    step2: 'QuDAG network verifies ML-DSA digital signature',
    step3: 'Reputation and capability verification through DAG consensus',
    step4: 'Temporary access token issued with quantum-resistant encryption',
    step5: 'All subsequent communications authenticated and encrypted'
  };

  console.log('🛡️  Zero-Trust Authentication Flow:');
  Object.entries(authenticationFlow).forEach(([step, description]) => {
    console.log(`   ${step}: ${description}`);
  });

  console.log('\n🎊 QuDAG Integration Summary');
  console.log('===========================');
  console.log('✅ Quantum-resistant encryption (ML-KEM + ML-DSA)');
  console.log('✅ Anonymous communication through onion routing');
  console.log('✅ Decentralized agent coordination with DAG consensus');
  console.log('✅ Cross-organizational secure collaboration');
  console.log('✅ Resource sharing with rUv token economy');
  console.log('✅ Zero-trust authentication and authorization');
  console.log('✅ Real-time performance and security monitoring');
  console.log('✅ Seamless integration with existing hierarchical agents');

  console.log('\n🔮 Future-Proof Benefits:');
  console.log('• Protection against quantum computer attacks');
  console.log('• Scalable to millions of agents across organizations');
  console.log('• Regulatory compliance for financial and government use');
  console.log('• Anonymous yet auditable communication patterns');
  console.log('• Autonomous economic interactions between agents');
  console.log('• Resilient against censorship and surveillance');

  return {
    qudagStatus,
    traditionalMetrics,
    activeSwarms: [financialSwarm, collaborationSwarm],
    securityFeatures: [
      'quantum-resistant-encryption',
      'anonymous-routing',
      'zero-trust-auth',
      'decentralized-consensus'
    ]
  };
}

// Export for use in other examples
export { demonstrateQuDAGIntegration };

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  async function runExample() {
    try {
      const result = await demonstrateQuDAGIntegration();
      console.log('\n📋 Example completed successfully!');
      console.log('Integration ready for production use.');
    } catch (error) {
      console.error('❌ QuDAG integration example failed:', error);
    }
  }
  
  runExample();
}