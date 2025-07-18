# Example MIPS Programs

This directory (`examples/`) contains several example MIPS programs implemented as Rust executables that demonstrate various features of the VMIPS Simulator. These examples showcase basic arithmetic, array manipulation, and algorithm implementations using the simulator's API directly.

## Running Examples

All examples are implemented as Rust programs that use the VMIPS simulator library. You can run them directly using Cargo:

```bash
cargo run --example [example_name]
```

For example:
```bash
cargo run --example bubble_sort
cargo run --example dot_product
cargo run --example factorial
```

## Available Examples

### `bubble_sort.rs`

-   **Description**: Demonstrates a single pass of the Bubble Sort algorithm on an array [5, 2, 8, 1, 9]. Shows how the largest element moves to the end.
-   **Location**: `examples/bubble_sort.rs`
-   **Concepts Demonstrated**: Array traversal, conditional swapping, memory load/store operations, comparison instructions.
-   **Expected Output**: Array after one pass with largest element (9) at the end.

### `dot_product.rs`

-   **Description**: Calculates the dot product of two vectors A = [1, 2, 3] and B = [4, 5, 6], resulting in 32.
-   **Location**: `examples/dot_product.rs`
-   **Concepts Demonstrated**: Vector operations, multiplication using MULT/MFLO, accumulation patterns, memory addressing.
-   **Expected Output**: Dot product result of 32.

### `factorial.rs`

-   **Description**: Computes 6! = 720 using step-by-step multiplication (6 × 5 × 4 × 3 × 2 × 1).
-   **Location**: `examples/factorial.rs`
-   **Concepts Demonstrated**: Sequential multiplication, MULT/MFLO instruction usage, iterative calculations.
-   **Expected Output**: Factorial result of 720.

### `matrix_multiply.rs`

-   **Description**: Multiplies two 2×2 matrices: [[1,2],[3,4]] × [[5,6],[7,8]] = [[19,22],[43,50]].
-   **Location**: `examples/matrix_multiply.rs`
-   **Concepts Demonstrated**: Matrix operations, nested calculations, multiple memory accesses, complex arithmetic.
-   **Expected Output**: Resulting 2×2 matrix with correct values.

### `simple_calculator.rs`

-   **Description**: Performs complex arithmetic: (15 + 25) × 3 - 10 ÷ 2 = 115, demonstrating order of operations.
-   **Location**: `examples/simple_calculator.rs`
-   **Concepts Demonstrated**: Multiple arithmetic operations, division using DIV/MFLO, order of operations, intermediate result storage.
-   **Expected Output**: Final calculation result of 115.

### `fibonacci.rs`

-   **Description**: Calculates the 10th Fibonacci number (F(10) = 55) using step-by-step computation.
-   **Location**: `examples/fibonacci.rs`
-   **Concepts Demonstrated**: Sequential calculations, mathematical sequences, iterative algorithms.
-   **Expected Output**: 10th Fibonacci number (55).

### `array_sum.rs`

-   **Description**: Sums an array of integers [10, 20, 30, 40, 50] to get 150.
-   **Location**: `examples/array_sum.rs`
-   **Concepts Demonstrated**: Array processing, accumulation, sequential memory access, addition operations.
-   **Expected Output**: Sum of array elements (150).

### `string_length.rs`

-   **Description**: Counts characters in the null-terminated string "HELLO" to get length 5.
-   **Location**: `examples/string_length.rs`
-   **Concepts Demonstrated**: String processing, null termination, character counting, conditional termination.
-   **Expected Output**: String length of 5.

## Studying Performance with Examples

You can use these examples to study the performance characteristics of the Timing Simulator. For instance, you can:

-   **Analyze Pipeline Stalls**: Run `bubble_sort.bin` on the timing simulator and observe the pipeline visualization to identify data and control hazards.
-   **Evaluate Cache Performance**: Modify the `matrix_multiply.s` program or the simulator's cache configurations to see the impact on cache hit/miss rates.
-   **Compare Execution Modes**: Run the same example on both the functional and timing simulators to understand the difference in execution speed and detail.

## Contributing New Examples

We welcome contributions of new example MIPS programs! If you have a well-commented and interesting MIPS assembly program that demonstrates a particular concept or algorithm, feel free to contribute it. Refer to the [Contributing](contributing.md) guide for more details.
