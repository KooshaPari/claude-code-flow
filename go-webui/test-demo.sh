#!/bin/bash

# Claude Flow Web UI Demo Script

echo "🌊 Claude Flow Web UI Demo"
echo "=========================="
echo ""

# Check if server is already running
if lsof -i:8080 > /dev/null 2>&1; then
    echo "⚠️ Port 8080 is already in use. Stopping existing server..."
    pkill -f claude-flow-webui || true
    sleep 2
fi

echo "🚀 Starting Claude Flow Web UI server..."
echo ""

# Start the server in background
./claude-flow-webui &
SERVER_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 3

# Check if server started successfully
if ! lsof -i:8080 > /dev/null 2>&1; then
    echo "❌ Server failed to start"
    exit 1
fi

echo "✅ Server started successfully!"
echo ""
echo "📊 Web UI Endpoints:"
echo "   Dashboard:     http://localhost:8080"
echo "   Neural UI:     http://localhost:8080/neural"
echo "   Agent Manager: http://localhost:8080/agents"
echo "   Memory Browser: http://localhost:8080/memory"
echo "   Swarm Visualizer: http://localhost:8080/swarm"
echo "   GitHub Integration: http://localhost:8080/github"
echo "   Performance Monitor: http://localhost:8080/performance"
echo "   Configuration: http://localhost:8080/config"
echo "   Terminal: http://localhost:8080/terminal"
echo ""
echo "🔗 API Endpoints:"
echo "   Status:        http://localhost:8080/api/dashboard"
echo "   Health:        http://localhost:8080/health"
echo "   WebSocket:     ws://localhost:8080/ws"
echo ""

# Test API endpoints
echo "🧪 Testing API endpoints..."
echo ""

echo "1. Health Check:"
if curl -s http://localhost:8080/health > /dev/null; then
    echo "   ✅ Health endpoint responding"
else
    echo "   ❌ Health endpoint not responding"
fi

echo "2. Dashboard API:"
if curl -s http://localhost:8080/api/dashboard > /dev/null; then
    echo "   ✅ Dashboard API responding"
else
    echo "   ❌ Dashboard API not responding"
fi

echo "3. Services Status:"
if curl -s http://localhost:8080/api/services/status > /dev/null; then
    echo "   ✅ Services status responding"
else
    echo "   ❌ Services status not responding"
fi

echo ""
echo "📱 Features Available:"
echo "   🎯 Real-time dashboard with system metrics"
echo "   🧠 Neural network training and management"
echo "   🤖 Agent lifecycle and performance monitoring"
echo "   💾 Interactive memory browser and search"
echo "   🐝 Live swarm topology visualization"
echo "   🐙 GitHub repository analysis and workflows"
echo "   📈 Performance monitoring with alerts"
echo "   ⚙️ Configuration management interface"
echo "   ⌨️ Embedded terminal emulator"
echo "   🔗 WebSocket real-time updates"
echo ""

# Show server logs for a few seconds
echo "📋 Server logs (last 10 lines):"
echo "================================"
sleep 2

echo ""
echo "🎉 Demo complete!"
echo ""
echo "💡 Tips:"
echo "   - Open http://localhost:8080 in your browser"
echo "   - Try the different navigation tabs"
echo "   - Check the real-time WebSocket updates"
echo "   - Explore the API endpoints"
echo ""
echo "🛑 To stop the server:"
echo "   kill $SERVER_PID"
echo "   or"
echo "   pkill -f claude-flow-webui"
echo ""

# Keep server running and show PID
echo "🔄 Server running in background (PID: $SERVER_PID)"
echo "📊 Access the dashboard: http://localhost:8080"
echo ""
echo "Press Ctrl+C to stop the demo and server"

# Wait for user to stop
trap "echo ''; echo '🛑 Stopping server...'; kill $SERVER_PID 2>/dev/null; echo '✅ Demo stopped'; exit 0" INT

# Keep script running
while kill -0 $SERVER_PID 2>/dev/null; do
    sleep 1
done

echo "❌ Server stopped unexpectedly"