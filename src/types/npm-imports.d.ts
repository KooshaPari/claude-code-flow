// Type stubs for npm: imports
// This file provides type definitions for npm modules imported using the npm: protocol

declare module 'npm:chalk@^4.1.2' {
  interface ChalkInstance {
    (...text: unknown[]): string;
    (text: TemplateStringsArray, ...placeholders: unknown[]): string;
    
    // Style functions
    reset: ChalkInstance;
    bold: ChalkInstance;
    dim: ChalkInstance;
    italic: ChalkInstance;
    underline: ChalkInstance;
    inverse: ChalkInstance;
    hidden: ChalkInstance;
    strikethrough: ChalkInstance;
    
    // Color functions
    black: ChalkInstance;
    red: ChalkInstance;
    green: ChalkInstance;
    yellow: ChalkInstance;
    blue: ChalkInstance;
    magenta: ChalkInstance;
    cyan: ChalkInstance;
    white: ChalkInstance;
    gray: ChalkInstance;
    grey: ChalkInstance;
    
    // Bright color functions
    blackBright: ChalkInstance;
    redBright: ChalkInstance;
    greenBright: ChalkInstance;
    yellowBright: ChalkInstance;
    blueBright: ChalkInstance;
    magentaBright: ChalkInstance;
    cyanBright: ChalkInstance;
    whiteBright: ChalkInstance;
    
    // Background color functions
    bgBlack: ChalkInstance;
    bgRed: ChalkInstance;
    bgGreen: ChalkInstance;
    bgYellow: ChalkInstance;
    bgBlue: ChalkInstance;
    bgMagenta: ChalkInstance;
    bgCyan: ChalkInstance;
    bgWhite: ChalkInstance;
    bgGray: ChalkInstance;
    bgGrey: ChalkInstance;
    
    // Bright background color functions
    bgBlackBright: ChalkInstance;
    bgRedBright: ChalkInstance;
    bgGreenBright: ChalkInstance;
    bgYellowBright: ChalkInstance;
    bgBlueBright: ChalkInstance;
    bgMagentaBright: ChalkInstance;
    bgCyanBright: ChalkInstance;
    bgWhiteBright: ChalkInstance;
    
    // Color support detection
    supportsColor: boolean | { level: number; hasBasic: boolean; has256: boolean; has16m: boolean };
    
    // Utility functions
    hex(color: string): ChalkInstance;
    rgb(r: number, g: number, b: number): ChalkInstance;
    hsl(h: number, s: number, l: number): ChalkInstance;
    hsv(h: number, s: number, v: number): ChalkInstance;
    hwb(h: number, w: number, b: number): ChalkInstance;
    ansi256(code: number): ChalkInstance;
    
    bgHex(color: string): ChalkInstance;
    bgRgb(r: number, g: number, b: number): ChalkInstance;
    bgHsl(h: number, s: number, l: number): ChalkInstance;
    bgHsv(h: number, s: number, v: number): ChalkInstance;
    bgHwb(h: number, w: number, b: number): ChalkInstance;
    bgAnsi256(code: number): ChalkInstance;
  }
  
  const chalk: ChalkInstance;
  export default chalk;
}

declare module 'npm:chalk@^5.3.0' {
  interface ChalkInstance {
    (...text: unknown[]): string;
    (text: TemplateStringsArray, ...placeholders: unknown[]): string;
    
    // Style functions
    reset: ChalkInstance;
    bold: ChalkInstance;
    dim: ChalkInstance;
    italic: ChalkInstance;
    underline: ChalkInstance;
    inverse: ChalkInstance;
    hidden: ChalkInstance;
    strikethrough: ChalkInstance;
    
    // Color functions
    black: ChalkInstance;
    red: ChalkInstance;
    green: ChalkInstance;
    yellow: ChalkInstance;
    blue: ChalkInstance;
    magenta: ChalkInstance;
    cyan: ChalkInstance;
    white: ChalkInstance;
    gray: ChalkInstance;
    grey: ChalkInstance;
    
    // Bright color functions
    blackBright: ChalkInstance;
    redBright: ChalkInstance;
    greenBright: ChalkInstance;
    yellowBright: ChalkInstance;
    blueBright: ChalkInstance;
    magentaBright: ChalkInstance;
    cyanBright: ChalkInstance;
    whiteBright: ChalkInstance;
    
