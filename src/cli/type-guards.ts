/**
 * Type guard utilities for CLI flag parsing
 * Provides safe type checking and conversion for CLI flags
 */

/**
 * Type guard to check if a value is a boolean
 */
export function isBoolean(value: unknown): value is boolean {
  return typeof value === 'boolean';
}

/**
 * Type guard to check if a value is a string
 */
export function isString(value: unknown): value is string {
  return typeof value === 'string';
}

/**
 * Type guard to check if a value is a number
 */
export function isNumber(value: unknown): value is number {
  return typeof value === 'number' && !isNaN(value);
}

/**
 * Type guard to check if a value is a valid integer string
 */
export function isIntegerString(value: unknown): value is string {
  return isString(value) && /^-?\d+$/.test(value);
}

/**
 * Type guard to check if a value is a valid float string
 */
export function isFloatString(value: unknown): value is string {
  return isString(value) && /^-?\d*\.?\d+$/.test(value);
}

/**
 * Safely get a boolean flag value
 */
export function getBooleanFlag(flags: Record<string, unknown>, key: string, defaultValue: boolean = false): boolean {
  const value = flags[key];
  if (isBoolean(value)) {
    return value;
  }
  if (isString(value)) {
    return value.toLowerCase() === 'true' || value === '1';
  }
  return defaultValue;
}

/**
 * Safely get a string flag value
 */
export function getStringFlag(flags: Record<string, unknown>, key: string, defaultValue: string = ''): string {
  const value = flags[key];
  if (isString(value)) {
    return value;
  }
  if (isNumber(value)) {
    return value.toString();
  }
  return defaultValue;
}

/**
 * Safely get a number flag value
 */
export function getNumberFlag(flags: Record<string, unknown>, key: string, defaultValue: number = 0): number {
  const value = flags[key];
  if (isNumber(value)) {
    return value;
  }
  if (isString(value)) {
    const parsed = parseFloat(value);
    if (!isNaN(parsed)) {
      return parsed;
    }
  }
  return defaultValue;
}

/**
 * Safely get an integer flag value
 */
export function getIntegerFlag(flags: Record<string, unknown>, key: string, defaultValue: number = 0): number {
  const value = flags[key];
  if (isNumber(value)) {
    return Math.floor(value);
  }
  if (isString(value)) {
    const parsed = parseInt(value, 10);
    if (!isNaN(parsed)) {
      return parsed;
    }
  }
  return defaultValue;
}

/**
 * Safely get a flag value with multiple possible keys (e.g., 'maxAgents' or 'max-agents')
 */
export function getMultiKeyFlag<T>(
  flags: Record<string, unknown>,
  keys: string[],
  getter: (flags: Record<string, unknown>, key: string, defaultValue: T) => T,
  defaultValue: T
): T {
  for (const key of keys) {
    if (key in flags) {
      return getter(flags, key, defaultValue);
    }
  }
  return defaultValue;
}

/**
 * Safely get a string array flag value (comma-separated)
 */
export function getStringArrayFlag(flags: Record<string, unknown>, key: string, defaultValue: string[] = []): string[] {
  const value = flags[key];
  if (isString(value)) {
    return value.split(',').map(s => s.trim()).filter(s => s.length > 0);
  }
  if (Array.isArray(value)) {
    return value.map(v => isString(v) ? v : String(v));
  }
  return defaultValue;
}

/**
 * Validate that a flag value is one of the allowed values
 */
export function validateEnumFlag<T extends string>(
  value: unknown,
  allowedValues: readonly T[],
  defaultValue: T
): T {
  if (isString(value) && allowedValues.includes(value as T)) {
    return value as T;
  }
  return defaultValue;
}

/**
 * Type-safe flag parsing utilities
 */
export const FlagParser = {
  boolean: getBooleanFlag,
  string: getStringFlag,
  number: getNumberFlag,
  integer: getIntegerFlag,
  stringArray: getStringArrayFlag,
  multiKey: getMultiKeyFlag,
  validateEnum: validateEnumFlag
};