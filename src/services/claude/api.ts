// Claude API service stub
export interface ClaudeAPI {
  complete(options: {
    messages: any[];
    model: string;
    max_tokens: number;
    temperature?: number;
  }): Promise<{
    content: Array<{ text: string }>;
    usage?: {
      input_tokens: number;
      output_tokens: number;
    };
  }>;
}

export function createClaudeAPI(): ClaudeAPI {
  return {
    async complete(options) {
      return {
        content: [{ text: 'Mock response' }],
        usage: {
          input_tokens: 100,
          output_tokens: 200
        }
      };
    }
  };
}