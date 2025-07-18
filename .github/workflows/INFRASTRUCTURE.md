# Project Infrastructure

This document describes the project infrastructure setup for VMIPS Rust.

## CI/CD Pipeline

VMIPS Rust uses GitHub Actions for continuous integration and delivery. The workflows are defined in the `.github/workflows` directory.

### CI Workflow (ci.yml)

The CI workflow runs on every push to the `main` and `dev-*` branches, as well as on pull requests to `main`. It performs the following tasks:

1. **Build and Test**: Builds the project and runs all tests
2. **Linting**: Checks code formatting with rustfmt and analyzes code with Clippy
3. **Code Coverage**: Generates coverage reports and uploads them to Codecov

### Release Workflow (release.yml)

The release workflow is triggered when you push a tag that follows the format `v*.*.*`. It automates the release process:

1. **Build Binaries**: Compiles release binaries for Linux, Windows, and macOS
2. **Create Release**: Creates a GitHub release with the generated binaries
3. **Generate Changelog**: Automatically generates a changelog from commit messages

## Setting Up Code Coverage

Code coverage is tracked using cargo-tarpaulin and uploaded to Codecov. The configuration file `codecov.yml` defines how coverage reports are processed.

### Codecov Setup

1. Go to [Codecov](https://codecov.io/) and sign in with your GitHub account
2. Add your repository to Codecov
3. Copy the repository token (optional, as GitHub Actions integration works without a token)

### Running Coverage Locally

To run coverage locally, install cargo-tarpaulin:

```bash
cargo install cargo-tarpaulin
```

Then generate the coverage report:

```bash
cargo tarpaulin --out Html
```

This will create an HTML report in the `tarpaulin-report.html` file.

## Linting and Code Formatting

VMIPS Rust uses rustfmt for code formatting and Clippy for linting.

### Rustfmt

The configuration for rustfmt is in the `rustfmt.toml` file. To check if your code is properly formatted:

```bash
cargo fmt -- --check
```

To automatically format your code:

```bash
cargo fmt
```

### Clippy

The configuration for Clippy is in the `.clippy.toml` file. To run Clippy:

```bash
cargo clippy
```

To automatically fix some Clippy warnings:

```bash
cargo clippy --fix
```

## Creating a Release

To create a new release:

1. Make sure all your changes are committed and pushed
2. Update the version in `Cargo.toml`
3. Create and push a new tag:

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

The release workflow will automatically build binaries and create a GitHub release.

## Adding a Badge to Your README

Add these badges to your README.md to show the status of your project:

```markdown
[![CI](https://github.com/yourusername/vmips-simulator/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/vmips-simulator/actions/workflows/ci.yml)
```

Replace `yourusername` with your GitHub username.