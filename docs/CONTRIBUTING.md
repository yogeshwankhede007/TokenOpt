# Contributing to TokenOpt

Thank you for your interest in contributing to TokenOpt! This document provides guidelines and best practices for contributing to this project.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Node.js 18+ (for TypeScript bindings)
- Python 3.8+ (for Python bindings)
- Git

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourorg/tokenoptimization.git
cd tokenoptimization

# Install Rust components
rustup component add rustfmt clippy

# Install development tools
cargo install cargo-audit cargo-deny cargo-outdated cargo-nextest cargo-tarpaulin criterion

# Build the project
cargo build --workspace --all-features

# Run tests
cargo test --workspace --all-features

# Run benchmarks
cargo bench --workspace --all-features
```

### Python Bindings Setup

```bash
cd bindings/python
pip install maturin[patchelf]
maturin develop --release
pip install pytest pytest-asyncio
pytest
```

### TypeScript Bindings Setup

```bash
cd bindings/typescript
npm ci
npm run build
npm test
```

## Development Workflow

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `perf/description` - Performance improvements
- `test/description` - Test additions/improvements

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Formatting
- `refactor` - Refactoring
- `perf` - Performance
- `test` - Testing
- `chore` - Maintenance
- `ci` - CI/CD
- `security` - Security

Example:
```
feat(optimizer): add support for custom routing rules

Add ability to define custom routing rules with priority-based matching.
This allows users to implement domain-specific routing logic.

Closes #123
```

### Pull Request Process

1. **Fork** the repository and create your branch
2. **Make changes** following the code style
3. **Add tests** for new functionality
4. **Update documentation** as needed
4. **Run the full test suite** locally
5. **Submit PR** with a clear description
6. **Address review feedback** promptly
7. **Squash commits** before merge (maintainer will do this)

### Code Style Guidelines

#### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` and `cargo clippy` before committing
- Maximum line length: 100 characters
- Prefer `?` over `unwrap()`/`expect()` in library code
- Document all public APIs with `///` comments
- Use `#[must_use]` for functions returning values that shouldn't be ignored

#### TypeScript

- Follow [Google TypeScript Style Guide](https://google.github.io/styleguide/tsguide.html)
- Use `eslint` and `prettier`
- Strict TypeScript config (`strict: true`)
- Document all exported APIs with JSDoc

#### Python

- Follow [PEP 8](https://pep8.org/) and [PEP 257](https://pep257.readthedocs.io/)
- Use `black`, `isort`, `flake8`, `mypy`
- Type hints required for all public functions
- Google-style docstrings

### Testing Requirements

- **Unit tests** for all new functions (target: 90%+ coverage)
- **Integration tests** for API endpoints and workflows
- **Property-based tests** for algorithms (using `proptest`)
- **Benchmarks** for performance-critical code (using `criterion`)
- **Contract tests** for FFI boundaries

Run tests with:
```bash
# Unit tests
cargo nextest run --workspace --all-features

# Property tests
cargo test --workspace --all-features proptest

# Benchmarks
cargo bench --workspace --all-features

# Coverage
cargo tarpaulin --workspace --all-features --out Html
```

### Documentation Standards

- All public APIs must have documentation
- Include examples in doc comments
- Keep README updated with new features
- Update CHANGELOG.md for notable changes
- Architecture decisions recorded in `docs/adr/`

### Performance Requirements

- No performance regressions without justification
- Benchmark critical paths before/after changes
- Profile memory usage for large inputs
- Target: <1ms for token counting 10K tokens

### Security Considerations

- No `unsafe` code without review
- Validate all external inputs
- Use `cargo audit` and `cargo deny` regularly
- Report security issues privately to security@tokenopt.dev

## Release Process

1. Version bump in `Cargo.toml` (workspace)
2. Update `CHANGELOG.md`
3. Create release PR
3. Tag release: `git tag v0.x.y`
4. CI publishes to crates.io, npm, PyPI
5. GitHub Release created automatically

## Getting Help

- **Discord**: [TokenOpt Community](https://discord.gg/tokenopt)
- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Email**: maintainers@tokenopt.dev

## Recognition

Contributors are recognized in:
- `AUTHORS.md`
- Release notes
- Hall of Fame on website

Thank you for contributing to TokenOpt!