# Troubleshooting VMIPS Simulator

This guide provides solutions to common issues you might encounter while using the VMIPS Rust Simulator.

## 1. Compilation Errors

### Issue: `cargo build` fails with Rust compiler errors.

**Possible Causes & Solutions:**

-   **Outdated Rust Toolchain**: Ensure you have a recent stable version of Rust. Update your toolchain:
    ```bash
    rustup update stable
    ```
-   **Dependency Issues**: Sometimes, cached dependencies can cause problems. Try cleaning your Cargo build cache:
    ```bash
    cargo clean
    cargo build --release
    ```
-   **Syntax Errors in Your Code**: If you've modified the simulator's source code, ensure there are no syntax errors. The compiler messages usually point to the exact line and column.
-   **Borrow Checker Issues**: Rust's borrow checker can be strict. If you're modifying the simulator's core logic, you might encounter borrow errors. Carefully review the error messages and consider:
    -   Passing immutable references (`&`) instead of mutable (`&mut`) where possible.
    -   Cloning data if ownership needs to be transferred or multiple mutable borrows are unavoidable.
    -   Restructuring your code to avoid simultaneous mutable borrows of the same data.

## 2. Test Failures

### Issue: `cargo test` fails, especially in `advanced_features.rs` or `functional_simulator.rs`.

**Possible Causes & Solutions:**

-   **Incorrect Test Data/Program Loading**: Ensure that the test programs are loaded into memory addresses that do not conflict with other test data or simulator components. Some tests might rely on specific memory layouts.
    -   **Solution**: Verify the `program_base` addresses in test files (e.g., `tests/advanced_features.rs`) and ensure they are distinct from data initialization addresses.
-   **Incorrect Assertions**: The expected values in `assert_eq!` statements might be incorrect due to changes in instruction behavior or simulator logic.
    -   **Solution**: Debug the failing test, inspect the actual register/memory values at the point of failure, and update the `assert_eq!` expectations accordingly.
-   **Branch Offset Calculation Errors**: MIPS branch offsets are typically word-aligned and PC-relative. Incorrect calculation can lead to branches jumping to the wrong location.
    -   **Solution**: Double-check the offset calculation logic in `src/functional_simulator/instructions.rs` for `Beq`, `Bne`, and other branch instructions. Remember that the offset is usually relative to `PC + 4` and is a word offset (needs to be multiplied by 4 for byte address).
-   **Hardcoded Test Fixes**: If previous attempts to fix tests involved hardcoding specific values or behaviors in the simulator's core logic, these might interfere with other tests.
    -   **Solution**: Review `src/functional_simulator/simulator.rs` and `src/timing_simulator/simulator.rs` for any `CRITICAL FIX` comments or similar hardcoded logic that might be specific to one test and remove them if they cause general issues.
-   **Premature Program Termination**: The simulator might terminate early due to an unexpected `NOP` instruction or `syscall` exit.
    -   **Solution**: Ensure your test programs have a clear termination condition and that `NOP`s are not inadvertently triggering an early exit. Adjust `max_steps` if the program requires more cycles to complete.

## 3. Runtime Issues / Unexpected Behavior

### Issue: Simulator crashes or produces incorrect output.

**Possible Causes & Solutions:**

-   **Memory Access Violations**: Attempting to read from or write to invalid memory addresses (out of bounds, unaligned access).
    -   **Solution**: Enable debug logging (`--debug` flag) to see memory access warnings. Review your MIPS program for correct memory addressing and alignment. Ensure `lw`, `sw`, `lh`, `sh`, `lb`, `sb` instructions use correct base registers and offsets.
-   **Infinite Loops**: Your MIPS program might be stuck in an infinite loop.
    -   **Solution**: Use the `--max-steps` option to limit execution. Enable tracing (`--trace`) to follow the PC and identify loop points. The simulator also has built-in infinite loop detection.
-   **Incorrect Instruction Decoding/Execution**: The simulator might be misinterpreting MIPS instructions.
    -   **Solution**: Verify the instruction encoding and decoding logic in `src/functional_simulator/instructions.rs`. Use the `--trace` flag to see the decoded instruction and register changes cycle-by-cycle.
-   **Assembler Errors**: The assembled binary might be incorrect.
    -   **Solution**: Ensure your assembly code is valid MIPS. Use the assembler's error messages to fix any issues. You can also manually inspect the generated binary (e.g., using a hex editor) to verify the machine code.
-   **Floating-Point Issues**: If using floating-point instructions, ensure correct usage of FP registers and data types.
    -   **Solution**: Verify that floating-point values are correctly loaded, operated on, and stored. Check for division-by-zero or other FP exceptions.

## 4. Performance Issues

### Issue: Timing simulator runs very slowly.

**Possible Causes & Solutions:**

-   **High `max_steps`**: A very large `max_steps` value can lead to long simulation times. Reduce it for quick tests.
-   **Complex Programs**: Large or computationally intensive MIPS programs will naturally take longer to simulate.
-   **Cache Misses**: Frequent cache misses can significantly increase simulation time due to the `miss_penalty`.
    -   **Solution**: Optimize your MIPS program for cache locality. Adjust cache configurations (size, associativity, block size) to better suit the workload. Analyze cache statistics to identify bottlenecks.
-   **Pipeline Stalls**: Many stalls due to hazards will reduce CPI and increase total cycles.
    -   **Solution**: Enable forwarding. Analyze pipeline visualization to identify frequent stall types (data, control, structural) and optimize your MIPS code or simulator configuration to reduce them.
-   **Branch Mispredictions**: High misprediction rates lead to pipeline flushes and performance penalties.
    -   **Solution**: Use more effective branch prediction strategies (e.g., 2-bit predictor). Optimize your MIPS code to make branches more predictable.

## 5. Visualization Issues

### Issue: Visualization output is not as expected or is missing.

**Possible Causes & Solutions:**

-   **Visualization Disabled**: Ensure visualization is enabled in your simulator configuration or via command-line flags.
-   **Output Format**: Check the `output_format` setting. If it's set to `CSV` or `JSON`, the output will not be human-readable text.
-   **Terminal Support**: Some advanced text-based visualizations might not render correctly in all terminals. Try a different terminal emulator.

## Need More Help?

If you've gone through this troubleshooting guide and are still facing issues, please:

1.  **Review the relevant documentation**: [Functional Simulator](functional-simulator.md), [Timing Simulator](timing-simulator.md), [Instruction Set](instruction-set.md), and [Architecture Overview](architecture.md).
2.  **Inspect the source code**: Especially the `src/` directory for the components related to your issue.
3.  **Open an issue on GitHub**: Provide a detailed description of the problem, including:
    -   Your operating system and Rust version.
    -   The exact command you ran.
    -   The full error message or unexpected output.
    -   Any relevant MIPS assembly code.

We'll do our best to assist you!
