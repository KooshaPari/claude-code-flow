/**
 * Comprehensive help system for Claude-Flow CLI
 */

import { stdio, console_ } from '../../utils/runtime.js';

// ANSI color codes for cross-platform terminal styling
type ColorFunction = {
  (str: string): string;
  bold: (str: string) => string;
};

const createColorFunction = (code: string, boldCode?: string): ColorFunction => {
  const fn = (str: string) => `\x1b[${code}m${str}\x1b[0m`;
  (fn as ColorFunction).bold = boldCode ? (str: string) => `\x1b[1m\x1b[${boldCode}m${str}\x1b[0m` : fn;
  return fn as ColorFunction;
};

const colors = {
  cyan: createColorFunction('36', '36'),
  yellow: createColorFunction('33', '33'),
  white: createColorFunction('37', '37'),
  gray: createColorFunction('90', '90'),
  red: createColorFunction('31', '31')
};

// Simple table implementation for help display
class HelpTable {
  private headers: string[] = [];
  private rows: string[][] = [];
  
  header(headers: string[]): this {
    this.headers = headers;
    return this;
  }
  
  push(row: string[]): this {
    this.rows.push(row);
    return this;
  }
  
  border(enabled: boolean = true): this {
    // Just a no-op for API compatibility
    return this;
  }
  
  render(): void {
    if (this.headers.length > 0) {
      const headerLine = this.headers.map((h, i) => h.padEnd(i === 0 ? 20 : 15)).join(' ');
      console.log(headerLine);
      console.log('─'.repeat(headerLine.length));
    }
    
    for (const row of this.rows) {
      const rowLine = row.map((cell, i) => {
        // Strip ANSI codes for width calculation, then pad
        const cleanCell = cell.replace(/\x1b\[[0-9;]*m/g, '');
        const padding = Math.max(0, (i === 0 ? 20 : 15) - cleanCell.length);
        return cell + ' '.repeat(padding);
      }).join(' ');
      console.log(rowLine);
    }
  }
}

// Simple prompt implementation for interactive help
class HelpSelect {
  static async prompt<T>(options: { 
    message: string; 
    options: Array<{ name: string; value: T }> 
  }): Promise<T> {
    console.log(colors.cyan.bold(options.message));
    console.log();
    
    options.options.forEach((option, index) => {
      console.log(`${colors.gray((index + 1).toString().padStart(2))}. ${colors.white(option.name)}`);
    });
    
    console.log();
    console.log(colors.gray('Enter number and press Enter: '));
    
    return new Promise<T>((resolve) => {
      const stdin = stdio.stdin;
      
      const handleInput = (data: string | Buffer) => {
        const input = data.toString().trim();
        const index = parseInt(input) - 1;
        
        if (index >= 0 && index < options.options.length) {
          resolve(options.options[index].value);
        } else if (input.toLowerCase() === 'exit' || input.toLowerCase() === 'q') {
          resolve(options.options.find(opt => opt.value === 'exit')?.value || options.options[0].value);
        } else {
          console.log(colors.red('Invalid selection. Please try again.'));
          console.log(colors.gray('Enter number and press Enter: '));
        }
      };
      
      if (typeof stdin.once === 'function') {
        // Node.js style
        stdin.once('data', handleInput);
        if (typeof stdin.resume === 'function') {
          stdin.resume();
        }
      } else {
        // Fallback for other environments
        setTimeout(() => resolve(options.options[0].value), 100);
      }
    });
  }
}

// Mock Command class for when @cliffy is not available
class MockCommand {
  private _name = '';
  private _description = '';
  private _usage = '';
  private _version = '';
  private _arguments: Array<{ name: string; description?: string; required?: boolean; optional?: boolean; variadic?: boolean; type?: string; choices?: string[]; default?: any; collect?: boolean; value?: any; list?: boolean; separator?: string; args?: any[] }> = [];
  private _options: Array<{ flags: string; description: string; required?: boolean; optional?: boolean; variadic?: boolean; default?: any; choices?: string[]; collect?: boolean; value?: any; conflicts?: string[]; depends?: string[]; standalone?: boolean; hidden?: boolean; global?: boolean; prepend?: boolean; args?: any[]; list?: boolean; separator?: string; type?: string }> = [];
  private _action?: Function;
  private _examples: Array<{ name: string; description: string }> = [];
  private _commands: MockCommand[] = [];
  private _globalOptions: Array<{ flags: string; description: string; required?: boolean; optional?: boolean; variadic?: boolean; default?: any; choices?: string[]; collect?: boolean; value?: any; conflicts?: string[]; depends?: string[]; standalone?: boolean; hidden?: boolean; global?: boolean; prepend?: boolean; args?: any[]; list?: boolean; separator?: string; type?: string }> = [];
  private _parent?: MockCommand;
  private _children: MockCommand[] = [];
  private _meta: any = {};
  private _env: Record<string, string> = {};
  private _types: Record<string, any> = {};
  private _hooks: Record<string, Array<(options: any, ...args: any[]) => void | Promise<void>>> = {};
  private _listeners: Record<string, Array<(...args: any[]) => void>> = {};
  private _executable = false;
  private _standalone = false;
  private _hidden = false;
  private _isAction = false;
  private _global = false;
  private _throwErrors = false;
  private _stopEarly = false;
  private _allowEmpty = false;
  private _allowExcess = false;

  // Configuration methods
  name(name: string): this {
    this._name = name;
    return this;
  }

  version(version: string, flags?: string, description?: string): this {
    this._version = version;
    return this;
  }

  description(desc: string): this {
    this._description = desc;
    return this;
  }

  usage(usage: string): this {
    this._usage = usage;
    return this;
  }

  example(name: string, description: string): this {
    this._examples.push({ name, description });
    return this;
  }

