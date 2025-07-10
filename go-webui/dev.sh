#!/bin/bash

# Development mode with auto-reload

set -e

echo "🔧 Starting Claude Flow Web UI in development mode..."

# Function to build and restart
build_and_restart() {
    echo "📦 Rebuilding..."
    ./build.sh
    echo "🔄 Restarting server..."
}

# Build initially
build_and_restart

# Start the server in background
./start.sh &
SERVER_PID=$!

# Watch for file changes (requires inotify-tools on Linux or fswatch on macOS)
if command -v fswatch &> /dev/null; then
    echo "👁️ Watching for file changes..."
    fswatch -o . | while read f; do
        echo "📁 Files changed, rebuilding..."
        kill $SERVER_PID 2>/dev/null || true
        build_and_restart
        ./start.sh &
        SERVER_PID=$!
    done
elif command -v inotifywait &> /dev/null; then
    echo "👁️ Watching for file changes..."
    while inotifywait -r -e modify,create,delete,move .; do
        echo "📁 Files changed, rebuilding..."
        kill $SERVER_PID 2>/dev/null || true
        build_and_restart
        ./start.sh &
        SERVER_PID=$!
    done
else
    echo "⚠️ File watching not available. Install fswatch (macOS) or inotify-tools (Linux) for auto-reload."
    wait $SERVER_PID
fi