    // Background color functions
    bgBlack: ChalkInstance;
    bgRed: ChalkInstance;
    bgGreen: ChalkInstance;
    bgYellow: ChalkInstance;
    bgBlue: ChalkInstance;
    bgMagenta: ChalkInstance;
    bgCyan: ChalkInstance;
    bgWhite: ChalkInstance;
    bgGray: ChalkInstance;
    bgGrey: ChalkInstance;
    
    // Bright background color functions
    bgBlackBright: ChalkInstance;
    bgRedBright: ChalkInstance;
    bgGreenBright: ChalkInstance;
    bgYellowBright: ChalkInstance;
    bgBlueBright: ChalkInstance;
    bgMagentaBright: ChalkInstance;
    bgCyanBright: ChalkInstance;
    bgWhiteBright: ChalkInstance;
    
    // Color support detection
    supportsColor: boolean | { level: number; hasBasic: boolean; has256: boolean; has16m: boolean };
    
    // Utility functions
    hex(color: string): ChalkInstance;
    rgb(r: number, g: number, b: number): ChalkInstance;
    hsl(h: number, s: number, l: number): ChalkInstance;
    hsv(h: number, s: number, v: number): ChalkInstance;
    hwb(h: number, w: number, b: number): ChalkInstance;
    ansi256(code: number): ChalkInstance;
    
    bgHex(color: string): ChalkInstance;
    bgRgb(r: number, g: number, b: number): ChalkInstance;
    bgHsl(h: number, s: number, l: number): ChalkInstance;
    bgHsv(h: number, s: number, v: number): ChalkInstance;
    bgHwb(h: number, w: number, b: number): ChalkInstance;
    bgAnsi256(code: number): ChalkInstance;
  }
  
  const chalk: ChalkInstance;
  export default chalk;
}

declare module 'npm:inquirer@^9.2.12' {
  interface Question {
    type?: string;
    name: string;
    message: string | ((answers: any) => string);
    default?: any;
    choices?: Array<any>;
    validate?: (input: any) => boolean | string;
    filter?: (input: any) => any;
    when?: (answers: any) => boolean;
    pageSize?: number;
    prefix?: string;
    suffix?: string;
  }
  
  interface Inquirer {
    prompt(questions: Question[]): Promise<any>;
    prompt(questions: Question): Promise<any>;
    registerPrompt(name: string, plugin: any): void;
    createPromptModule(): any;
  }
  
  const inquirer: Inquirer;
  export default inquirer;
}

declare module 'npm:ora@^7.0.1' {
  interface Ora {
    start(text?: string): Ora;
    stop(): Ora;
    succeed(text?: string): Ora;
    fail(text?: string): Ora;
    warn(text?: string): Ora;
    info(text?: string): Ora;
    stopAndPersist(options?: { symbol?: string; text?: string }): Ora;
    clear(): Ora;
    render(): Ora;
    
    text: string;
    color: string;
    spinner: string | object;
    indent: number;
    isSpinning: boolean;
  }
  
  interface OraOptions {
    text?: string;
    spinner?: string | object;
    color?: string;
    hideCursor?: boolean;
    indent?: number;
    interval?: number;
    stream?: NodeJS.WritableStream;
    isEnabled?: boolean;
    isSilent?: boolean;
    discardStdin?: boolean;
  }
  
  function ora(text?: string): Ora;
  function ora(options?: OraOptions): Ora;
  
  export default ora;
}

declare module 'npm:nanoid@^5.0.4' {
  export function nanoid(size?: number): string;
  export function customAlphabet(alphabet: string, size: number): () => string;
  export function customRandom(alphabet: string, size: number, random: () => number): () => string;
  export function urlAlphabet(): string;
  export function random(bytes: number): Uint8Array;
}

declare module 'npm:fs-extra@^11.2.0' {
  import * as fs from 'fs';
  
  interface CopyOptions {
    overwrite?: boolean;
    errorOnExist?: boolean;
    dereference?: boolean;
    preserveTimestamps?: boolean;
    filter?: (src: string, dest: string) => boolean;
  }
  
  interface MoveOptions {
    overwrite?: boolean;
  }
  
  interface RemoveOptions {
    maxRetries?: number;
    retryDelay?: number;
  }
  
  interface EnsureOptions {
    mode?: number;
  }
  
  interface ReadJsonOptions {
    encoding?: string;
    flag?: string;
    throws?: boolean;
    reviver?: (key: string, value: any) => any;
  }
  