  examples(examples: Array<{ name: string; description: string }>): this {
    this._examples = examples;
    return this;
  }

  // Options
  option(flags: string, description: string, defaultValue?: any): this;
  option(flags: string, description: string, options: { default?: any; required?: boolean; collect?: boolean; value?: any; choices?: string[]; conflicts?: string[]; depends?: string[]; standalone?: boolean; hidden?: boolean; global?: boolean; prepend?: boolean; args?: any[]; list?: boolean; separator?: string; type?: string }): this;
  option(flags: string, description: string, fn: (value: any, previous: any) => any, defaultValue?: any): this;
  option(flags: string, description: string, optionsOrFnOrDefault?: any, defaultValue?: any): this {
    const option: any = { flags, description };
    
    if (typeof optionsOrFnOrDefault === 'function') {
      option.parseArg = optionsOrFnOrDefault;
      option.default = defaultValue;
    } else if (typeof optionsOrFnOrDefault === 'object' && optionsOrFnOrDefault !== null) {
      Object.assign(option, optionsOrFnOrDefault);
    } else if (optionsOrFnOrDefault !== undefined) {
      option.default = optionsOrFnOrDefault;
    }

    this._options.push(option);
    return this;
  }

  globalOption(flags: string, description: string, defaultValue?: any): this;
  globalOption(flags: string, description: string, options: { default?: any; required?: boolean; collect?: boolean; value?: any; choices?: string[]; conflicts?: string[]; depends?: string[]; standalone?: boolean; hidden?: boolean; global?: boolean; prepend?: boolean; args?: any[]; list?: boolean; separator?: string; type?: string }): this;
  globalOption(flags: string, description: string, fn: (value: any, previous: any) => any, defaultValue?: any): this;
  globalOption(flags: string, description: string, optionsOrFnOrDefault?: any, defaultValue?: any): this {
    const option: any = { flags, description, global: true };
    
    if (typeof optionsOrFnOrDefault === 'function') {
      option.parseArg = optionsOrFnOrDefault;
      option.default = defaultValue;
    } else if (typeof optionsOrFnOrDefault === 'object' && optionsOrFnOrDefault !== null) {
      Object.assign(option, optionsOrFnOrDefault);
    } else if (optionsOrFnOrDefault !== undefined) {
      option.default = optionsOrFnOrDefault;
    }

    this._globalOptions.push(option);
    return this;
  }

  // Arguments
  arguments(desc: string): this {
    // Parse argument description like "[option:string]" or "<required:string>"
    this._arguments.push({ name: desc });
    return this;
  }

  argument(name: string, description?: string, defaultValue?: any): this;
  argument(name: string, description: string, options: { default?: any; required?: boolean; optional?: boolean; variadic?: boolean; type?: string; choices?: string[]; collect?: boolean; value?: any; list?: boolean; separator?: string; args?: any[] }): this;
  argument(name: string, description: string, fn: (value: any, previous: any) => any, defaultValue?: any): this;
  argument(name: string, description?: string, optionsOrFnOrDefault?: any, defaultValue?: any): this {
    const argument: any = { name, description };
    
    if (typeof optionsOrFnOrDefault === 'function') {
      argument.parseArg = optionsOrFnOrDefault;
      argument.default = defaultValue;
    } else if (typeof optionsOrFnOrDefault === 'object' && optionsOrFnOrDefault !== null) {
      Object.assign(argument, optionsOrFnOrDefault);
    } else if (optionsOrFnOrDefault !== undefined) {
      argument.default = optionsOrFnOrDefault;
    }

    this._arguments.push(argument);
    return this;
  }

  // Commands
  command(name: string, cmd?: MockCommand): MockCommand;
  command(name: string, description?: string): MockCommand;
  command(name: string, description: string, options: any): MockCommand;
  command(name: string, cmdOrDescription?: MockCommand | string, options?: any): MockCommand {
    if (cmdOrDescription instanceof MockCommand) {
      this._commands.push(cmdOrDescription);
      cmdOrDescription._parent = this;
      this._children.push(cmdOrDescription);
      return cmdOrDescription;
    } else {
      const newCommand = new MockCommand();
      newCommand._name = name;
      if (typeof cmdOrDescription === 'string') {
        newCommand._description = cmdOrDescription;
      }
      if (options) {
        Object.assign(newCommand, options);
      }
      this._commands.push(newCommand);
      newCommand._parent = this;
      this._children.push(newCommand);
      return newCommand;
    }
  }

  // Actions
  action(fn: (options: any, ...args: any[]) => void | Promise<void>): this {
    this._action = fn;
    this._isAction = true;
    return this;
  }

  // Parsing (mock implementations)
  async parse(args?: string[], options?: { from?: string; run?: boolean }): Promise<any> {
    if (options?.run !== false && this._action) {
      await this._action({}, ...(args || []));
    }
    return {};
  }

  parseOptions(args: string[]): any {
    return {};
  }

  parseArguments(args: string[]): any {
    return {};
  }

  // Help
  showHelp(): void {
    console.log(this.getHelp());
  }

  outputHelp(): void {
    this.showHelp();
  }

  help(command?: string): string {
    return this.getHelp();
  }

  helpInformation(): string {
    return this.getHelp();
  }

  getHelp(): string {
    return `Help for ${this._name || 'command'}: ${this._description}`;
  }

  // Completion
  complete(complete: (cmd: MockCommand, parent?: MockCommand) => string[] | Promise<string[]>): this {
    return this;
  }

  // Utilities
  getName(): string {
    return this._name;
  }

  getDescription(): string {
    return this._description;
  }

  getUsage(): string {
    return this._usage;
  }

  getVersion(): string {
    return this._version;
  }

