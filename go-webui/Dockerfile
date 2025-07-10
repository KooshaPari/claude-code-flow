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
