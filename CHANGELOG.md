# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2025-08-22

### Fixed
- **Code Quality**: Resolved all major Clippy warnings for production-ready code
  - Fixed module inception warning in assembler module (renamed assembler.rs to core.rs)
  - Removed "Error" suffix from `AssemblerError` enum variants for better naming
  - Implemented `Default` trait for `Assembler` struct
  - Replaced `.unwrap()` calls with proper pattern matching in main.rs
  - Fixed empty `println!()` usage in main_assembler.rs
  - Added proper error handling patterns throughout codebase
- **rustfmt Configuration**: Updated for stable Rust channel compatibility
  - Removed nightly-only features (format_strings, wrap_comments, etc.)
  - Added comprehensive documentation for configuration options
  - Ensures consistent formatting across stable and nightly channels
- **Error Handling**: Improved error propagation and display
  - Made `AssemblerError` publicly accessible for testing
  - Enhanced error message formatting and context
  - Better error handling in file loading operations

### Added
- **Enhanced Test Coverage**: New comprehensive edge case tests
  - Assembler Default trait validation
  - Error display formatting verification
  - Functional simulator boundary condition testing
  - Timing simulator configuration validation
  - Pipeline configuration edge cases
  - Instruction decode robustness tests
  - Memory alignment and consistency checks
  - Error propagation verification
  - State consistency validation
- **Code Quality**: All major Clippy warnings resolved
  - Eliminated 25+ warning instances
  - Improved code maintainability and readability
  - Enhanced type safety and error handling

### Changed
- **Module Structure**: Reorganized assembler module for better encapsulation
- **Error Types**: Simplified enum variant naming for consistency
- **Development Experience**: Improved code formatting and linting workflow

### Technical Improvements
- **Build System**: All builds now pass without warnings
- **Code Standards**: Consistent formatting across entire codebase
- **Type Safety**: Better error handling patterns throughout
- **Testing**: Enhanced coverage for edge cases and boundary conditions

## [0.2.1] - 2025-07-19

### Added
- **Enhanced Error Handling System**: Comprehensive `SimulatorError` enum with specific error types
  - Memory bounds checking with `MemoryOutOfBounds` and `MemoryMisaligned` errors
  - Address overflow protection with `AddressOverflow` error
  - Branch target validation with `InvalidBranchTarget` error
  - Division by zero protection with `DivisionByZero` error
  - Invalid instruction handling with `InvalidInstruction` error
- **Advanced Algorithm Support Foundation**:
  - `LoopDetector` module for identifying and optimizing loop patterns
  - `RegisterAllocator` module for register usage analysis and optimization
  - `PcManager` module for robust program counter management
- **Enhanced Pipeline Visualization**: Improved pipeline stage display in timing simulator
  - Real-time instruction flow visualization through pipeline stages
  - Multiple output formats (Text, CSV, JSON)
  - Status indicators for pipeline stages (Busy, Stalled, Flushed)
  - Comprehensive instruction type support in visualization
- **Comprehensive Test Suite**: New `error_handling.rs` with 8 test cases
- **Code Quality Improvements**: Fixed critical Clippy errors and warnings

### Enhanced
- **Memory Safety**: Improved bounds checking and address validation
- **Branch Handling**: Enhanced branch offset calculations with overflow protection
- **Exception System**: Added `Debug` trait to `Exception` enum for better error reporting
- **Algorithm Support**: Better handling of simple loops and memory access patterns
- **Pipeline Visualization**: Fixed pipeline stage display to show instructions flowing through the pipeline
- **Educational Value**: Improved visual representation of pipeline execution for better understanding
- **Instruction Coverage**: Enhanced visualization support for all MIPS instruction types

### Fixed
- **Clippy Compliance**: Resolved critical errors and reduced warnings significantly
- **Memory Access**: Enhanced validation for memory operations
- **Branch Instructions**: Improved branch target calculation and validation
- **Code Quality**: Removed redundant operations and improved maintainability
- **Pipeline Visualization**: Fixed empty pipeline stage display in timing simulator visualization
- **Instruction Flow**: Corrected instruction progression through pipeline stages for accurate visualization
- **Missing Imports**: Added required imports for pipeline visualization functionality

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
- Critical compilation issues and unused imports
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
###
 Known Issues
- 94 Clippy warnings remain to be addressed in v0.2.1
- CI configured to allow warnings for this release to focus on functionality