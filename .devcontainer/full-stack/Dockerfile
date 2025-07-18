FROM mcr.microsoft.com/devcontainers/universal:2-linux

# Install additional development tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        build-essential \
        curl \
        git \
        sudo \
        zsh \
        vim \
        htop \
        postgresql-client \
        redis-tools

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add clippy rustfmt

# Install Python tools
RUN curl -LsSf https://astral.sh/uv/install.sh | sh

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash

# Install Deno  
RUN curl -fsSL https://deno.land/install.sh | sh

# Create vscode user
RUN groupadd --gid 1000 vscode \
    && useradd --uid 1000 --gid vscode --shell /bin/zsh --create-home vscode \
    && echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/vscode \
    && chmod 0440 /etc/sudoers.d/vscode

USER vscode
