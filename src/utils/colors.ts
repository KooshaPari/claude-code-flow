/**
 * Cross-platform color utility for CLI output
 * Provides consistent color functions across different environments
 */

export interface ColorFunction {
  (text: string): string;
  bold?: ColorFunction;
  dim?: ColorFunction;
  italic?: ColorFunction;
  underline?: ColorFunction;
}

// ANSI color codes
const ANSI_CODES = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  italic: '\x1b[3m',
  underline: '\x1b[4m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  white: '\x1b[37m',
  gray: '\x1b[90m',
  black: '\x1b[30m',
  brightRed: '\x1b[91m',
  brightGreen: '\x1b[92m',
  brightYellow: '\x1b[93m',
  brightBlue: '\x1b[94m',
  brightMagenta: '\x1b[95m',
  brightCyan: '\x1b[96m',
  brightWhite: '\x1b[97m',
};

// Check if colors should be enabled
function shouldUseColors(): boolean {
  if (typeof process !== 'undefined') {
    // Node.js environment
    return !process.env.NO_COLOR && 
           (!!process.env.FORCE_COLOR || 
            (process.stdout && process.stdout.isTTY));
  }
  
  // Deno environment
  if (typeof (globalThis as any).Deno !== 'undefined') {
    try {
      return (globalThis as any).Deno.isatty((globalThis as any).Deno.stdout.rid);
    } catch {
      return true; // Default to colors in Deno
    }
  }
  
  return true; // Default to colors
}

let colorEnabled = shouldUseColors();

function createColorFunction(colorCode: string): ColorFunction {
  const colorFn = (text: string): string => {
    if (!colorEnabled) return text;
    return `${colorCode}${text}${ANSI_CODES.reset}`;
  };

  // Add style methods
  colorFn.bold = (text: string): string => {
    if (!colorEnabled) return text;
    return `${ANSI_CODES.bold}${colorCode}${text}${ANSI_CODES.reset}`;
  };
  
  colorFn.dim = (text: string): string => {
    if (!colorEnabled) return text;
    return `${ANSI_CODES.dim}${colorCode}${text}${ANSI_CODES.reset}`;
  };
  
  colorFn.italic = (text: string): string => {
    if (!colorEnabled) return text;
    return `${ANSI_CODES.italic}${colorCode}${text}${ANSI_CODES.reset}`;
  };
  
  colorFn.underline = (text: string): string => {
    if (!colorEnabled) return text;
    return `${ANSI_CODES.underline}${colorCode}${text}${ANSI_CODES.reset}`;
  };

  return colorFn;
}

// Create style-only functions
function createStyleFunction(styleCode: string): ColorFunction {
  return (text: string): string => {
    if (!colorEnabled) return text;
    return `${styleCode}${text}${ANSI_CODES.reset}`;
  };
}

export const colors = {
  // Colors
  red: Object.assign(createColorFunction(ANSI_CODES.red), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.red)
  }),
  green: Object.assign(createColorFunction(ANSI_CODES.green), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.green)
  }),
  yellow: Object.assign(createColorFunction(ANSI_CODES.yellow), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.yellow)
  }),
  blue: Object.assign(createColorFunction(ANSI_CODES.blue), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.blue)
  }),
  magenta: Object.assign(createColorFunction(ANSI_CODES.magenta), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.magenta)
  }),
  cyan: Object.assign(createColorFunction(ANSI_CODES.cyan), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.cyan)
  }),
  white: Object.assign(createColorFunction(ANSI_CODES.white), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.white)
  }),
  gray: Object.assign(createColorFunction(ANSI_CODES.gray), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.gray)
  }),
  grey: Object.assign(createColorFunction(ANSI_CODES.gray), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.gray)
  }), // Alias
  black: Object.assign(createColorFunction(ANSI_CODES.black), {
    bold: createColorFunction(ANSI_CODES.bold + ANSI_CODES.black)
  }),
  
  // Bright colors
  brightRed: createColorFunction(ANSI_CODES.brightRed),
  brightGreen: createColorFunction(ANSI_CODES.brightGreen),
  brightYellow: createColorFunction(ANSI_CODES.brightYellow),
  brightBlue: createColorFunction(ANSI_CODES.brightBlue),
  brightMagenta: createColorFunction(ANSI_CODES.brightMagenta),
  brightCyan: createColorFunction(ANSI_CODES.brightCyan),
  brightWhite: createColorFunction(ANSI_CODES.brightWhite),
  
  // Styles
  bold: createStyleFunction(ANSI_CODES.bold),
  dim: createStyleFunction(ANSI_CODES.dim),
  italic: createStyleFunction(ANSI_CODES.italic),
  underline: createStyleFunction(ANSI_CODES.underline),
  
  // Reset
  reset: ANSI_CODES.reset,
  
  // Utility functions
  setColorEnabled: (enabled: boolean) => {
    colorEnabled = enabled;
  },
  
  getColorEnabled: () => colorEnabled,
  
  stripColors: (text: string): string => {
    return text.replace(/\x1b\[[0-9;]*m/g, '');
  },
};

// Default export for convenience
export default colors;

// Aliases for common styling patterns
export const red = colors.red;
export const green = colors.green;
export const yellow = colors.yellow;
export const blue = colors.blue;
export const magenta = colors.magenta;
export const cyan = colors.cyan;
export const white = colors.white;
export const gray = colors.gray;
export const bold = colors.bold;
export const dim = colors.dim;
export const italic = colors.italic;
export const underline = colors.underline;