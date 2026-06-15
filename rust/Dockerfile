FROM rust:trixie

# System packages commonly required to build and test Rust crates with native deps.
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Ensure rustup is present and install standard development toolchain components.
RUN rustup component add \
    rustfmt \
    clippy \
    llvm-tools-preview

# Extra Cargo plugins and Miri (nightly-only).
RUN cargo install cargo-nextest --locked && \
    cargo install cargo-watch --locked && \
    rustup toolchain install nightly --profile minimal && \
    rustup component add --toolchain nightly miri

# Ensure cargo/rustup are on PATH in interactive bash shells.
RUN echo '. /usr/local/cargo/env' >> /etc/bash.bashrc

# Default to a stable workspace mount point used by docker-compose.
WORKDIR /workspace

CMD ["sleep", "infinity"]
