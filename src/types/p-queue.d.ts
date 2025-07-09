// Stub types for p-queue
declare module 'p-queue' {
  export default class PQueue {
    constructor(options?: { concurrency?: number });
    add<T>(fn: () => Promise<T>): Promise<T>;
    onIdle(): Promise<void>;
    clear(): void;
    readonly size: number;
  }
}