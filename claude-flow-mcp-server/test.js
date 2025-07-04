#!/usr/bin/env node

/**
 * Test script for Claude-Flow MCP Server
 * 
 * This script tests the MCP server functionality by simulating
 * MCP client requests and validating responses.
 */

import { spawn } from 'child_process';
import { promisify } from 'util';

const sleep = promisify(setTimeout);

class MCPTester {
  constructor() {
    this.server = null;
    this.testResults = [];
  }

  async startServer() {
    console.log('Starting MCP server...');
    
    this.server = spawn('node', ['server.js'], {
      stdio: ['pipe', 'pipe', 'pipe'],
      env: { 
        ...process.env, 
        CLAUDE_FLOW_LOG_LEVEL: 'debug',
        CLAUDE_FLOW_PATH: 'echo claude-flow-mock' // Mock for testing
      }
    });

    this.server.stderr.on('data', (data) => {
      console.log('Server:', data.toString().trim());
    });

    // Wait for server to start
    await sleep(1000);
    
    if (this.server.killed) {
      throw new Error('Server failed to start');
    }
    
    console.log('✓ Server started successfully');
  }

  async stopServer() {
    if (this.server) {
      this.server.kill('SIGTERM');
      await sleep(1000);
      if (!this.server.killed) {
        this.server.kill('SIGKILL');
      }
      console.log('✓ Server stopped');
    }
  }

  async sendRequest(request) {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        reject(new Error('Request timeout'));
      }, 5000);

      let response = '';
      
      const onData = (data) => {
        response += data.toString();
        try {
          const parsed = JSON.parse(response);
          clearTimeout(timeout);
          this.server.stdout.off('data', onData);
          resolve(parsed);
        } catch (e) {
          // Continue collecting data
        }
      };

      this.server.stdout.on('data', onData);
      this.server.stdin.write(JSON.stringify(request) + '\n');
    });
  }

  async testInitialize() {
    console.log('\nTesting initialize...');
    
    const request = {
      jsonrpc: '2.0',
      id: 1,
      method: 'initialize',
      params: {
        protocolVersion: '2024-11-05',
        capabilities: {
          tools: {}
        },
        clientInfo: {
          name: 'test-client',
          version: '1.0.0'
        }
      }
    };

    try {
      const response = await this.sendRequest(request);
      
      if (response.result && response.result.serverInfo) {
        console.log('✓ Initialize successful');
        this.testResults.push({ test: 'initialize', passed: true });
        return true;
      } else {
        console.log('✗ Initialize failed - invalid response');
        this.testResults.push({ test: 'initialize', passed: false, error: 'Invalid response' });
        return false;
      }
    } catch (error) {
      console.log('✗ Initialize failed:', error.message);
      this.testResults.push({ test: 'initialize', passed: false, error: error.message });
      return false;
    }
  }

  async testListTools() {
    console.log('\nTesting list tools...');
    
    const request = {
      jsonrpc: '2.0',
      id: 2,
      method: 'tools/list',
      params: {}
    };

    try {
      const response = await this.sendRequest(request);
      
      if (response.result && response.result.tools && Array.isArray(response.result.tools)) {
        console.log(`✓ List tools successful - found ${response.result.tools.length} tools`);
        
        // Validate some expected tools
        const toolNames = response.result.tools.map(t => t.name);
        const expectedTools = [
          'claude_flow_start',
          'claude_flow_status',
          'claude_flow_agent_spawn',
          'claude_flow_task_create',
          'claude_flow_memory_store'
        ];
        
        const missingTools = expectedTools.filter(name => !toolNames.includes(name));
        if (missingTools.length === 0) {
          console.log('✓ All expected tools found');
          this.testResults.push({ test: 'list_tools', passed: true });
          return true;
        } else {
          console.log('✗ Missing tools:', missingTools);
          this.testResults.push({ test: 'list_tools', passed: false, error: `Missing tools: ${missingTools.join(', ')}` });
          return false;
        }
      } else {
        console.log('✗ List tools failed - invalid response');
        this.testResults.push({ test: 'list_tools', passed: false, error: 'Invalid response' });
        return false;
      }
    } catch (error) {
      console.log('✗ List tools failed:', error.message);
      this.testResults.push({ test: 'list_tools', passed: false, error: error.message });
      return false;
    }
  }

  async testCallTool() {
    console.log('\nTesting call tool (status)...');
    
    const request = {
      jsonrpc: '2.0',
      id: 3,
      method: 'tools/call',
      params: {
        name: 'claude_flow_status',
        arguments: {
          verbose: false,
          json: true
        }
      }
    };

    try {
      const response = await this.sendRequest(request);
      
      if (response.result && response.result.content) {
        console.log('✓ Call tool successful');
        this.testResults.push({ test: 'call_tool', passed: true });
        return true;
      } else {
        console.log('✗ Call tool failed - invalid response');
        this.testResults.push({ test: 'call_tool', passed: false, error: 'Invalid response' });
        return false;
      }
    } catch (error) {
      console.log('✗ Call tool failed:', error.message);
      this.testResults.push({ test: 'call_tool', passed: false, error: error.message });
      return false;
    }
  }

  async testInvalidTool() {
    console.log('\nTesting invalid tool call...');
    
    const request = {
      jsonrpc: '2.0',
      id: 4,
      method: 'tools/call',
      params: {
        name: 'invalid_tool',
        arguments: {}
      }
    };

    try {
      const response = await this.sendRequest(request);
      
      if (response.error && response.error.code === -32601) {
        console.log('✓ Invalid tool properly rejected');
        this.testResults.push({ test: 'invalid_tool', passed: true });
        return true;
      } else {
        console.log('✗ Invalid tool not properly rejected');
        this.testResults.push({ test: 'invalid_tool', passed: false, error: 'Should have returned error' });
        return false;
      }
    } catch (error) {
      console.log('✗ Invalid tool test failed:', error.message);
      this.testResults.push({ test: 'invalid_tool', passed: false, error: error.message });
      return false;
    }
  }

  async runAllTests() {
    console.log('🧪 Starting Claude-Flow MCP Server Tests\n');
    
    try {
      await this.startServer();
      
      // Run tests
      await this.testInitialize();
      await this.testListTools();
      await this.testCallTool();
      await this.testInvalidTool();
      
    } catch (error) {
      console.error('Test suite failed:', error.message);
    } finally {
      await this.stopServer();
    }

    // Print results
    console.log('\n📊 Test Results:');
    console.log('================');
    
    let passed = 0;
    let failed = 0;
    
    for (const result of this.testResults) {
      const status = result.passed ? '✓' : '✗';
      const error = result.error ? ` (${result.error})` : '';
      console.log(`${status} ${result.test}${error}`);
      
      if (result.passed) {
        passed++;
      } else {
        failed++;
      }
    }
    
    console.log(`\nTotal: ${this.testResults.length}, Passed: ${passed}, Failed: ${failed}`);
    
    if (failed === 0) {
      console.log('\n🎉 All tests passed!');
      process.exit(0);
    } else {
      console.log('\n❌ Some tests failed');
      process.exit(1);
    }
  }
}

// Run tests if this script is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const tester = new MCPTester();
  tester.runAllTests().catch(console.error);
}
