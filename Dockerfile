# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /app

# Install ALL build dependencies
# build-essential provides gcc/make needed for rusqlite bundled SQLite compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Reduce memory usage during compilation
ENV CARGO_BUILD_JOBS=2
ENV CARGO_INCREMENTAL=0
ENV RUST_BACKTRACE=1

# Copy workspace manifest and lock first (Docker layer caching)
COPY Cargo.toml Cargo.lock ./

# Copy all workspace crates
COPY crates/ crates/
COPY server/ server/
COPY rules/ rules/

# Build only the server binary in release mode
RUN cargo build --release -p mcpshield-server

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from builder
COPY --from=builder /app/target/release/mcpshield-server .

# Copy rules (needed at runtime for detection)
COPY rules/ rules/

# Create persistent data directory for SQLite
RUN mkdir -p /data

# Default environment
ENV WAITLIST_DB=/data/waitlist.db
ENV PORT=8080
ENV HOST=0.0.0.0

EXPOSE 8080

CMD ["./mcpshield-server"]
