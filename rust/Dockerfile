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

# Single shell-setup script sourced by both login and non-login interactive shells:
# puts cargo/rustup on PATH and enables bash + git completion.
RUN cat > /etc/profile.d/dev.sh <<'EOF'
# Cargo/rustup on PATH (all shells)
[ -f /usr/local/cargo/env ] && . /usr/local/cargo/env

# Programmable completion (interactive bash only)
case "$-" in
  *i*)
    if [ -n "$BASH_VERSION" ] && ! shopt -oq posix; then
      [ -f /usr/share/bash-completion/bash_completion ] && . /usr/share/bash-completion/bash_completion
      # git completion (subcommands, refs, and aliases)
      [ -f /usr/share/bash-completion/completions/git ] && . /usr/share/bash-completion/completions/git
    fi
    ;;
esac
EOF
RUN printf '\n# Dev shell setup (login shells get this via /etc/profile.d)\n[ -f /etc/profile.d/dev.sh ] && . /etc/profile.d/dev.sh\n' >> /etc/bash.bashrc

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