  interface WriteJsonOptions {
    encoding?: string;
    flag?: string;
    mode?: number;
    spaces?: number | string;
    EOL?: string;
    replacer?: (key: string, value: any) => any;
  }
  
  // Re-export all fs functions
  export * from 'fs';
  
  // Additional fs-extra functions
  export function copy(src: string, dest: string, options?: CopyOptions): Promise<void>;
  export function copySync(src: string, dest: string, options?: CopyOptions): void;
  export function move(src: string, dest: string, options?: MoveOptions): Promise<void>;
  export function moveSync(src: string, dest: string, options?: MoveOptions): void;
  export function remove(path: string, options?: RemoveOptions): Promise<void>;
  export function removeSync(path: string, options?: RemoveOptions): void;
  export function emptyDir(path: string): Promise<void>;
  export function emptyDirSync(path: string): void;
  export function ensureFile(path: string): Promise<void>;
  export function ensureFileSync(path: string): void;
  export function ensureDir(path: string, options?: EnsureOptions): Promise<void>;
  export function ensureDirSync(path: string, options?: EnsureOptions): void;
  export function ensureLink(srcPath: string, dstPath: string): Promise<void>;
  export function ensureLinkSync(srcPath: string, dstPath: string): void;
  export function ensureSymlink(srcPath: string, dstPath: string, type?: string): Promise<void>;
  export function ensureSymlinkSync(srcPath: string, dstPath: string, type?: string): void;
  export function mkdirp(path: string, options?: EnsureOptions): Promise<void>;
  export function mkdirpSync(path: string, options?: EnsureOptions): void;
  export function pathExists(path: string): Promise<boolean>;
  export function pathExistsSync(path: string): boolean;
  export function readJson(path: string, options?: ReadJsonOptions): Promise<any>;
  export function readJsonSync(path: string, options?: ReadJsonOptions): any;
  export function writeJson(path: string, object: any, options?: WriteJsonOptions): Promise<void>;
  export function writeJsonSync(path: string, object: any, options?: WriteJsonOptions): void;
  export function outputFile(path: string, data: string | Buffer, options?: fs.WriteFileOptions): Promise<void>;
  export function outputFileSync(path: string, data: string | Buffer, options?: fs.WriteFileOptions): void;
  export function outputJson(path: string, object: any, options?: WriteJsonOptions): Promise<void>;
  export function outputJsonSync(path: string, object: any, options?: WriteJsonOptions): void;
}

declare module 'npm:commander@^11.1.0' {
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
  
  interface Command {
    args: string[];
    commands: Command[];
    options: Option[];
    parent: Command | null;
    
    // Command configuration
    command(nameAndArgs: string, description?: string, opts?: any): Command;
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
    
    // Options
    option(flags: string, description?: string, defaultValue?: any): Command;
    option(flags: string, description?: string, fn?: (value: string, previous: any) => any, defaultValue?: any): Command;
    requiredOption(flags: string, description?: string, defaultValue?: any): Command;
    requiredOption(flags: string, description?: string, fn?: (value: string, previous: any) => any, defaultValue?: any): Command;
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
    
    // Parse and execute
    parse(argv?: string[], options?: any): Command;
    parseAsync(argv?: string[], options?: any): Promise<Command>;
    opts(): any;
    optsWithGlobals(): any;
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
  }
  
  function createCommand(name?: string): Command;
  function createOption(flags: string, description?: string): Option;
  function createArgument(name: string, description?: string): any;
  
  export { Command, createCommand, createOption, createArgument, CommanderError, Option };
  export default Command;
}

