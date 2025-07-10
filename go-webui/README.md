# Claude Flow Web UI (Go + WebAssembly)

A high-performance web interface for Claude Flow built with Go and WebAssembly.

## Features

- 🚀 **High Performance**: Go backend with WebAssembly frontend
- 🔄 **Real-time Updates**: WebSocket-based live data
- 📊 **Comprehensive Dashboard**: System metrics and status
- 🧠 **Neural Network Management**: Training and model management
- 🤖 **Agent Management**: Lifecycle and performance monitoring
- 💾 **Memory Browser**: Interactive memory exploration
- 🐝 **Swarm Visualizer**: Live topology visualization
- 🐙 **GitHub Integration**: Repository analysis and workflows
- 📈 **Performance Monitor**: Real-time metrics and alerts
- ⚙️ **Configuration Manager**: Interactive settings
- ⌨️ **Terminal Emulator**: Embedded command interface

## Quick Start

### Prerequisites

- Go 1.21 or later
- Modern web browser with WebAssembly support

### Build and Run

```bash
# Build the application
./build.sh

# Start the server
./start.sh

# Or run in development mode with auto-reload
./dev.sh
```

### Docker

```bash
# Build and run with Docker
docker-compose up --build

# Or build manually
docker build -t claude-flow-webui .
docker run -p 8080:8080 claude-flow-webui
```

## Architecture

- **Backend**: Go HTTP server with WebSocket support
- **Frontend**: Go WebAssembly module
- **UI**: Responsive HTML/CSS with real-time updates
- **Communication**: REST API + WebSocket for live data

## API Endpoints

- `GET /` - Main dashboard
- `GET /neural` - Neural network management
- `GET /agents` - Agent management
- `GET /memory` - Memory browser
- `GET /swarm` - Swarm visualizer
- `GET /api/*` - REST API endpoints
- `WS /ws` - WebSocket connection

## Development

### File Structure

```
go-webui/
├── main.go              # Main server
├── helpers.go           # Helper functions
├── templates.go         # HTML templates
├── wasm/
│   ├── main.go         # WebAssembly frontend
│   ├── main.wasm       # Compiled WASM module
│   └── wasm_exec.js    # Go WASM runtime
├── static/
│   ├── styles.css      # CSS styles
│   └── assets/         # Static assets
└── templates/          # HTML templates
```

### Building Components

1. **WebAssembly Module**: Handles UI rendering and interactions
2. **Go Server**: Provides REST API and WebSocket endpoints
3. **Templates**: HTML templates with embedded JavaScript
4. **Styles**: Responsive CSS with dark theme

### Adding New Features

1. Add new API endpoints in `main.go`
2. Add corresponding WASM handlers in `wasm/main.go`
3. Create HTML templates in `templates.go`
4. Add styles in `static/styles.css`

## Configuration

Environment variables:
- `PORT`: Server port (default: 8080)
- `HOST`: Server host (default: localhost)

## License

MIT License - see LICENSE file for details.
