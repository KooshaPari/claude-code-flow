// Commander.js compatibility types for resolving type mismatches
// This module provides enhanced types to fix common Commander.js compatibility issues

declare module 'commander' {
  interface CommanderError extends Error {
    code: string;
    exitCode: number;
    message: string;
    nestedError?: string;
  }

  interface Option {
    flags: string;
    description: string;
    required: boolean;
    optional: boolean;
    variadic: boolean;
    mandatory: boolean;
    short?: string;
    long?: string;
    negate: boolean;
    defaultValue?: any;
    defaultValueDescription?: string;
    presetArg?: any;
    envVar?: string;
    parseArg?: (value: string, previous: any) => any;
    hidden: boolean;
    argChoices?: string[];
  }

  interface CommandOptions {
    isDefault?: boolean;
    hidden?: boolean;
    noHelp?: boolean;
    executableFile?: string;
  }

  interface ExecutableCommandOptions extends CommandOptions {
    executableFile: string;
  }

  interface OptionValues {
    [key: string]: any;
  }

  interface Command {
    args: string[];
    commands: Command[];
    options: Option[];
    parent: Command | null;
    
    // Configuration methods
    command(nameAndArgs: string, opts?: CommandOptions): Command;
    command(nameAndArgs: string, description?: string, opts?: ExecutableCommandOptions): Command;
    alias(alias: string): Command;
    aliases(aliases: string[]): Command;
    description(str: string): Command;
    summary(str: string): Command;
    usage(str: string): Command;
    name(str: string): Command;
    nameFromFilename(filename: string): Command;
    
    // Arguments
    arguments(desc: string): Command;
    argument(name: string, description?: string, fn?: (value: string, previous: any) => any, defaultValue?: any): Command;
    createArgument(name: string, description?: string): any;
    
    // Options with proper overloads
    option(flags: string, description?: string, defaultValue?: any): Command;
    option(flags: string, description?: string, fn?: (value: string, previous: any) => any, defaultValue?: any): Command;
    option(flags: string, description: string, options: { default?: any; choices?: string[]; variadic?: boolean; mandatory?: boolean; hidden?: boolean; conflicts?: string[]; implies?: string[]; env?: string; preset?: any }): Command;
    
    requiredOption(flags: string, description?: string, defaultValue?: any): Command;
    requiredOption(flags: string, description?: string, fn?: (value: string, previous: any) => any, defaultValue?: any): Command;
    requiredOption(flags: string, description: string, options: { default?: any; choices?: string[]; variadic?: boolean; hidden?: boolean; conflicts?: string[]; implies?: string[]; env?: string; preset?: any }): Command;
    
    createOption(flags: string, description?: string): Option;
    addOption(option: Option): Command;
    
    // Action
    action(fn: (...args: any[]) => void | Promise<void>): Command;
    
    // Subcommands
    addCommand(cmd: Command, opts?: any): Command;
    
    // Help
    helpOption(flags?: string, description?: string): Command;
    addHelpText(position: string, text: string): Command;
    help(cb?: (str: string) => string): void;
    outputHelp(cb?: (str: string) => string): void;
    helpInformation(cb?: (str: string) => string): string;
    showHelp(): void;
    
    // Parse and execute
    parse(argv?: string[], options?: any): Command;
    parseAsync(argv?: string[], options?: any): Promise<Command>;
    opts(): OptionValues;
    optsWithGlobals(): OptionValues;
    getOptionValue(key: string): any;
    setOptionValue(key: string, value: any): Command;
    
    // Events
    on(event: string, listener: (...args: any[]) => void): Command;
    emit(event: string, ...args: any[]): boolean;
    
    // Utility
    version(str: string, flags?: string, description?: string): Command;
    allowUnknownOption(arg?: boolean): Command;
    allowExcessArguments(arg?: boolean): Command;
    enablePositionalOptions(arg?: boolean): Command;
    passThroughOptions(arg?: boolean): Command;
    storeOptionsAsProperties(arg?: boolean): Command;
    combineFlagAndOptionalValue(arg?: boolean): Command;
    
