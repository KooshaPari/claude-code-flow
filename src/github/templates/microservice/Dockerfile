# Multi-stage build for {{PROJECT_NAME}} microservice
# Stage 1: Dependencies and build
FROM node:18-alpine AS builder

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies (including dev dependencies for build)
RUN npm ci --only=production --silent

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Stage 2: Production runtime
FROM node:18-alpine AS runtime

# Add metadata
LABEL maintainer="{{MAINTAINER_EMAIL}}"
LABEL description="{{PROJECT_DESCRIPTION}}"
LABEL version="{{VERSION}}"

# Create app user for security
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install only production dependencies
RUN npm ci --only=production --silent && \
    npm cache clean --force

# Copy built application from builder stage
COPY --from=builder --chown=appuser:appgroup /app/dist ./dist
COPY --from=builder --chown=appuser:appgroup /app/src ./src

# Copy configuration files
COPY --chown=appuser:appgroup .env.example .env
COPY --chown=appuser:appgroup docker-entrypoint.sh ./

# Make entrypoint script executable
RUN chmod +x docker-entrypoint.sh

# Create necessary directories
RUN mkdir -p logs temp && \
    chown -R appuser:appgroup logs temp

# Install dumb-init for proper signal handling
RUN apk add --no-cache dumb-init

# Switch to non-root user
USER appuser

# Expose port
EXPOSE {{PORT}}

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
  CMD curl -f http://localhost:{{PORT}}/health || exit 1

# Set environment variables
ENV NODE_ENV=production
ENV PORT={{PORT}}

# Use dumb-init for proper signal handling
ENTRYPOINT ["dumb-init", "--"]

# Start the application
CMD ["./docker-entrypoint.sh"]