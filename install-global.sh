#!/bin/bash

# Global installation script for Claude-Flow (klaude)
set -e

echo "🚀 Installing Claude-Flow globally as 'klaude'..."

# Get the current directory (where the project is)
PROJECT_DIR="$(pwd)"
KLAUDE_SCRIPT="$PROJECT_DIR/klaude"

# Check if klaude script exists
if [ ! -f "$KLAUDE_SCRIPT" ]; then
    echo "❌ Error: klaude script not found in $PROJECT_DIR"
    echo "Please run this script from the claude-flow project directory"
    exit 1
fi

# Make sure klaude is executable
chmod +x "$KLAUDE_SCRIPT"

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -w "$HOME/.local/bin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    # Make sure ~/.local/bin exists and is in PATH
    mkdir -p "$HOME/.local/bin"
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        echo "⚠️  Adding $HOME/.local/bin to PATH"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
    fi
else
    echo "❌ Error: Cannot find writable directory for installation"
    echo "Please run with sudo or ensure ~/.local/bin exists and is writable"
    exit 1
fi

# Create the global klaude script
GLOBAL_KLAUDE="$INSTALL_DIR/klaude"

echo "📦 Installing klaude to $GLOBAL_KLAUDE..."

cat > "$GLOBAL_KLAUDE" << EOF
#!/usr/bin/env node

/**
 * Global klaude binary - Claude-Flow CLI wrapper
 * Installed from: $PROJECT_DIR
 */

import { spawn } from 'child_process';
import path from 'path';
import fs from 'fs';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Project directory where claude-flow is installed
const PROJECT_DIR = '$PROJECT_DIR';

// Try different CLI entry points in order of preference
const cliOptions = [
  path.join(PROJECT_DIR, 'src', 'cli', 'simple-cli.js'),
  path.join(PROJECT_DIR, 'cli.js'),
  path.join(PROJECT_DIR, 'src', 'cli', 'main.ts')
];

function findWorkingCli() {
  for (const cliPath of cliOptions) {
    if (fs.existsSync(cliPath)) {
      return cliPath;
    }
  }
  return null;
}

function runCli() {
  const cliPath = findWorkingCli();
  
  if (!cliPath) {
    console.error('❌ Error: Could not find claude-flow CLI implementation');
    console.error('Expected one of:');
    cliOptions.forEach(opt => console.error(\`  - \${opt}\`));
    console.error(\`\\nProject directory: \${PROJECT_DIR}\`);
    console.error('\\n💡 Try reinstalling or check if the project directory still exists');
    process.exit(1);
  }

  // Determine how to run the CLI
  let command, args;
  
  if (cliPath.endsWith('.js')) {
    command = 'node';
    args = [cliPath, ...process.argv.slice(2)];
  } else if (cliPath.endsWith('.ts')) {
    // Try tsx first, then npx tsx
    command = 'npx';
    args = ['tsx', cliPath, ...process.argv.slice(2)];
  }

  const child = spawn(command, args, {
    stdio: 'inherit',
    cwd: PROJECT_DIR
  });

  child.on('error', (error) => {
    console.error('❌ Failed to run klaude:', error.message);
    if (cliPath.endsWith('.ts')) {
      console.log('\\n💡 Try installing tsx: npm install -g tsx');
    }
    process.exit(1);
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

// Show help if no arguments
if (process.argv.length === 2) {
  console.log(\`
🤖 Klaude - Claude-Flow CLI (Global Installation)

Usage: klaude <command> [options]

Available commands:
  start         Start the orchestration system
  mcp           MCP server operations
  agent         Agent management
  swarm         Swarm operations
  sparc         SPARC methodology tools
  status        Show system status
  help          Show help information

Examples:
  klaude start                    # Start the system
  klaude mcp serve               # Start MCP server
  klaude agent create myagent    # Create an agent
  klaude status                  # Check status
  klaude help                    # Show detailed help

Project location: \${PROJECT_DIR}
For more information: klaude help
\`);
  process.exit(0);
}

runCli();
EOF

# Make the global script executable
chmod +x "$GLOBAL_KLAUDE"

echo "✅ klaude installed successfully!"
echo ""
echo "📍 Installation details:"
echo "   Global binary: $GLOBAL_KLAUDE"
echo "   Project directory: $PROJECT_DIR"
echo ""
echo "🧪 Testing installation..."
if command -v klaude >/dev/null 2>&1; then
    echo "✅ klaude is available in PATH"
    echo ""
    echo "🎉 Installation complete! You can now use 'klaude' from anywhere."
    echo ""
    echo "Try: klaude status"
else
    echo "⚠️  klaude may not be in your PATH yet"
    echo "   You may need to restart your terminal or run:"
    echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo ""
echo "📚 Next steps:"
echo "1. Set up MCP server for Claude Desktop (see below)"
echo "2. Test with: klaude mcp serve"
echo "3. Check status with: klaude status"
