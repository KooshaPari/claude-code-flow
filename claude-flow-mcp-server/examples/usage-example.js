#!/usr/bin/env node

/**
 * Claude-Flow MCP Server Usage Examples
 * 
 * This script demonstrates how to use the Claude-Flow MCP Server
 * programmatically from a Node.js application.
 */

import { spawn } from 'child_process';
import { promisify } from 'util';

const sleep = promisify(setTimeout);

class ClaudeFlowMCPClient {
  constructor() {
    this.server = null;
    this.requestId = 0;
  }

  async connect() {
    console.log('🔌 Connecting to Claude-Flow MCP Server...');
    
    this.server = spawn('node', ['../server.js'], {
      stdio: ['pipe', 'pipe', 'pipe'],
      env: { 
        ...process.env, 
        CLAUDE_FLOW_LOG_LEVEL: 'info'
      }
    });

    this.server.stderr.on('data', (data) => {
      console.log('Server Log:', data.toString().trim());
    });

    // Initialize the connection
    await this.initialize();
    console.log('✅ Connected successfully!\n');
  }

  async disconnect() {
    if (this.server) {
      this.server.kill('SIGTERM');
      await sleep(1000);
      console.log('🔌 Disconnected from server');
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
      }, 10000);

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
      clientInfo: { name: 'example-client', version: '1.0.0' }
    });
    
    if (response.error) {
      throw new Error(`Initialize failed: ${response.error.message}`);
    }
    
    return response.result;
  }

  async listTools() {
    const response = await this.sendRequest('tools/list');
    if (response.error) {
      throw new Error(`List tools failed: ${response.error.message}`);
    }
    return response.result.tools;
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
}

// Example usage scenarios
class UsageExamples {
  constructor() {
    this.client = new ClaudeFlowMCPClient();
  }

  async run() {
    try {
      await this.client.connect();
      
      console.log('🚀 Running Claude-Flow MCP Usage Examples\n');
      
      // Example 1: List available tools
      await this.example1_listTools();
      
      // Example 2: Check system status
      await this.example2_systemStatus();
      
      // Example 3: Start the orchestration system
      await this.example3_startSystem();
      
      // Example 4: Agent management
      await this.example4_agentManagement();
      
      // Example 5: Task management
      await this.example5_taskManagement();
      
      // Example 6: Memory operations
      await this.example6_memoryOperations();
      
      console.log('✅ All examples completed successfully!');
      
    } catch (error) {
      console.error('❌ Example failed:', error.message);
    } finally {
      await this.client.disconnect();
    }
  }

  async example1_listTools() {
    console.log('📋 Example 1: List Available Tools');
    console.log('=====================================');
    
    const tools = await this.client.listTools();
    console.log(`Found ${tools.length} tools:`);
    
    tools.forEach(tool => {
      console.log(`  • ${tool.name}: ${tool.description}`);
    });
    
    console.log('');
  }

  async example2_systemStatus() {
    console.log('📊 Example 2: Check System Status');
    console.log('==================================');
    
    const result = await this.client.callTool('claude_flow_status', {
      verbose: true,
      json: true
    });
    
    console.log('System Status:', JSON.stringify(result, null, 2));
    console.log('');
  }

  async example3_startSystem() {
    console.log('🚀 Example 3: Start Orchestration System');
    console.log('=========================================');
    
    const result = await this.client.callTool('claude_flow_start', {
      daemon: false,
      port: 3000,
      verbose: true
    });
    
    console.log('Start Result:', result.success ? '✅ Success' : '❌ Failed');
    if (result.output) {
      console.log('Output:', result.output);
    }
    console.log('');
  }

  async example4_agentManagement() {
    console.log('🤖 Example 4: Agent Management');
    console.log('===============================');
    
    // Spawn a research agent
    console.log('Spawning research agent...');
    const spawnResult = await this.client.callTool('claude_flow_agent_spawn', {
      type: 'researcher',
      name: 'example-researcher',
      priority: 7,
      capabilities: ['web-search', 'data-analysis']
    });
    
    console.log('Spawn Result:', spawnResult.success ? '✅ Success' : '❌ Failed');
    
    // List all agents
    console.log('Listing all agents...');
    const listResult = await this.client.callTool('claude_flow_agent_list', {
      verbose: true
    });
    
    console.log('Agents:', JSON.stringify(listResult.agents, null, 2));
    console.log('');
  }

  async example5_taskManagement() {
    console.log('📋 Example 5: Task Management');
    console.log('==============================');
    
    // Create a research task
    console.log('Creating research task...');
    const createResult = await this.client.callTool('claude_flow_task_create', {
      type: 'research',
      description: 'Research AI development trends in 2024',
      priority: 8
    });
    
    console.log('Create Result:', createResult.success ? '✅ Success' : '❌ Failed');
    
    // List all tasks
    console.log('Listing all tasks...');
    const listResult = await this.client.callTool('claude_flow_task_list', {
      verbose: true
    });
    
    console.log('Tasks:', JSON.stringify(listResult.tasks, null, 2));
    console.log('');
  }

  async example6_memoryOperations() {
    console.log('🧠 Example 6: Memory Operations');
    console.log('================================');
    
    // Store some information
    console.log('Storing information in memory...');
    const storeResult = await this.client.callTool('claude_flow_memory_store', {
      key: 'example_data',
      value: JSON.stringify({
        timestamp: new Date().toISOString(),
        data: 'This is example data stored via MCP',
        tags: ['example', 'mcp', 'demo']
      }),
      namespace: 'examples',
      tags: ['example', 'demo']
    });
    
    console.log('Store Result:', storeResult.success ? '✅ Success' : '❌ Failed');
    
    // Query the stored information
    console.log('Querying stored information...');
    const queryResult = await this.client.callTool('claude_flow_memory_query', {
      query: 'example_data',
      namespace: 'examples',
      limit: 5
    });
    
    console.log('Query Result:', JSON.stringify(queryResult.results, null, 2));
    console.log('');
  }
}

// Run examples if this script is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const examples = new UsageExamples();
  examples.run().catch(console.error);
}
