# DevContainer Cache

This directory is used to cache dependencies and build artifacts to improve devcontainer startup times.

## What's Cached

- Rust cargo cache and target artifacts
- Python package cache (UV/pip)
- Node.js npm/yarn cache
- VS Code extension cache

## Maintenance

The cache is automatically managed by the devcontainer setup. If you encounter issues, you can:

1. Clear the cache by deleting this directory's contents
2. Rebuild the devcontainer to regenerate the cache

This cache is mounted as a volume in the devcontainer to persist across rebuilds.
