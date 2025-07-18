# Functional Simulator

The Functional Simulator provides a cycle-accurate model of a MIPS processor that executes instructions sequentially without modeling the pipeline or complex timing characteristics. It is ideal for verifying program logic and debugging.

## Overview

The functional simulator focuses on the correct execution of MIPS instructions. It accurately models:

-   **Register File State**: Manages 32 general-purpose registers, HI/LO registers for multiplication/division results, and 32 floating-point registers.
-   **Memory System**: Provides a byte-addressable memory space with support for word, halfword, and byte accesses, including proper alignment checks and memory protection.
-   **Instruction Behavior**: Implements the precise architectural behavior of a comprehensive set of MIPS instructions, including integer, floating-point, and control flow operations.
-   **Control Flow**: Handles branches, jumps, and system calls to manage program execution flow.

This component is useful for:
-   Verifying the correctness of MIPS assembly programs.
-   Debugging MIPS assembly code at a high level of abstraction.
-   Understanding the fundamental execution semantics of MIPS programs.

## Key Components

### Simulator Core

The main simulator component (`src/functional_simulator/simulator.rs`) orchestrates the simulation process, implementing:

-   The core fetch-decode-execute cycle.
-   Program Counter (PC) management and updates.
-   Robust exception handling for various error conditions.
-   Support for breakpoints and step-by-step execution for debugging.

### Instruction Set

The instruction set module (`src/functional_simulator/instructions.rs`) defines and implements the behavior of each MIPS instruction. It handles:

-   Instruction encoding and decoding.
-   Execution logic for over 70 MIPS instructions, including R-type, I-type, J-type, and floating-point operations.
-   Interaction with the register file and memory system to perform operations.

### Memory System

The memory component (`src/functional_simulator/memory.rs`) models the main memory, providing:

-   A configurable byte-addressable memory space.
-   Support for reading and writing words (4 bytes), halfwords (2 bytes), and bytes.
-   Strict memory alignment checks for word and halfword accesses, raising exceptions on misalignment.
-   Basic memory protection and bounds checking.

### Register File

The register file (`src/functional_simulator/registers.rs`) manages the processor's registers, including:

-   32 general-purpose integer registers (`$zero`, `$at`, `$v0-$v1`, `$a0-$a3`, `$t0-$t9`, `$s0-$s7`, `$k0-$k1`, `$gp`, `$sp`, `$fp`, `$ra`).
-   Special registers: `HI` and `LO` for multiplication and division results.
-   32 floating-point registers (`$f0-$f31`).
-   The Program Counter (`PC`) and Floating-Point Control Status Register (`FCSR`).

## Usage

### Running the Functional Simulator

To run the functional simulator with a compiled MIPS binary:

```bash
cargo run --bin vmips_rust functional [options] <binary_file>
```

**Options:**
-   `--memory-size <size>`: Specify the memory size in bytes (default: 32768).
-   `--max-steps <steps>`: Set a maximum number of instructions to execute to prevent infinite loops (default: 1,000,000).
-   `--breakpoint <address>`: Add a breakpoint at a specific memory address.
-   `--trace`: Enable detailed instruction tracing.
-   `--debug`: Enable debug output.

### Example Workflow

First, assemble your MIPS assembly file using the built-in assembler:

```bash
cargo run --bin vmips_rust assemble input.s output.bin
```

Then, run the functional simulator with the generated binary:

```bash
cargo run --bin vmips_rust functional output.bin
```

## Program Execution Flow

The functional simulator executes programs by following these steps:

1.  **Program Loading**: The MIPS binary (containing data and text sections) is loaded into the simulator's memory.
2.  **Initialization**: General-purpose registers, floating-point registers, and special registers (HI, LO, PC, FCSR) are initialized to their default states.
3.  **Execution Loop**: The simulator enters a loop, performing the following for each instruction:
    -   **Fetch**: Retrieves the instruction word from memory at the current Program Counter (PC).
    -   **Decode**: Interprets the instruction word to identify the operation and its operands.
    -   **Execute**: Performs the operation, updating register values and/or memory contents as required.
    -   **PC Update**: Increments the PC to point to the next instruction, or updates it to a new target address for branches and jumps.
4.  **Termination**: The simulation continues until a termination condition is met (e.g., a `syscall` exit, a `break` instruction, reaching the end of the program, or exceeding the maximum instruction limit).
5.  **Final State Display**: After termination, the final state of the registers and relevant memory locations is displayed.

## Advanced Features

### Debugging Support

The functional simulator offers robust debugging capabilities:

-   **Breakpoints**: Set at specific memory addresses to pause execution.
-   **Step-by-Step Execution**: Execute one instruction at a time for detailed analysis.
-   **Register and Memory Inspection**: View the contents of registers and memory at any point during execution.
-   **Instruction Tracing**: Output a detailed log of each instruction executed, including PC, instruction word, and register changes.

### System Calls

The simulator supports a subset of MIPS system calls (syscalls) to interact with the host environment, enabling:

-   Console input/output (e.g., printing integers, strings).
-   Basic file operations (e.g., opening, reading, writing, closing files).
-   Program termination.

### Exception Handling

The simulator models various exception conditions that can occur during program execution:

-   **Invalid Instruction**: Encountering an unrecognized or malformed instruction.
-   **Memory Access Violations**: Attempting to access memory out of bounds or with incorrect alignment.
-   **Arithmetic Exceptions**: Such as division by zero.
-   **Breakpoint Exceptions**: Triggered by `break` instructions or user-defined breakpoints.

## Limitations

While comprehensive for functional correctness, the functional simulator has some inherent limitations:

-   **No Pipeline Modeling**: It does not simulate pipeline stages, hazards, or forwarding. For microarchitectural analysis, use the [Timing Simulator](timing-simulator.md).
-   **Simplified I/O**: System call implementation is basic and does not fully replicate a complex operating system.
-   **No Virtual Memory**: Does not include a Memory Management Unit (MMU) or virtual memory translation.

## Next Steps

After mastering the Functional Simulator, consider exploring:

-   The [Timing Simulator](timing-simulator.md) for in-depth pipeline and cache performance analysis.
-   The [Instruction Set](instruction-set.md) documentation for a full list of supported MIPS instructions.
-   The [Assembler](assembler.md) to write and compile your own MIPS programs.
-   The [Examples](examples.md) directory for practical MIPS assembly programs.
