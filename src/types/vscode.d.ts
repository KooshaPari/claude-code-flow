// Stub types for vscode
declare module 'vscode' {
  export interface Terminal {
    name: string;
    processId: Promise<number>;
    creationOptions: TerminalOptions;
    exitStatus: TerminalExitStatus | undefined;
    state: TerminalState;
    sendText: (text: string, addNewLine?: boolean) => void;
    show: (preserveFocus?: boolean) => void;
    hide: () => void;
    dispose: () => void;
  }

  export interface TerminalOptions {
    name?: string;
    shellPath?: string;
    shellArgs?: string[] | string;
    cwd?: string;
    env?: { [key: string]: string | null | undefined };
    iconPath?: any;
    color?: any;
    strictEnv?: boolean;
  }

  export interface TerminalExitStatus {
    code: number;
    reason: TerminalExitReason;
  }

  export enum TerminalExitReason {
    Unknown = 0,
    Shutdown = 1,
    Process = 2,
    User = 3,
    Extension = 4
  }

  export interface TerminalState {
    isInteractedWith: boolean;
  }

  export class EventEmitter<T = any> {
    constructor();
    event: Event<T>;
    fire(data: T): void;
    dispose(): void;
  }

  export interface Event<T> {
    (listener: (e: T) => any, thisArgs?: any, disposables?: any[]): any;
  }

  export interface ExtensionContext {
    subscriptions: any[];
    workspaceState: any;
    globalState: any;
    extensionPath: string;
    storagePath: string | undefined;
    globalStoragePath: string;
    logPath: string;
    extensionUri: any;
    globalStorageUri: any;
    logUri: any;
    storageUri: any | undefined;
    environmentVariableCollection: any;
    asAbsolutePath: (relativePath: string) => string;
  }

  export const window: {
    showInformationMessage: (message: string) => Promise<void>;
    showErrorMessage: (message: string) => Promise<void>;
    createTerminal: (options?: TerminalOptions) => Terminal;
    onDidCloseTerminal: Event<Terminal>;
    registerTerminalProfileProvider: (id: string, provider: any) => any;
  };
  export const workspace: {
    workspaceFolders: any[];
    getConfiguration: (section?: string) => any;
  };
  export const commands: {
    registerCommand: (command: string, callback: (...args: any[]) => any) => any;
    executeCommand: (command: string, ...args: any[]) => Promise<any>;
  };
}