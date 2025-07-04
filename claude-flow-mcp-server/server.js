#!/usr/bin/env node

/**
 * Claude-Flow MCP Server
 * 
 * This MCP server wraps the Claude-Flow CLI to provide programmatic access
 * to the Claude-Flow orchestration system for LLMs and other MCP clients.
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} from '@modelcontextprotocol/sdk/types.js';
import { spawn } from 'child_process';
import { promisify } from 'util';
import { z } from 'zod';

// Configuration
const CONFIG = {
  claudeFlowPath: process.env.CLAUDE_FLOW_PATH || 'npx claude-flow',
  timeout: parseInt(process.env.CLAUDE_FLOW_TIMEOUT) || 30000,
  maxOutputSize: parseInt(process.env.CLAUDE_FLOW_MAX_OUTPUT) || 1024 * 1024, // 1MB
  logLevel: process.env.CLAUDE_FLOW_LOG_LEVEL || 'info',
};

// Utility function to execute claude-flow commands
async function executeClaudeFlowCommand(command, args = [], options = {}) {
  return new Promise((resolve, reject) => {
    const timeout = options.timeout || CONFIG.timeout;
    const cmdParts = CONFIG.claudeFlowPath.split(' ');
    const executable = cmdParts[0];
    const baseArgs = cmdParts.slice(1);
    const fullArgs = [...baseArgs, command, ...args];

    if (CONFIG.logLevel === 'debug') {
      console.error(`Executing: ${executable} ${fullArgs.join(' ')}`);
    }

    const child = spawn(executable, fullArgs, {
      stdio: ['pipe', 'pipe', 'pipe'],
      env: { ...process.env, CLAUDE_FLOW_MCP_MODE: 'true' },
      ...options.spawnOptions
    });

    let stdout = '';
    let stderr = '';
    let timedOut = false;

    const timer = setTimeout(() => {
      timedOut = true;
      child.kill('SIGTERM');
      setTimeout(() => child.kill('SIGKILL'), 5000);
    }, timeout);

    child.stdout.on('data', (data) => {
      stdout += data.toString();
      if (stdout.length > CONFIG.maxOutputSize) {
        child.kill('SIGTERM');
        reject(new Error('Output size exceeded maximum limit'));
      }
    });

    child.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    child.on('close', (code) => {
      clearTimeout(timer);
      
      if (timedOut) {
        reject(new Error(`Command timed out after ${timeout}ms`));
        return;
      }

      const result = {
        code,
        stdout: stdout.trim(),
        stderr: stderr.trim(),
        success: code === 0
      };

      if (CONFIG.logLevel === 'debug') {
        console.error(`Command result:`, result);
      }

      resolve(result);
    });

    child.on('error', (error) => {
      clearTimeout(timer);
      reject(new Error(`Failed to execute command: ${error.message}`));
    });
  });
}

// Utility function to parse JSON output safely
function parseJsonOutput(output) {
  try {
    return JSON.parse(output);
  } catch (e) {
    // If not JSON, return structured text
    return {
      type: 'text',
      content: output,
      lines: output.split('\n').filter(line => line.trim())
    };
  }
}

// Tool definitions - Designed for LLM-to-LLM delegation
const TOOLS = [
  {
    name: 'delegate_to_claude_flow',
    description: 'Delegate a complex task to the Claude-Flow orchestration system. Use this when you need to break down complex work into subtasks and coordinate multiple specialized agents.',
    inputSchema: {
      type: 'object',
      properties: {
        task_description: {
          type: 'string',
          description: 'Clear description of the overall task you want Claude-Flow to handle'
        },
        task_type: {
          type: 'string',
          enum: ['research', 'development', 'analysis', 'implementation', 'coordination', 'review'],
          description: 'Primary type of work needed'
        },
        priority: {
          type: 'number',
          minimum: 1,
          maximum: 10,
          description: 'Task priority (1=low, 10=critical)',
          default: 5
        },
        requirements: {
          type: 'array',
          items: { type: 'string' },
          description: 'Specific requirements or constraints for the task'
        },
        expected_deliverables: {
          type: 'array',
          items: { type: 'string' },
          description: 'What outputs/deliverables you expect from this task'
        },
        context: {
          type: 'string',
          description: 'Additional context or background information for the task'
        }
      },
      required: ['task_description', 'task_type']
    }
  },
  {
    name: 'check_claude_flow_progress',
    description: 'Check the progress of tasks delegated to Claude-Flow. Use this to monitor ongoing work and get status updates.',
    inputSchema: {
      type: 'object',
      properties: {
        task_id: {
          type: 'string',
          description: 'Specific task ID to check (optional - if not provided, shows all active tasks)'
        },
        include_details: {
          type: 'boolean',
          description: 'Include detailed progress information and agent activities',
          default: true
        }
      }
    }
  },
  {
    name: 'ask_claude_flow_question',
    description: 'Ask Claude-Flow a specific question or request clarification about ongoing work. Use this for interactive communication.',
    inputSchema: {
      type: 'object',
      properties: {
        question: {
          type: 'string',
          description: 'Your question or request for Claude-Flow'
        },
        context_task_id: {
          type: 'string',
          description: 'Related task ID if this question is about a specific task'
        },
        urgency: {
          type: 'string',
          enum: ['low', 'normal', 'high', 'critical'],
          description: 'How urgent this question is',
          default: 'normal'
        }
      },
      required: ['question']
    }
  },
  {
    name: 'get_claude_flow_capabilities',
    description: 'Get information about what Claude-Flow can do and what agents/tools are available. Use this to understand delegation options.',
    inputSchema: {
      type: 'object',
      properties: {
        detailed: {
          type: 'boolean',
          description: 'Get detailed capability information',
          default: false
        }
      }
    }
  },
  {
    name: 'claude_flow_start',
    description: 'Initialize and start the Claude-Flow orchestration system if it is not already running',
    inputSchema: {
      type: 'object',
      properties: {
        daemon: {
          type: 'boolean',
          description: 'Run as daemon in background',
          default: false
        },
        port: {
          type: 'number',
          description: 'MCP server port',
          default: 3000
        },
        ui: {
          type: 'boolean',
          description: 'Start with web UI',
          default: false
        },
        verbose: {
          type: 'boolean',
          description: 'Enable verbose logging',
          default: false
        }
      }
    }
  },
  {
    name: 'retrieve_claude_flow_results',
    description: 'Retrieve completed work and results from Claude-Flow. Use this to get deliverables from finished tasks.',
    inputSchema: {
      type: 'object',
      properties: {
        task_id: {
          type: 'string',
          description: 'Task ID to retrieve results for'
        },
        format: {
          type: 'string',
          enum: ['summary', 'detailed', 'raw'],
          description: 'How to format the results',
          default: 'detailed'
        },
        include_artifacts: {
          type: 'boolean',
          description: 'Include any files or artifacts created',
          default: true
        }
      },
      required: ['task_id']
    }
  },
  {
    name: 'instruct_claude_flow',
    description: 'Give specific instructions or guidance to Claude-Flow about how to approach a task. Use this for course correction or additional guidance.',
    inputSchema: {
      type: 'object',
      properties: {
        instruction: {
          type: 'string',
          description: 'Your instruction or guidance for Claude-Flow'
        },
        task_id: {
          type: 'string',
          description: 'Task ID this instruction relates to (optional)'
        },
        instruction_type: {
          type: 'string',
          enum: ['guidance', 'correction', 'clarification', 'priority_change', 'scope_change'],
          description: 'Type of instruction',
          default: 'guidance'
        }
      },
      required: ['instruction']
    }
  },
  {
    name: 'claude_flow_agent_spawn',
    description: 'Spawn a new agent in the system',
    inputSchema: {
      type: 'object',
      properties: {
        type: {
          type: 'string',
          enum: ['coordinator', 'researcher', 'implementer', 'analyst', 'custom'],
          description: 'Type of agent to spawn'
        },
        name: {
          type: 'string',
          description: 'Name for the agent'
        },
        priority: {
          type: 'number',
          minimum: 1,
          maximum: 10,
          description: 'Agent priority (1-10)',
          default: 5
        },
        capabilities: {
          type: 'array',
          items: { type: 'string' },
          description: 'List of agent capabilities'
        }
      },
      required: ['type']
    }
  },
  {
    name: 'claude_flow_agent_list',
    description: 'List all agents in the system',
    inputSchema: {
      type: 'object',
      properties: {
        includeTerminated: {
          type: 'boolean',
          description: 'Include terminated agents',
          default: false
        },
        filterByType: {
          type: 'string',
          enum: ['coordinator', 'researcher', 'implementer', 'analyst', 'custom'],
          description: 'Filter agents by type'
        },
        verbose: {
          type: 'boolean',
          description: 'Include detailed agent information',
          default: false
        }
      }
    }
  },
  {
    name: 'claude_flow_task_create',
    description: 'Create a new task in the system',
    inputSchema: {
      type: 'object',
      properties: {
        type: {
          type: 'string',
          enum: ['research', 'implementation', 'analysis', 'review', 'coordination'],
          description: 'Type of task'
        },
        description: {
          type: 'string',
          description: 'Task description'
        },
        priority: {
          type: 'number',
          minimum: 1,
          maximum: 10,
          description: 'Task priority (1-10)',
          default: 5
        },
        assignTo: {
          type: 'string',
          description: 'Agent ID to assign task to'
        },
        dependencies: {
          type: 'array',
          items: { type: 'string' },
          description: 'List of task IDs this task depends on'
        }
      },
      required: ['type', 'description']
    }
  },
  {
    name: 'claude_flow_task_list',
    description: 'List all tasks in the system',
    inputSchema: {
      type: 'object',
      properties: {
        status: {
          type: 'string',
          enum: ['pending', 'running', 'completed', 'failed', 'cancelled'],
          description: 'Filter tasks by status'
        },
        assignedTo: {
          type: 'string',
          description: 'Filter tasks assigned to specific agent'
        },
        verbose: {
          type: 'boolean',
          description: 'Include detailed task information',
          default: false
        }
      }
    }
  },
  {
    name: 'claude_flow_memory_store',
    description: 'Store information in the system memory',
    inputSchema: {
      type: 'object',
      properties: {
        key: {
          type: 'string',
          description: 'Memory key/identifier'
        },
        value: {
          type: 'string',
          description: 'Value to store'
        },
        namespace: {
          type: 'string',
          description: 'Memory namespace',
          default: 'default'
        },
        tags: {
          type: 'array',
          items: { type: 'string' },
          description: 'Tags for categorization'
        }
      },
      required: ['key', 'value']
    }
  },
  {
    name: 'claude_flow_memory_query',
    description: 'Query information from system memory',
    inputSchema: {
      type: 'object',
      properties: {
        query: {
          type: 'string',
          description: 'Search query or key'
        },
        namespace: {
          type: 'string',
          description: 'Memory namespace to search in'
        },
        limit: {
          type: 'number',
          description: 'Maximum number of results',
          default: 10
        },
        tags: {
          type: 'array',
          items: { type: 'string' },
          description: 'Filter by tags'
        }
      },
      required: ['query']
    }
  },
  {
    name: 'claude_flow_monitor',
    description: 'Get real-time system monitoring information',
    inputSchema: {
      type: 'object',
      properties: {
        duration: {
          type: 'number',
          description: 'Monitoring duration in seconds',
          default: 10
        },
        metrics: {
          type: 'array',
          items: { type: 'string' },
          description: 'Specific metrics to monitor'
        }
      }
    }
  },
  {
    name: 'claude_flow_config_get',
    description: 'Get configuration values',
    inputSchema: {
      type: 'object',
      properties: {
        key: {
          type: 'string',
          description: 'Configuration key to retrieve'
        },
        section: {
          type: 'string',
          description: 'Configuration section'
        }
      }
    }
  },
  {
    name: 'claude_flow_config_set',
    description: 'Set configuration values',
    inputSchema: {
      type: 'object',
      properties: {
        key: {
          type: 'string',
          description: 'Configuration key to set'
        },
        value: {
          type: 'string',
          description: 'Configuration value'
        },
        section: {
          type: 'string',
          description: 'Configuration section'
        }
      },
      required: ['key', 'value']
    }
  },
  {
    name: 'claude_flow_workflow_execute',
    description: 'Execute a workflow configuration',
    inputSchema: {
      type: 'object',
      properties: {
        workflow: {
          type: 'string',
          description: 'Workflow name or path to workflow file'
        },
        parameters: {
          type: 'object',
          description: 'Workflow parameters'
        },
        async: {
          type: 'boolean',
          description: 'Execute workflow asynchronously',
          default: false
        }
      },
      required: ['workflow']
    }
  },
  {
    name: 'claude_flow_session_list',
    description: 'List active terminal sessions',
    inputSchema: {
      type: 'object',
      properties: {
        verbose: {
          type: 'boolean',
          description: 'Include detailed session information',
          default: false
        }
      }
    }
  },
  {
    name: 'claude_flow_execute_command',
    description: 'Execute a command in a terminal session',
    inputSchema: {
      type: 'object',
      properties: {
        command: {
          type: 'string',
          description: 'Command to execute'
        },
        sessionId: {
          type: 'string',
          description: 'Terminal session ID'
        },
        timeout: {
          type: 'number',
          description: 'Command timeout in seconds',
          default: 30
        },
        cwd: {
          type: 'string',
          description: 'Working directory'
        }
      },
      required: ['command']
    }
  }
];

// Create the MCP server
const server = new Server(
  {
    name: 'claude-flow-mcp-server',
    version: '1.0.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// List tools handler
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: TOOLS,
  };
});

// Call tool handler
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    switch (name) {
      case 'delegate_to_claude_flow': {
        // This is the main delegation tool - creates a comprehensive task
        console.error(`Delegating task: ${args.task_description}`);

        // First ensure Claude-Flow is running
        const statusResult = await executeClaudeFlowCommand('status');
        if (!statusResult.success) {
          // Try to start Claude-Flow
          await executeClaudeFlowCommand('start', ['--daemon']);
        }

        // Create a comprehensive task with context
        const taskFlags = ['create', args.task_type, args.task_description];
        if (args.priority) taskFlags.push('--priority', args.priority.toString());

        const taskResult = await executeClaudeFlowCommand('task', taskFlags);

        // Store additional context in memory
        if (args.context || args.requirements || args.expected_deliverables) {
          const contextData = {
            task_description: args.task_description,
            task_type: args.task_type,
            requirements: args.requirements || [],
            expected_deliverables: args.expected_deliverables || [],
            context: args.context || '',
            delegated_at: new Date().toISOString(),
            delegated_by: 'mcp_client'
          };

          await executeClaudeFlowCommand('memory', [
            'store',
            `task_context_${Date.now()}`,
            JSON.stringify(contextData),
            '--namespace', 'delegated_tasks',
            '--tags', 'delegation,context'
          ]);
        }

        // Spawn appropriate agents based on task type
        const agentType = {
          'research': 'researcher',
          'development': 'implementer',
          'analysis': 'analyst',
          'implementation': 'implementer',
          'coordination': 'coordinator',
          'review': 'reviewer'
        }[args.task_type] || 'coordinator';

        const agentResult = await executeClaudeFlowCommand('agent', [
          'spawn', agentType,
          '--name', `${args.task_type}-agent-${Date.now()}`,
          '--priority', (args.priority || 5).toString()
        ]);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: taskResult.success && agentResult.success,
                message: `Task delegated to Claude-Flow successfully. ${agentType} agent spawned to handle ${args.task_type} work.`,
                task_info: parseJsonOutput(taskResult.stdout),
                agent_info: parseJsonOutput(agentResult.stdout),
                delegation_summary: {
                  task_type: args.task_type,
                  priority: args.priority || 5,
                  requirements_count: (args.requirements || []).length,
                  deliverables_count: (args.expected_deliverables || []).length,
                  has_context: !!args.context
                },
                next_steps: [
                  "Use 'check_claude_flow_progress' to monitor task progress",
                  "Use 'ask_claude_flow_question' if you need to provide additional guidance",
                  "Use 'retrieve_claude_flow_results' when the task is complete"
                ]
              }, null, 2)
            }
          ]
        };
      }

      case 'check_claude_flow_progress': {
        // Get task status and agent activity
        const taskFlags = ['list'];
        if (args.task_id) taskFlags.push('--filter-id', args.task_id);
        if (args.include_details) taskFlags.push('--verbose');

        const taskResult = await executeClaudeFlowCommand('task', taskFlags);
        const agentResult = await executeClaudeFlowCommand('agent', ['list', '--verbose']);
        const systemResult = await executeClaudeFlowCommand('status');

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: taskResult.success,
                progress_summary: {
                  tasks: parseJsonOutput(taskResult.stdout),
                  active_agents: parseJsonOutput(agentResult.stdout),
                  system_status: parseJsonOutput(systemResult.stdout)
                },
                interpretation: args.task_id ?
                  `Progress report for task ${args.task_id}` :
                  "Overview of all active Claude-Flow work",
                recommendations: [
                  "Tasks in 'running' status are actively being worked on",
                  "Tasks in 'pending' status are waiting for agent assignment",
                  "Use 'instruct_claude_flow' if you need to provide guidance"
                ]
              }, null, 2)
            }
          ]
        };
      }

      case 'ask_claude_flow_question': {
        // Store the question in memory and create a coordination task
        const questionData = {
          question: args.question,
          context_task_id: args.context_task_id,
          urgency: args.urgency || 'normal',
          asked_at: new Date().toISOString(),
          asked_by: 'mcp_client'
        };

        await executeClaudeFlowCommand('memory', [
          'store',
          `question_${Date.now()}`,
          JSON.stringify(questionData),
          '--namespace', 'questions',
          '--tags', `question,${args.urgency},communication`
        ]);

        // Create a coordination task to handle the question
        const taskResult = await executeClaudeFlowCommand('task', [
          'create', 'coordination',
          `Address question: ${args.question}`,
          '--priority', args.urgency === 'critical' ? '10' : args.urgency === 'high' ? '8' : '5'
        ]);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: taskResult.success,
                message: "Question submitted to Claude-Flow for consideration",
                question_logged: args.question,
                urgency: args.urgency,
                task_created: parseJsonOutput(taskResult.stdout),
                guidance: "Claude-Flow will address this question through its coordination system. Check progress with 'check_claude_flow_progress'."
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_start': {
        const flags = [];
        if (args.daemon) flags.push('--daemon');
        if (args.port) flags.push('--port', args.port.toString());
        if (args.ui) flags.push('--ui');
        if (args.verbose) flags.push('--verbose');

        const result = await executeClaudeFlowCommand('start', flags);
        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                message: result.success ? 'Claude-Flow orchestration system is now running and ready to accept delegated tasks' : 'Failed to start Claude-Flow',
                output: result.stdout,
                error: result.stderr,
                exitCode: result.code,
                next_steps: result.success ? [
                  "Use 'delegate_to_claude_flow' to assign complex tasks",
                  "Use 'get_claude_flow_capabilities' to see what it can do"
                ] : ["Check system requirements and try again"]
              }, null, 2)
            }
          ]
        };
      }

      case 'get_claude_flow_capabilities': {
        // Get comprehensive information about what Claude-Flow can do
        const statusResult = await executeClaudeFlowCommand('status', ['--verbose']);
        const agentTypesResult = await executeClaudeFlowCommand('agent', ['types']);
        const toolsResult = await executeClaudeFlowCommand('tools', ['list']);

        const capabilities = {
          system_status: parseJsonOutput(statusResult.stdout),
          available_agent_types: [
            {
              type: 'researcher',
              description: 'Specializes in information gathering, web research, and data analysis',
              best_for: ['market research', 'competitive analysis', 'fact-checking', 'literature reviews']
            },
            {
              type: 'implementer',
              description: 'Handles coding, development, and technical implementation tasks',
              best_for: ['software development', 'API integration', 'automation scripts', 'technical solutions']
            },
            {
              type: 'analyst',
              description: 'Performs data analysis, pattern recognition, and insights generation',
              best_for: ['data analysis', 'trend identification', 'performance metrics', 'reporting']
            },
            {
              type: 'coordinator',
              description: 'Manages complex multi-step workflows and coordinates between agents',
              best_for: ['project management', 'workflow orchestration', 'task coordination', 'quality assurance']
            },
            {
              type: 'reviewer',
              description: 'Reviews work quality, provides feedback, and ensures standards',
              best_for: ['code review', 'content review', 'quality control', 'compliance checking']
            }
          ],
          delegation_patterns: {
            simple_tasks: "Use 'delegate_to_claude_flow' with clear task_description and task_type",
            complex_projects: "Break into phases, delegate each phase separately, use coordination agents",
            research_heavy: "Use researcher agents with specific requirements and expected deliverables",
            technical_work: "Use implementer agents with detailed technical specifications",
            ongoing_monitoring: "Use 'check_claude_flow_progress' regularly and 'ask_claude_flow_question' for guidance"
          },
          memory_capabilities: "Claude-Flow maintains context across tasks and can reference previous work",
          coordination_features: "Can handle dependencies between tasks and coordinate multiple agents"
        };

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: true,
                capabilities: capabilities,
                delegation_guide: {
                  when_to_delegate: [
                    "Tasks requiring multiple specialized skills",
                    "Complex research or analysis projects",
                    "Multi-step technical implementations",
                    "Work that benefits from parallel processing",
                    "Tasks requiring ongoing monitoring and coordination"
                  ],
                  how_to_delegate: [
                    "1. Use 'delegate_to_claude_flow' with clear description",
                    "2. Specify requirements and expected deliverables",
                    "3. Monitor progress with 'check_claude_flow_progress'",
                    "4. Provide guidance with 'ask_claude_flow_question' or 'instruct_claude_flow'",
                    "5. Retrieve results with 'retrieve_claude_flow_results'"
                  ]
                }
              }, null, 2)
            }
          ]
        };
      }

      case 'retrieve_claude_flow_results': {
        // Get completed task results and deliverables
        const taskResult = await executeClaudeFlowCommand('task', ['status', args.task_id]);
        const memoryResult = await executeClaudeFlowCommand('memory', [
          'query', `task_${args.task_id}`,
          '--namespace', 'results',
          '--limit', '10'
        ]);

        const taskInfo = parseJsonOutput(taskResult.stdout);
        const results = parseJsonOutput(memoryResult.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: taskResult.success,
                task_status: taskInfo,
                results: results,
                deliverables: {
                  format: args.format,
                  artifacts_included: args.include_artifacts,
                  summary: args.format === 'summary' ?
                    "Task completed - see results for key outcomes" :
                    "Detailed results and artifacts available"
                },
                interpretation: taskInfo.status === 'completed' ?
                  "Task completed successfully - results ready for review" :
                  `Task status: ${taskInfo.status} - results may be partial`
              }, null, 2)
            }
          ]
        };
      }

      case 'instruct_claude_flow': {
        // Provide guidance or instructions to Claude-Flow
        const instructionData = {
          instruction: args.instruction,
          task_id: args.task_id,
          instruction_type: args.instruction_type || 'guidance',
          provided_at: new Date().toISOString(),
          provided_by: 'mcp_client'
        };

        // Store instruction in memory
        await executeClaudeFlowCommand('memory', [
          'store',
          `instruction_${Date.now()}`,
          JSON.stringify(instructionData),
          '--namespace', 'instructions',
          '--tags', `instruction,${args.instruction_type},guidance`
        ]);

        // Create a coordination task to handle the instruction
        const taskResult = await executeClaudeFlowCommand('task', [
          'create', 'coordination',
          `Process instruction: ${args.instruction}`,
          '--priority', '7'
        ]);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: taskResult.success,
                message: "Instruction provided to Claude-Flow",
                instruction_type: args.instruction_type,
                instruction: args.instruction,
                coordination_task: parseJsonOutput(taskResult.stdout),
                guidance: "Claude-Flow will incorporate this instruction into its work. Monitor progress to see the impact."
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_agent_spawn': {
        const flags = ['spawn', args.type];
        if (args.name) flags.push('--name', args.name);
        if (args.priority) flags.push('--priority', args.priority.toString());
        if (args.capabilities) {
          flags.push('--capabilities', args.capabilities.join(','));
        }

        const result = await executeClaudeFlowCommand('agent', flags);
        const parsedOutput = parseJsonOutput(result.stdout);
        
        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                agent: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_agent_list': {
        const flags = ['list'];
        if (args.includeTerminated) flags.push('--include-terminated');
        if (args.filterByType) flags.push('--filter-type', args.filterByType);
        if (args.verbose) flags.push('--verbose');

        const result = await executeClaudeFlowCommand('agent', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                agents: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_task_create': {
        const flags = ['create', args.type, args.description];
        if (args.priority) flags.push('--priority', args.priority.toString());
        if (args.assignTo) flags.push('--assign-to', args.assignTo);
        if (args.dependencies) flags.push('--deps', args.dependencies.join(','));

        const result = await executeClaudeFlowCommand('task', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                task: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_task_list': {
        const flags = ['list'];
        if (args.status) flags.push('--status', args.status);
        if (args.assignedTo) flags.push('--assigned-to', args.assignedTo);
        if (args.verbose) flags.push('--verbose');

        const result = await executeClaudeFlowCommand('task', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                tasks: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_memory_store': {
        const flags = ['store', args.key, args.value];
        if (args.namespace) flags.push('--namespace', args.namespace);
        if (args.tags) flags.push('--tags', args.tags.join(','));

        const result = await executeClaudeFlowCommand('memory', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                stored: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_memory_query': {
        const flags = ['query', args.query];
        if (args.namespace) flags.push('--namespace', args.namespace);
        if (args.limit) flags.push('--limit', args.limit.toString());
        if (args.tags) flags.push('--tags', args.tags.join(','));

        const result = await executeClaudeFlowCommand('memory', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                results: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_monitor': {
        const flags = [];
        if (args.duration) flags.push('--duration', args.duration.toString());
        if (args.metrics) flags.push('--metrics', args.metrics.join(','));

        const result = await executeClaudeFlowCommand('monitor', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                monitoring: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_config_get': {
        const flags = ['get'];
        if (args.key) flags.push(args.key);
        if (args.section) flags.push('--section', args.section);

        const result = await executeClaudeFlowCommand('config', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                config: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_config_set': {
        const flags = ['set', args.key, args.value];
        if (args.section) flags.push('--section', args.section);

        const result = await executeClaudeFlowCommand('config', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                updated: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_workflow_execute': {
        const flags = ['workflow', args.workflow];
        if (args.parameters) {
          flags.push('--params', JSON.stringify(args.parameters));
        }
        if (args.async) flags.push('--async');

        const result = await executeClaudeFlowCommand('task', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                workflow: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_session_list': {
        const flags = [];
        if (args.verbose) flags.push('--verbose');

        const result = await executeClaudeFlowCommand('session', flags);
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                sessions: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      case 'claude_flow_execute_command': {
        const flags = ['exec', args.command];
        if (args.sessionId) flags.push('--session', args.sessionId);
        if (args.timeout) flags.push('--timeout', args.timeout.toString());
        if (args.cwd) flags.push('--cwd', args.cwd);

        const result = await executeClaudeFlowCommand('session', flags, {
          timeout: (args.timeout || 30) * 1000
        });
        const parsedOutput = parseJsonOutput(result.stdout);

        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                success: result.success,
                execution: parsedOutput,
                error: result.stderr,
                exitCode: result.code
              }, null, 2)
            }
          ]
        };
      }

      default:
        throw new McpError(
          ErrorCode.MethodNotFound,
          `Unknown tool: ${name}`
        );
    }
  } catch (error) {
    throw new McpError(
      ErrorCode.InternalError,
      `Tool execution failed: ${error.message}`
    );
  }
});

// Start the server
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  
  console.error('Claude-Flow MCP Server started');
  console.error(`Configuration: ${JSON.stringify(CONFIG, null, 2)}`);
}

main().catch((error) => {
  console.error('Server failed to start:', error);
  process.exit(1);
});
