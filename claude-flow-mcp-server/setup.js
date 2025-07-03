#!/usr/bin/env node

/**
 * Setup script for Claude-Flow MCP Server
 * 
 * This script helps users set up the MCP server with their MCP client
 * and validates the installation.
 */

import { spawn } from 'child_process';
import { promisify } from 'util';
import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const sleep = promisify(setTimeout);

class MCPSetup {
  constructor() {
    this.config = {
      claudeFlowPath: 'npx claude-flow',
      timeout: 30000,
      logLevel: 'info'
    };
  }

  async checkPrerequisites() {
    console.log('🔍 Checking prerequisites...\n');

    // Check Node.js version
    const nodeVersion = process.version;
    const majorVersion = parseInt(nodeVersion.slice(1).split('.')[0]);
    
    if (majorVersion >= 18) {
      console.log(`✓ Node.js ${nodeVersion} (>= 18.0.0)`);
    } else {
      console.log(`✗ Node.js ${nodeVersion} (requires >= 18.0.0)`);
      return false;
    }

    // Check if claude-flow is available
    try {
      const result = await this.executeCommand('npx', ['claude-flow', '--help']);
      if (result.code === 0) {
        console.log('✓ Claude-Flow CLI available');
      } else {
        console.log('✗ Claude-Flow CLI not found');
        console.log('  Install with: npm install -g claude-flow');
        return false;
      }
    } catch (error) {
      console.log('✗ Claude-Flow CLI not accessible');
      console.log('  Install with: npm install -g claude-flow');
      return false;
    }

    // Check MCP SDK
    try {
      await import('@modelcontextprotocol/sdk/server/index.js');
      console.log('✓ MCP SDK available');
    } catch (error) {
      console.log('✗ MCP SDK not found');
      console.log('  Run: npm install');
      return false;
    }

    console.log('\n✅ All prerequisites met!\n');
    return true;
  }

  async executeCommand(command, args, options = {}) {
    return new Promise((resolve, reject) => {
      const child = spawn(command, args, {
        stdio: ['pipe', 'pipe', 'pipe'],
        ...options
      });

      let stdout = '';
      let stderr = '';

      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      child.on('close', (code) => {
        resolve({
          code,
          stdout: stdout.trim(),
          stderr: stderr.trim()
        });
      });

      child.on('error', reject);
    });
  }

  async generateMCPConfig() {
    console.log('📝 Generating MCP configuration...\n');

    const serverPath = join(__dirname, 'server.js');
    
    const config = {
      mcpServers: {
        "claude-flow": {
          command: "node",
          args: [serverPath],
          env: {
            CLAUDE_FLOW_PATH: this.config.claudeFlowPath,
            CLAUDE_FLOW_TIMEOUT: this.config.timeout.toString(),
            CLAUDE_FLOW_LOG_LEVEL: this.config.logLevel
          },
          capabilities: {
            tools: true,
            prompts: false,
            resources: false
          },
          description: "Claude-Flow orchestration system MCP server"
        }
      }
    };

    const configPath = join(__dirname, 'config', 'generated-mcp-config.json');
    
    // Ensure config directory exists
    const configDir = dirname(configPath);
    if (!existsSync(configDir)) {
      mkdirSync(configDir, { recursive: true });
    }

    writeFileSync(configPath, JSON.stringify(config, null, 2));
    
    console.log(`✓ MCP configuration generated: ${configPath}`);
    console.log('\n📋 Add this to your MCP client configuration:');
    console.log('```json');
    console.log(JSON.stringify(config, null, 2));
    console.log('```\n');

    return configPath;
  }

  async testServer() {
    console.log('🧪 Testing MCP server...\n');

    try {
      // Import and run the test
      const { default: MCPTester } = await import('./test.js');
      const tester = new MCPTester();
      
      console.log('Running basic connectivity test...');
      
      // Just test server startup and basic functionality
      await tester.startServer();
      await sleep(2000);
      await tester.stopServer();
      
      console.log('✓ Server test completed successfully\n');
      return true;
    } catch (error) {
      console.log(`✗ Server test failed: ${error.message}\n`);
      return false;
    }
  }

  async promptUser(question) {
    // Simple prompt implementation for Node.js
    process.stdout.write(question + ' ');
    
    return new Promise((resolve) => {
      process.stdin.once('data', (data) => {
        resolve(data.toString().trim());
      });
    });
  }

  async interactiveSetup() {
    console.log('🚀 Claude-Flow MCP Server Interactive Setup\n');
    console.log('This will help you configure the MCP server for your environment.\n');

    // Ask for Claude-Flow path
    const claudeFlowPath = await this.promptUser(
      `Claude-Flow command path (default: ${this.config.claudeFlowPath}):`
    );
    if (claudeFlowPath) {
      this.config.claudeFlowPath = claudeFlowPath;
    }

    // Ask for timeout
    const timeout = await this.promptUser(
      `Command timeout in milliseconds (default: ${this.config.timeout}):`
    );
    if (timeout && !isNaN(parseInt(timeout))) {
      this.config.timeout = parseInt(timeout);
    }

    // Ask for log level
    const logLevel = await this.promptUser(
      `Log level (debug/info/warn/error, default: ${this.config.logLevel}):`
    );
    if (logLevel && ['debug', 'info', 'warn', 'error'].includes(logLevel)) {
      this.config.logLevel = logLevel;
    }

    console.log('\n📋 Configuration Summary:');
    console.log(`  Claude-Flow Path: ${this.config.claudeFlowPath}`);
    console.log(`  Timeout: ${this.config.timeout}ms`);
    console.log(`  Log Level: ${this.config.logLevel}\n`);
  }

  async run() {
    try {
      // Check if running interactively
      const isInteractive = process.argv.includes('--interactive') || process.argv.includes('-i');
      
      if (isInteractive) {
        await this.interactiveSetup();
      }

      // Check prerequisites
      const prereqsOk = await this.checkPrerequisites();
      if (!prereqsOk) {
        console.log('❌ Prerequisites not met. Please install required dependencies.\n');
        process.exit(1);
      }

      // Generate configuration
      const configPath = await this.generateMCPConfig();

      // Test server
      const testOk = await this.testServer();
      if (!testOk) {
        console.log('⚠️  Server test failed, but configuration was generated.\n');
      }

      console.log('🎉 Setup completed successfully!\n');
      console.log('Next steps:');
      console.log('1. Add the generated configuration to your MCP client');
      console.log('2. Restart your MCP client');
      console.log('3. Test the connection with claude_flow_status tool\n');
      
      console.log('For more information, see README.md\n');

    } catch (error) {
      console.error('❌ Setup failed:', error.message);
      process.exit(1);
    }
  }
}

// Handle command line arguments
const args = process.argv.slice(2);

if (args.includes('--help') || args.includes('-h')) {
  console.log(`
Claude-Flow MCP Server Setup

Usage: node setup.js [options]

Options:
  -i, --interactive    Run interactive setup
  -h, --help          Show this help message

Examples:
  node setup.js                 # Quick setup with defaults
  node setup.js --interactive   # Interactive configuration
`);
  process.exit(0);
}

// Run setup if this script is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  const setup = new MCPSetup();
  setup.run().catch(console.error);
}
