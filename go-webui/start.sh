#!/bin/bash

# Start Claude Flow Web UI

set -e

PORT=${PORT:-8080}
HOST=${HOST:-localhost}

echo "🌊 Starting Claude Flow Web UI..."
echo "📊 Dashboard: http://$HOST:$PORT"
echo "🧠 Neural UI: http://$HOST:$PORT/neural"
echo "🤖 Agent Manager: http://$HOST:$PORT/agents"
echo "💾 Memory Browser: http://$HOST:$PORT/memory"
echo "🐝 Swarm Visualizer: http://$HOST:$PORT/swarm"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

./claude-flow-webui
