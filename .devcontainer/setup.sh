#!/bin/bash

echo "🚀 Setting up Firox development environment..."

# Install additional tools
echo "📦 Installing additional development tools..."

# Install UV for Python package management
curl -LsSf https://astral.sh/uv/install.sh | sh
export PATH="$HOME/.cargo/bin:$PATH"

# Install Bun
curl -fsSL https://bun.sh/install | bash
export PATH="$HOME/.bun/bin:$PATH"

# Install Deno
curl -fsSL https://deno.land/install.sh | sh
export PATH="$HOME/.deno/bin:$PATH"

# Install additional Rust tools
echo "🦀 Installing Rust development tools..."
rustup component add clippy rustfmt
cargo install cargo-watch cargo-edit cargo-audit

# Setup Python environment
echo "🐍 Setting up Python environment..."
cd python/peaz && uv sync && cd ../..

# Ensure Rust workspace is ready
echo "🔧 Setting up Rust workspace..."
cd rust && cargo check && cd ..

# Create cache directory
mkdir -p /home/vscode/.cache

echo "✅ Setup complete! Happy coding! 🎉"
echo ""
echo "Available commands:"
echo "  - Rust: cargo build, cargo run, cargo test"
echo "  - Python: uv run python, uv add <package>"
echo "  - Deno: deno run, deno test"
echo "  - Bun: bun run, bun install"
echo ""
echo "Workspace structure:"
echo "  - /rust     - Rust workspace with multiple crates"
echo "  - /python   - Python projects (using UV)"
echo "  - /deno     - Deno/TypeScript projects"
echo "  - /bun      - Bun/Node.js projects"