  getExamples(): Array<{ name: string; description: string }> {
    return this._examples;
  }

  getOptions(): Array<any> {
    return this._options;
  }

  getArguments(): Array<any> {
    return this._arguments;
  }

  getCommands(): MockCommand[] {
    return this._commands;
  }

  getGlobalOptions(): Array<any> {
    return this._globalOptions;
  }

  getOption(name: string): any | undefined {
    return this._options.find(opt => opt.flags.includes(name));
  }

  getArgument(name: string): any | undefined {
    return this._arguments.find(arg => arg.name === name);
  }

  getCommand(name: string): MockCommand | undefined {
    return this._commands.find(cmd => cmd._name === name);
  }

  hasOption(name: string): boolean {
    return this.getOption(name) !== undefined;
  }

  hasArgument(name: string): boolean {
    return this.getArgument(name) !== undefined;
  }

  hasCommand(name: string): boolean {
    return this.getCommand(name) !== undefined;
  }

  // State
  isExecutable(): boolean {
    return this._executable;
  }

  isStandalone(): boolean {
    return this._standalone;
  }

  isHidden(): boolean {
    return this._hidden;
  }

  isAction(): boolean {
    return this._isAction;
  }

  isGlobal(): boolean {
    return this._global;
  }

  // Error handling
  throwErrors(throwErrors?: boolean): this {
    if (throwErrors !== undefined) {
      this._throwErrors = throwErrors;
    }
    return this;
  }

  // Configuration
  stopEarly(stopEarly?: boolean): this {
    if (stopEarly !== undefined) {
      this._stopEarly = stopEarly;
    }
    return this;
  }

  allowEmpty(allowEmpty?: boolean): this {
    if (allowEmpty !== undefined) {
      this._allowEmpty = allowEmpty;
    }
    return this;
  }

  allowExcess(allowExcess?: boolean): this {
    if (allowExcess !== undefined) {
      this._allowExcess = allowExcess;
    }
    return this;
  }

  // Environment
  env(envVars: Record<string, string>): this {
    this._env = envVars;
    return this;
  }

  // Meta
  meta(meta: any): this {
    this._meta = meta;
    return this;
  }

  getMeta(): any {
    return this._meta;
  }

  // Parent/Child relationships
  getParent(): MockCommand | undefined {
    return this._parent;
  }

  getChildren(): MockCommand[] {
    return this._children;
  }

  // Execution
  async execute(options?: any, ...args: any[]): Promise<any> {
    if (this._action) {
      await this._action(options || {}, ...args);
    }
  }

  // Types
  type(name: string, type: any): this {
    this._types[name] = type;
    return this;
  }

  // Reset
  reset(): this {
    this._name = '';
    this._description = '';
    this._usage = '';
    this._version = '';
    this._arguments = [];
    this._options = [];
    this._action = undefined;
    this._examples = [];
    this._commands = [];
    this._globalOptions = [];
    this._parent = undefined;
    this._children = [];
    this._meta = {};
    this._env = {};
    this._types = {};
    this._hooks = {};
    this._listeners = {};
    return this;
  }

  // Clone
  clone(): MockCommand {
    const cloned = new MockCommand();
    cloned._name = this._name;
    cloned._description = this._description;
    cloned._usage = this._usage;
    cloned._version = this._version;
    cloned._arguments = [...this._arguments];
    cloned._options = [...this._options];
    cloned._action = this._action;
    cloned._examples = [...this._examples];
    cloned._globalOptions = [...this._globalOptions];
    cloned._meta = { ...this._meta };
    cloned._env = { ...this._env };
    cloned._types = { ...this._types };
    return cloned;
  }

  // Validation
  validate(): this {
    return this;
  }

  // Hook
  hook(event: string, fn: (options: any, ...args: any[]) => void | Promise<void>): this {
    if (!this._hooks[event]) {
      this._hooks[event] = [];
    }
    this._hooks[event].push(fn);
    return this;
  }

  // Events
  on(event: string, listener: (...args: any[]) => void): this {
    if (!this._listeners[event]) {
      this._listeners[event] = [];
    }
    this._listeners[event].push(listener);
    return this;
  }

  off(event: string, listener: (...args: any[]) => void): this {
    if (this._listeners[event]) {
      this._listeners[event] = this._listeners[event].filter(l => l !== listener);
    }
    return this;
  }

  once(event: string, listener: (...args: any[]) => void): this {
    const onceListener = (...args: any[]) => {
      listener(...args);
      this.off(event, onceListener);
    };
    this.on(event, onceListener);
    return this;
  }

  emit(event: string, ...args: any[]): boolean {
    if (this._listeners[event]) {
      this._listeners[event].forEach(listener => listener(...args));
      return this._listeners[event].length > 0;
    }
    return false;
  }

  // Property arrays (readonly properties)
  get options(): Array<any> {
    return this._options;
  }

  get commandArguments(): Array<any> {
    return this._arguments;
  }

  get commands(): MockCommand[] {
    return this._commands;
  }

  get globalOptions(): Array<any> {
    return this._globalOptions;
  }

