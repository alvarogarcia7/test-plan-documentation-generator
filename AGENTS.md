1→# Agent Guide
2→
3→## Setup & Commands
4→```bash
5→# No setup required - Rust toolchain is the only dependency
6→cargo build --release        # Build the project
7→make lint                    # Run fmt-check + clippy
8→make test                    # Run unit tests + E2E test
9→# No dev server - this is a CLI tool
10→```
11→
12→## Tech Stack
13→- **Language**: Rust 2021 edition
14→- **CLI**: clap with derive features
15→- **Templating**: Tera (Jinja2-like)
16→- **Validation**: jsonschema for schema validation
17→- **Testing**: cargo test + insta for snapshots
18→
19→## Architecture
20→- Single-file CLI tool (`src/main.rs`) that validates and renders test documentation
21→- Input: JSON schemas + Jinja2 templates + YAML data files
22→- Output: Markdown test plan documentation
23→- Build artifacts go to `/target` (gitignored)
24→
25→## Code Style
26→- Format: `tab_spaces = 4`, `reorder_imports = true`
27→- Clippy: max 7 function args, moderate type complexity allowed
28→- No comments unless necessary for complex logic
29→