declare module 'npm:blessed@^0.1.81' {
  namespace blessed {
    interface NodeOptions {
      screen?: any;
      parent?: any;
      children?: any[];
      left?: number | string;
      right?: number | string;
      top?: number | string;
      bottom?: number | string;
      width?: number | string;
      height?: number | string;
      alwaysScroll?: boolean;
      scrollable?: boolean;
      keys?: boolean;
      vi?: boolean;
      mouse?: boolean;
      hidden?: boolean;
      label?: string;
      align?: 'left' | 'center' | 'right';
      valign?: 'top' | 'middle' | 'bottom';
      shrink?: boolean;
      padding?: number | { left?: number; right?: number; top?: number; bottom?: number };
      margin?: number | { left?: number; right?: number; top?: number; bottom?: number };
      style?: any;
      border?: any;
      tags?: boolean;
      content?: string;
      clickable?: boolean;
      input?: boolean;
      focused?: boolean;
      focusable?: boolean;
      shadow?: boolean;
      scrollbar?: any;
      hoverBg?: string;
      hoverText?: string;
      draggable?: boolean;
      fg?: string;
      bg?: string;
      bold?: boolean;
      underline?: boolean;
      blink?: boolean;
      inverse?: boolean;
      invisible?: boolean;
      transparent?: boolean;
      wrap?: boolean;
      dockBorders?: boolean;
      ignoreLocked?: boolean;
      ignoreKeys?: boolean;
      keyable?: boolean;
      name?: string;
      type?: string;
      ch?: string;
      position?: any;
      noOverflow?: boolean;
      dwidth?: number;
      dheight?: number;
      debug?: boolean;
      track?: any;
      handleClick?: boolean;
      handleHover?: boolean;
      handleKeys?: boolean;
      handleMouse?: boolean;
      handleFocus?: boolean;
      handleBlur?: boolean;
      handleSubmit?: boolean;
      handleCancel?: boolean;
      handleAction?: boolean;
      handleSelect?: boolean;
      handleMove?: boolean;
      handleResize?: boolean;
      handleScroll?: boolean;
      handleTab?: boolean;
      handleBackTab?: boolean;
      handleEnter?: boolean;
      handleEscape?: boolean;
      handleUp?: boolean;
      handleDown?: boolean;
      handleLeft?: boolean;
      handleRight?: boolean;
      handleHome?: boolean;
      handleEnd?: boolean;
      handlePageUp?: boolean;
      handlePageDown?: boolean;
      handleInsert?: boolean;
      handleDelete?: boolean;
      handleBackspace?: boolean;
      handleF1?: boolean;
      handleF2?: boolean;
      handleF3?: boolean;
      handleF4?: boolean;
      handleF5?: boolean;
      handleF6?: boolean;
      handleF7?: boolean;
      handleF8?: boolean;
      handleF9?: boolean;
      handleF10?: boolean;
      handleF11?: boolean;
      handleF12?: boolean;
    }
    
    interface Element {
      // Event methods
      on(event: string, listener: (...args: any[]) => void): this;
      once(event: string, listener: (...args: any[]) => void): this;
      off(event: string, listener: (...args: any[]) => void): this;
      emit(event: string, ...args: any[]): boolean;
      
      // Display methods
      render(): void;
      show(): void;
      hide(): void;
      focus(): void;
      blur(): void;
      
      // Content methods
      setContent(content: string): void;
      getContent(): string;
      setText(text: string): void;
      getText(): string;
      insertLine(i: number, line: string): void;
      deleteLine(i: number): void;
      getLine(i: number): string;
      getLines(): string[];
      setLine(i: number, line: string): void;
      insertBottom(text: string): void;
      insertTop(text: string): void;
      
      // Position methods
      setPosition(options: any): void;
      
      // Style methods
      setStyle(style: any): void;
      
      // Scroll methods
      scroll(offset: number): void;
      scrollTo(offset: number): void;
      scrollPerc(percentage: number): void;
      resetScroll(): void;
      
      // Hierarchy methods
      append(node: Element): void;
      prepend(node: Element): void;
      remove(node: Element): void;
      insert(node: Element, i: number): void;
      insertBefore(node: Element, refNode: Element): void;
      insertAfter(node: Element, refNode: Element): void;
      detach(): void;
      
      // Screen reference
      screen: Screen;
      parent: Element;
      children: Element[];
      
      // Properties
      left: number;
      right: number;
      top: number;
      bottom: number;
      width: number;
      height: number;
      aleft: number;
      aright: number;
      atop: number;
      abottom: number;
      awidth: number;
      aheight: number;
      rleft: number;
      rright: number;
      rtop: number;
      rbottom: number;
      rwidth: number;
      rheight: number;
      
      // State
      hidden: boolean;
      visible: boolean;
      focused: boolean;
      destroyed: boolean;
      
      // Methods
      destroy(): void;
      free(): void;
      kill(): void;
      
      // Input
      readInput(callback: (err: Error | null, data: string) => void): void;
      setIndex(index: number): void;
      
      // Screenshot
      screenshot(xi?: number, xl?: number, yi?: number, yl?: number): string;
    }
    
    interface Screen extends Element {
      program: any;
      tput: any;
      
