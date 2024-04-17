# VMIPS Rust

VMIPS Rust is a Rust implementation of a MIPS-like processor simulator. It includes a functional simulator and a timing simulator for the VMIPS architecture.

## Features

- Functional Simulator: Simulates the execution of VMIPS instructions and updates the architectural state accordingly.
- Timing Simulator: Simulates the timing behavior of a pipelined VMIPS processor with cache hierarchy.
- Supports a wide range of VMIPS instructions, including arithmetic, logical, memory access, control flow, and special instructions.
- Implements hazard detection and resolution mechanisms for the pipelined architecture.
- Provides a cache hierarchy simulation with configurable cache sizes, associativity, and replacement policies.
- Includes debugging and tracing capabilities for analyzing the execution of VMIPS programs.

## Prerequisites

- Rust programming language (version 1.x.x)
- Cargo package manager

## Getting Started

1. Clone the repository:
   ```shell
   git clone git@github.com:muditbhargava66/vimps-simulator.git
   ```

2. Change to the project directory:
   ```shell
   cd vmips-rust
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
   - Modify the `program` variable in the `main.rs` file to specify your VMIPS assembly instructions.

6. Analyze the simulation results:
   - The simulation log file (`vmips_rust.log`) will be generated in the project directory.
   - Examine the log file for detailed information about the executed instructions, register values, and memory accesses.

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

## Contributing

Contributions to VMIPS Rust are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).