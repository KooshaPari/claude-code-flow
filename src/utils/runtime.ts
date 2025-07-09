/**
 * Runtime detection and compatibility utilities for Deno/Node.js environments
 */

export type RuntimeEnvironment = 'deno' | 'node' | 'browser' | 'unknown';

/**
 * Detects the current runtime environment
 */
export function detectRuntime(): RuntimeEnvironment {
  // Check for Deno
  if (typeof globalThis !== 'undefined' && 'Deno' in globalThis && (globalThis as any).Deno?.version?.deno) {
    return 'deno';
  }
  
  // Check for Node.js
  if (typeof process !== 'undefined' && process?.versions?.node) {
    return 'node';
  }
  
  // Check for browser
  if (typeof window !== 'undefined' && typeof document !== 'undefined') {
    return 'browser';
  }
  
  return 'unknown';
}

/**
 * Gets the current runtime environment
 */
export const RUNTIME = detectRuntime();

/**
 * Checks if running in Deno
 */
export const isDeno = RUNTIME === 'deno';

/**
 * Checks if running in Node.js
 */
export const isNode = RUNTIME === 'node';

/**
 * Checks if running in browser
 */
export const isBrowser = RUNTIME === 'browser';

/**
 * Runtime-agnostic environment variables
 */
export const env = {
  get(key: string): string | undefined {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.env.get(key);
    } else if (isNode) {
      return process.env[key];
    }
    return undefined;
  },
  
  set(key: string, value: string): void {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      (globalThis as any).Deno.env.set(key, value);
    } else if (isNode) {
      process.env[key] = value;
    }
  },
  
  has(key: string): boolean {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.env.has(key);
    } else if (isNode) {
      return key in process.env;
    }
    return false;
  },
  
  delete(key: string): void {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      (globalThis as any).Deno.env.delete(key);
    } else if (isNode) {
      delete process.env[key];
    }
  },
  
  toObject(): Record<string, string> {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.env.toObject();
    } else if (isNode) {
      return { ...process.env } as Record<string, string>;
    }
    return {};
  }
};

/**
 * Runtime-agnostic command line arguments
 */
export const args = {
  get(): string[] {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.args;
    } else if (isNode) {
      return process.argv.slice(2);
    }
    return [];
  }
};

/**
 * Runtime-agnostic process information
 */
export const processInfo = {
  get pid(): number {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.pid;
    } else if (isNode) {
      return process.pid;
    }
    return 0;
  },
  
  get platform(): string {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.build.os;
    } else if (isNode) {
      return process.platform;
    }
    return 'unknown';
  },
  
  get arch(): string {
    if (isDeno && typeof globalThis !== 'undefined' && 'Deno' in globalThis) {
      return (globalThis as any).Deno.build.arch;
    } else if (isNode) {
      return process.arch;
    }
    return 'unknown';
  },
  
  exit(code = 0): never {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      (globalThis as any).Deno.exit(code);
    } else if (isNode) {
      process.exit(code);
    }
    throw new Error('Process exit not supported in this environment');
  },
  
  kill(pid: number, signal: string | number = 'SIGTERM'): void {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      (globalThis as any).Deno.kill(pid, signal as any);
    } else if (isNode) {
      process.kill(pid, signal as NodeJS.Signals);
    } else {
      throw new Error('Process kill not supported in this environment');
    }
  },
  
  cwd(): string {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      return (globalThis as any).Deno.cwd();
    } else if (isNode) {
      return process.cwd();
    }
    throw new Error('Current working directory not available in this environment');
  },
  
  chdir(path: string): void {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      (globalThis as any).Deno.chdir(path);
    } else if (isNode) {
      process.chdir(path);
    } else {
      throw new Error('Change directory not supported in this environment');
    }
  }
};

/**
 * Runtime-agnostic memory usage
 */
export const memoryUsage = {
  get(): {
    rss: number;
    heapUsed: number;
    heapTotal: number;
    external: number;
    arrayBuffers: number;
  } {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      const mem = (globalThis as any).Deno.memoryUsage();
      return {
        rss: mem.rss,
        heapUsed: mem.heapUsed,
        heapTotal: mem.heapTotal,
        external: mem.external,
        arrayBuffers: mem.arrayBuffers
      };
    } else if (isNode) {
      const mem = process.memoryUsage();
      return {
        rss: mem.rss,
        heapUsed: mem.heapUsed,
        heapTotal: mem.heapTotal,
        external: mem.external,
        arrayBuffers: mem.arrayBuffers
      };
    }
    return {
      rss: 0,
      heapUsed: 0,
      heapTotal: 0,
      external: 0,
      arrayBuffers: 0
    };
  }
};

/**
 * Runtime-agnostic signal handling
 */
export const signals = {
  addListener(signal: string, handler: () => void): void {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      (globalThis as any).Deno.addSignalListener(signal as any, handler);
    } else if (isNode) {
      process.on(signal as NodeJS.Signals, handler);
    }
  },
  
  removeListener(signal: string, handler: () => void): void {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      (globalThis as any).Deno.removeSignalListener(signal as any, handler);
    } else if (isNode) {
      process.off(signal as NodeJS.Signals, handler);
    }
  }
};

