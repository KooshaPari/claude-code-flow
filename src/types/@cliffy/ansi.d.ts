// Stub types for @cliffy/ansi
declare module '@cliffy/ansi/colors' {
  interface ColorFunction {
    (text: string): string;
    bold: (text: string) => string;
    dim: (text: string) => string;
    italic: (text: string) => string;
    underline: (text: string) => string;
  }
  
  export const colors: {
    red: ColorFunction;
    green: ColorFunction;
    blue: ColorFunction;
    yellow: ColorFunction;
    cyan: ColorFunction;
    magenta: ColorFunction;
    bold: ColorFunction;
    dim: ColorFunction;
    gray: ColorFunction;
    white: ColorFunction;
    black: ColorFunction;
    reset: string;
    underline: ColorFunction;
  };
}

export {};