# VMIPS Simulator Documentation

Welcome to the documentation for the VMIPS Simulator project. This directory contains detailed information about the simulator's architecture, features, and usage.

## Table of Contents

- [Getting Started](getting-started.md) - Installation and basic usage
- [Architecture Overview](architecture.md) - High-level design of the simulator
- [Instruction Set](instruction-set.md) - Supported MIPS instructions
- [Functional Simulator](functional-simulator.md) - Details about the functional simulator
- [Timing Simulator](timing-simulator.md) - Details about the pipelined timing simulator
- [Assembler](assembler.md) - Using the assembler to convert assembly code to machine code
- [Examples](examples.md) - Example programs and usage scenarios
- [Troubleshooting](troubleshooting.md) - Common issues and solutions
- [Contributing](contributing.md) - Guidelines for contributing to the project

## Project Overview

VMIPS Simulator v0.2.1 is a comprehensive MIPS processor simulator written in Rust. It includes:

1.  **Functional Simulator** - Accurately executes MIPS instructions with enhanced error handling
2.  **Timing Simulator** - Models configurable pipeline with improved visualization
   - 5-stage pipeline with hazard detection
   - Tomasulo's algorithm for out-of-order execution
   - Superscalar capabilities and advanced branch prediction
   - Real-time pipeline visualization with instruction flow
3.  **Enhanced Assembler** - Converts MIPS assembly code to machine code
4.  **Advanced Visualization Tools** - Pipeline behavior, cache hierarchy, and memory/register state
   - Multiple output formats (Text, CSV, JSON)
   - Status indicators for pipeline stages
   - Comprehensive instruction type support

### New in v0.2.1
- **Enhanced Error Handling**: Comprehensive error detection and reporting system
- **Improved Pipeline Visualization**: Real-time instruction flow through pipeline stages
- **Algorithm Support Foundation**: Loop detection, register allocation, and PC management
- **Production-Ready Code Quality**: All critical warnings resolved

This simulator is designed for educational purposes, allowing users to understand processor behavior, pipeline hazards, cache performance, and advanced architectural concepts like out-of-order execution and branch prediction.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