/**
 * Runtime-agnostic standard input/output
 */
export const stdio = {
  get stdin() {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      return (globalThis as any).Deno.stdin;
    } else if (isNode) {
      return process.stdin;
    }
    throw new Error('stdin not available in this environment');
  },
  
  get stdout() {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      return (globalThis as any).Deno.stdout;
    } else if (isNode) {
      return process.stdout;
    }
    throw new Error('stdout not available in this environment');
  },
  
  get stderr() {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      return (globalThis as any).Deno.stderr;
    } else if (isNode) {
      return process.stderr;
    }
    throw new Error('stderr not available in this environment');
  }
};

/**
 * Runtime-agnostic file system operations
 */
export const fs = {
  async readTextFile(path: string): Promise<string> {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      return await (globalThis as any).Deno.readTextFile(path);
    } else if (isNode) {
      const { readFile } = await import('fs/promises');
      return await readFile(path, 'utf-8');
    }
    throw new Error('File system operations not supported in this environment');
  },
  
  async writeTextFile(path: string, data: string): Promise<void> {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      await (globalThis as any).Deno.writeTextFile(path, data);
    } else if (isNode) {
      const { writeFile } = await import('fs/promises');
      await writeFile(path, data, 'utf-8');
    } else {
      throw new Error('File system operations not supported in this environment');
    }
  },
  
  async mkdir(path: string, options?: { recursive?: boolean }): Promise<void> {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      await (globalThis as any).Deno.mkdir(path, options);
    } else if (isNode) {
      const { mkdir } = await import('fs/promises');
      await mkdir(path, options);
    } else {
      throw new Error('File system operations not supported in this environment');
    }
  },
  
  async remove(path: string): Promise<void> {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      await (globalThis as any).Deno.remove(path);
    } else if (isNode) {
      const { unlink } = await import('fs/promises');
      await unlink(path);
    } else {
      throw new Error('File system operations not supported in this environment');
    }
  },
  
  async stat(path: string): Promise<{ isFile: boolean; isDirectory: boolean; size: number; mtime: Date | null }> {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      const info = await (globalThis as any).Deno.stat(path);
      return {
        isFile: info.isFile,
        isDirectory: info.isDirectory,
        size: info.size,
        mtime: info.mtime
      };
    } else if (isNode) {
      const { stat } = await import('fs/promises');
      const info = await stat(path);
      return {
        isFile: info.isFile(),
        isDirectory: info.isDirectory(),
        size: info.size,
        mtime: info.mtime
      };
    }
    throw new Error('File system operations not supported in this environment');
  },
  
  async readDir(path: string): Promise<Array<{ name: string; isFile: boolean; isDirectory: boolean }>> {
    if (isDeno && typeof globalThis !== "undefined" && "Deno" in globalThis) {
      const entries = [];
      for await (const entry of (globalThis as any).Deno.readDir(path)) {
        entries.push({
          name: entry.name,
          isFile: entry.isFile,
          isDirectory: entry.isDirectory
        });
      }
      return entries;
    } else if (isNode) {
      const { readdir } = await import('fs/promises');
      const entries = await readdir(path, { withFileTypes: true });
      return entries.map(entry => ({
        name: entry.name,
        isFile: entry.isFile(),
        isDirectory: entry.isDirectory()
      }));
    }
    throw new Error('File system operations not supported in this environment');
  }
};

/**
 * Runtime-agnostic console operations
 */
export const console_ = {
  clear(): void {
    if (isDeno || isNode) {
      console.clear();
    }
  },
  
  log(...args: unknown[]): void {
    console.log(...args);
  },
  
  error(...args: unknown[]): void {
    console.error(...args);
  },
  
  warn(...args: unknown[]): void {
    console.warn(...args);
  },
  
  info(...args: unknown[]): void {
    console.info(...args);
  }
};

/**
 * Runtime-agnostic text encoder/decoder
 */
export const textCodec = {
  encode(text: string): Uint8Array {
    return new TextEncoder().encode(text);
  },
  
  decode(data: Uint8Array): string {
    return new TextDecoder().decode(data);
  }
};

/**
 * Runtime compatibility errors
 */
export class RuntimeCompatibilityError extends Error {
  constructor(message: string, public runtime: RuntimeEnvironment) {
    super(`Runtime compatibility error (${runtime}): ${message}`);
    this.name = 'RuntimeCompatibilityError';
  }
}

/**
 * Ensures the current runtime supports the required features
 */
export function requireRuntime(required: RuntimeEnvironment[]): void {
  if (!required.includes(RUNTIME)) {
    throw new RuntimeCompatibilityError(
      `This feature requires ${required.join(' or ')} runtime, but detected ${RUNTIME}`,
      RUNTIME
    );
  }
}

/**
 * Warns about runtime compatibility issues
 */
export function warnRuntimeCompatibility(feature: string, supportedRuntimes: RuntimeEnvironment[]): void {
  if (!supportedRuntimes.includes(RUNTIME)) {
    console.warn(
      `Warning: ${feature} may not work properly in ${RUNTIME}. ` +
      `Supported runtimes: ${supportedRuntimes.join(', ')}`
    );
  }
}