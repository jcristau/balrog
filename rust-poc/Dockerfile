# Multi-stage build for Balrog Rust PoC
FROM rust:1.85 as builder

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/balrog-rust-poc /app/balrog-rust-poc

# Set default environment variables
ENV PORT=9010
ENV CACHE_CONTROL="public, max-age=90"

EXPOSE 9010

CMD ["/app/balrog-rust-poc"]
