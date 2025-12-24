# Build stage
FROM rust:1.70 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/stock_analyst /app/stock_analyst
COPY --from=builder /app/target/release/train_models /app/train_models
COPY --from=builder /app/target/release/evaluate_models /app/evaluate_models

# Copy web assets
COPY web ./web

# Copy configuration
COPY .env.example ./.env

# Expose port
EXPOSE 8080

# Run the application
CMD ["./stock_analyst"]