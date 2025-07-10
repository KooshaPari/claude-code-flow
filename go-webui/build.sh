#!/bin/bash

# Claude Flow Web UI Build Script

set -e

echo "🔧 Building Claude Flow Web UI..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Go is installed
if ! command -v go &> /dev/null; then
    print_error "Go is not installed. Please install Go 1.21 or later."
    exit 1
fi

# Check Go version
GO_VERSION=$(go version | awk '{print $3}' | sed 's/go//')
REQUIRED_VERSION="1.21"

if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$GO_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
    print_error "Go version $REQUIRED_VERSION or later is required. Current version: $GO_VERSION"
    exit 1
fi

print_success "Go version $GO_VERSION detected"

# Create necessary directories
print_status "Creating directories..."
mkdir -p wasm
mkdir -p static
mkdir -p templates

# Download Go dependencies
print_status "Downloading Go dependencies..."
go mod tidy
if [ $? -ne 0 ]; then
    print_error "Failed to download dependencies"
    exit 1
fi

# Build WebAssembly module
print_status "Building WebAssembly module..."
cd wasm

# Set environment variables for WASM build
export GOOS=js
export GOARCH=wasm

# Build the WASM module
go build -o main.wasm main.go
if [ $? -ne 0 ]; then
    print_error "Failed to build WebAssembly module"
    exit 1
fi

print_success "WebAssembly module built: wasm/main.wasm"

# Copy wasm_exec.js from Go installation
WASM_EXEC_JS=$(go env GOROOT)/misc/wasm/wasm_exec.js
if [ -f "$WASM_EXEC_JS" ]; then
    cp "$WASM_EXEC_JS" ./wasm_exec.js
    print_success "Copied wasm_exec.js"
else
    print_warning "wasm_exec.js not found in Go installation. You may need to download it manually."
fi

cd ..

# Build the main server
print_status "Building main server..."
unset GOOS
unset GOARCH

go build -o claude-flow-webui main.go helpers.go templates.go
if [ $? -ne 0 ]; then
    print_error "Failed to build main server"
    exit 1
fi

print_success "Main server built: claude-flow-webui"

# Create a simple favicon
print_status "Creating assets..."
if command -v convert &> /dev/null; then
    # Create a simple favicon using ImageMagick if available
    convert -size 32x32 xc:none -fill "#2196F3" -draw "circle 16,16 16,8" static/favicon.ico 2>/dev/null || true
fi

# Create a basic robots.txt
cat > static/robots.txt << EOF
User-agent: *
Disallow: /api/
Allow: /
EOF

# Create a basic manifest.json for PWA
cat > static/manifest.json << EOF
{
  "name": "Claude Flow Web UI",
  "short_name": "Claude Flow",
  "description": "AI-powered development workflow automation",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0f1419",
  "theme_color": "#2196F3",
  "icons": [
    {
      "src": "/static/favicon.ico",
      "sizes": "32x32",
      "type": "image/x-icon"
    }
  ]
}
EOF

# Create start script
print_status "Creating start script..."
cat > start.sh << 'EOF'
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
EOF

chmod +x start.sh

# Create development script
cat > dev.sh << 'EOF'
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
EOF

chmod +x dev.sh

# Create Docker support
print_status "Creating Docker configuration..."
cat > Dockerfile << 'EOF'
# Claude Flow Web UI Docker Image

FROM golang:1.21-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git

# Set working directory
WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build WebAssembly module
ENV GOOS=js GOARCH=wasm
RUN cd wasm && go build -o main.wasm main.go

# Build main server
ENV GOOS=linux GOARCH=amd64
RUN go build -o claude-flow-webui main.go helpers.go templates.go

# Copy wasm_exec.js
RUN cp $(go env GOROOT)/misc/wasm/wasm_exec.js wasm/

# Final stage
FROM alpine:latest

# Install ca-certificates for HTTPS
RUN apk --no-cache add ca-certificates

# Create non-root user
RUN addgroup -g 1000 -S appuser && \
    adduser -u 1000 -S appuser -G appuser

# Set working directory
WORKDIR /app

# Copy binary and assets from builder
COPY --from=builder /app/claude-flow-webui .
COPY --from=builder /app/wasm ./wasm/
COPY --from=builder /app/static ./static/
COPY --from=builder /app/templates ./templates/

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Start the application
CMD ["./claude-flow-webui"]
EOF

# Create docker-compose.yml
cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  claude-flow-webui:
    build: .
    ports:
      - "8080:8080"
    environment:
      - PORT=8080
      - HOST=0.0.0.0
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  # Optional: Add reverse proxy with SSL
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - claude-flow-webui
    restart: unless-stopped
EOF

# Create nginx configuration
cat > nginx.conf << 'EOF'
events {
    worker_connections 1024;
}

http {
    upstream claude-flow {
        server claude-flow-webui:8080;
    }

    server {
        listen 80;
        server_name localhost;

        location / {
            proxy_pass http://claude-flow;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        location /ws {
            proxy_pass http://claude-flow;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
EOF

# Create README for the web UI
cat > README.md << 'EOF'
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
EOF

# Final checks and summary
print_status "Running final checks..."

# Check if all files were created
REQUIRED_FILES=(
    "claude-flow-webui"
    "wasm/main.wasm"
    "wasm/wasm_exec.js"
    "static/styles.css"
    "start.sh"
    "dev.sh"
    "Dockerfile"
    "docker-compose.yml"
)

MISSING_FILES=()
for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        MISSING_FILES+=("$file")
    fi
done

if [ ${#MISSING_FILES[@]} -ne 0 ]; then
    print_warning "Some files were not created:"
    for file in "${MISSING_FILES[@]}"; do
        echo "  - $file"
    done
fi

# Display file sizes
print_status "Build summary:"
echo "  📦 Main server: $(du -h claude-flow-webui 2>/dev/null | cut -f1 || echo 'N/A')"
echo "  🧠 WASM module: $(du -h wasm/main.wasm 2>/dev/null | cut -f1 || echo 'N/A')"
echo "  🎨 CSS styles: $(du -h static/styles.css 2>/dev/null | cut -f1 || echo 'N/A')"

print_success "Build completed successfully!"
echo ""
echo "🚀 To start the server:"
echo "   ./start.sh"
echo ""
echo "🔧 To start in development mode:"
echo "   ./dev.sh"
echo ""
echo "🐳 To run with Docker:"
echo "   docker-compose up --build"
echo ""
print_status "The web UI will be available at http://localhost:8080"
EOF

chmod +x build.sh