      // Screen methods
      render(): void;
      alloc(): void;
      realloc(): void;
      draw(start: number, end: number): void;
      clear(): void;
      clearRegion(xi: number, xl: number, yi: number, yl: number): void;
      
      // Input methods
      key(name: string | string[], listener: (ch: string, key: any) => void): void;
      onceKey(name: string | string[], listener: (ch: string, key: any) => void): void;
      unkey(name: string | string[], listener: (ch: string, key: any) => void): void;
      
      // Focus methods
      saveFocus(): Element;
      restoreFocus(): void;
      rewindFocus(): void;
      previousFocus(): void;
      nextFocus(): void;
      focusPop(): Element;
      focusPush(element: Element): void;
      
      // Grab methods
      grabKeys: boolean;
      grabMouse: boolean;
      
      // Title methods
      title: string;
      setTitle(title: string): void;
      
      // Cursor methods
      cursorShape(shape: string, blink: boolean): void;
      cursorColor(color: string): void;
      cursorReset(): void;
      
      // Screenshot
      screenshot(xi?: number, xl?: number, yi?: number, yl?: number): string;
      
      // Destroy
      destroy(): void;
      
      // Properties
      width: number;
      height: number;
      cols: number;
      rows: number;
      
      // Layout
      layout: any;
      
      // History
      history: string[];
      
      // Warning
      warn: boolean;
      
      // Autofocus
      autofocus: Element;
      
      // Lockkeys
      lockKeys: boolean;
      
      // Fullscreen
      fullscreen: boolean;
      
      // Grab
      grab: boolean;
      
      // Warnings
      warnings: boolean;
      
      // Resizable
      resizable: boolean;
      
      // Dockborders
      dockBorders: boolean;
      
      // Ignorelocked
      ignoreLocked: boolean;
      
      // Forceunicode
      forceUnicode: boolean;
      
      // Fastcsr
      fastCSR: boolean;
      
      // Usebuffer
      useBuffer: boolean;
      
      // Smartcsr
      smartCSR: boolean;
      
      // Tagsansi
      tagsAnsi: boolean;
      
      // Cursor
      cursor: any;
      
      // Focused
      focused: Element;
      
      // Hover
      hover: Element;
      
      // Terminal
      terminal: string;
      
      // Debug
      debug: boolean;
      
      // Log
      log: (...args: any[]) => void;
      
      // Dump
      dump(): string;
      
      // Leave
      leave(): void;
      
      // Enter
      enter(): void;
      
      // Sigtstp
      sigtstp(callback: () => void): void;
      
      // Copytoall
      copyToAll(element: Element): void;
      
      // Spawn
      spawn(file: string, args: string[], options: any): any;
      
      // Exec
      exec(file: string, args: string[], options: any, callback: (err: Error | null, stdout: string, stderr: string) => void): any;
      
      // Readdir
      readdir(path: string, callback: (err: Error | null, files: string[]) => void): void;
      
      // Seteffects
      setEffects(element: Element, fel: Element, over: any, out: any, effects: any, temp: any): void;
      
      // Insertline
      insertLine(n: number, y: number, top: number, bottom: number): void;
      
      // Deleteline
      deleteLine(n: number, y: number, top: number, bottom: number): void;
      
      // Insertbottom
      insertBottom(top: number, bottom: number): void;
      
      // Inserttop
      insertTop(top: number, bottom: number): void;
      
      // Enablemouse
      enableMouse(): void;
      
      // Enablekeys
      enableKeys(): void;
      
      // Enableinput
      enableInput(): void;
      
      // Copymode
      copyMode(): void;
      
      // Cursorpos
      cursorPos(x: number, y: number): void;
      
      // Cursorreset
      cursorReset(): void;
      
      // Savecursor
      saveCursor(): void;
      
      // Restorecursor
      restoreCursor(): void;
      
      // Hidecursor
      hideCursor(): void;
      
      // Showcursor
      showCursor(): void;
      
      // Bell
      bell(): void;
      
      // Beep
      beep(): void;
    }
    
