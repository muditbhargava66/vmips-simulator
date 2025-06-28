# Getting Started with VMIPS Simulator

This guide will help you install, configure, and run your first program on the VMIPS Simulator.

## Prerequisites

- Rust and Cargo (1.50.0 or newer)
- Basic understanding of MIPS assembly language
- Text editor or IDE

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/muditbhargava66/vmips-simulator.git
   cd vmips-simulator
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be located at `target/release/vmips_rust`

## Running Your First Program

### Using the Functional Simulator

1. Create a simple MIPS assembly program (e.g., `example.s`):
   ```asm
   # Simple add program
   addi $2, $0, 10     # Set $2 to 10
   addi $3, $0, 20     # Set $3 to 20
   add $4, $2, $3      # $4 = $2 + $3
   ```

2. Assemble the program:
   ```bash
   ./target/release/main_assembler example.s -o example.bin
   ```

3. Run the functional simulator:
   ```bash
   ./target/release/vmips_rust functional
   ```

### Using the Timing Simulator

To run the timing simulator:

```bash
./target/release/vmips_rust timing
```

This will execute the same program, but model the pipeline behavior and provide timing statistics.

## Command Line Options

- `functional` - Run the functional simulator
- `timing` - Run the timing simulator
- `[memory_size]` - Optional parameter to specify the memory size in bytes (default: 8192)

Example:
```bash
./target/release/vmips_rust timing 16384
```

## Next Steps

- Check out the [examples](examples.md) for more complex programs
- Learn about the [Instruction Set](instruction-set.md) supported by the simulator
- Explore the [Architecture Overview](architecture.md) to understand the simulator's design

## Troubleshooting

If you encounter any issues, please consult the [Troubleshooting](troubleshooting.md) guide.
