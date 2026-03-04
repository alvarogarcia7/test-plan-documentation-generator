# Stage 1: deps - Build dependencies only
FROM rust:1.92-bookworm AS deps

WORKDIR /app

# Install sccache
COPY scripts/install-sccache.sh scripts/lib/logger.sh /tmp/scripts/
COPY scripts/lib /tmp/scripts/lib/
RUN chmod +x /tmp/scripts/install-sccache.sh && \
    /tmp/scripts/install-sccache.sh --ci && \
    rm -rf /tmp/scripts

# Set sccache environment variables
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/app/.sccache/docker

# Create cache directory and copy host cache if it exists
RUN mkdir -p /app/.sccache/docker
COPY .sccache/host /app/.sccache/docker/ 2>/dev/null || true

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached)
RUN cargo build --release

# Display cache statistics
RUN sccache --show-stats

# Stage 2: builder - Build the actual application
FROM rust:1.92-bookworm AS builder

WORKDIR /app

# Install sccache
COPY scripts/install-sccache.sh scripts/lib/logger.sh /tmp/scripts/
COPY scripts/lib /tmp/scripts/lib/
RUN chmod +x /tmp/scripts/install-sccache.sh && \
    /tmp/scripts/install-sccache.sh --ci && \
    rm -rf /tmp/scripts

# Set sccache environment variables
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/app/.sccache/docker

# Create cache directory and copy host cache if it exists
RUN mkdir -p /app/.sccache/docker
COPY .sccache/host /app/.sccache/docker/ 2>/dev/null || true

# Copy dependencies from deps stage
COPY --from=deps /app/target target
COPY --from=deps /usr/local/cargo /usr/local/cargo
COPY --from=deps /app/.sccache/docker /app/.sccache/docker

# Copy manifests
COPY Cargo.toml Cargo.lock ./

COPY Makefile ./

# Copy full source
COPY src ./src
COPY tests ./tests
COPY data ./data

# Build the application against cached dependencies
RUN cargo build --release

# Display cache statistics
RUN sccache --show-stats

RUN RUST_BACKTRACE=full cargo test

RUN make test

RUN ./target/release/test-plan-doc-gen \
        --output ./data/output.actual.md \
        --container ./data/container/schema.json ./data/container/template.j2 ./data/container/data.yml \
        --test-case ./data/test_case/schema.json ./data/test_case/template.j2 ./data/test_case/*yml

RUN ./target/release/test-plan-doc-gen \
    --output ./data/output.actual.md \
    --container ./data/container/schema.json ./data/container/template.j2 ./data/container/data.yml \
    --test-case ./data/verification_methods ./data/test_case/gsma_4.4.2.2_TC.yml ./data/test_case/gsma_4.4.2.3_TC.yml ./data/test_case/gsma_4.4.2.4_AN.yml ./data/test_case/gsma_4.4.2.5_DM.yml ./data/test_case/gsma_4.4.2.6_IN.yml

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