    // Exit handling
    exitOverride(fn?: (err: CommanderError) => never | void): Command;
    
    // Hook
    hook(event: string, listener: (thisCommand: Command, actionCommand: Command) => void | Promise<void>): Command;
    
    // Configuration
    configureHelp(configuration: any): Command;
    configureOutput(configuration: any): Command;
    copyInheritedSettings(sourceCommand: Command): Command;
    
    // Execution context
    createCommand(name?: string): Command;
    showHelpAfterError(displayHelp?: boolean | string): Command;
    showSuggestionAfterError(displaySuggestion?: boolean): Command;
  }

  class Command {
    constructor(name?: string);
    
    // Static methods
    static createCommand(name?: string): Command;
    static createOption(flags: string, description?: string): Option;
    static createArgument(name: string, description?: string): any;
  }

  function createCommand(name?: string): Command;
  function createOption(flags: string, description?: string): Option;
  function createArgument(name: string, description?: string): any;

  export { Command, createCommand, createOption, createArgument, CommanderError, Option, OptionValues, CommandOptions, ExecutableCommandOptions };
  export default Command;
}

// Additional compatibility fixes for specific Commander.js usage patterns
declare global {
  namespace Commander {
    interface Command {
      // Fix for showHelp method availability
      showHelp(): void;
      
      // Fix for option overloads with object parameters
      option(flags: string, description: string, options: { default?: any }): Command;
      
      // Fix for command overloads
      command(nameAndArgs: string, cmd: Command): Command;
    }
  }
}

// Type augmentation for better compatibility with existing code
declare module 'commander' {
  interface Command {
    // Ensure these methods are always available
    showHelp(): void;
    
    // Enhanced option method with all possible signatures
    option(flags: string): Command;
    option(flags: string, description: string): Command;
    option(flags: string, description: string, defaultValue: any): Command;
    option(flags: string, description: string, parseArg: (value: string, previous: any) => any): Command;
    option(flags: string, description: string, parseArg: (value: string, previous: any) => any, defaultValue: any): Command;
    option(flags: string, description: string, regexp: RegExp): Command;
    option(flags: string, description: string, regexp: RegExp, defaultValue: any): Command;
    option(flags: string, description: string, options: { 
      default?: any; 
      choices?: string[]; 
      variadic?: boolean; 
      mandatory?: boolean; 
      hidden?: boolean; 
      conflicts?: string[]; 
      implies?: string[]; 
      env?: string; 
      preset?: any;
    }): Command;
    
    // Enhanced command method with all possible signatures
    command(nameAndArgs: string): Command;
    command(nameAndArgs: string, description: string): Command;
    command(nameAndArgs: string, opts: CommandOptions): Command;
    command(nameAndArgs: string, description: string, opts: ExecutableCommandOptions): Command;
    command(nameAndArgs: string, cmd: Command): Command;
  }
}

// VSCode extension compatibility
declare module 'vscode' {
  export interface ExtensionContext {
    subscriptions: Array<{ dispose(): any }>;
    workspaceState: any;
    globalState: any;
    extensionPath: string;
    storagePath?: string;
    globalStoragePath: string;
    logPath: string;
    extensionUri: any;
    environmentVariableCollection: any;
    asAbsolutePath(relativePath: string): string;
  }

  export interface Terminal {
    name: string;
    processId: Thenable<number | undefined>;
    creationOptions: any;
    exitStatus: any;
    state: any;
    show(preserveFocus?: boolean): void;
    hide(): void;
    sendText(text: string, addNewLine?: boolean): void;
    dispose(): void;
  }

  export interface TerminalOptions {
    name?: string;
    shellPath?: string;
    shellArgs?: string[] | string;
    cwd?: string | any;
    env?: { [key: string]: string | null | undefined };
    strictEnv?: boolean;
    hideFromUser?: boolean;
    message?: string;
    iconPath?: any;
    color?: any;
    location?: any;
    isTransient?: boolean;
  }

