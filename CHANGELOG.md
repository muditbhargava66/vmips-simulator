# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-07-19

### Added
- Modern CLI interface using clap with subcommands
- Comprehensive command-line options for both simulators
- Pipeline visualization controls for timing simulator
- Configurable memory size and logging levels
- Integration tests for CLI functionality
- Security policy documentation
- Production-ready CI/CD pipeline
- ELF binary loading support for real MIPS programs
- Comprehensive API documentation with examples
- Step-by-step tutorials for computer architecture concepts
- Property-based testing with proptest and quickcheck
- Edge case testing for robustness
- Automated release process with cross-platform binaries
- Comprehensive example programs with working implementations:
  - Array sum calculation with step-by-step processing
  - Bubble sort algorithm demonstration (single pass)
  - Dot product calculation for vectors
  - Factorial computation using iterative approach
  - Fibonacci number generation
  - Matrix multiplication for 2x2 matrices
  - Simple calculator with complex arithmetic operations
  - String length calculation for null-terminated strings
- Enhanced CI/CD pipeline with example testing and benchmarks
- Improved code formatting and linting compliance
- Better error handling and debugging information in examples

### Changed
- Improved CLI with better help messages and validation
- Enhanced error handling and user feedback
- Updated dependencies to latest stable versions
- Better project documentation and examples
- Simplified example implementations for better educational value
- Replaced complex loop-based algorithms with step-by-step direct calculations
- Updated CI configuration to remove codecov dependency
- Enhanced documentation with current project status
- Improved code quality with comprehensive linting fixes

### Fixed
- Memory initialization issues in simulators
- Pipeline visualization rendering
- Log file creation and management
- Infinite loop issues in array_sum.rs example
- Branch offset calculation errors in factorial.rs
- Complex loop logic that didn't work with current simulator
- All Clippy warnings and formatting issues
- Memory initialization consistency across examples

## [0.1.0] - 2024-11-01

### Added
- Initial release of VMIPS Rust simulator
- Functional MIPS processor simulator
- Timing simulator with pipeline support
- Tomasulo's algorithm implementation
- Branch prediction capabilities
- Cache hierarchy simulation
- Comprehensive MIPS instruction set support
- Example programs and documentation