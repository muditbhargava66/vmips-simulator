# Example MIPS Programs

This directory (`examples/`) contains several example MIPS assembly programs that demonstrate various features of the VMIPS Simulator, including basic arithmetic, array manipulation, and algorithm implementations. These examples can be used to test the functional and timing simulators, and to understand how different MIPS instructions and architectural features behave.

## Running Examples

To run any of the example programs, you first need to assemble them into a binary format using the `main_assembler` tool, and then execute the binary with either the functional or timing simulator.

### General Steps:

1.  **Assemble the example program**: Replace `[example_name].s` with the actual assembly file (e.g., `bubble_sort.s`).
    ```bash
    cargo run --bin main_assembler assemble examples/[example_name].s [output_name].bin
    ```
    Example:
    ```bash
    cargo run --bin main_assembler assemble examples/bubble_sort.s bubble_sort.bin
    ```

2.  **Run the assembled binary**: Choose either the `functional` or `timing` simulator.
    ```bash
    cargo run --bin vmips_rust functional [output_name].bin
    # OR
    cargo run --bin vmips_rust timing [output_name].bin
    ```
    Example:
    ```bash
    cargo run --bin vmips_rust functional bubble_sort.bin
    ```

## Available Examples

### `bubble_sort.s`

-   **Description**: Implements the Bubble Sort algorithm to sort an array of integers in ascending order. This example demonstrates array manipulation, loops, and conditional branching.
-   **Location**: `examples/bubble_sort.s`
-   **Concepts Demonstrated**: Loops, conditional branches, memory access (loads and stores), array indexing.

### `dot_product.s`

-   **Description**: Calculates the dot product of two vectors. This example showcases basic arithmetic operations and iterative processing of data structures.
-   **Location**: `examples/dot_product.s`
-   **Concepts Demonstrated**: Loops, multiplication, addition, memory access.

### `factorial.s`

-   **Description**: Computes the factorial of a given non-negative integer using an iterative approach. Useful for understanding basic arithmetic and loop structures.
-   **Location**: `examples/factorial.s`
-   **Concepts Demonstrated**: Loops, multiplication, basic integer operations.

### `matrix_multiply.s`

-   **Description**: Implements matrix multiplication for two small matrices. This is a more complex example demonstrating nested loops, multi-dimensional array access, and extensive arithmetic operations.
-   **Location**: `examples/matrix_multiply.s`
-   **Concepts Demonstrated**: Nested loops, complex memory addressing, multiplication, addition.

## Studying Performance with Examples

You can use these examples to study the performance characteristics of the Timing Simulator. For instance, you can:

-   **Analyze Pipeline Stalls**: Run `bubble_sort.bin` on the timing simulator and observe the pipeline visualization to identify data and control hazards.
-   **Evaluate Cache Performance**: Modify the `matrix_multiply.s` program or the simulator's cache configurations to see the impact on cache hit/miss rates.
-   **Compare Execution Modes**: Run the same example on both the functional and timing simulators to understand the difference in execution speed and detail.

## Contributing New Examples

We welcome contributions of new example MIPS programs! If you have a well-commented and interesting MIPS assembly program that demonstrates a particular concept or algorithm, feel free to contribute it. Refer to the [Contributing](contributing.md) guide for more details.
