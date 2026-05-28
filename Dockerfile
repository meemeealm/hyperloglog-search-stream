# ==========================================================
# STAGE 1: The Builder
# ==========================================================
FROM rust:latest AS builder

WORKDIR /app

# 1. Copy the configuration files
COPY Cargo.toml ./

# 3. Copy actual source code
COPY src ./src

RUN cargo build --release

# ==========================================================
# STAGE 2: The Final Tiny Runtime
# ==========================================================
FROM debian:trixie-slim

WORKDIR /app

# Install SSL certificates (needed if your app makes outbound HTTPS requests)
# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/mini_hll_search ./rust-engine

EXPOSE 3000

# Run the application
CMD ["./rust-engine"]