    interface ScreenOptions extends NodeOptions {
      program?: any;
      smartCSR?: boolean;
      fastCSR?: boolean;
      useBCE?: boolean;
      resizeTimeout?: number;
      tabSize?: number;
      autoPadding?: boolean;
      cursors?: boolean;
      debug?: boolean;
      dump?: boolean;
      ignoreDockContrast?: boolean;
      fullUnicode?: boolean;
      dockBorders?: boolean;
      title?: string;
      warnings?: boolean;
      forceUnicode?: boolean;
      sendFocus?: boolean;
      log?: string;
      terminal?: string;
      cursor?: any;
      input?: any;
      output?: any;
      tput?: any;
      term?: string;
      resizable?: boolean;
      grabKeys?: boolean;
      grabMouse?: boolean;
      lockKeys?: boolean;
      focusable?: boolean;
      clickable?: boolean;
      mouse?: boolean;
      keyable?: boolean;
      autopadding?: boolean;
      tagsAnsi?: boolean;
      useBuffer?: boolean;
      normalizeKeys?: boolean;
      fullscreen?: boolean;
      focused?: Element;
      hover?: Element;
      ignoreLocked?: boolean;
      dwidth?: number;
      dheight?: number;
      tpadding?: number;
      rpadding?: number;
      bpadding?: number;
      lpadding?: number;
      buffer?: string;
      
      // Event handlers
      artificialCursor?: boolean;
      cursorBlink?: boolean;
      cursorShape?: string;
      cursorColor?: string;
      
      // Environment
      env?: any;
      
      // Process
      process?: any;
      
      // Optimization
      optimization?: any;
      
      // Extensions
      extensions?: any;
      
      // Autofocus
      autofocus?: Element;
      
      // Warnings
      warnings?: boolean;
      
      // Reflow
      reflow?: boolean;
      
      // Artificalcursor
      artificialCursor?: boolean;
      
      // Cursorshape
      cursorShape?: string;
      
      // Cursor color
      cursorColor?: string;
      
      // Resizeable
      resizable?: boolean;
      
      // Fullscreen
      fullscreen?: boolean;
      
      // Grab
      grab?: boolean;
      
      // Lockkeys
      lockKeys?: boolean;
      
      // Ignorelocked
      ignoreLocked?: boolean;
      
      // Sendkeys
      sendKeys?: boolean;
      
      // Normalizekeys
      normalizeKeys?: boolean;
      
      // Fastcsr
      fastCSR?: boolean;
      
      // Smartcsr
      smartCSR?: boolean;
      
      // Usebuffer
      useBuffer?: boolean;
      
      // Tabsize
      tabSize?: number;
      
      // Autopadding
      autoPadding?: boolean;
      
      // Tagsansi
      tagsAnsi?: boolean;
      
      // Warnings
      warnings?: boolean;
      
      // Forceuni
      forceUnicode?: boolean;
      
      // Fullunicode
      fullUnicode?: boolean;
      
      // Dockborders
      dockBorders?: boolean;
      
      // Resettimeout
      resizeTimeout?: number;
      
      // Usebce
      useBCE?: boolean;
      
      // Cursors
      cursors?: boolean;
      
      // Dump
      dump?: boolean;
      
      // Debug
      debug?: boolean;
      
      // Ignoredockcontrast
      ignoreDockContrast?: boolean;
      
      // Log
      log?: string;
      
      // Terminal
      terminal?: string;
      
      // Cursor
      cursor?: any;
      
      // Input
      input?: any;
      
      // Output
      output?: any;
      
      // Tput
      tput?: any;
      
      // Term
      term?: string;
      
      // Process
      process?: any;
    }
    
    interface Box extends Element {
      // Box specific properties
      ch: string;
      
      // Box specific methods
      setContent(content: string): void;
      getContent(): string;
    }
    
    interface List extends Element {
      // List specific properties
      items: string[];
      selected: number;
      
      // List specific methods
      add(item: string): void;
      addItem(item: string): void;
      removeItem(item: string): void;
      setItems(items: string[]): void;
      getItem(index: number): string;
      getItemIndex(item: string): number;
      select(index: number): void;
      move(offset: number): void;
      up(amount?: number): void;
      down(amount?: number): void;
      pick(callback: (err: Error | null, item: string) => void): void;
      fuzzyFind(text: string, callback: (err: Error | null, item: string) => void): void;
    }
    
    interface TextBox extends Element {
      // TextBox specific properties
      value: string;
      
      // TextBox specific methods
      readInput(callback: (err: Error | null, value: string) => void): void;
      input(callback: (err: Error | null, value: string) => void): void;
      setInput(text: string): void;
      clearInput(): void;
      submit(): void;
      cancel(): void;
      getValue(): string;
      setValue(value: string): void;
    }
    
