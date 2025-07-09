/**
 * Command compatibility layer to resolve conflicts between @cliffy/command and commander.js
 * This provides unified types and interfaces for command handling across the codebase
 */

// Common command interface that both libraries can implement
export interface BaseCommand {
  name(name: string): this;
  description(desc: string): this;
  version(version: string, flags?: string, description?: string): this;
  usage(usage: string): this;
  option(flags: string, description: string, defaultValue?: any): this;
  arguments(desc: string): this;
  action(fn: (...args: any[]) => void | Promise<void>): this;
  parse(args?: string[], options?: any): Promise<any> | any;
  help(command?: string): string;
  showHelp(): void;
  execute?(options?: any, ...args: any[]): Promise<any>;
}

// Type guards to determine which Command type we're dealing with
export function isCliffyCommand(cmd: any): cmd is import('@cliffy/command').Command {
  return cmd && typeof cmd.getName === 'function' && typeof cmd.getDescription === 'function';
}

export function isCommanderCommand(cmd: any): cmd is import('commander').Command {
  return cmd && typeof cmd.opts === 'function' && Array.isArray(cmd.args);
}

export function isMockCommand(cmd: any): cmd is MockCommand {
  return cmd && cmd.constructor && cmd.constructor.name === 'MockCommand';
}

// Mock command interface for compatibility
export interface MockCommand extends BaseCommand {
  getName(): string;
  getDescription(): string;
  getUsage(): string;
  getVersion(): string;
  getExamples(): Array<{ name: string; description: string }>;
  getOptions(): Array<any>;
  getArguments(): Array<any>;
  getCommands(): MockCommand[];
  getGlobalOptions(): Array<any>;
  getOption(name: string): any | undefined;
  getArgument(name: string): any | undefined;
  getCommand(name: string): MockCommand | undefined;
  hasOption(name: string): boolean;
  hasArgument(name: string): boolean;
  hasCommand(name: string): boolean;
  isExecutable(): boolean;
  isStandalone(): boolean;
  isHidden(): boolean;
  isAction(): boolean;
  isGlobal(): boolean;
  clone(): MockCommand;
  reset(): this;
  validate(): this;
}

// Unified command type that can be any of the supported command types
export type UnifiedCommand = 
  | import('@cliffy/command').Command 
  | import('commander').Command 
  | MockCommand;

// Command factory function to create appropriate command type
export function createCommand(name?: string): UnifiedCommand {
  // Try to create @cliffy command first (preferred)
  try {
    const { Command } = require('@cliffy/command');
    return new Command(name);
  } catch (error) {
    // Fall back to commander.js
    try {
      const { Command } = require('commander');
      const cmd = new Command(name);
      return cmd;
    } catch (error) {
      // Fall back to MockCommand for compatibility
      const MockCommand = require('../cli/commands/help').MockCommand;
      const cmd = new MockCommand();
      if (name) cmd.name(name);
      return cmd;
    }
  }
}

// Command adapter to provide consistent interface across different command types
export class CommandAdapter {
  private command: UnifiedCommand;

  constructor(command: UnifiedCommand) {
    this.command = command;
  }

  getName(): string {
    if (isCliffyCommand(this.command)) {
      return this.command.getName();
    } else if (isCommanderCommand(this.command)) {
      return this.command.name();
    } else if (isMockCommand(this.command)) {
      return this.command.getName();
    }
    return '';
  }

  getDescription(): string {
    if (isCliffyCommand(this.command)) {
      return this.command.getDescription();
    } else if (isCommanderCommand(this.command)) {
      return this.command.description();
    } else if (isMockCommand(this.command)) {
      return this.command.getDescription();
    }
    return '';
  }

  setName(name: string): this {
    this.command.name(name);
    return this;
  }

  setDescription(desc: string): this {
    this.command.description(desc);
    return this;
  }

  addOption(flags: string, description: string, defaultValue?: any): this {
    this.command.option(flags, description, defaultValue);
    return this;
  }

  addArgument(desc: string): this {
    this.command.arguments(desc);
    return this;
  }

  setAction(fn: (...args: any[]) => void | Promise<void>): this {
    this.command.action(fn);
    return this;
  }

  async parse(args?: string[], options?: any): Promise<any> {
    if (typeof this.command.parse === 'function') {
      const result = this.command.parse(args, options);
      return result instanceof Promise ? result : Promise.resolve(result);
    }
    return Promise.resolve({});
  }

  showHelp(): void {
    if (typeof this.command.showHelp === 'function') {
      this.command.showHelp();
    } else if (typeof this.command.help === 'function') {
      console.log(this.command.help());
    }
  }

  getUnderlyingCommand(): UnifiedCommand {
    return this.command;
  }
}

// Utility functions for command handling
export function adaptCommand(command: UnifiedCommand): CommandAdapter {
  return new CommandAdapter(command);
}

export function ensureCommandCompatibility(command: any): UnifiedCommand {
  if (isCliffyCommand(command) || isCommanderCommand(command) || isMockCommand(command)) {
    return command;
  }
  
  // If it's not a recognized command type, wrap it in a mock command
  const MockCommand = require('../cli/commands/help').MockCommand;
  const mockCmd = new MockCommand();
  
  // Try to copy basic properties if they exist
  if (command.name && typeof command.name === 'string') {
    mockCmd.name(command.name);
  }
  if (command.description && typeof command.description === 'string') {
    mockCmd.description(command.description);
  }
  if (command.action && typeof command.action === 'function') {
    mockCmd.action(command.action);
  }
  
  return mockCmd;
}

// Type assertion helpers
export function assertCliffyCommand(cmd: any): asserts cmd is import('@cliffy/command').Command {
  if (!isCliffyCommand(cmd)) {
    throw new Error('Expected @cliffy/command Command instance');
  }
}

export function assertCommanderCommand(cmd: any): asserts cmd is import('commander').Command {
  if (!isCommanderCommand(cmd)) {
    throw new Error('Expected commander.js Command instance');
  }
}

export function assertMockCommand(cmd: any): asserts cmd is MockCommand {
  if (!isMockCommand(cmd)) {
    throw new Error('Expected MockCommand instance');
  }
}

// Export types for convenience
export type CliffyCommand = import('@cliffy/command').Command;
export type CommanderCommand = import('commander').Command;