<div align="center">

# VMIPS Rust

[![CI](https://github.com/muditbhargava66/vmips-simulator/actions/workflows/ci.yml/badge.svg)](https://github.com/muditbhargava66/vmips-simulator/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Last Commit](https://img.shields.io/github/last-commit/muditbhargava66/vmips-simulator)](https://github.com/muditbhargava66/vmips-simulator/commits/main)
[![Contributors](https://img.shields.io/github/contributors/muditbhargava66/vmips-simulator)](https://github.com/muditbhargava66/vmips-simulator/graphs/contributors)

**VMIPS Rust is a Rust implementation of a MIPS processor simulator. It includes a functional simulator and a timing simulator for the VMIPS architecture.**
</div>

## Features

- **Functional Simulator**: Simulates the execution of MIPS instructions and updates the architectural state accordingly.
- **Timing Simulator**: Simulates the timing behavior of a pipelined MIPS processor with configurable pipeline stages, latencies, and forwarding paths.
- **Advanced Processor Features**:
  - **Tomasulo's Algorithm**: Supports out-of-order execution with register renaming, reorder buffer (ROB), and reservation stations.
  - **Superscalar Execution**: Configurable to issue multiple instructions per cycle.
- **Comprehensive MIPS Support**: Supports a wide range of MIPS instructions, including arithmetic, logical, memory access, control flow, and special instructions, including a complete floating-point instruction set.
- **Cache Hierarchy**: Provides a multi-level cache hierarchy simulation (L1, L2) with configurable sizes, associativity, block sizes, and replacement policies.
- **Branch Prediction**: Implements advanced branch prediction techniques, including 2-bit saturating counters and Branch Target Buffer (BTB).
- **Hazard Detection & Resolution**: Includes mechanisms for detecting and resolving data and control hazards in the pipeline.
- **Debugging & Visualization Tools**: Features extensive debugging capabilities, cycle-by-cycle pipeline visualization, and performance statistics.

## Supported MIPS Instructions

The simulator supports a comprehensive set of MIPS instructions, including but not limited to:

- **R-type**: ADD, SUB, AND, OR, SLT, SLL, SRL, SRA, SLLV, SRLV, SRAV, JR, JALR, MULT, DIV, DIVU, MFLO, MFHI, MTLO, MTHI, XOR, NOR.
- **I-type**: ADDI, ADDIU, LW, SW, BEQ, BNE, LUI, ORI, ANDI, XORI, SLTI, SLTIU, LB, LBU, LH, LHU, SB, SH.
- **J-type**: J, JAL.
- **Branch**: BGTZ, BLEZ, BLTZ, BGEZ.
- **Floating-Point (Coprocessor 1)**: ADD.S, SUB.S, MUL.S, DIV.S, ABS.S, NEG.S, MOV.S, CVT.S.W, CVT.W.S, C.EQ.S, C.LT.S, C.LE.S, LWC1, SWC1, BC1T, BC1F.
- **Special**: SYSCALL, BREAK, NOP.

For a complete list and details, please refer to the source code in `src/functional_simulator/instructions.rs`.

## Prerequisites

- Rust programming language (version 1.56.0 or later)
- Cargo package manager

## Getting Started

1. Clone the repository:
   ```shell
   git clone https://github.com/muditbhargava66/vmips-simulator.git
   ```

2. Change to the project directory:
   ```shell
   cd vmips-simulator
   ```

3. Build the project:
   ```shell
   cargo build
   ```

4. Run the VMIPS simulator:
   - Functional Simulator:
     ```shell
     cargo run --bin vmips_rust functional
     ```
   - Timing Simulator:
     ```shell
     cargo run --bin vmips_rust timing
     ```

5. Customize the VMIPS program:
   - Modify the `create_test_program()` function in the `main.rs` file to specify your VMIPS assembly instructions.

6. Analyze the simulation results:
   - The simulation will display detailed information about the executed instructions, register values, and memory accesses.
   - For more detailed logging, set the environment variable `RUST_LOG=debug`.

## Running Examples

The `examples` directory contains example VMIPS programs that demonstrate the usage of the simulator.

To run an example program:
```shell
cargo run --example dot_product
```

## Running Tests

The project includes unit tests for the functional and timing simulators.

To run the tests:
```shell
cargo test
```

## Project Structure

- `src/`
  - `assembler/` - MIPS assembler for converting assembly to machine code
  - `functional_simulator/` - Implementation of the functional simulator
    - `simulator.rs` - Main simulator logic
    - `registers.rs` - Register file implementation
    - `memory.rs` - Memory implementation
    - `instructions.rs` - MIPS instruction definitions and execution
  - `timing_simulator/` - Implementation of the timing simulator
    - `simulator.rs` - Timing simulator logic
    - `pipeline.rs` - Pipeline implementation
    - `components.rs` - Cache, branch predictor, and other components
    - `config.rs` - Configuration for caches and pipeline
    - `tomasulo.rs` - Tomasulo's algorithm implementation for out-of-order execution
    - `visualization.rs` - Utilities for pipeline and cache visualization
  - `utils/` - Utility functions and types
    - `logger.rs` - Logging utilities
    - `parser.rs` - Parsing utilities for MIPS instructions
    - `syscall.rs` - System call handling
  - `lib.rs` - Library exports
  - `main.rs` - Command-line interface for running simulations
  - `main_assembler.rs` - Command-line interface for the assembler
- `benches/` - Benchmarking suite for performance analysis
- `docs/` - Project documentation, architecture, and getting started guides
- `examples/` - Example MIPS assembly programs
- `tests/` - Unit and integration tests for the simulators

## Contributing

Contributions to VMIPS Rust are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## Repository Banner

The GitHub repository includes a social media preview banner located at `assets/github-banner.svg`. This banner is designed to provide an attractive preview when the repository is shared on social media platforms.

## License

This project is licensed under the [MIT License](LICENSE)

