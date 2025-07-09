# 🐳 Klaude-Flow Docker Image
FROM node:18-alpine AS builder

# 📦 Install build dependencies
RUN apk add --no-cache python3 make g++

# 🏗️ Build stage
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

# 📁 Copy source code
COPY . .

# 🔨 Build the application
RUN npm run build:ts

# 🏃 Production stage
FROM node:18-alpine

# 📋 Add metadata
LABEL maintainer="KooshaPari <koosha@klaudeflow.dev>"
LABEL version="1.0.72"
LABEL description="Advanced AI agent orchestration system for Klaude Code"

# 🔐 Create non-root user
RUN addgroup -g 1001 -S klaude && \
    adduser -S klaude -u 1001

# 📦 Install runtime dependencies
RUN apk add --no-cache \
    dumb-init \
    curl \
    git \
    openssh-client

# 🏠 Set working directory
WORKDIR /app

# 👤 Change ownership
RUN chown -R klaude:klaude /app

# 🔄 Switch to non-root user
USER klaude

# 📦 Copy production dependencies
COPY --from=builder --chown=klaude:klaude /app/node_modules ./node_modules

# 📁 Copy built application
COPY --from=builder --chown=klaude:klaude /app/dist ./dist
COPY --from=builder --chown=klaude:klaude /app/package.json ./
COPY --from=builder --chown=klaude:klaude /app/.claude ./.claude

# 🔧 Create CLI symlink
RUN ln -s /app/dist/cli/main.js /usr/local/bin/klaude-flow

# 📊 Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD node dist/cli/main.js --version || exit 1

# 🚀 Default command
ENTRYPOINT ["dumb-init", "--"]
CMD ["node", "dist/cli/main.js"]

# 📄 Expose common ports
EXPOSE 3000 3001 8080

# 🏷️ Add labels for better organization
LABEL org.opencontainers.image.source="https://github.com/KooshaPari/klaude-flow"
LABEL org.opencontainers.image.documentation="https://github.com/KooshaPari/klaude-flow#readme"
LABEL org.opencontainers.image.licenses="MIT"