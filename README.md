# VMIPS Rust

VMIPS Rust is a Rust implementation of a MIPS processor simulator. It includes a functional simulator and a timing simulator for the VMIPS architecture.

## Features

- **Functional Simulator**: Simulates the execution of MIPS instructions and updates the architectural state accordingly.
- **Timing Simulator**: Simulates the timing behavior of a pipelined MIPS processor with cache hierarchy.
- **Comprehensive MIPS Support**: Supports 40+ MIPS instructions, including arithmetic, logical, memory access, control flow, and special instructions.
- **Hazard Detection**: Implements hazard detection and resolution mechanisms for the pipelined architecture.
- **Cache Hierarchy**: Provides a cache hierarchy simulation with configurable cache sizes, associativity, and replacement policies.
- **Debugging Tools**: Includes debugging and tracing capabilities for analyzing the execution of MIPS programs.

## Supported MIPS Instructions

- R-type: ADD, SUB, AND, OR, SLT, SLL, SRL, JR, MULT, MFLO
- I-type: ADDI, ADDIU, LW, SW, BEQ, BNE, LUI, ORI
- J-type: J

## Prerequisites

- Rust programming language (version 1.56.0 or later)
- Cargo package manager

## Getting Started

1. Clone the repository:
   ```shell
   git clone git@github.com:muditbhargava66/vmips-simulator.git
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
  - `functional_simulator/` - Implementation of the functional simulator
    - `simulator.rs` - Main simulator logic
    - `registers.rs` - Register file implementation
    - `memory.rs` - Memory implementation
    - `instructions.rs` - MIPS instruction definitions and execution
  - `timing_simulator/` - Implementation of the timing simulator
    - `simulator.rs` - Timing simulator logic
    - `pipeline.rs` - Pipeline implementation
    - `components.rs` - Cache and other components
    - `config.rs` - Configuration for caches and pipeline
  - `utils/` - Utility functions and types
    - `logger.rs` - Logging utilities
    - `parser.rs` - Parsing utilities for MIPS instructions
  - `lib.rs` - Library exports
  - `main.rs` - Command-line interface

## Contributing

Contributions to VMIPS Rust are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE)