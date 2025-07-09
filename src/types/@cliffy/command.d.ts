// Enhanced stub types for @cliffy/command
declare module '@cliffy/command' {
  interface IOption {
    flags: string;
    description: string;
    required?: boolean;
    optional?: boolean;
    variadic?: boolean;
    default?: any;
    choices?: string[];
    collect?: boolean;
    value?: any;
    conflicts?: string[];
    depends?: string[];
    standalone?: boolean;
    hidden?: boolean;
    global?: boolean;
    prepend?: boolean;
    args?: any[];
    list?: boolean;
    separator?: string;
    type?: string;
    // Add more option properties as needed
  }

  interface IArgument {
    name: string;
    description?: string;
    required?: boolean;
    optional?: boolean;
    variadic?: boolean;
    type?: string;
    choices?: string[];
    default?: any;
    collect?: boolean;
    value?: any;
    list?: boolean;
    separator?: string;
    args?: any[];
  }

  interface IAction {
    (options: any, ...args: any[]): void | Promise<void>;
  }

  interface ICommandOptions {
    description?: string;
    examples?: string[];
    arguments?: IArgument[];
    options?: IOption[];
    commands?: Command[];
    action?: IAction;
    globalOptions?: boolean;
    noGlobals?: boolean;
    hidden?: boolean;
    executable?: boolean;
    standalone?: boolean;
    version?: string;
    versionOption?: string | boolean;
    help?: boolean;
    helpOption?: string | boolean;
    throwErrors?: boolean;
    stopEarly?: boolean;
    allowEmpty?: boolean;
    allowExcess?: boolean;
    collect?: boolean;
    default?: any;
    required?: boolean;
    optional?: boolean;
    variadic?: boolean;
    prepend?: boolean;
    complete?: (cmd: Command, parent?: Command) => string[] | Promise<string[]>;
    fn?: IAction;
    args?: any[];
    meta?: any;
    name?: string;
    path?: string;
    opts?: any;
    parent?: Command;
    children?: Command[];
    isExecutable?: boolean;
    isStandalone?: boolean;
    isHidden?: boolean;
    isAction?: boolean;
    isGlobal?: boolean;
    isOption?: boolean;
    isArgument?: boolean;
    isCommand?: boolean;
    isHelp?: boolean;
    isVersion?: boolean;
    isCompletion?: boolean;
    isExample?: boolean;
    isEnvironment?: boolean;
    isType?: boolean;
    isDefault?: boolean;
    isRequired?: boolean;
    isOptional?: boolean;
    isVariadic?: boolean;
    isConflicts?: boolean;
    isDepends?: boolean;
    isCollect?: boolean;
    isList?: boolean;
    isValue?: boolean;
    isFlags?: boolean;
    isDescription?: boolean;
    isChoice?: boolean;
    isChoices?: boolean;
    isSeparator?: boolean;
    isPrepend?: boolean;
    isComplete?: boolean;
    isFn?: boolean;
    isArgs?: boolean;
    isMeta?: boolean;
    isName?: boolean;
    isPath?: boolean;
    isOpts?: boolean;
    isParent?: boolean;
    isChildren?: boolean;
  }

  export class Command {
    constructor(options?: ICommandOptions);
    constructor(name?: string, description?: string);
    
    // Configuration methods
    name(name: string): this;
    version(version: string, flags?: string, description?: string): this;
    description(desc: string): this;
    usage(usage: string): this;
    example(name: string, description: string): this;
    examples(examples: Array<{ name: string; description: string }>): this;
    
    // Options
    option(flags: string, description: string, defaultValue?: any): this;
    option(flags: string, description: string, options: { default?: any; required?: boolean; collect?: boolean; value?: any; choices?: string[]; conflicts?: string[]; depends?: string[]; standalone?: boolean; hidden?: boolean; global?: boolean; prepend?: boolean; args?: any[]; list?: boolean; separator?: string; type?: string }): this;
    option(flags: string, description: string, fn: (value: any, previous: any) => any, defaultValue?: any): this;
    
