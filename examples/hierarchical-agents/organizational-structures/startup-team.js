/**
 * Startup Team Organization Example
 * Demonstrates creating a complete startup organization with hierarchical agents
 */

import { OrganizationalScaffold } from '../../../src/organization/org-scaffold.js';
import { HierarchicalTaskSpawner } from '../../../src/agents/hierarchical-task-spawner.js';

async function createStartupOrganization() {
  console.log('🚀 Startup Organization Example');
  console.log('===============================\n');

  // Initialize organizational scaffold
  const orgScaffold = new OrganizationalScaffold(
    hierarchicalSystem, taskSpawner, communicationSystem, 
    agentManager, taskCoordinator, memory
  );

  // Step 1: Create the startup organization
  console.log('📋 Step 1: Creating Startup Organization');
  console.log('---------------------------------------');

  const startup = await orgScaffold.createOrganization(
    'startup', // template
    'TechVenture AI', // name
    {
      // Customizations for our specific startup
      structure: {
        maxLevels: 3,
        spanOfControl: { min: 2, max: 6, default: 4 },
        hierarchyType: 'flat',
        flexibilityLevel: 'adaptive',
        autonomyDistribution: [
          { level: 0, rolePattern: 'ceo', permissions: ['*'], constraints: [], escalationThreshold: 0 },
          { level: 1, rolePattern: 'lead', permissions: ['delegate', 'hire', 'budget'], constraints: ['approval'], escalationThreshold: 2 },
          { level: 2, rolePattern: 'developer|researcher', permissions: ['execute', 'research'], constraints: ['scope'], escalationThreshold: 3 }
        ]
      },
      departments: [
        {
          id: 'engineering',
          name: 'Engineering',
          purpose: 'AI product development and technical infrastructure',
          requiredRoles: ['tech-lead', 'senior-developer', 'ai-engineer'],
          targetSize: { min: 3, max: 8, optimal: 5 },
          budget: { 
            computational: { cpu: 16, memory: 32768, storage: 5000 },
            operational: { taskCapacity: 100, concurrency: 10 },
            temporal: { operatingHours: 12, responseTimes: 300 }
          },
          kpis: [
            { name: 'feature_velocity', metric: 'features_per_sprint', target: 8, measurement: 'count', frequency: 'weekly' },
            { name: 'code_quality', metric: 'test_coverage', target: 0.85, measurement: 'percentage', frequency: 'daily' },
            { name: 'bug_rate', metric: 'bugs_per_kloc', target: 2, measurement: 'ratio', frequency: 'weekly' }
          ],
          dependencies: ['product'],
          outputs: ['features', 'infrastructure', 'documentation']
        },
        {
          id: 'product',
          name: 'Product',
          purpose: 'Product strategy, user research, and market analysis',
          requiredRoles: ['product-manager', 'ux-researcher', 'data-analyst'],
          targetSize: { min: 2, max: 4, optimal: 3 },
          budget: {
            computational: { cpu: 4, memory: 8192, storage: 1000 },
            operational: { taskCapacity: 50, concurrency: 5 },
            temporal: { operatingHours: 10, responseTimes: 600 }
          },
          kpis: [
            { name: 'user_satisfaction', metric: 'nps_score', target: 50, measurement: 'score', frequency: 'monthly' },
            { name: 'feature_adoption', metric: 'adoption_rate', target: 0.7, measurement: 'percentage', frequency: 'weekly' }
          ],
          dependencies: [],
          outputs: ['requirements', 'user-research', 'product-strategy']
        },
        {
          id: 'growth',
          name: 'Growth',
          purpose: 'Marketing, sales, and customer acquisition',
          requiredRoles: ['growth-lead', 'content-creator', 'sales-agent'],
          targetSize: { min: 2, max: 5, optimal: 3 },
          budget: {
            computational: { cpu: 2, memory: 4096, storage: 500 },
            operational: { taskCapacity: 75, concurrency: 8 },
            temporal: { operatingHours: 10, responseTimes: 900 }
          },
          kpis: [
            { name: 'customer_acquisition', metric: 'new_customers_per_month', target: 100, measurement: 'count', frequency: 'monthly' },
            { name: 'conversion_rate', metric: 'trial_to_paid', target: 0.15, measurement: 'percentage', frequency: 'weekly' }
          ],
          dependencies: ['product'],
          outputs: ['leads', 'content', 'campaigns']
        }
      ]
    }
  );

  console.log(`✅ Startup created: ${startup.name}`);
  console.log(`🏢 Organization ID: ${startup.id}`);
  console.log(`👥 Initial team size: ${startup.performanceMetrics.totalAgents}`);
  console.log(`🏗️  Departments: ${startup.departments.size}`);

  // Step 2: Build the initial team
  console.log('\n📋 Step 2: Building Initial Team');
  console.log('-------------------------------');

  // Add CTO (Technical Leadership)
  const cto = await orgScaffold.addAgentToOrganization(
    startup.id,
    'coordinator',
    {
      title: 'Chief Technology Officer',
      type: 'executive',
      level: 1,
      permissions: [
        { action: 'spawn-agent', resource: 'engineering' },
        { action: 'delegate-task', resource: 'technical' },
        { action: 'make-decision', resource: 'technical-strategy' },
        { action: 'access-memory', resource: 'engineering' }
      ],
      canSpawnAgents: true,
      maxSubordinates: 8,
      reportingFrequency: 43200000, // 12 hours
      decisionAuthority: ['technical', 'operational']
    },
    'engineering'
  );

  console.log(`👔 Added CTO: ${cto.id}`);

  // Add Product Manager
  const productManager = await orgScaffold.addAgentToOrganization(
    startup.id,
    'coordinator',
    {
      title: 'Product Manager',
      type: 'manager',
      level: 1,
      permissions: [
        { action: 'spawn-agent', resource: 'product' },
        { action: 'delegate-task', resource: 'product' },
        { action: 'access-memory', resource: 'product' }
      ],
      canSpawnAgents: true,
      maxSubordinates: 5,
      reportingFrequency: 43200000,
      decisionAuthority: ['product', 'user-experience']
    },
    'product'
  );

  console.log(`📊 Added Product Manager: ${productManager.id}`);

  // Add Growth Lead
  const growthLead = await orgScaffold.addAgentToOrganization(
    startup.id,
    'coordinator',
    {
      title: 'Growth Lead',
      type: 'manager',
      level: 1,
      permissions: [
        { action: 'spawn-agent', resource: 'growth' },
        { action: 'delegate-task', resource: 'marketing' },
        { action: 'access-memory', resource: 'growth' }
      ],
      canSpawnAgents: true,
      maxSubordinates: 4,
      reportingFrequency: 86400000, // 24 hours
      decisionAuthority: ['marketing', 'sales']
    },
    'growth'
  );

  console.log(`📈 Added Growth Lead: ${growthLead.id}`);

  // Step 3: Execute organizational tasks to build the team
  console.log('\n📋 Step 3: Executing Team Building Tasks');
  console.log('---------------------------------------');

  // CTO builds engineering team
  const engineeringTeamTask = await orgScaffold.executeOrganizationalTask(
    startup.id,
    {
      id: { id: 'build-eng-team', swarmId: 'startup', sequence: 1, priority: 8 },
      type: 'coordination',
      objective: 'Build a world-class AI engineering team capable of developing cutting-edge ML products',
      requirements: {
        teamSize: 4,
        roles: ['senior-ai-engineer', 'full-stack-developer', 'ml-engineer', 'devops-engineer'],
        skills: ['python', 'tensorflow', 'react', 'kubernetes', 'aws'],
        experience: 'senior-level'
      },
      constraints: { budget: 50000, timeline: '2 weeks' },
      metadata: { priority: 'critical', department: 'engineering' },
      dependencies: [],
      estimatedDuration: 7200000, // 2 hours for planning and execution
      priority: 8
    },
    {
      department: 'engineering',
      assignToRole: 'Chief Technology Officer',
      priority: 8,
      requiresApproval: false
    }
  );

  console.log(`✅ Engineering team building task: ${engineeringTeamTask.taskId}`);
  console.log(`🤖 Assigned to: ${engineeringTeamTask.assignedAgent.id}`);

  // Product Manager builds product team
  const productTeamTask = await orgScaffold.executeOrganizationalTask(
    startup.id,
    {
      id: { id: 'build-product-team', swarmId: 'startup', sequence: 2, priority: 7 },
      type: 'coordination',
      objective: 'Assemble a product team focused on AI-driven user experiences and data-driven decisions',
      requirements: {
        teamSize: 3,
        roles: ['ux-researcher', 'data-analyst', 'product-designer'],
        skills: ['user-research', 'data-analysis', 'figma', 'sql', 'a-b-testing'],
        experience: 'mid-to-senior-level'
      },
      constraints: { budget: 30000, timeline: '1 week' },
      metadata: { priority: 'high', department: 'product' },
      dependencies: [],
      estimatedDuration: 5400000, // 1.5 hours
      priority: 7
    },
    {
      department: 'product',
      assignToRole: 'Product Manager',
      priority: 7,
      requiresApproval: false
    }
  );

  console.log(`✅ Product team building task: ${productTeamTask.taskId}`);

  // Growth Lead builds growth team
  const growthTeamTask = await orgScaffold.executeOrganizationalTask(
    startup.id,
    {
      id: { id: 'build-growth-team', swarmId: 'startup', sequence: 3, priority: 6 },
      type: 'coordination',
      objective: 'Create a growth team capable of scaling user acquisition and retention for AI products',
      requirements: {
        teamSize: 2,
        roles: ['content-creator', 'performance-marketer'],
        skills: ['content-marketing', 'seo', 'paid-advertising', 'analytics', 'copywriting'],
        experience: 'mid-level'
      },
      constraints: { budget: 20000, timeline: '1 week' },
      metadata: { priority: 'medium', department: 'growth' },
      dependencies: ['build-product-team'],
      estimatedDuration: 3600000, // 1 hour
      priority: 6
    },
    {
      department: 'growth',
      assignToRole: 'Growth Lead',
      priority: 6,
      requiresApproval: false
    }
  );

  console.log(`✅ Growth team building task: ${growthTeamTask.taskId}`);

  // Step 4: Execute product development task
  console.log('\n📋 Step 4: Product Development Sprint');
  console.log('----------------------------------');

  const productDevelopmentTask = await orgScaffold.executeOrganizationalTask(
    startup.id,
    {
      id: { id: 'mvp-development', swarmId: 'startup', sequence: 4, priority: 9 },
      type: 'development',
      objective: 'Develop MVP of AI-powered customer service chatbot with natural language processing',
      requirements: {
        features: ['nlp-processing', 'conversation-flow', 'integration-api', 'dashboard', 'analytics'],
        technologies: ['python', 'transformers', 'fastapi', 'react', 'postgresql'],
        quality: 'production-ready',
        timeline: '4 weeks'
      },
      constraints: { 
        performance: 'sub-200ms response time',
        scalability: '1000 concurrent users',
        security: 'enterprise-grade'
      },
      metadata: { 
        priority: 'critical', 
        crossFunctional: true,
        stakeholders: ['cto', 'product-manager', 'growth-lead']
      },
      dependencies: ['build-eng-team', 'build-product-team'],
      estimatedDuration: 14400000, // 4 hours for sprint planning
      priority: 9
    },
    {
      requiresApproval: true,
      priority: 9
    }
  );

  console.log(`✅ MVP development task: ${productDevelopmentTask.taskId}`);
  console.log(`🚀 Cross-functional team assigned`);
  
  if (productDevelopmentTask.organizationalImpact) {
    console.log('\n📊 Organizational Impact:');
    console.log(`   New agents spawned: ${productDevelopmentTask.organizationalImpact.newAgents}`);
    console.log(`   Resource utilization: ${(productDevelopmentTask.organizationalImpact.resourceUtilization * 100).toFixed(1)}%`);
    console.log(`   Team efficiency: ${(productDevelopmentTask.organizationalImpact.teamEfficiency * 100).toFixed(1)}%`);
    console.log(`   Communication volume: ${productDevelopmentTask.organizationalImpact.communicationVolume} msgs/hour`);
  }

  // Step 5: Monitor organizational health
  console.log('\n📋 Step 5: Organizational Health Check');
  console.log('------------------------------------');

  const orgStatus = await orgScaffold.getOrganizationStatus(startup.id);
  
  console.log(`🏢 Organization: ${orgStatus.organization.name}`);
  console.log(`💚 Health Score: ${(orgStatus.health * 100).toFixed(1)}%`);
  console.log(`👥 Total Agents: ${orgStatus.metrics.totalAgents}`);
  console.log(`⚡ Task Throughput: ${orgStatus.metrics.taskThroughput} tasks/hour`);
  console.log(`🤝 Collaboration Index: ${(orgStatus.metrics.collaborationIndex * 100).toFixed(1)}%`);
  console.log(`🎯 Autonomy Index: ${(orgStatus.metrics.autonomyIndex * 100).toFixed(1)}%`);
  console.log(`💬 Communication Health: ${(orgStatus.communicationHealth * 100).toFixed(1)}%`);

  if (orgStatus.recommendations.length > 0) {
    console.log('\n💡 Recommendations:');
    orgStatus.recommendations.forEach((rec, index) => {
      console.log(`   ${index + 1}. ${rec}`);
    });
  }

  // Step 6: Scale the organization based on success
  console.log('\n📋 Step 6: Scaling Based on Growth');
  console.log('---------------------------------');

  // Simulate successful MVP leading to scaling needs
  const scalingResult = await orgScaffold.scaleOrganization(
    startup.id,
    'up',
    {
      agentType: 'developer',
      targetCount: 2,
      targetRole: 'Full Stack Developer'
    }
  );

  console.log(`✅ Scaling operation: ${scalingResult.success ? 'Success' : 'Failed'}`);
  console.log(`📈 Changes made: ${scalingResult.changes.length}`);
  console.log(`➕ New agents: ${scalingResult.newAgents.length}`);

  if (scalingResult.newAgents.length > 0) {
    console.log('   Added agents:');
    scalingResult.newAgents.forEach((agent, index) => {
      console.log(`   ${index + 1}. ${agent.id} (${agent.type})`);
    });
  }

  console.log('\n🎉 Startup organization example completed!');
  console.log('\nWhat we accomplished:');
  console.log('• Created a complete startup organization with 3 departments');
  console.log('• Built specialized teams through hierarchical task delegation');
  console.log('• Executed cross-functional product development');
  console.log('• Monitored organizational health and performance');
  console.log('• Scaled the organization based on growth needs');
  console.log('• Demonstrated real-world startup workflow patterns');

  return startup;
}