  // Readonly properties that expose internal state
  get readonly(): {
    readonly name: string;
    readonly description: string;
    readonly usage: string;
    readonly version: string;
    readonly examples: Array<{ name: string; description: string }>;
    readonly parent: MockCommand | undefined;
    readonly children: MockCommand[];
    readonly executable: boolean;
    readonly standalone: boolean;
    readonly hidden: boolean;
    readonly action: boolean;
    readonly global: boolean;
    readonly throwErrors: boolean;
    readonly stopEarly: boolean;
    readonly allowEmpty: boolean;
    readonly allowExcess: boolean;
    readonly meta: any;
    readonly env: Record<string, string>;
    readonly types: Record<string, any>;
    readonly hooks: Record<string, Array<(options: any, ...args: any[]) => void | Promise<void>>>;
    readonly listeners: Record<string, Array<(...args: any[]) => void>>;
  } {
    return {
      name: this._name,
      description: this._description,
      usage: this._usage,
      version: this._version,
      examples: this._examples,
      parent: this._parent,
      children: this._children,
      executable: this._executable,
      standalone: this._standalone,
      hidden: this._hidden,
      action: this._isAction,
      global: this._global,
      throwErrors: this._throwErrors,
      stopEarly: this._stopEarly,
      allowEmpty: this._allowEmpty,
      allowExcess: this._allowExcess,
      meta: this._meta,
      env: this._env,
      types: this._types,
      hooks: this._hooks,
      listeners: this._listeners,
    };
  }
}

export const helpCommand = new MockCommand()
  .description('Comprehensive help system with examples and tutorials')
  .arguments('[topic:string]')
  .option('-i, --interactive', 'Start interactive help mode')
  .option('-e, --examples', 'Show examples for the topic')
  .option('--tutorial', 'Show tutorial for the topic')
  .option('--all', 'Show all available help topics')
  .action(async (options: { interactive?: boolean; examples?: boolean; tutorial?: boolean; all?: boolean }, topic: string | undefined) => {
    try {
      if (options.interactive) {
        await startInteractiveHelp();
      } else if (options.all) {
        showAllTopics();
      } else if (topic) {
        await showTopicHelp(topic, options);
      } else {
        showMainHelp();
      }
    } catch (error) {
      console.error(colors.red('Error in help system:'), error instanceof Error ? error.message : String(error));
      process.exit(1);
    }
  });

interface HelpTopic {
  name: string;
  description: string;
  category: 'basic' | 'advanced' | 'workflow' | 'configuration' | 'troubleshooting';
  examples?: HelpExample[];
  tutorial?: string[];
  related?: string[];
}

interface HelpExample {
  description: string;
  command: string;
  explanation?: string;
}

const HELP_TOPICS: HelpTopic[] = [
  {
    name: 'getting-started',
    description: 'Basic introduction to Claude-Flow',
    category: 'basic',
    tutorial: [
      'Welcome to Claude-Flow! This tutorial will get you started.',
      '1. First, initialize a configuration file:',
      '   claude-flow config init',
      '',
      '2. Start the orchestration system:',
      '   claude-flow start',
      '',
      '3. In another terminal, spawn your first agent:',
      '   claude-flow agent spawn researcher --name "My Research Agent"',
      '',
      '4. Create a task for the agent:',
      '   claude-flow task create research "Find information about AI trends"',
      '',
      '5. Monitor progress:',
      '   claude-flow status',
      '',
      'You can also use the interactive REPL mode:',
      '   claude-flow repl',
      '',
      'For more help, try: claude-flow help <topic>'
    ],
    related: ['agents', 'tasks', 'configuration']
  },
  {
    name: 'agents',
    description: 'Working with Claude-Flow agents',
    category: 'basic',
    examples: [
      {
        description: 'Spawn a research agent',
        command: 'claude-flow agent spawn researcher --name "Research Assistant"',
        explanation: 'Creates a new research agent with specialized capabilities for information gathering'
      },
      {
        description: 'List all active agents',
        command: 'claude-flow agent list',
        explanation: 'Shows all currently running agents with their status and task counts'
      },
      {
        description: 'Get detailed agent information',
        command: 'claude-flow agent info agent-001',
        explanation: 'Displays comprehensive information about a specific agent'
      },
      {
        description: 'Terminate an agent',
        command: 'claude-flow agent terminate agent-001',
        explanation: 'Safely shuts down an agent and reassigns its tasks'
      }
    ],
    tutorial: [
      'Agents are the core workers in Claude-Flow. Each agent has:',
      '• A unique ID (automatically generated)',
      '• A name (for easy identification)',
      '• A type (coordinator, researcher, implementer, analyst, custom)',
      '• Capabilities (what the agent can do)',
      '• A system prompt (instructions for the agent)',
      '',
      'Agent Types:',
      '• coordinator: Plans and delegates tasks',
      '• researcher: Gathers and analyzes information',
      '• implementer: Writes code and creates solutions',
      '• analyst: Identifies patterns and generates insights',
      '• custom: User-defined behavior',
      '',
      'Best Practices:',
      '• Use descriptive names for your agents',
      '• Match agent types to your workflow needs',
      '• Monitor agent performance with "claude-flow status"',
      '• Terminate idle agents to free resources'
    ],
    related: ['tasks', 'workflows', 'coordination']
  },
  {
    name: 'tasks',
    description: 'Creating and managing tasks',
    category: 'basic',
    examples: [
      {
        description: 'Create a research task',
        command: 'claude-flow task create research "Find papers on quantum computing" --priority 5',
        explanation: 'Creates a high-priority research task with specific instructions'
      },
      {
        description: 'Create a task with dependencies',
        command: 'claude-flow task create analysis "Analyze research results" --dependencies task-001',
        explanation: 'Creates a task that waits for task-001 to complete before starting'
      },
      {
        description: 'Assign task to specific agent',
        command: 'claude-flow task create implementation "Write API client" --assign agent-003',
        explanation: 'Directly assigns a task to a specific agent'
      },
      {
        description: 'Monitor task progress',
        command: 'claude-flow task status task-001',
        explanation: 'Shows detailed status and progress information for a task'
      },
      {
        description: 'Cancel a running task',
        command: 'claude-flow task cancel task-001 --reason "Requirements changed"',
        explanation: 'Stops a task and provides a reason for cancellation'
      }
    ],
    tutorial: [
      'Tasks are units of work that agents execute. Key concepts:',
      '',
      'Task Properties:',
      '• ID: Unique identifier',
      '• Type: Category of work (research, implementation, analysis, etc.)',
      '• Description: What needs to be done',
      '• Priority: Execution order (0-10, higher = more urgent)',
      '• Dependencies: Tasks that must complete first',
      '• Input: Data needed by the task',
      '• Status: Current state (pending, running, completed, failed)',
      '',
      'Task Lifecycle:',
      '1. Created (pending status)',
      '2. Queued (waiting for agent)',
      '3. Assigned (agent selected)',
      '4. Running (actively being worked on)',
      '5. Completed/Failed (final state)',
      '',
      'Task Dependencies:',
      '• Tasks can depend on other tasks',
      '• Dependencies must complete before task starts',
      '• Use for sequential workflows',
      '• Circular dependencies are not allowed'
    ],
    related: ['agents', 'workflows', 'coordination']
  },
  {
    name: 'claude',
    description: 'Spawning Claude instances with specific configurations',
    category: 'basic',
    examples: [
      {
        description: 'Spawn Claude with web research capabilities',
        command: 'claude-flow claude spawn "implement user authentication" --research --parallel',
        explanation: 'Creates a Claude instance with WebFetchTool and BatchTool for parallel web research'
      },
      {
        description: 'Spawn Claude without permission prompts',
        command: 'claude-flow claude spawn "fix payment bug" --no-permissions',
        explanation: 'Runs Claude with --dangerously-skip-permissions flag to avoid interruptions'
      },
      {
        description: 'Spawn Claude with custom tools',
        command: 'claude-flow claude spawn "analyze codebase" --tools "View,Edit,GrepTool,LS"',
        explanation: 'Specifies exactly which tools Claude can use for the task'
      },
      {
        description: 'Spawn Claude with test coverage target',
        command: 'claude-flow claude spawn "write unit tests" --coverage 95 --commit feature',
        explanation: 'Sets test coverage goal to 95% and commits after each feature'
      },
      {
        description: 'Dry run to preview command',
        command: 'claude-flow claude spawn "build API" --mode backend-only --dry-run',
        explanation: 'Shows what would be executed without actually running Claude'
      }
    ],
    tutorial: [
      'The claude spawn command launches Claude instances with specific configurations.',
      '',
      'Available Options:',
      '• --tools, -t: Specify allowed tools (default: View,Edit,Replace,GlobTool,GrepTool,LS,Bash)',
      '• --no-permissions: Skip permission prompts with --dangerously-skip-permissions',
      '• --config, -c: Path to MCP configuration file',
      '• --mode, -m: Development mode (full, backend-only, frontend-only, api-only)',
      '• --parallel: Enable BatchTool and dispatch_agent for parallel execution',
      '• --research: Enable WebFetchTool for web research capabilities',
      '• --coverage: Test coverage target percentage (default: 80)',
      '• --commit: Commit frequency (phase, feature, manual)',
      '• --verbose, -v: Enable verbose output',
      '• --dry-run, -d: Preview what would be executed',
      '',
      'Environment Variables Set:',
      '• CLAUDE_INSTANCE_ID: Unique identifier for the Claude instance',
      '• CLAUDE_FLOW_MODE: Development mode setting',
      '• CLAUDE_FLOW_COVERAGE: Target test coverage percentage',
      '• CLAUDE_FLOW_COMMIT: Commit frequency setting',
      '',
      'Common Use Cases:',
      '• Full-stack development: --mode full --parallel',
      '• API development: --mode backend-only --coverage 90',
      '• Bug fixing: --no-permissions --verbose',
      '• Research tasks: --research --parallel',
      '• Test writing: --coverage 95 --commit feature'
    ],
    related: ['agents', 'tasks', 'workflows']
  },
  {
    name: 'workflows',
    description: 'Building complex multi-step workflows',
    category: 'workflow',
    examples: [
      {
        description: 'Run a workflow from file',
        command: 'claude-flow workflow run research-pipeline.json --watch',
        explanation: 'Executes a workflow definition and monitors progress in real-time'
      },
      {
        description: 'Validate workflow before running',
        command: 'claude-flow workflow validate my-workflow.json --strict',
        explanation: 'Checks workflow syntax and dependencies without executing'
      },
      {
        description: 'Generate workflow template',
        command: 'claude-flow workflow template research --output research-workflow.json',
        explanation: 'Creates a pre-configured workflow template for research tasks'
      },
      {
        description: 'Monitor running workflows',
        command: 'claude-flow workflow list --all',
        explanation: 'Shows all workflows including completed ones'
      },
      {
        description: 'Stop a running workflow',
        command: 'claude-flow workflow stop workflow-001 --force',
        explanation: 'Immediately stops all tasks in a workflow'
      }
    ],
    tutorial: [
      'Workflows orchestrate multiple tasks and agents. Structure:',
      '',
      'Workflow Definition (JSON):',
      '{',
      '  "name": "Research and Analysis",',
      '  "description": "Multi-stage research workflow",',
      '  "agents": [',
      '    {"id": "researcher", "type": "researcher"},',
      '    {"id": "analyzer", "type": "analyst"}',
      '  ],',
      '  "tasks": [',
      '    {',
      '      "id": "research-task",',
      '      "type": "research",',
      '      "description": "Gather information",',
      '      "assignTo": "researcher"',
      '    },',
      '    {',
      '      "id": "analyze-task",',
      '      "type": "analysis",',
      '      "description": "Analyze findings",',
      '      "assignTo": "analyzer",',
      '      "depends": ["research-task"]',
      '    }',
      '  ]',
      '}',
      '',
      'Workflow Features:',
      '• Variable substitution: ${variable}',
      '• Conditional execution',
      '• Parallel task execution',
      '• Error handling and retries',
      '• Progress monitoring',
      '',
      'Best Practices:',
      '• Start with simple workflows',
      '• Use descriptive task names',
      '• Plan dependencies carefully',
      '• Test with --dry-run first'
    ],
    related: ['tasks', 'agents', 'templates']
  },
  {
    name: 'configuration',
    description: 'Configuring Claude-Flow settings',
    category: 'configuration',
    examples: [
      {
        description: 'Initialize default configuration',
        command: 'claude-flow config init --template development',
        explanation: 'Creates a configuration file optimized for development'
      },
      {
        description: 'View current configuration',
        command: 'claude-flow config show --diff',
        explanation: 'Shows only settings that differ from defaults'
      },
      {
        description: 'Update a setting',
        command: 'claude-flow config set orchestrator.maxConcurrentAgents 20',
        explanation: 'Changes the maximum number of concurrent agents'
      },
      {
        description: 'Save configuration profile',
        command: 'claude-flow config profile save production',
        explanation: 'Saves current settings as a named profile'
      },
      {
        description: 'Load configuration profile',
        command: 'claude-flow config profile load development',
        explanation: 'Switches to a previously saved configuration profile'
      }
    ],
    tutorial: [
      'Configuration controls all aspects of Claude-Flow behavior.',
      '',
      'Main Configuration Sections:',
      '',
      '• orchestrator: Core system settings',
      '  - maxConcurrentAgents: How many agents can run simultaneously',
      '  - taskQueueSize: Maximum pending tasks',
      '  - healthCheckInterval: How often to check system health',
      '',
      '• terminal: Terminal integration settings',
      '  - type: Terminal type (auto, vscode, native)',
      '  - poolSize: Number of terminal sessions to maintain',
      '',
      '• memory: Memory management settings',
      '  - backend: Storage type (sqlite, markdown, hybrid)',
      '  - cacheSizeMB: Memory cache size',
      '  - retentionDays: How long to keep data',
      '',
      '• mcp: Model Context Protocol settings',
      '  - transport: Communication method (stdio, http)',
      '  - port: Network port for HTTP transport',
      '',
      'Configuration Files:',
      '• Global: ~/.claude-flow/config.json',
      '• Project: ./claude-flow.config.json',
      '• Profiles: ~/.claude-flow/profiles/',
      '',
      'Environment Variables:',
      '• CLAUDE_FLOW_LOG_LEVEL: Override log level',
      '• CLAUDE_FLOW_MAX_AGENTS: Override agent limit',
      '• CLAUDE_FLOW_MCP_PORT: Override MCP port'
    ],
    related: ['profiles', 'environment', 'troubleshooting']
  },
  {
    name: 'monitoring',
    description: 'Monitoring system health and performance',
    category: 'advanced',
    examples: [
      {
        description: 'Check system status',
        command: 'claude-flow status --watch',
        explanation: 'Continuously monitors system health and updates every few seconds'
      },
      {
        description: 'Start monitoring dashboard',
        command: 'claude-flow monitor --interval 5',
        explanation: 'Opens a live dashboard with real-time metrics and graphs'
      },
      {
        description: 'View component-specific status',
        command: 'claude-flow status --component orchestrator',
        explanation: 'Shows detailed status for a specific system component'
      },
      {
        description: 'Monitor in compact mode',
        command: 'claude-flow monitor --compact --no-graphs',
        explanation: 'Simplified monitoring view without visual graphs'
      }
    ],
    tutorial: [
      'Claude-Flow provides comprehensive monitoring capabilities.',
      '',
      'Monitoring Commands:',
      '• status: Point-in-time system status',
      '• monitor: Live dashboard with continuous updates',
      '',
      'Key Metrics:',
      '• System Health: Overall status (healthy/degraded/unhealthy)',
      '• Resource Usage: CPU, memory, agent count',
      '• Component Status: Individual system components',
      '• Agent Activity: Active agents and their tasks',
      '• Task Queue: Pending and completed tasks',
      '• Performance Graphs: Historical trends',
      '',
      'Monitoring Best Practices:',
      '• Check status before starting large workflows',
      '• Monitor during heavy usage',
      '• Watch for resource exhaustion',
      '• Track task completion rates',
      '• Set up alerts for critical issues',
      '',
      'Troubleshooting with Monitoring:',
      '• High CPU: Too many concurrent tasks',
      '• High Memory: Large cache or memory leaks',
      '• Failed Tasks: Agent or system issues',
      '• Slow Performance: Resource constraints'
    ],
    related: ['status', 'performance', 'troubleshooting']
  },
  {
    name: 'sessions',
    description: 'Managing sessions and state persistence',
    category: 'advanced',
    examples: [
      {
        description: 'Save current session',
        command: 'claude-flow session save "Development Session" --description "Working on API integration"',
        explanation: 'Saves all current agents, tasks, and memory state'
      },
      {
        description: 'List saved sessions',
        command: 'claude-flow session list',
        explanation: 'Shows all saved sessions with creation dates and metadata'
      },
      {
        description: 'Restore a session',
        command: 'claude-flow session restore session-001 --merge',
        explanation: 'Restores session state, merging with current state'
      },
      {
        description: 'Export session to file',
        command: 'claude-flow session export session-001 backup.json --include-memory',
        explanation: 'Creates a portable backup including agent memory'
      },
      {
        description: 'Clean up old sessions',
        command: 'claude-flow session clean --older-than 30 --dry-run',
        explanation: 'Shows what sessions would be deleted (older than 30 days)'
      }
    ],
    tutorial: [
      'Sessions capture the complete state of your Claude-Flow environment.',
      '',
      'What Sessions Include:',
      '• All active agents and their configurations',
      '• Current task queue and status',
      '• Agent memory and conversation history',
      '• System configuration snapshot',
      '',
      'Session Use Cases:',
      '• Save work-in-progress',
      '• Share team configurations',
      '• Backup before major changes',
      '• Reproduce issues for debugging',
      '• Switch between projects',
      '',
      'Session Management:',
      '• Automatic checksums for integrity',
      '• Compression for large sessions',
      '• Selective restore (agents only, tasks only)',
      '• Version compatibility checking',
      '',
      'Best Practices:',
      '• Save sessions before major changes',
      '• Use descriptive names and tags',
      '• Regular cleanup of old sessions',
      '• Export important sessions as backups',
      '• Test restore before relying on sessions'
    ],
    related: ['backup', 'state', 'persistence']
  },
  {
    name: 'repl',
    description: 'Using the interactive REPL mode',
    category: 'basic',
    examples: [
      {
        description: 'Start REPL mode',
        command: 'claude-flow repl',
        explanation: 'Opens interactive command line with tab completion'
      },
      {
        description: 'REPL with custom history file',
        command: 'claude-flow repl --history-file .my-history',
        explanation: 'Uses a specific file for command history'
      },
      {
        description: 'Skip welcome banner',
        command: 'claude-flow repl --no-banner',
        explanation: 'Starts REPL in minimal mode'
      }
    ],
    tutorial: [
      'The REPL (Read-Eval-Print Loop) provides an interactive interface.',
      '',
      'REPL Features:',
      '• Tab completion for commands and arguments',
      '• Command history (up/down arrows)',
      '• Real-time connection status',
      '• Built-in help system',
      '• Command aliases and shortcuts',
      '',
      'Special REPL Commands:',
      '• help: Show available commands',
      '• status: Check system status',
      '• connect: Connect to orchestrator',
      '• history: View command history',
      '• clear: Clear screen',
      '• cd/pwd: Navigate directories',
      '',
      'REPL Tips:',
      '• Use tab completion extensively',
      '• Check connection status regularly',
      '• Use "help <command>" for detailed help',
      '• History is saved between sessions',
      '• Ctrl+C or "exit" to quit'
    ],
    related: ['completion', 'interactive', 'commands']
  },
  {
    name: 'troubleshooting',
    description: 'Diagnosing and fixing common issues',
    category: 'troubleshooting',
    examples: [
      {
        description: 'Check system health',
        command: 'claude-flow status --component all',
        explanation: 'Comprehensive health check of all components'
      },
      {
        description: 'Enable debug logging',
        command: 'claude-flow start --log-level debug',
        explanation: 'Start with verbose logging for debugging'
      },
      {
        description: 'Validate configuration',
        command: 'claude-flow config validate claude-flow.config.json --strict',
        explanation: 'Check configuration file for errors'
      },
      {
        description: 'Reset to defaults',
        command: 'claude-flow config reset --confirm',
        explanation: 'Restore default configuration settings'
      }
    ],
    tutorial: [
      'Common issues and solutions:',
      '',
      'Connection Issues:',
      '• Problem: "Connection refused" errors',
      '• Solution: Ensure Claude-Flow is started with "claude-flow start"',
      '• Check: MCP transport settings match between client and server',
      '',
      'Agent Issues:',
      '• Problem: Agents not spawning',
      '• Solution: Check agent limits in configuration',
      '• Check: Available system resources',
      '',
      'Task Issues:',
      '• Problem: Tasks stuck in pending state',
      '• Solution: Verify agent availability and task dependencies',
      '• Check: Task queue size limits',
      '',
      'Performance Issues:',
      '• Problem: Slow response times',
      '• Solution: Reduce concurrent agents or increase resources',
      '• Check: Memory usage and cache settings',
      '',
      'Configuration Issues:',
      '• Problem: Settings not taking effect',
      '• Solution: Validate configuration file syntax',
      '• Check: Environment variable overrides',
      '',
      'Debug Commands:',
      '• claude-flow status: System health check',
      '• claude-flow config validate: Configuration check',
      '• claude-flow --verbose: Enable detailed logging',
      '• claude-flow monitor: Real-time diagnostics'
    ],
    related: ['monitoring', 'configuration', 'debugging']
  }
];

function showMainHelp(): void {
  console.log(colors.cyan.bold('Claude-Flow Help System'));
  console.log('─'.repeat(50));
  console.log();
  
  console.log(colors.white('Claude-Flow is an advanced AI agent orchestration system.'));
  console.log(colors.white('Use this help system to learn about features and best practices.'));
  console.log();
  
  console.log(colors.yellow.bold('Quick Start:'));
  console.log(colors.gray('  claude-flow help getting-started    # Beginner tutorial'));
  console.log(colors.gray('  claude-flow help --interactive      # Interactive help mode'));
  console.log(colors.gray('  claude-flow help <topic>            # Specific topic help'));
  console.log();
  
  console.log(colors.yellow.bold('Help Categories:'));
  
  const categories = {
    basic: 'Essential concepts and commands',
    workflow: 'Building and managing workflows',
    configuration: 'System configuration and profiles',
    advanced: 'Advanced features and monitoring',
    troubleshooting: 'Problem diagnosis and solutions'
  };
  
  for (const [category, description] of Object.entries(categories)) {
    console.log();
    console.log(colors.cyan.bold(`${category.toUpperCase()}:`));
    console.log(colors.white(`  ${description}`));
    
    const topics = HELP_TOPICS.filter(t => t.category === category);
    for (const topic of topics) {
      console.log(colors.gray(`    ${topic.name.padEnd(20)} ${topic.description}`));
    }
  }
  
  console.log();
  console.log(colors.gray('Use "claude-flow help <topic>" for detailed information.'));
  console.log(colors.gray('Use "claude-flow help --all" to see all topics.'));
}

function showAllTopics(): void {
  console.log(colors.cyan.bold('All Help Topics'));
  console.log('─'.repeat(50));
  
  const table = new HelpTable()
    .header(['Topic', 'Category', 'Description'])
    .border(true);

  for (const topic of HELP_TOPICS) {
    table.push([
      colors.cyan.bold(topic.name),
      colors.yellow.bold(topic.category),
      topic.description
    ]);
  }
  
  table.render();
  
  console.log();
  console.log(colors.gray('Use "claude-flow help <topic>" for detailed information.'));
}

async function showTopicHelp(topicName: string, options: { examples?: boolean; tutorial?: boolean }): Promise<void> {
  const topic = HELP_TOPICS.find(t => t.name === topicName);
  
  if (!topic) {
    console.log(colors.red(`Help topic '${topicName}' not found.`));
    console.log();
    
    // Suggest similar topics
    const similar = HELP_TOPICS.filter(t => 
      t.name.includes(topicName) || 
      t.description.toLowerCase().includes(topicName.toLowerCase())
    );
    
    if (similar.length > 0) {
      console.log(colors.gray('Did you mean:'));
      for (const suggestion of similar) {
        console.log(colors.cyan(`  ${suggestion.name}`));
      }
    } else {
      console.log(colors.gray('Use "claude-flow help --all" to see all topics.'));
    }
    return;
  }
  
  console.log(colors.cyan.bold(`Help: ${topic.name}`));
  console.log('─'.repeat(50));
  console.log(colors.white(topic.description));
  console.log();
  
  if (options.tutorial && topic.tutorial) {
    console.log(colors.yellow.bold('Tutorial:'));
    console.log('─'.repeat(20));
    for (const line of topic.tutorial) {
      if (line.trim().startsWith('claude-flow')) {
        console.log(colors.cyan(`  ${line}`));
      } else if (line.trim() === '') {
        console.log();
      } else {
        console.log(colors.white(line));
      }
    }
    console.log();
  }
  
  if (options.examples && topic.examples) {
    console.log(colors.yellow.bold('Examples:'));
    console.log('─'.repeat(20));
    for (const example of topic.examples) {
      console.log(colors.white.bold(`${example.description}:`));
      console.log(colors.cyan(`  ${example.command}`));
      if (example.explanation) {
        console.log(colors.gray(`  ${example.explanation}`));
      }
      console.log();
    }
  }
  
  if (!options.examples && !options.tutorial) {
    // Show both by default
    if (topic.tutorial) {
      console.log(colors.yellow.bold('Overview:'));
      console.log('─'.repeat(20));
      const overview = topic.tutorial.slice(0, 5);
      for (const line of overview) {
        if (line.trim() === '') {
          console.log();
        } else {
          console.log(colors.white(line));
        }
      }
      console.log();
      console.log(colors.gray('Use --tutorial for complete tutorial.'));
      console.log();
    }
    
    if (topic.examples) {
      console.log(colors.yellow.bold('Common Examples:'));
      console.log('─'.repeat(20));
      const commonExamples = topic.examples.slice(0, 3);
      for (const example of commonExamples) {
        console.log(colors.cyan(`  ${example.command}`));
        console.log(colors.gray(`    ${example.description}`));
      }
      if (topic.examples.length > 3) {
        console.log(colors.gray(`    ... and ${topic.examples.length - 3} more`));
      }
      console.log();
      console.log(colors.gray('Use --examples for all examples.'));
      console.log();
    }
  }
  
  if (topic.related && topic.related.length > 0) {
    console.log(colors.yellow.bold('Related Topics:'));
    console.log('─'.repeat(20));
    for (const related of topic.related) {
      console.log(colors.cyan(`  claude-flow help ${related}`));
    }
    console.log();
  }
}

async function startInteractiveHelp(): Promise<void> {
  try {
    console.log(colors.cyan.bold('Interactive Help Mode'));
    console.log('─'.repeat(30));
    console.log();
    
    while (true) {
      const categories = [
        { name: 'Getting Started', value: 'getting-started' },
        { name: 'Agents', value: 'agents' },
        { name: 'Tasks', value: 'tasks' },
        { name: 'Workflows', value: 'workflows' },
        { name: 'Configuration', value: 'configuration' },
        { name: 'Monitoring', value: 'monitoring' },
        { name: 'Sessions', value: 'sessions' },
        { name: 'REPL Mode', value: 'repl' },
        { name: 'Troubleshooting', value: 'troubleshooting' },
        { name: 'Browse All Topics', value: 'all' },
        { name: 'Exit', value: 'exit' }
      ];
      
      const choice = await HelpSelect.prompt<string>({
        message: 'What would you like help with?',
        options: categories,
      });
      
      if (choice === 'exit') {
        console.log(colors.gray('Goodbye!'));
        break;
      }
      
      console.log();
      
      if (choice === 'all') {
        showAllTopics();
      } else {
        await showTopicHelp(choice, { tutorial: true, examples: true });
      }
      
      console.log();
      console.log(colors.gray('Press Enter to continue...'));
      await new Promise<void>(resolve => {
        const stdin = stdio.stdin;
        if (typeof stdin.read === 'function') {
          // Deno-style API
          const buffer = new Uint8Array(1);
          stdin.read(buffer).then(() => resolve());
        } else if (typeof stdin.once === 'function') {
          // Node.js-style API
          stdin.once('data', () => resolve());
          stdin.resume();
        } else {
          // Fallback - just wait a bit
          setTimeout(() => resolve(), 100);
        }
      });
      
      console_.clear();
    }
  } catch (error) {
    console.error(colors.red('Error in interactive help:'), error instanceof Error ? error.message : String(error));
    throw error;
  }
}