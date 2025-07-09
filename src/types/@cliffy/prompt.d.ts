// Enhanced stub types for @cliffy/prompt
declare module '@cliffy/prompt' {
  export interface BasePromptOptions {
    message: string;
    default?: any;
    validate?: (value: any, previous?: any) => boolean | string | Promise<boolean | string>;
    transform?: (value: any, previous?: any) => any | Promise<any>;
    hint?: string;
    pointer?: string;
    indent?: string;
    listPointer?: string;
    maxRows?: number;
    minLength?: number;
    maxLength?: number;
    keys?: Record<string, string>;
    cbreak?: boolean;
    prefix?: string;
    suffix?: string;
    reader?: any;
    writer?: any;
    beforePrompt?: (prompt: any) => void | Promise<void>;
    afterPrompt?: (prompt: any, result: any) => void | Promise<void>;
    onCancel?: (prompt: any) => void | Promise<void>;
    onSubmit?: (prompt: any, result: any) => void | Promise<void>;
    onKeypress?: (prompt: any, key: any) => void | Promise<void>;
    onError?: (prompt: any, error: Error) => void | Promise<void>;
  }

  export interface SelectOption<T = any> {
    name?: string;
    value: T;
    disabled?: boolean | string;
    group?: string;
  }

  export interface SelectOptions<T = any> extends BasePromptOptions {
    options: Array<SelectOption<T> | string | T>;
    search?: boolean;
    searchLabel?: string;
    searchPlaceholder?: string;
    searchIcon?: string;
    limit?: number;
    loop?: boolean;
    info?: boolean;
    infoLabel?: string;
    infoText?: string;
    groupBy?: (option: SelectOption<T>) => string;
    sort?: boolean | ((a: SelectOption<T>, b: SelectOption<T>) => number);
    multiple?: boolean;
    complete?: boolean;
    includeSelected?: boolean;
    min?: number;
    max?: number;
    check?: string;
    uncheck?: string;
    separator?: string;
    disabled?: boolean | string | ((option: SelectOption<T>) => boolean | string);
    format?: (option: SelectOption<T>) => string;
  }

  export interface InputOptions extends BasePromptOptions {
    type?: 'text' | 'password' | 'email' | 'url' | 'tel' | 'search' | 'number' | 'date' | 'time' | 'datetime-local' | 'month' | 'week' | 'color' | 'range' | 'file' | 'hidden';
    placeholder?: string;
    suggestions?: string[] | ((input: string) => string[] | Promise<string[]>);
    complete?: boolean;
    list?: boolean;
    info?: boolean;
    infoLabel?: string;
    infoText?: string;
    files?: boolean;
    id?: string;
    completeOptions?: {
      complete?: boolean;
      list?: boolean;
      info?: boolean;
      infoLabel?: string;
      infoText?: string;
    };
    mask?: string;
    hidden?: boolean;
    replace?: string;
    icon?: string;
    label?: string;
    clearable?: boolean;
    search?: boolean;
    searchLabel?: string;
    searchPlaceholder?: string;
    searchIcon?: string;
    limit?: number;
    loop?: boolean;
    groupBy?: (suggestion: string) => string;
    sort?: boolean | ((a: string, b: string) => number);
    format?: (suggestion: string) => string;
    includeInput?: boolean;
    caseSensitive?: boolean;
    fuzzy?: boolean;
    distance?: number;
    threshold?: number;
  }

  export interface ConfirmOptions extends BasePromptOptions {
    yes?: string[];
    no?: string[];
    active?: string;
    inactive?: string;
    confirmText?: string;
    cancelText?: string;
  }

  export interface NumberOptions extends BasePromptOptions {
    min?: number;
    max?: number;
    float?: boolean;
    round?: number;
    step?: number;
    placeholder?: string;
    suggestions?: number[] | ((input: number) => number[] | Promise<number[]>);
    complete?: boolean;
    list?: boolean;
    info?: boolean;
    infoLabel?: string;
    infoText?: string;
    format?: (value: number) => string;
    parse?: (value: string) => number;
  }

  export interface ListOptions extends BasePromptOptions {
    separator?: string;
    min?: number;
    max?: number;
    suggestions?: string[] | ((input: string) => string[] | Promise<string[]>);
    complete?: boolean;
    info?: boolean;
    infoLabel?: string;
    infoText?: string;
    type?: 'text' | 'number';
    format?: (values: any[]) => string;
    parse?: (value: string) => any;
  }

  export interface CheckboxOptions<T = any> extends BasePromptOptions {
    options: Array<SelectOption<T> | string | T>;
    search?: boolean;
    searchLabel?: string;
    searchPlaceholder?: string;
    searchIcon?: string;
    limit?: number;
    loop?: boolean;
    info?: boolean;
    infoLabel?: string;
    infoText?: string;
    groupBy?: (option: SelectOption<T>) => string;
    sort?: boolean | ((a: SelectOption<T>, b: SelectOption<T>) => number);
    min?: number;
    max?: number;
    check?: string;
    uncheck?: string;
    separator?: string;
    disabled?: boolean | string | ((option: SelectOption<T>) => boolean | string);
    format?: (option: SelectOption<T>) => string;
    includeSelected?: boolean;
    complete?: boolean;
  }

  export interface ToggleOptions extends BasePromptOptions {
    active?: string;
    inactive?: string;
    yes?: string[];
    no?: string[];
  }

  export interface SecretOptions extends BasePromptOptions {
    label?: string;
    hidden?: boolean;
    mask?: string;
    minLength?: number;
    maxLength?: number;
  }