// Demonstration of day-in-the-life scenario
async function dayInTheLifeScenario(startupId) {
  console.log('\n🌅 Day in the Life: Startup Operations');
  console.log('====================================\n');

  // Morning standup (coordinated across all teams)
  console.log('🌅 9:00 AM - Morning Standup');
  console.log('---------------------------');
  
  // Each team lead runs their standup
  const standupTasks = [
    'Engineering standup: Sprint progress review and blocker identification',
    'Product standup: User feedback review and priority alignment',
    'Growth standup: Campaign performance analysis and optimization planning'
  ];

  for (const [index, standup] of standupTasks.entries()) {
    console.log(`📅 ${index + 1}. ${standup}`);
  }

  // Mid-morning: High-priority customer issue
  console.log('\n🚨 10:30 AM - Customer Escalation');
  console.log('--------------------------------');
  
  console.log('🔥 Critical bug reported by enterprise customer');
  console.log('🎯 Auto-spawning crisis response team...');
  console.log('   • Senior Engineer (bug investigation)');
  console.log('   • Customer Success Agent (communication)');
  console.log('   • Product Manager (impact assessment)');
  console.log('   • DevOps Engineer (hotfix deployment)');

  // Afternoon: Feature development
  console.log('\n☀️  2:00 PM - Feature Development Sprint');
  console.log('--------------------------------------');
  
  console.log('🔨 Engineering team spawns specialized agents for new feature:');
  console.log('   • Frontend Specialist (UI implementation)');
  console.log('   • Backend Specialist (API development)');
  console.log('   • ML Engineer (algorithm optimization)');
  console.log('   • QA Specialist (test automation)');

  // Late afternoon: Growth experiment
  console.log('\n🌆 4:00 PM - Growth Experiment Launch');
  console.log('-----------------------------------');
  
  console.log('📊 Growth team launches A/B test campaign:');
  console.log('   • Content Creator (ad copy variants)');
  console.log('   • Performance Marketer (campaign setup)');
  console.log('   • Data Analyst (metrics tracking)');

  // End of day: Reporting and planning
  console.log('\n🌙 6:00 PM - Daily Wrap-up');
  console.log('-------------------------');
  
  console.log('📝 Automated end-of-day reports generated');
  console.log('📈 Performance metrics aggregated');
  console.log('🎯 Tomorrow\'s priorities identified');
  console.log('💤 Non-critical agents enter idle state');

  console.log('\n✨ End of day summary:');
  console.log('• 12 agents spawned dynamically based on needs');
  console.log('• 3 critical issues resolved through auto-escalation');
  console.log('• 2 new features shipped to production');
  console.log('• 85% team utilization with optimal resource allocation');
}

// Export for use in other examples
export { createStartupOrganization, dayInTheLifeScenario };

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  async function runExample() {
    try {
      const startup = await createStartupOrganization();
      await dayInTheLifeScenario(startup.id);
    } catch (error) {
      console.error('❌ Example failed:', error);
    }
  }
  
  runExample();
}