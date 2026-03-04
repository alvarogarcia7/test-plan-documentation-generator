# Agent Guide

## Setup & Commands
```bash
# No setup required - Rust toolchain is the only dependency
cargo build --release        # Build the project
make lint                    # Run fmt-check + clippy
make test                    # Run unit tests + E2E test
# No dev server - this is a CLI tool
```

## Commands

### Build Cache (sccache)
```bash
make install-sccache         # Install sccache locally for build caching
make sccache-stats           # Show sccache cache statistics
make sccache-clean           # Clean sccache cache directories
```

**Setup**: Run `make install-sccache` to install sccache locally. This enables compilation caching to speed up rebuilds.

**Cache Structure**:
- `.sccache/host` - Cache directory for host builds
- `.sccache/docker` - Cache directory for Docker builds

## Tech Stack
- **Language**: Rust 2021 edition
- **CLI**: clap with derive features
- **Templating**: Tera (Jinja2-like)
- **Validation**: jsonschema for schema validation
- **Testing**: cargo test + insta for snapshots

## Architecture
- Single-file CLI tool (`src/main.rs`) that validates and renders test documentation
- Input: JSON schemas + Jinja2 templates + YAML data files
- Output: Markdown test plan documentation
- Build artifacts go to `/target` (gitignored)

## Code Style
- Format: `tab_spaces = 4`, `reorder_imports = true`
- Clippy: max 7 function args, moderate type complexity allowed
- No comments unless necessary for complex logic
