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

VMIPS Simulator is a comprehensive MIPS processor simulator written in Rust. It includes:

1. A functional simulator that accurately executes MIPS instructions
2. A timing simulator that models a 5-stage pipeline with hazard detection and forwarding
3. An assembler that converts MIPS assembly code to machine code
4. Visualization tools for pipeline behavior and memory/register state

This simulator is designed for educational purposes, allowing users to understand processor behavior, pipeline hazards, and performance characteristics of MIPS programs.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