    interface Button extends Element {
      // Button specific methods
      press(): void;
    }
    
    interface Checkbox extends Element {
      // Checkbox specific properties
      checked: boolean;
      
      // Checkbox specific methods
      check(): void;
      uncheck(): void;
      toggle(): void;
    }
    
    interface RadioSet extends Element {
      // RadioSet specific methods
      select(index: number): void;
    }
    
    interface RadioButton extends Element {
      // RadioButton specific properties
      checked: boolean;
      
      // RadioButton specific methods
      check(): void;
      uncheck(): void;
    }
    
    interface Prompt extends Element {
      // Prompt specific methods
      input(text: string, value: string, callback: (err: Error | null, value: string) => void): void;
      setInput(text: string): void;
      clearInput(): void;
    }
    
    interface Question extends Element {
      // Question specific methods
      ask(text: string, callback: (err: Error | null, value: boolean) => void): void;
    }
    
    interface Message extends Element {
      // Message specific methods
      log(text: string): void;
      display(text: string, time: number, callback: (err: Error | null) => void): void;
      error(text: string, callback: (err: Error | null) => void): void;
    }
    
    interface Loading extends Element {
      // Loading specific methods
      load(text: string): void;
      stop(): void;
    }
    
    interface ProgressBar extends Element {
      // ProgressBar specific properties
      filled: number;
      value: number;
      
      // ProgressBar specific methods
      setProgress(progress: number): void;
      reset(): void;
    }
    
    interface FileManager extends Element {
      // FileManager specific properties
      cwd: string;
      
      // FileManager specific methods
      refresh(dir?: string, callback?: (err: Error | null) => void): void;
      pick(callback: (err: Error | null, file: string) => void): void;
      reset(dir?: string, callback?: (err: Error | null) => void): void;
    }
    
    interface Listbar extends Element {
      // Listbar specific properties
      items: any[];
      commands: any[];
      
      // Listbar specific methods
      add(item: any): void;
      addItem(item: any): void;
      removeItem(item: any): void;
      setItems(items: any[]): void;
      select(index: number): void;
      selectTab(index: number): void;
      move(offset: number): void;
      moveTab(offset: number): void;
    }
    
    interface Log extends Element {
      // Log specific methods
      log(text: string): void;
      add(text: string): void;
    }
    
    interface Table extends Element {
      // Table specific properties
      rows: any[][];
      data: any[][];
      
      // Table specific methods
      setData(data: any[][]): void;
      setRows(rows: any[][]): void;
    }
    
    interface Terminal extends Element {
      // Terminal specific properties
      pty: any;
      
      // Terminal specific methods
      write(data: string): void;
      screenshot(xi?: number, xl?: number, yi?: number, yl?: number): string;
      term: string;
      bootstrap(): void;
      reset(): void;
    }
    
    interface Image extends Element {
      // Image specific properties
      file: string;
      
      // Image specific methods
      setImage(file: string): void;
    }
    
    interface Video extends Element {
      // Video specific properties
      file: string;
      
      // Video specific methods
      setVideo(file: string): void;
      play(): void;
      pause(): void;
      stop(): void;
    }
    
    interface Layout extends Element {
      // Layout specific properties
      layout: string;
      
      // Layout specific methods
      isRendered(): boolean;
      getLast(flex?: boolean): Element;
      getLastCoords(flex?: boolean): any;
    }
    
    interface Line extends Element {
      // Line specific properties
      ch: string;
      type: string;
      orientation: string;
      
      // Line specific methods
      setOrientation(orientation: string): void;
    }
    
    interface ScrollableBox extends Box {
      // ScrollableBox specific properties
      scrollable: boolean;
      
      // ScrollableBox specific methods
      scroll(offset: number): void;
      scrollTo(offset: number): void;
      scrollPerc(percentage: number): void;
      setScroll(offset: number): void;
      getScroll(): number;
      getScrollHeight(): number;
      getScrollPerc(): number;
      resetScroll(): void;
    }
    
    interface ScrollableText extends ScrollableBox {
      // ScrollableText specific methods
      scroll(offset: number): void;
      scrollTo(offset: number): void;
      scrollPerc(percentage: number): void;
      setScroll(offset: number): void;
      getScroll(): number;
      getScrollHeight(): number;
      getScrollPerc(): number;
      resetScroll(): void;
    }
    
    interface BigText extends Element {
      // BigText specific properties
      ch: string;
      