  // Main prompt functions
  export function prompt<T = any>(prompts: Array<BasePromptOptions | ((result: any) => BasePromptOptions | Promise<BasePromptOptions>)>): Promise<T>;
  export function prompt<T = any>(prompt: BasePromptOptions | ((result?: any) => BasePromptOptions | Promise<BasePromptOptions>)): Promise<T>;
  
  // Individual prompt classes (for backward compatibility)
  export class Input {
    static prompt(options: InputOptions): Promise<string>;
    constructor(options: InputOptions);
    prompt(): Promise<string>;
  }
  
  export class Select {
    static prompt<T = any>(options: SelectOptions<T>): Promise<T>;
    constructor<T = any>(options: SelectOptions<T>);
    prompt<T = any>(): Promise<T>;
  }
  
  export class Confirm {
    static prompt(options: ConfirmOptions): Promise<boolean>;
    constructor(options: ConfirmOptions);
    prompt(): Promise<boolean>;
  }
  
  export class Number {
    static prompt(options: NumberOptions): Promise<number>;
    constructor(options: NumberOptions);
    prompt(): Promise<number>;
  }
  
  export class List {
    static prompt(options: ListOptions): Promise<string[]>;
    constructor(options: ListOptions);
    prompt(): Promise<string[]>;
  }

  export class Checkbox {
    static prompt<T = any>(options: CheckboxOptions<T>): Promise<T[]>;
    constructor<T = any>(options: CheckboxOptions<T>);
    prompt<T = any>(): Promise<T[]>;
  }

  export class Toggle {
    static prompt(options: ToggleOptions): Promise<boolean>;
    constructor(options: ToggleOptions);
    prompt(): Promise<boolean>;
  }

  export class Secret {
    static prompt(options: SecretOptions): Promise<string>;
    constructor(options: SecretOptions);
    prompt(): Promise<string>;
  }

  // Individual prompt functions (modern style)
  export function input(options: InputOptions): Promise<string>;
  export function select<T = any>(options: SelectOptions<T>): Promise<T>;
  export function multiSelect<T = any>(options: SelectOptions<T> & { multiple: true }): Promise<T[]>;
  export function confirm(options: ConfirmOptions): Promise<boolean>;
  export function number(options: NumberOptions): Promise<number>;
  export function list(options: ListOptions): Promise<string[]>;
  export function checkbox<T = any>(options: CheckboxOptions<T>): Promise<T[]>;
  export function toggle(options: ToggleOptions): Promise<boolean>;
  export function secret(options: SecretOptions): Promise<string>;

  // Advanced prompt functions
  export function search<T = any>(options: SelectOptions<T> & { search: true }): Promise<T>;
  export function autocomplete(options: InputOptions & { suggestions: string[] | ((input: string) => string[] | Promise<string[]>) }): Promise<string>;
  
  // Utility functions
  export function createPrompt<T = any>(type: string, options: BasePromptOptions): Promise<T>;
  export function registerPrompt(name: string, prompt: any): void;
  export function getPrompt(name: string): any;
  export function hasPrompt(name: string): boolean;
  export function removePrompt(name: string): boolean;
  export function getPrompts(): Record<string, any>;
  export function clearPrompts(): void;

  // Error handling
  export class PromptError extends Error {
    name: 'PromptError';
    message: string;
    prompt: any;
    options: any;
    input: string;
    cursor: number;
    selected: any;
    suggestions: any[];
    cancelled: boolean;
  }
  
  export class ValidationError extends PromptError {
    name: 'ValidationError';
    validation: string;
  }
  
  export class CancelError extends PromptError {
    name: 'CancelError';
    cancelled: true;
  }

  // Constants
  export const PROMPT_TYPES: Record<string, string>;
  export const KEY_MAPPINGS: Record<string, string>;
  export const ESCAPE_SEQUENCES: Record<string, string>;

  // Validation helpers
  export const validators: {
    required: (message?: string) => (value: any) => boolean | string;
    minLength: (min: number, message?: string) => (value: string) => boolean | string;
    maxLength: (max: number, message?: string) => (value: string) => boolean | string;
    pattern: (regex: RegExp, message?: string) => (value: string) => boolean | string;
    email: (message?: string) => (value: string) => boolean | string;
    url: (message?: string) => (value: string) => boolean | string;
    number: (message?: string) => (value: any) => boolean | string;
    integer: (message?: string) => (value: any) => boolean | string;
    positive: (message?: string) => (value: number) => boolean | string;
    negative: (message?: string) => (value: number) => boolean | string;
    min: (min: number, message?: string) => (value: number) => boolean | string;
    max: (max: number, message?: string) => (value: number) => boolean | string;
    range: (min: number, max: number, message?: string) => (value: number) => boolean | string;
    oneOf: (values: any[], message?: string) => (value: any) => boolean | string;
    custom: (fn: (value: any) => boolean | string, message?: string) => (value: any) => boolean | string;
  };
  
  // Transform helpers
  export const transforms: {
    trim: (value: string) => string;
    toLowerCase: (value: string) => string;
    toUpperCase: (value: string) => string;
    capitalize: (value: string) => string;
    toNumber: (value: string) => number;
    toInteger: (value: string) => number;
    toBoolean: (value: any) => boolean;
    toArray: (value: string, separator?: string) => string[];
    toString: (value: any) => string;
    toDate: (value: string) => Date;
    toJSON: (value: string) => any;
    custom: <T, R>(fn: (value: T) => R) => (value: T) => R;
  };
}