  export namespace window {
    export function createTerminal(options?: TerminalOptions): Terminal;
    export function createTerminal(name?: string, shellPath?: string, shellArgs?: string[]): Terminal;
    export const activeTerminal: Terminal | undefined;
    export const terminals: readonly Terminal[];
    export function showInformationMessage(message: string, ...items: string[]): Thenable<string | undefined>;
    export function showErrorMessage(message: string, ...items: string[]): Thenable<string | undefined>;
    export function showWarningMessage(message: string, ...items: string[]): Thenable<string | undefined>;
  }

  export namespace workspace {
    export const workspaceFolders: readonly any[] | undefined;
    export function findFiles(include: any, exclude?: any, maxResults?: number, token?: any): Thenable<any[]>;
    export function openTextDocument(uri: any): Thenable<any>;
    export function openTextDocument(fileName: string): Thenable<any>;
    export function saveAll(includeUntitled?: boolean): Thenable<boolean>;
    export function applyEdit(edit: any): Thenable<boolean>;
    export function createFileSystemWatcher(globPattern: any, ignoreCreateEvents?: boolean, ignoreChangeEvents?: boolean, ignoreDeleteEvents?: boolean): any;
    export function getConfiguration(section?: string, resource?: any): any;
  }

  export namespace commands {
    export function registerCommand(command: string, callback: (...args: any[]) => any, thisArg?: any): any;
    export function executeCommand<T>(command: string, ...rest: any[]): Thenable<T>;
    export function getCommands(filterInternal?: boolean): Thenable<string[]>;
  }

  export namespace env {
    export const appName: string;
    export const appRoot: string;
    export const language: string;
    export const clipboard: any;
    export const machineId: string;
    export const sessionId: string;
    export const remoteName: string | undefined;
    export const shell: string;
    export const uriScheme: string;
    export function openExternal(target: any): Thenable<boolean>;
    export function asExternalUri(target: any): Thenable<any>;
  }

  export enum FileType {
    Unknown = 0,
    File = 1,
    Directory = 2,
    SymbolicLink = 64
  }

  export interface FileStat {
    type: FileType;
    ctime: number;
    mtime: number;
    size: number;
  }

  export interface FileSystem {
    stat(uri: any): Thenable<FileStat>;
    readDirectory(uri: any): Thenable<[string, FileType][]>;
    createDirectory(uri: any): Thenable<void>;
    readFile(uri: any): Thenable<Uint8Array>;
    writeFile(uri: any, content: Uint8Array): Thenable<void>;
    deleteFile(uri: any, options?: { recursive?: boolean; useTrash?: boolean }): Thenable<void>;
    rename(source: any, target: any, options?: { overwrite?: boolean }): Thenable<void>;
    copy(source: any, target: any, options?: { overwrite?: boolean }): Thenable<void>;
  }

  export namespace fs {
    export function stat(uri: any): Thenable<FileStat>;
    export function readDirectory(uri: any): Thenable<[string, FileType][]>;
    export function createDirectory(uri: any): Thenable<void>;
    export function readFile(uri: any): Thenable<Uint8Array>;
    export function writeFile(uri: any, content: Uint8Array): Thenable<void>;
    export function deleteFile(uri: any, options?: { recursive?: boolean; useTrash?: boolean }): Thenable<void>;
    export function rename(source: any, target: any, options?: { overwrite?: boolean }): Thenable<void>;
    export function copy(source: any, target: any, options?: { overwrite?: boolean }): Thenable<void>;
  }

  export class Uri {
    static file(path: string): Uri;
    static parse(value: string): Uri;
    static joinPath(base: Uri, ...pathSegments: string[]): Uri;
    static from(components: any): Uri;
    
    readonly scheme: string;
    readonly authority: string;
    readonly path: string;
    readonly query: string;
    readonly fragment: string;
    readonly fsPath: string;
    
