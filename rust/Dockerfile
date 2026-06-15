FROM rust:trixie

# System packages commonly required to build and test Rust crates with native deps.
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash \
    bash-completion \
    build-essential \
    ca-certificates \
    curl \
    git \
    pkg-config \
    libssl-dev \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Ensure rustup is present and install standard development toolchain components.
RUN rustup component add \
    rustfmt \
    clippy \
    llvm-tools-preview

# Extra Cargo plugins and Miri (nightly-only).
RUN cargo install cargo-edit --locked && \
    cargo install cargo-nextest --locked && \
    cargo install cargo-watch --locked && \
    rustup toolchain install nightly --profile minimal && \
    rustup component add --toolchain nightly miri

# Ensure cargo/rustup are on PATH and enable bash completions in interactive shells.
RUN cat >> /etc/bash.bashrc <<'EOF'

# Cargo/rustup
. /usr/local/cargo/env

# Bash completion
if ! shopt -oq posix; then
  if [ -f /usr/share/bash-completion/bash_completion ]; then
    . /usr/share/bash-completion/bash_completion
  elif [ -f /etc/bash_completion ]; then
    . /etc/bash_completion
  fi
fi
EOF

# Default to a stable workspace mount point used by docker-compose.
WORKDIR /workspace

CMD ["sleep", "infinity"]
