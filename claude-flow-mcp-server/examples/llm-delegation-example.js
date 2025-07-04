#!/usr/bin/env node

/**
 * LLM-to-LLM Delegation Example
 * 
 * This example demonstrates how an LLM can delegate complex tasks to Claude-Flow
 * as if it were a subordinate LLM with specialized multi-agent capabilities.
 */

import { spawn } from 'child_process';
import { promisify } from 'util';

const sleep = promisify(setTimeout);

class LLMDelegationExample {
  constructor() {
    this.server = null;
    this.requestId = 0;
  }

  async connect() {
    console.log('🤖 Starting LLM-to-LLM Delegation Example');
    console.log('==========================================\n');
    
    this.server = spawn('node', ['../server.js'], {
      stdio: ['pipe', 'pipe', 'pipe'],
      env: { 
        ...process.env, 
        CLAUDE_FLOW_LOG_LEVEL: 'info'
      }
    });

    this.server.stderr.on('data', (data) => {
      const log = data.toString().trim();
      if (log.includes('Claude-Flow MCP Server started')) {
        console.log('✅ Connected to Claude-Flow MCP Server\n');
      }
    });

    await this.initialize();
    await sleep(1000);
  }

  async disconnect() {
    if (this.server) {
      this.server.kill('SIGTERM');
      await sleep(1000);
      console.log('\n🔌 Disconnected from Claude-Flow');
    }
  }

  async sendRequest(method, params = {}) {
    return new Promise((resolve, reject) => {
      const id = ++this.requestId;
      const request = {
        jsonrpc: '2.0',
        id,
        method,
        params
      };

      const timeout = setTimeout(() => {
        reject(new Error('Request timeout'));
      }, 15000);

      let response = '';
      
      const onData = (data) => {
        response += data.toString();
        try {
          const lines = response.split('\n').filter(line => line.trim());
          for (const line of lines) {
            const parsed = JSON.parse(line);
            if (parsed.id === id) {
              clearTimeout(timeout);
              this.server.stdout.off('data', onData);
              resolve(parsed);
              return;
            }
          }
        } catch (e) {
          // Continue collecting data
        }
      };

      this.server.stdout.on('data', onData);
      this.server.stdin.write(JSON.stringify(request) + '\n');
    });
  }

  async initialize() {
    const response = await this.sendRequest('initialize', {
      protocolVersion: '2024-11-05',
      capabilities: { tools: {} },
      clientInfo: { name: 'llm-delegation-example', version: '1.0.0' }
    });
    
    if (response.error) {
      throw new Error(`Initialize failed: ${response.error.message}`);
    }
  }

  async callTool(name, args = {}) {
    const response = await this.sendRequest('tools/call', {
      name,
      arguments: args
    });
    
    if (response.error) {
      throw new Error(`Tool call failed: ${response.error.message}`);
    }
    
    return JSON.parse(response.result.content[0].text);
  }

