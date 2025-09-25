# Production Dockerfile for Tauri Desktop Application
FROM node:20-alpine AS frontend-builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY tsconfig*.json ./
COPY vite.config.ts ./
COPY tailwind.config.js ./
COPY postcss.config.js ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY src/ ./src/
COPY public/ ./public/
COPY index.html ./

# Build frontend
RUN npm run build

# Rust/Tauri build stage
FROM rust:1.75-slim AS tauri-builder

# Install system dependencies for Tauri
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/

# Copy built frontend
COPY --from=frontend-builder /app/dist ./dist/

# Copy Tauri source
COPY src-tauri/ ./src-tauri/

# Build Tauri app
WORKDIR /app/src-tauri
RUN cargo build --release

# Final stage - minimal runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-37 \
    libgtk-3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built application
COPY --from=tauri-builder /app/src-tauri/target/release/ez-tauri ./

# Create non-root user
RUN useradd -r -s /bin/false tauri-app
USER tauri-app

# Expose any required ports (if your app has a web server component)
# EXPOSE 8080

ENTRYPOINT ["./ez-tauri"]