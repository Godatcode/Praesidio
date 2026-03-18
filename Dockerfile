# Stage 1: Build
FROM rust:1.82-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY server/ server/
COPY rules/ rules/

# Build only the server binary in release mode
RUN cargo build --release -p mcpshield-server

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/mcpshield-server .

# Copy rules (needed at runtime)
COPY rules/ rules/

# Create data directory for SQLite
RUN mkdir -p /data

ENV WAITLIST_DB=/data/waitlist.db
ENV PORT=8080

EXPOSE 8080

CMD ["./mcpshield-server"]
