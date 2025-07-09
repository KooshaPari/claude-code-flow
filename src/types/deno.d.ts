// Stub types for Deno global in Node.js environment
declare global {
  namespace Deno {
    interface ConnectOptions {
      port: number;
      hostname?: string;
      transport?: "tcp";
    }
    
    interface Conn {
      rid: number;
      localAddr: any;
      remoteAddr: any;
      read(buf: Uint8Array): Promise<number | null>;
      write(buf: Uint8Array): Promise<number>;
      close(): void;
    }
    
    function connect(options: ConnectOptions): Promise<Conn>;
    
    const args: string[];
    const env: {
      get(key: string): string | undefined;
      set(key: string, value: string): void;
    };
    
    function readTextFile(path: string): Promise<string>;
    function writeTextFile(path: string, data: string): Promise<void>;
    function mkdir(path: string, options?: { recursive?: boolean }): Promise<void>;
    function exit(code?: number): never;
    
    class Command {
      constructor(command: string, options?: any);
      spawn(): any;
      output(): Promise<any>;
    }
  }
}