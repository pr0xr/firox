# Firox Development Containers

This repository includes multiple devcontainer configurations to support the different languages and tools used in the Firox project.

## Available Configurations

### 1. **Multi-Language Environment** (Default)
- **Path**: `.devcontainer/devcontainer.json`
- **Description**: Complete development environment supporting all languages
- **Includes**: Rust, Python, Node.js, Deno, Bun
- **Best for**: Full-stack development, working across multiple project areas

### 2. **Rust Development**
- **Path**: `.devcontainer/rust/devcontainer.json`  
- **Description**: Optimized Rust development environment
- **Includes**: Latest Rust toolchain, Clippy, Rustfmt, Cargo tools
- **Best for**: Working primarily on Rust crates and libraries

### 3. **Python Development**
- **Path**: `.devcontainer/python/devcontainer.json`
- **Description**: Python-focused environment with UV package manager
- **Includes**: Python 3.12, UV, Ruff, Black formatter
- **Best for**: Python scripting and the `peaz` project

### 4. **Node.js/TypeScript Development**
- **Path**: `.devcontainer/nodejs/devcontainer.json`
- **Description**: JavaScript/TypeScript environment with modern tools
- **Includes**: Node.js 20, Bun, Deno, TypeScript support
- **Best for**: Working on Deno/Bun projects and JavaScript tooling

## How to Use

### VS Code with Dev Containers Extension

1. **Install the Dev Containers extension** in VS Code
2. **Open the repository** in VS Code
3. **Choose your environment**:
   - For the default multi-language setup: VS Code will automatically detect the main devcontainer
   - For specific environments: Use `Ctrl+Shift+P` → "Dev Containers: Reopen in Container" → Select folder

### GitHub Codespaces

1. **Create a new Codespace** from this repository
2. **Choose configuration**: GitHub will prompt you to select which devcontainer to use
3. **Start developing**: Your environment will be ready in minutes

## What's Included

### Development Tools
- **Rust**: Latest stable toolchain with Clippy, Rustfmt, and Cargo extensions
- **Python**: Version 3.12 with UV package manager for fast dependency management
- **Node.js**: Version 20 with npm, plus Bun and Deno runtimes
- **Git**: Latest version with full GitHub integration

### VS Code Extensions
- **Language Support**: Rust Analyzer, Python, TypeScript
- **Formatters**: Black (Python), Rustfmt (Rust), Prettier (JS/TS)
- **Linters**: Clippy (Rust), Ruff (Python), ESLint (JS/TS)
- **Utilities**: GitHub Copilot, Hex Editor, YAML/JSON support

### Pre-configured Settings
- **Rust**: Workspace-aware Rust Analyzer with Clippy integration
- **Python**: UV package management with Ruff linting
- **Deno**: Enabled specifically for the `/deno` directory
- **Port Forwarding**: Common development ports (3000, 8000, 8080, 9000)

## Project Structure Support

The devcontainer is configured to work with the existing project structure:

```
firox/
├── rust/           # Rust workspace with multiple crates
├── python/         # Python projects (UV-managed)
├── deno/           # Deno/TypeScript projects  
├── bun/            # Bun/Node.js projects
├── p/              # Additional projects
└── doc/            # Documentation
```

## Quick Start Commands

### Rust Development
```bash
cd rust
cargo build                    # Build all crates
cargo test                     # Run tests
cargo watch -x check          # Watch mode
```

### Python Development  
```bash
cd python/peaz
uv sync                        # Install dependencies
uv run python main.py         # Run with UV
```

### Deno Development
```bash
cd deno
deno run --allow-net download.ts    # Run Deno scripts
```

### Bun Development
```bash
cd bun
bun install                    # Install packages
bun run dev                    # Start development
```

## Customization

You can customize any devcontainer by:

1. **Modifying the JSON files** to add/remove features or extensions
2. **Updating setup scripts** to install additional tools
3. **Adding new configurations** for specific workflows

## Troubleshooting

### Container Build Issues
- Try rebuilding: `Ctrl+Shift+P` → "Dev Containers: Rebuild Container"
- Check Docker/Podman is running and accessible

### Missing Tools
- Run the setup script manually: `bash .devcontainer/setup.sh`
- Check feature installation in the devcontainer.json

### Performance
- Consider using the specific environment devcontainers for better performance
- Enable Docker BuildKit for faster builds

## Contributing

When adding new tools or languages to the project:

1. **Update the appropriate devcontainer.json** files
2. **Add necessary VS Code extensions** for language support  
3. **Update setup scripts** if additional installation steps are needed
4. **Test the configuration** by rebuilding the container