  async runDelegationExample() {
    try {
      await this.connect();
      
      console.log('🎯 Scenario: Primary LLM needs market research for a new product launch\n');
      
      // Step 1: Understand Claude-Flow's capabilities
      console.log('📋 Step 1: Understanding Claude-Flow capabilities...');
      const capabilities = await this.callTool('get_claude_flow_capabilities');
      
      console.log('✅ Claude-Flow can handle these types of work:');
      capabilities.capabilities.available_agent_types.forEach(agent => {
        console.log(`   • ${agent.type}: ${agent.description}`);
      });
      console.log('');

      // Step 2: Start Claude-Flow system
      console.log('🚀 Step 2: Starting Claude-Flow orchestration system...');
      const startResult = await this.callTool('claude_flow_start', { daemon: true });
      console.log(`✅ ${startResult.message}\n`);

      // Step 3: Delegate a complex research task
      console.log('📊 Step 3: Delegating market research task to Claude-Flow...');
      const delegation = await this.callTool('delegate_to_claude_flow', {
        task_description: "Research the competitive landscape for AI-powered customer service chatbots, focusing on enterprise solutions",
        task_type: "research",
        priority: 8,
        requirements: [
          "Include at least 10 major competitors",
          "Focus on enterprise pricing models",
          "Analyze AI/NLP capabilities and differentiators",
          "Include integration capabilities (CRM, helpdesk, etc.)"
        ],
        expected_deliverables: [
          "Competitive analysis report with company profiles",
          "Feature comparison matrix highlighting AI capabilities", 
          "Pricing analysis with enterprise tier breakdown",
          "Market positioning and differentiation analysis"
        ],
        context: "We're launching an AI customer service platform and need comprehensive competitive intelligence to inform our product roadmap and pricing strategy"
      });

      console.log('✅ Task successfully delegated to Claude-Flow!');
      console.log(`   Task Type: ${delegation.delegation_summary.task_type}`);
      console.log(`   Priority: ${delegation.delegation_summary.priority}`);
      console.log(`   Requirements: ${delegation.delegation_summary.requirements_count}`);
      console.log(`   Expected Deliverables: ${delegation.delegation_summary.deliverables_count}`);
      console.log('\n📝 Claude-Flow Response:');
      console.log(`   ${delegation.message}`);
      console.log('\n🎯 Next Steps:');
      delegation.next_steps.forEach(step => console.log(`   • ${step}`));
      console.log('');

      // Step 4: Monitor progress
      console.log('⏱️  Step 4: Monitoring task progress...');
      await sleep(2000); // Simulate some time passing
      
      const progress = await this.callTool('check_claude_flow_progress', {
        include_details: true
      });

      console.log('📊 Progress Update:');
      console.log(`   System Status: ${progress.success ? 'Active' : 'Issues detected'}`);
      console.log(`   Recommendation: ${progress.recommendations[0]}`);
      console.log('');

      // Step 5: Provide additional guidance
      console.log('💬 Step 5: Providing additional guidance to Claude-Flow...');
      const question = await this.callTool('ask_claude_flow_question', {
        question: "Please prioritize companies that have raised Series B funding or later, as these represent the most established competition. Also include any recent acquisitions in the space.",
        urgency: "normal"
      });

      console.log('✅ Question submitted to Claude-Flow:');
      console.log(`   "${question.question_logged}"`);
      console.log(`   Urgency: ${question.urgency}`);
      console.log(`   Guidance: ${question.guidance}`);
      console.log('');

      // Step 6: Provide specific instructions
      console.log('📝 Step 6: Providing specific instructions for refinement...');
      const instruction = await this.callTool('instruct_claude_flow', {
        instruction: "Focus the pricing analysis on per-agent/per-seat models and API usage tiers. Many enterprises prefer predictable per-user pricing over usage-based models.",
        instruction_type: "scope_change"
      });

      console.log('✅ Instruction provided to Claude-Flow:');
      console.log(`   Type: ${instruction.instruction_type}`);
      console.log(`   Instruction: "${instruction.instruction}"`);
      console.log(`   Guidance: ${instruction.guidance}`);
      console.log('');

      // Step 7: Simulate checking for completion and retrieving results
      console.log('📋 Step 7: Checking for completed results...');
      console.log('   (In a real scenario, you would check periodically until the task is complete)');
      console.log('   (Then use retrieve_claude_flow_results to get the final deliverables)');
      console.log('');

      console.log('🎉 LLM-to-LLM Delegation Example Complete!');
      console.log('==========================================');
      console.log('');
      console.log('📚 What happened:');
      console.log('   1. Primary LLM understood Claude-Flow\'s capabilities');
      console.log('   2. Delegated a complex research task with clear requirements');
      console.log('   3. Claude-Flow automatically spawned appropriate agents');
      console.log('   4. Primary LLM monitored progress and provided guidance');
      console.log('   5. Interactive refinement through questions and instructions');
      console.log('');
      console.log('🔄 In a real scenario:');
      console.log('   • Claude-Flow would coordinate multiple specialized agents');
      console.log('   • Research agents would gather competitive intelligence');
      console.log('   • Analysis agents would structure findings into deliverables');
      console.log('   • Coordinator agents would ensure quality and completeness');
      console.log('   • Primary LLM would receive structured, actionable results');
      
    } catch (error) {
      console.error('❌ Delegation example failed:', error.message);
    } finally {
      await this.disconnect();
    }
  }
}

// Run the example
const example = new LLMDelegationExample();
example.runDelegationExample().catch(console.error);