    globalOption(flags: string, description: string, defaultValue?: any): this;
    globalOption(flags: string, description: string, options: { default?: any; required?: boolean; collect?: boolean; value?: any; choices?: string[]; conflicts?: string[]; depends?: string[]; standalone?: boolean; hidden?: boolean; global?: boolean; prepend?: boolean; args?: any[]; list?: boolean; separator?: string; type?: string }): this;
    globalOption(flags: string, description: string, fn: (value: any, previous: any) => any, defaultValue?: any): this;
    
    // Arguments
    arguments(desc: string): this;
    argument(name: string, description?: string, defaultValue?: any): this;
    argument(name: string, description: string, options: { default?: any; required?: boolean; optional?: boolean; variadic?: boolean; type?: string; choices?: string[]; collect?: boolean; value?: any; list?: boolean; separator?: string; args?: any[] }): this;
    argument(name: string, description: string, fn: (value: any, previous: any) => any, defaultValue?: any): this;
    
    // Commands
    command(name: string, cmd?: Command): Command;
    command(name: string, description?: string): Command;
    command(name: string, description: string, options: ICommandOptions): Command;
    
    // Actions
    action(fn: IAction): this;
    
    // Parsing
    parse(args?: string[], options?: { from?: string; run?: boolean }): Promise<any>;
    parseOptions(args: string[]): any;
    parseArguments(args: string[]): any;
    
    // Help
    showHelp(): void;
    outputHelp(): void;
    help(command?: string): string;
    helpInformation(): string;
    getHelp(): string;
    
    // Completion
    complete(complete: (cmd: Command, parent?: Command) => string[] | Promise<string[]>): this;
    
    // Utilities
    getName(): string;
    getDescription(): string;
    getUsage(): string;
    getVersion(): string;
    getExamples(): Array<{ name: string; description: string }>;
    getOptions(): IOption[];
    getArguments(): IArgument[];
    getCommands(): Command[];
    getGlobalOptions(): IOption[];
    getOption(name: string): IOption | undefined;
    getArgument(name: string): IArgument | undefined;
    getCommand(name: string): Command | undefined;
    hasOption(name: string): boolean;
    hasArgument(name: string): boolean;
    hasCommand(name: string): boolean;
    
    // State
    isExecutable(): boolean;
    isStandalone(): boolean;
    isHidden(): boolean;
    isAction(): boolean;
    isGlobal(): boolean;
    
    // Error handling
    throwErrors(throwErrors?: boolean): this;
    
    // Configuration
    stopEarly(stopEarly?: boolean): this;
    allowEmpty(allowEmpty?: boolean): this;
    allowExcess(allowExcess?: boolean): this;
    
    // Environment
    env(envVars: Record<string, string>): this;
    
    // Meta
    meta(meta: any): this;
    getMeta(): any;
    
    // Parent/Child relationships
    getParent(): Command | undefined;
    getChildren(): Command[];
    
    // Execution
    execute(options?: any, ...args: any[]): Promise<any>;
    
    // Types
    type(name: string, type: any): this;
    
    // Reset
    reset(): this;
    
    // Clone
    clone(): Command;
    
    // Validation
    validate(): this;
    
    // Hook
    hook(event: string, fn: (options: any, ...args: any[]) => void | Promise<void>): this;
    
    // Events
    on(event: string, listener: (...args: any[]) => void): this;
    off(event: string, listener: (...args: any[]) => void): this;
    once(event: string, listener: (...args: any[]) => void): this;
    emit(event: string, ...args: any[]): boolean;
    
    // Options and arguments arrays
    options: IOption[];
    arguments: IArgument[];
    commands: Command[];
    globalOptions: IOption[];
    
    // Properties
    readonly name: string;
    readonly description: string;
    readonly usage: string;
    readonly version: string;
    readonly examples: Array<{ name: string; description: string }>;
    readonly parent: Command | undefined;
    readonly children: Command[];
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
  }
  
