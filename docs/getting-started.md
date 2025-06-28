# Getting Started with VMIPS Simulator

This guide will walk you through the process of setting up, building, and running MIPS programs on the VMIPS Simulator. It covers both the functional and timing simulators, as well as the integrated assembler.

## Prerequisites

Before you begin, ensure you have the following installed:

-   **Rust and Cargo**: Version 1.70.0 or newer. You can install it via `rustup`.
-   **Basic MIPS Assembly Knowledge**: Familiarity with MIPS instruction set and assembly programming concepts will be beneficial.
-   **Text Editor or IDE**: For writing and editing MIPS assembly code.

## Installation

### From Source

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/muditbhargava66/vmips-simulator.git
    cd vmips-simulator
    ```

2.  **Build the project**:
    ```bash
    cargo build --release
    ```
    This command compiles the entire project, including the functional simulator, timing simulator, and assembler. The compiled binaries will be located in the `target/release/` directory.

    -   `target/release/vmips_rust`: The main simulator executable.
    -   `target/release/main_assembler`: The standalone assembler executable.

## Running Your First Program

VMIPS Simulator allows you to run MIPS assembly programs using either the functional or timing simulator. First, you'll need to write and assemble your MIPS code.

### Step 1: Write MIPS Assembly Code

Create a new file, for example, `my_program.s`, and add some MIPS assembly instructions. Here's a simple example:

```asm
.data
my_data: .word 10, 20, 30

.text
.globl main
main:
    lw $t0, my_data($zero)      # Load first word (10) into $t0
    lw $t1, my_data+4($zero)    # Load second word (20) into $t1
    add $t2, $t0, $t1           # Add $t0 and $t1, store in $t2 (result 30)
    li $v0, 10                  # Syscall code for exit
    syscall                     # Exit program
```

### Step 2: Assemble the Program

Use the built-in assembler to convert your `.s` file into a binary executable (`.bin`):

```bash
cargo run --bin main_assembler assemble my_program.s my_program.bin
```

This command will create `my_program.bin` in your project root directory.

### Step 3: Run the Simulator

Now you can run your assembled program using either the functional or timing simulator.

#### Using the Functional Simulator

The functional simulator executes instructions sequentially, focusing on correctness rather than timing.

```bash
cargo run --bin vmips_rust functional my_program.bin
```

#### Using the Timing Simulator

The timing simulator models a pipelined MIPS processor, including hazards, caches, and advanced features like Tomasulo's algorithm and branch prediction. It provides detailed timing statistics.

```bash
cargo run --bin vmips_rust timing my_program.bin
```

## Command Line Options

The main `vmips_rust` executable supports various command-line arguments to control simulation behavior:

```bash
cargo run --bin vmips_rust <simulator_type> <binary_file> [options]
```

-   `<simulator_type>`: `functional` or `timing`.
-   `<binary_file>`: Path to the assembled MIPS binary file (e.g., `my_program.bin`).

**Common Options:**
-   `--memory-size <size>`: Sets the total memory size in bytes for the simulator (default: 32768 bytes).
-   `--max-steps <steps>`: Sets a maximum number of instructions to execute, preventing infinite loops (default: 1,000,000).
-   `--breakpoint <address>`: Adds a breakpoint at the specified memory address (hex or decimal).
-   `--trace`: Enables detailed instruction tracing output.
-   `--debug`: Enables verbose debug output.

**Timing Simulator Specific Options:**
-   `--pipeline-stages <num>`: Configures the number of pipeline stages (default: 5).
-   `--no-forwarding`: Disables data forwarding.
-   `--no-branch-prediction`: Disables branch prediction.
-   `--tomasulo`: Enables Tomasulo's algorithm for out-of-order execution.
-   `--superscalar <width>`: Enables superscalar execution with the specified width (e.g., `--superscalar 2`).
-   `--l1d-cache <size> <assoc> <block_size>`: Configures L1 data cache (e.g., `--l1d-cache 4096 4 64`).
-   `--l1i-cache <size> <assoc> <block_size>`: Configures L1 instruction cache.
-   `--l2-cache <size> <assoc> <block_size>`: Configures L2 cache.

## Next Steps

-   Explore the [Instruction Set](instruction-set.md) documentation for a full list of supported MIPS instructions.
-   Dive deeper into the simulator's design with the [Architecture Overview](architecture.md).
-   Check out the [Examples](examples.md) directory for more complex MIPS programs and usage scenarios.
-   For detailed information on each simulator, refer to [Functional Simulator](functional-simulator.md) and [Timing Simulator](timing-simulator.md) documentation.

## Troubleshooting

If you encounter any issues, please consult the [Troubleshooting](troubleshooting.md) guide.
