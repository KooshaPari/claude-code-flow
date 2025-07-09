// Deno polyfill for Node.js environment
import * as fs from 'fs/promises';
import * as path from 'path';
import { spawn } from 'child_process';

// Global Deno polyfill
(globalThis as any).Deno = {
  args: process.argv.slice(2),
  
  env: {
    get: (key: string) => process.env[key],
    set: (key: string, value: string) => {
      process.env[key] = value;
    }
  },
  
  readTextFile: async (filePath: string) => {
    return await fs.readFile(filePath, 'utf-8');
  },
  
  writeTextFile: async (filePath: string, content: string) => {
    await fs.writeFile(filePath, content, 'utf-8');
  },
  
  mkdir: async (dirPath: string, options?: { recursive?: boolean }) => {
    await fs.mkdir(dirPath, { recursive: options?.recursive ?? true });
  },
  
  exit: (code?: number) => {
    process.exit(code);
  },
  
  connect: async (options: any) => {
    // Mock connection
    return {
      rid: Math.random(),
      localAddr: { hostname: 'localhost', port: options.port },
      remoteAddr: { hostname: options.hostname, port: options.port },
      read: async () => 0,
      write: async () => 0,
      close: () => {}
    };
  },
  
  Command: class {
    constructor(private command: string, private options?: any) {}
    
    spawn() {
      return spawn(this.command, [], this.options);
    }
    
    async output() {
      return { success: true, code: 0, stdout: '', stderr: '' };
    }
  }
};