  // Additional exports
  export interface HelpOptions {
    colors?: boolean;
    compact?: boolean;
    long?: boolean;
    indent?: number;
    lineLength?: number;
    maxWidth?: number;
    sort?: boolean;
    hint?: boolean;
    hideDefaults?: boolean;
    hideExamples?: boolean;
    hideOptions?: boolean;
    hideCommands?: boolean;
    hideArguments?: boolean;
    hideEnvironment?: boolean;
    hideVersion?: boolean;
    hideHelp?: boolean;
    hideCompletion?: boolean;
    showGlobalOptions?: boolean;
    showVersion?: boolean;
    showHelp?: boolean;
    showCompletion?: boolean;
    showEnvironment?: boolean;
    showExamples?: boolean;
    showOptions?: boolean;
    showCommands?: boolean;
    showArguments?: boolean;
    showDefaults?: boolean;
    showHint?: boolean;
    showSort?: boolean;
    showColors?: boolean;
    showCompact?: boolean;
    showLong?: boolean;
    showIndent?: boolean;
    showLineLength?: boolean;
    showMaxWidth?: boolean;
  }
  
  export interface CompleteOptions {
    complete?: (cmd: Command, parent?: Command) => string[] | Promise<string[]>;
    args?: any[];
    meta?: any;
    name?: string;
    path?: string;
    opts?: any;
    parent?: Command;
    children?: Command[];
  }
  
  export interface ParseOptions {
    from?: string;
    run?: boolean;
    args?: string[];
    options?: any;
    arguments?: any;
    commands?: Command[];
    globalOptions?: IOption[];
    meta?: any;
    env?: Record<string, string>;
    types?: Record<string, any>;
    hooks?: Record<string, Array<(options: any, ...args: any[]) => void | Promise<void>>>;
    listeners?: Record<string, Array<(...args: any[]) => void>>;
  }
  
  export interface ValidationError extends Error {
    name: 'ValidationError';
    message: string;
    cmd: Command;
    args: string[];
    options: any;
    arguments: any;
  }
  
  export interface CommandError extends Error {
    name: 'CommandError';
    message: string;
    cmd: Command;
    args: string[];
    options: any;
    arguments: any;
    exitCode?: number;
  }
  
  // Utility functions
  export function command(name?: string, description?: string): Command;
  export function parseArgs(args: string[]): any;
  export function parseOptions(args: string[]): any;
  export function parseArguments(args: string[]): any;
  export function help(command: Command, options?: HelpOptions): string;
  export function complete(command: Command, options?: CompleteOptions): string[] | Promise<string[]>;
  export function validate(command: Command): void;
  export function execute(command: Command, options?: any, ...args: any[]): Promise<any>;
  export function hook(event: string, fn: (options: any, ...args: any[]) => void | Promise<void>): void;
  export function on(event: string, listener: (...args: any[]) => void): void;
  export function off(event: string, listener: (...args: any[]) => void): void;
  export function once(event: string, listener: (...args: any[]) => void): void;
  export function emit(event: string, ...args: any[]): boolean;
  export function reset(): void;
  export function clone(command: Command): Command;
  export function type(name: string, type: any): void;
  export function env(envVars: Record<string, string>): void;
  export function meta(meta: any): void;
  export function getMeta(): any;
  export function getParent(): Command | undefined;
  export function getChildren(): Command[];
  export function getOptions(): IOption[];
  export function getArguments(): IArgument[];
  export function getCommands(): Command[];
  export function getGlobalOptions(): IOption[];
  export function getOption(name: string): IOption | undefined;
  export function getArgument(name: string): IArgument | undefined;
  export function getCommand(name: string): Command | undefined;
  export function hasOption(name: string): boolean;
  export function hasArgument(name: string): boolean;
  export function hasCommand(name: string): boolean;
  export function isExecutable(): boolean;
  export function isStandalone(): boolean;
  export function isHidden(): boolean;
  export function isAction(): boolean;
  export function isGlobal(): boolean;
  export function throwErrors(throwErrors?: boolean): void;
  export function stopEarly(stopEarly?: boolean): void;
  export function allowEmpty(allowEmpty?: boolean): void;
  export function allowExcess(allowExcess?: boolean): void;
  export function getName(): string;
  export function getDescription(): string;
  export function getUsage(): string;
  export function getVersion(): string;
  export function getExamples(): Array<{ name: string; description: string }>;
  export function getHelp(): string;
  export function showHelp(): void;
  export function outputHelp(): void;
  export function helpInformation(): string;
}