    with(change: any): Uri;
    toString(skipEncoding?: boolean): string;
    toJSON(): any;
  }

  export interface Event<T> {
    (listener: (e: T) => any, thisArg?: any, disposables?: any[]): any;
  }

  export interface Disposable {
    dispose(): any;
  }

  export class EventEmitter<T> {
    readonly event: Event<T>;
    fire(data: T): void;
    dispose(): void;
  }

  export namespace debug {
    export const activeDebugSession: any;
    export const activeDebugConsole: any;
    export const breakpoints: any[];
    export function registerDebugConfigurationProvider(debugType: string, provider: any, triggerKind?: any): Disposable;
    export function registerDebugAdapterDescriptorFactory(debugType: string, factory: any): Disposable;
    export function startDebugging(folder: any, nameOrConfiguration: string | any, parentSessionOrOptions?: any): Thenable<boolean>;
    export function addBreakpoints(breakpoints: any[]): void;
    export function removeBreakpoints(breakpoints: any[]): void;
    export function asDebugSourceUri(source: any, session?: any): any;
  }

  export interface QuickPickItem {
    label: string;
    description?: string;
    detail?: string;
    picked?: boolean;
    alwaysShow?: boolean;
  }

  export interface QuickPickOptions {
    matchOnDescription?: boolean;
    matchOnDetail?: boolean;
    placeHolder?: string;
    ignoreFocusOut?: boolean;
    canPickMany?: boolean;
    onDidSelectItem?(item: QuickPickItem | string): any;
  }

  export interface InputBoxOptions {
    value?: string;
    valueSelection?: [number, number];
    prompt?: string;
    placeHolder?: string;
    password?: boolean;
    ignoreFocusOut?: boolean;
    validateInput?(value: string): string | undefined | null | Thenable<string | undefined | null>;
  }

  export interface MessageItem {
    title: string;
    isCloseAffordance?: boolean;
  }

  export interface MessageOptions {
    modal?: boolean;
  }

  export interface Progress<T> {
    report(value: T): void;
  }

  export interface ProgressOptions<T> {
    location: any;
    title?: string;
    cancellable?: boolean;
  }

  export enum ProgressLocation {
    SourceControl = 1,
    Window = 10,
    Notification = 15
  }

  export interface CancellationToken {
    isCancellationRequested: boolean;
    onCancellationRequested: Event<any>;
  }

  export class CancellationTokenSource {
    token: CancellationToken;
    cancel(): void;
    dispose(): void;
  }

  export enum ViewColumn {
    Active = -1,
    Beside = -2,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
  }

  export enum StatusBarAlignment {
    Left = 1,
    Right = 2
  }

  export interface StatusBarItem {
    alignment: StatusBarAlignment;
    priority?: number;
    text: string;
    tooltip?: string | any;
    color?: string | any;
    backgroundColor?: any;
    command?: string | any;
    accessibilityInformation?: any;
    show(): void;
    hide(): void;
    dispose(): void;
  }

  export enum ConfigurationTarget {
    Global = 1,
    Workspace = 2,
    WorkspaceFolder = 3
  }

  export interface WorkspaceConfiguration {
    get<T>(section: string): T | undefined;
    get<T>(section: string, defaultValue: T): T;
    has(section: string): boolean;
    inspect<T>(section: string): any;
    update(section: string, value: any, configurationTarget?: ConfigurationTarget | boolean, overrideInLanguage?: boolean): Thenable<void>;
  }

  export interface TextDocument {
    uri: Uri;
    fileName: string;
    isUntitled: boolean;
    languageId: string;
    version: number;
    isDirty: boolean;
    isClosed: boolean;
    save(): Thenable<boolean>;
    eol: any;
    lineCount: number;
    lineAt(line: number): any;
    lineAt(position: any): any;
    offsetAt(position: any): number;
    positionAt(offset: number): any;
    getText(range?: any): string;
    getWordRangeAtPosition(position: any, regex?: RegExp): any;
    validateRange(range: any): any;
    validatePosition(position: any): any;
  }

