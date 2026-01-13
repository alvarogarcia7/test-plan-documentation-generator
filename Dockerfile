# Stage 1: deps - Build dependencies only
FROM rust:1.92-bookworm AS deps

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached)
RUN cargo build --release

# Stage 2: builder - Build the actual application
FROM rust:1.92-bookworm AS builder

WORKDIR /app

# Copy dependencies from deps stage
COPY --from=deps /app/target target
COPY --from=deps /usr/local/cargo /usr/local/cargo

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy full source
COPY src ./src
COPY tests ./tests

# Build the application against cached dependencies
RUN cargo build --release

# Stage 3: runtime - Final lightweight image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies: git
RUN apt-get update && \
    apt-get install -y git && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binaries from builder
COPY --from=builder /app/target/release/test-plan-doc-gen /usr/local/bin/test-plan-doc-gen

# Copy data directory
COPY data ./data

# Set default command
CMD ["test-plan-doc-gen"]