      // BigText specific methods
      setContent(content: string): void;
      getContent(): string;
    }
    
    interface Textarea extends ScrollableBox {
      // Textarea specific properties
      value: string;
      
      // Textarea specific methods
      readInput(callback: (err: Error | null, value: string) => void): void;
      input(callback: (err: Error | null, value: string) => void): void;
      setInput(text: string): void;
      clearInput(): void;
      submit(): void;
      cancel(): void;
      getValue(): string;
      setValue(value: string): void;
    }
    
    // Factory functions
    function screen(options?: ScreenOptions): Screen;
    function box(options?: NodeOptions): Box;
    function text(options?: NodeOptions): Element;
    function line(options?: NodeOptions): Line;
    function scrollableBox(options?: NodeOptions): ScrollableBox;
    function scrollableText(options?: NodeOptions): ScrollableText;
    function bigText(options?: NodeOptions): BigText;
    function list(options?: NodeOptions): List;
    function form(options?: NodeOptions): Element;
    function input(options?: NodeOptions): Element;
    function textarea(options?: NodeOptions): Textarea;
    function textbox(options?: NodeOptions): TextBox;
    function button(options?: NodeOptions): Button;
    function checkbox(options?: NodeOptions): Checkbox;
    function radioSet(options?: NodeOptions): RadioSet;
    function radioButton(options?: NodeOptions): RadioButton;
    function prompt(options?: NodeOptions): Prompt;
    function question(options?: NodeOptions): Question;
    function message(options?: NodeOptions): Message;
    function loading(options?: NodeOptions): Loading;
    function listbar(options?: NodeOptions): Listbar;
    function log(options?: NodeOptions): Log;
    function table(options?: NodeOptions): Table;
    function listtable(options?: NodeOptions): Table;
    function terminal(options?: NodeOptions): Terminal;
    function image(options?: NodeOptions): Image;
    function video(options?: NodeOptions): Video;
    function layout(options?: NodeOptions): Layout;
    function progressbar(options?: NodeOptions): ProgressBar;
    function filemanager(options?: NodeOptions): FileManager;
    function element(options?: NodeOptions): Element;
    function node(options?: NodeOptions): Element;
    
    // Program creation
    function program(options?: any): any;
    
    // Escape functions
    function escape(text: string): string;
    function cleanTags(text: string): string;
    function stripTags(text: string): string;
    function generateTags(style: any, text: string): string;
    
    // Utility functions
    function parseTags(text: string): any;
    function parseContent(content: string): any;
    function unicode(): any;
    function codeToKey(code: number): string;
    function keyToCode(key: string): number;
    const helpers: any;
    
    // Colors
    const colors: any;
    
    // Blessed program
    const Program: any;
    
    // Blessed widget
    const Widget: any;
    
    // Blessed node
    const Node: any;
    
    // Blessed screen
    const Screen: any;
    
    // Blessed element
    const Element: any;
    
    // Blessed box
    const Box: any;
    
    // Blessed text
    const Text: any;
    
    // Blessed line
    const Line: any;
    
    // Blessed scrollable box
    const ScrollableBox: any;
    
    // Blessed scrollable text
    const ScrollableText: any;
    
    // Blessed big text
    const BigText: any;
    
    // Blessed list
    const List: any;
    
    // Blessed form
    const Form: any;
    
    // Blessed input
    const Input: any;
    
    // Blessed textarea
    const Textarea: any;
    
    // Blessed textbox
    const Textbox: any;
    
    // Blessed button
    const Button: any;
    
    // Blessed checkbox
    const Checkbox: any;
    
    // Blessed radio set
    const RadioSet: any;
    
    // Blessed radio button
    const RadioButton: any;
    
    // Blessed prompt
    const Prompt: any;
    
    // Blessed question
    const Question: any;
    
    // Blessed message
    const Message: any;
    
    // Blessed loading
    const Loading: any;
    
    // Blessed listbar
    const Listbar: any;
    
    // Blessed log
    const Log: any;
    
    // Blessed table
    const Table: any;
    
    // Blessed listtable
    const ListTable: any;
    
    // Blessed terminal
    const Terminal: any;
    
    // Blessed image
    const Image: any;
    
    // Blessed video
    const Video: any;
    
    // Blessed layout
    const Layout: any;
    
    // Blessed progressbar
    const ProgressBar: any;
    
    // Blessed filemanager
    const FileManager: any;
  }
  
  export = blessed;
}