  export interface TextEditor {
    document: TextDocument;
    selection: any;
    selections: any[];
    visibleRanges: any[];
    options: any;
    viewColumn?: ViewColumn;
    edit(callback: (editBuilder: any) => void, options?: any): Thenable<boolean>;
    insertSnippet(snippet: any, location?: any, options?: any): Thenable<boolean>;
    setDecorations(decorationType: any, rangesOrOptions: any[]): void;
    revealRange(range: any, revealType?: any): void;
    show(column?: ViewColumn): void;
    hide(): void;
  }

  export namespace window {
    export function showQuickPick(items: string[], options?: QuickPickOptions, token?: CancellationToken): Thenable<string | undefined>;
    export function showQuickPick<T extends QuickPickItem>(items: T[], options?: QuickPickOptions, token?: CancellationToken): Thenable<T | undefined>;
    export function showQuickPick(items: Thenable<string[]>, options?: QuickPickOptions, token?: CancellationToken): Thenable<string | undefined>;
    export function showQuickPick<T extends QuickPickItem>(items: Thenable<T[]>, options?: QuickPickOptions, token?: CancellationToken): Thenable<T | undefined>;
    export function showInputBox(options?: InputBoxOptions, token?: CancellationToken): Thenable<string | undefined>;
    export function withProgress<R>(options: ProgressOptions<any>, task: (progress: Progress<any>, token: CancellationToken) => Thenable<R>): Thenable<R>;
    export function createStatusBarItem(alignment?: StatusBarAlignment, priority?: number): StatusBarItem;
    export function createStatusBarItem(id: string, alignment?: StatusBarAlignment, priority?: number): StatusBarItem;
    export const activeTextEditor: TextEditor | undefined;
    export const visibleTextEditors: TextEditor[];
    export function showTextDocument(document: TextDocument, column?: ViewColumn, preserveFocus?: boolean): Thenable<TextEditor>;
    export function showTextDocument(document: TextDocument, options?: any): Thenable<TextEditor>;
    export function showTextDocument(uri: Uri, options?: any): Thenable<TextEditor>;
  }

  export interface TreeItem {
    label?: string | any;
    id?: string;
    iconPath?: string | Uri | any;
    description?: string | boolean;
    tooltip?: string | any;
    command?: any;
    collapsibleState?: any;
    contextValue?: string;
    resourceUri?: Uri;
    accessibilityInformation?: any;
  }

  export enum TreeItemCollapsibleState {
    None = 0,
    Collapsed = 1,
    Expanded = 2
  }

  export interface TreeDataProvider<T> {
    onDidChangeTreeData?: Event<T | undefined | null | void>;
    getTreeItem(element: T): TreeItem | Thenable<TreeItem>;
    getChildren(element?: T): Thenable<T[]> | T[];
    getParent?(element: T): Thenable<T | undefined> | T | undefined;
    resolveTreeItem?(item: TreeItem, element: T, token: CancellationToken): Thenable<TreeItem> | TreeItem;
  }

  export interface TreeView<T> {
    onDidExpandElement: Event<any>;
    onDidCollapseElement: Event<any>;
    onDidChangeSelection: Event<any>;
    onDidChangeVisibility: Event<any>;
    onDidChangeCheckboxState: Event<any>;
    selection: T[];
    visible: boolean;
    title?: string;
    description?: string;
    message?: string;
    reveal(element: T, options?: any): Thenable<void>;
    dispose(): void;
  }

  export interface TreeViewOptions<T> {
    treeDataProvider: TreeDataProvider<T>;
    showCollapseAll?: boolean;
    canSelectMany?: boolean;
    dragAndDropController?: any;
    manageCheckboxStateManually?: boolean;
  }

  export namespace window {
    export function createTreeView<T>(viewId: string, options: TreeViewOptions<T>): TreeView<T>;
    export function registerTreeDataProvider<T>(viewId: string, treeDataProvider: TreeDataProvider<T>): Disposable;
  }
}