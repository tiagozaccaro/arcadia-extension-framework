# Arcadia Extension Framework

A Rust library for building extensions in the Arcadia application ecosystem.

## Development

### Branch Protection

Pull requests to the `main` branch must pass all CI checks, which include:

- Clippy linting with warnings treated as errors
- Unit tests
- Cross-platform builds on Ubuntu, macOS, and Windows

Ensure your code has no warnings or errors before submitting a PR.

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Building

```bash
cargo build --release
```
