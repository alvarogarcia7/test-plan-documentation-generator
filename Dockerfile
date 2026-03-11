FROM rust:1.92-bookworm AS deps

WORKDIR /app

# Install sccache
COPY scripts/install-sccache.sh scripts/lib/logger.sh /tmp/scripts/
COPY scripts/lib /tmp/scripts/lib/
RUN apt-get update && \
    apt-get install -y sccache && \
    rm -rf /var/lib/apt/lists/*

RUN whereis sccache
RUN sccache --version

ENV HOME="/root"
ENV PATH="$PATH:$HOME/.cargo/bin"

# Create cache directory and copy host cache if it exists
RUN mkdir -p /app/.sccache/docker
#COPY .sccache/host /app/.sccache/docker/

# Set sccache environment variables
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/app/.sccache/docker

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached)
RUN cargo build --release && \
    cargo build # debug build to populate debug cache

# Display cache statistics
RUN sccache --show-stats

# Remove dummy artifacts to ensure clean build in next stage
RUN rm -rf src

RUN mkdir -p /app/.sccache/docker
COPY .sccache/host /app/.sccache/docker/

# Copy manifests
COPY Cargo.toml Cargo.lock ./

COPY Makefile ./

# Copy full source
COPY src ./src
COPY tests ./tests
COPY data ./data

RUN mkdir -p ".cargo"; cargo vendor --locked > .cargo/config.toml

# Build the application against cached dependencies
RUN cargo build --all --all-features --release && \
    cargo build --all --all-features  # debug build to populate debug cache

# Display cache statistics
RUN sccache --show-stats

# Copy data directory
COPY data ./data

RUN RUST_BACKTRACE=full cargo test --release --all-features && \
    RUST_BACKTRACE=full cargo test           --all-features

RUN make test

RUN make test-e2e

RUN make test-e2e-asciidoc

# Install program
RUN cp target/release/tpdg /usr/local/bin/tpdg

# Set default command
CMD ["tpdg"]
