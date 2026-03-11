# Test Plan Documentation Generator

[![CI](https://github.com/YOUR_USERNAME/YOUR_REPO/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/YOUR_REPO/actions)
[![codecov](https://codecov.io/gh/YOUR_USERNAME/YOUR_REPO/branch/main/graph/badge.svg)](https://codecov.io/gh/YOUR_USERNAME/YOUR_REPO)

A Rust CLI tool for generating test plan documentation from JSON schemas, Jinja2 templates, and YAML data files.

## Features

- Validates and renders test documentation using JSON schemas
- Supports Jinja2-like templating with Tera
- Generates Markdown and AsciiDoc output formats
- Built-in schema validation with jsonschema
- Comprehensive test coverage with minimum 70% threshold

## Installation

Requires Rust toolchain installed on your system.

```bash
cargo build --release
```

## Usage

```bash
./target/release/test-plan-doc-gen \
  --output ./output.md \
  --container ./container/schema.json ./container/template.j2 ./container/data.yml \
  --test-case ./verification_methods ./test_case1.yml ./test_case2.yml
```

## Development

### Build

```bash
cargo build --release
```

### Testing

```bash
make test  # Run unit tests + E2E tests
```

### Linting

```bash
make lint  # Run fmt-check + clippy
```

### Coverage

Coverage is automatically generated in CI/CD with a minimum threshold of 70%. To generate coverage locally:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out html --output-dir ./coverage
```

### Docker

```bash
make docker-build
```

## Tech Stack

- **Language**: Rust 2021 edition
- **CLI**: clap with derive features
- **Templating**: Tera (Jinja2-like)
- **Validation**: jsonschema for schema validation
- **Testing**: cargo test + insta for snapshots

## License

For open source projects, say how it is licensed.
