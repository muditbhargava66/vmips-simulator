# Contributing to VMIPS Rust Simulator v0.2.1

We welcome contributions to the VMIPS Rust Simulator! Whether it's bug fixes, new features, improved documentation, or performance optimizations, your help is greatly appreciated. With v0.2.1, we've established production-ready code quality standards and enhanced testing procedures.

## How to Contribute

1.  **Fork the Repository**: Start by forking the `vmips-simulator` repository on GitHub.
2.  **Clone Your Fork**: Clone your forked repository to your local machine:
    ```bash
    git clone https://github.com/YOUR_USERNAME/vmips-simulator.git
    cd vmips-simulator
    ```
3.  **Create a New Branch**: Create a new branch for your feature or bug fix. Use a descriptive name (e.g., `feature/add-tomasulo`, `fix/branch-prediction-bug`).
    ```bash
    git checkout -b feature/your-feature-name
    ```
4.  **Make Your Changes**: Implement your changes, adhering to the project's coding style and conventions.
5.  **Test Your Changes**: Ensure your changes work as expected and do not introduce regressions. Run existing tests and add new ones if necessary.
6.  **Commit Your Changes**: Write clear, concise, and descriptive commit messages. Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification if possible (e.g., `feat: add new instruction`, `fix: correct branch offset`).
7.  **Push to Your Fork**: Push your local branch to your forked repository on GitHub.
    ```bash
    git push origin feature/your-feature-name
    ```
8.  **Create a Pull Request (PR)**: Open a Pull Request from your feature branch to the `main` branch of the original `vmips-simulator` repository. Provide a detailed description of your changes.

## Code Style and Quality

-   **Rustfmt**: We use `rustfmt` to enforce consistent code formatting. Before committing, run:
    ```bash
    cargo fmt --check
    ```
    To automatically format your code:
    ```bash
    cargo fmt
    ```
-   **Clippy**: We use `clippy` for linting to catch common mistakes and improve code quality. Ensure your code passes `clippy` checks:
    ```bash
    cargo clippy --all-targets -- -D warnings
    ```
    Address any warnings or errors reported by Clippy.
-   **Existing Conventions**: Try to match the coding style, naming conventions, and architectural patterns already present in the codebase.

## Testing

-   **Run Existing Tests**: Always run the full test suite before submitting a PR to ensure your changes haven't broken existing functionality:
    ```bash
    cargo test --all
    ```
-   **Write New Tests**: If you're adding a new feature or fixing a bug, please include unit or integration tests that cover your changes. This helps prevent regressions and ensures correctness.

## Pull Request Guidelines

-   **Descriptive Title**: Your PR title should briefly summarize the changes.
-   **Detailed Description**: Provide a clear and concise description of your changes. Explain:
    -   What problem your PR solves.
    -   How you solved it.
    -   Any relevant design decisions or trade-offs.
    -   How to test your changes (if not covered by automated tests).
-   **Link Issues**: If your PR addresses an open issue, link it in the description (e.g., `Fixes #123`).
-   **One Feature/Fix per PR**: Keep your PRs focused on a single feature or bug fix to make reviews easier.

## Documentation

-   **Update Documentation**: If your changes affect how the simulator is used, its features, or its architecture, please update the relevant documentation files in the `docs/` directory (e.g., `README.md`, `getting-started.md`, `instruction-set.md`).
-   **Code Comments**: Add comments to complex or non-obvious parts of your code, explaining *why* something is done, not just *what* is done.

Thank you for contributing to VMIPS Rust!