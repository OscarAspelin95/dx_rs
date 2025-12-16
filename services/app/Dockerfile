# Dockerfile for Dioxus Fullstack Web Application
# Multi-stage build for optimized image size

# ============================================
# Stage 1: Build the application
# ============================================
FROM rust:1.90-bookworm AS builder

WORKDIR /app

# Install system dependencies for Dioxus and WebAssembly
RUN apt-get update && apt-get install -y --no-install-recommends \
	build-essential \
	libssl-dev \
	pkg-config \
	curl \
	&& rm -rf /var/lib/apt/lists/*

# Add WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install Dioxus CLI
RUN cargo install dioxus-cli --version 0.7.2

# Copy workspace files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members
COPY app ./app
COPY api ./api
COPY fastq_service ./fastq_service
COPY shared ./shared

# Build the Dioxus app for web (fullstack mode)
WORKDIR /app/app
RUN dx build --release --platform web

# ============================================
# Stage 2: Runtime
# ============================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
	ca-certificates \
	libssl3 \
	&& rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd --create-home --shell /bin/bash appuser

WORKDIR /app

# Copy the built application from builder
# The dx build creates output in target/dx/<app-name>/release/web/
COPY --from=builder /app/target/dx/app/release/web/ ./

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

USER appuser

# Expose the port the app runs on
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# Run the server binary
ENTRYPOINT ["./server"]
