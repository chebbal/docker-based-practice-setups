FROM rust:trixie

# Host user mapped into the image so bind-mounted files keep host ownership.
ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=1000

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
    sudo \
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

# Create a user matching the host UID/GID and give it the shared toolchain dirs.
RUN if ! getent group "${USER_GID}" >/dev/null; then groupadd --gid "${USER_GID}" "${USERNAME}"; fi \
    && useradd --badname --uid "${USER_UID}" --gid "${USER_GID}" --create-home --shell /bin/bash "${USERNAME}" \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/"${USERNAME}" \
    && chmod 0440 /etc/sudoers.d/"${USERNAME}" \
    && chown -R "${USER_UID}:${USER_GID}" "${CARGO_HOME}" "${RUSTUP_HOME}"

# Default to a stable workspace mount point used by docker-compose.
RUN mkdir -p /workspace && chown "${USER_UID}:${USER_GID}" /workspace
WORKDIR /workspace

USER ${USERNAME}

CMD ["sleep", "infinity"]
