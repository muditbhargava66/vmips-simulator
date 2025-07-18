# Timing Simulator

The Timing Simulator provides a detailed, cycle-accurate model of a MIPS processor, allowing for in-depth analysis of microarchitectural features, performance, and pipeline behavior.

## Overview

Unlike the [Functional Simulator](functional-simulator.md), the Timing Simulator delves into the microarchitectural details of a MIPS processor. It models:

-   **Configurable Pipeline**: From a classic 5-stage in-order pipeline to advanced out-of-order execution with Tomasulo's algorithm.
-   **Hazard Detection and Resolution**: Comprehensive handling of data, control, and structural hazards.
-   **Data Forwarding**: Implements forwarding paths to minimize stalls due to data dependencies.
-   **Advanced Branch Prediction**: Supports various prediction schemes, including 2-bit saturating counters and a Branch Target Buffer (BTB).
-   **Multi-level Memory Hierarchy**: Models L1 instruction and data caches, and an optional L2 cache, with configurable parameters and policies.
-   **Out-of-Order Execution (Tomasulo's Algorithm)**: A detailed implementation of Tomasulo's algorithm, including reservation stations, reorder buffer, and common data bus.
-   **Superscalar Execution**: Ability to simulate multiple instruction issues per cycle.
-   **Performance Analysis**: Collects and reports a wide array of performance metrics.

This component is essential for:
-   Understanding complex pipeline dynamics and microarchitectural interactions.
-   Analyzing program performance under different hardware configurations.
-   Studying the impact of hazards, caches, and branch prediction on execution speed.
-   Researching and experimenting with advanced processor design concepts.

## Pipeline Architecture

### Configurable Pipeline Stages

The Timing Simulator supports a flexible pipeline configuration. By default, it models a classic 5-stage RISC pipeline:

1.  **Fetch (IF)**: Fetches the instruction from the instruction cache.
2.  **Decode (ID)**: Decodes the instruction, reads register operands, and detects hazards.
3.  **Execute (EX)**: Performs ALU operations, address calculations, and handles branch outcomes.
4.  **Memory (MEM)**: Accesses the data cache for load and store operations.
5.  **Writeback (WB)**: Writes results back to the register file.

Each stage's latency can be configured, allowing for simulation of different pipeline depths and complexities.

### Hazard Handling

The simulator implements sophisticated mechanisms to detect and resolve pipeline hazards:

-   **Data Hazards (RAW, WAR, WAW)**: Occur when an instruction depends on the result of a previous instruction that has not yet completed. Handled primarily through:
    -   **Data Forwarding**: Results are forwarded directly from producing stages to consuming stages, bypassing the register file.
    -   **Stalling**: If forwarding is not possible (e.g., load-use hazard), the pipeline is stalled until the required data is available.
-   **Control Hazards**: Arise from branch and jump instructions, which alter the program flow. Mitigated by:
    -   **Branch Prediction**: The simulator predicts the outcome of branches to avoid stalling the pipeline.
    -   **Pipeline Flushing**: If a branch prediction is incorrect, the misfetched instructions are flushed from the pipeline, incurring a misprediction penalty.
-   **Structural Hazards**: Occur when multiple instructions attempt to use the same hardware resource simultaneously. Resolved by stalling one of the conflicting instructions.

### Advanced Branch Prediction

To minimize the impact of control hazards, the simulator includes several branch prediction schemes:

-   **Static Prediction**: Simple prediction strategies (e.g., always taken, always not taken).
-   **1-bit Dynamic Prediction**: Remembers the last outcome of a branch.
-   **2-bit Saturating Counter**: A more accurate dynamic predictor that uses a 2-bit state machine to predict branch outcomes.
-   **Branch Target Buffer (BTB)**: A cache that stores the predicted target addresses of recently executed branch instructions, enabling faster branch resolution.

### Memory Hierarchy

VMIPS Rust features a detailed memory hierarchy simulation:

-   **Separate L1 Caches**: Independent L1 instruction cache and L1 data cache.
-   **Optional Unified L2 Cache**: An optional second-level cache that serves as a victim cache for L1 misses.
-   **Configurable Cache Parameters**: Users can specify cache size, associativity (direct-mapped, set-associative), and block size.
-   **Replacement Policies**: Supports LRU (Least Recently Used), FIFO (First-In, First-Out), Random, and LFU (Least Frequently Used).
-   **Write Policies**: Includes Write-Through (writes to cache and main memory simultaneously) and Write-Back (writes only to cache, updates main memory on eviction).
-   **Allocation Policies**: Supports Write-Allocate (block is brought into cache on a write miss) and No-Write-Allocate (writes directly to main memory on a write miss).
-   **Prefetching**: Basic prefetching strategies can be enabled to reduce miss rates.

## Advanced Microarchitectural Features

### Out-of-Order Execution with Tomasulo's Algorithm

For a deeper understanding of modern processor designs, the Timing Simulator includes a robust implementation of Tomasulo's algorithm, enabling out-of-order execution:

-   **Reservation Stations**: Instructions are dispatched to available reservation stations, where they wait for their operands.
-   **Register Renaming**: Eliminates false data dependencies (WAR and WAW hazards) by mapping architectural registers to a larger pool of physical registers.
-   **Reorder Buffer (ROB)**: Instructions complete execution out of order but commit their results to the architectural state in program order, ensuring precise exceptions.
-   **Common Data Bus (CDB)**: Results from functional units are broadcast on the CDB, allowing dependent instructions in reservation stations and the ROB to quickly acquire their operands.

### Superscalar Execution

The simulator can model superscalar processors, capable of issuing multiple instructions per cycle. The `superscalar_width` parameter allows you to configure how many instructions can be issued in parallel, demonstrating the benefits and challenges of instruction-level parallelism.

## Usage

### Running the Timing Simulator

To run the timing simulator with an assembled MIPS binary:

```bash
cargo run --bin vmips_rust timing <binary_file> [options]
```

Refer to the [Getting Started](getting-started.md) guide for a full list of command-line options, including those for configuring pipeline stages, cache parameters, and advanced features.

## Visualization

The Timing Simulator offers powerful visualization capabilities to observe the internal workings of the processor:

-   **Cycle-by-Cycle Pipeline View**: Displays the instruction in each pipeline stage, along with its status (Busy, Stalled, Flushed).
-   **Hazard Visualization**: Highlights active data and control hazards, showing where stalls or flushes occur.
-   **Cache Hit/Miss Patterns**: Provides insights into cache performance by tracking hits and misses.
-   **Register and Memory State**: Allows inspection of the architectural state at any point.

Example visualization output (text-based):

```text
=== Pipeline State at Cycle 5 ===
+-------+-------+-------+-------+-------+
| Fetch | Decode| Exec  | Mem   | Write |
+-------+-------+-------+-------+-------+
| LW  B | ADD B | SUB B | AND B | OR  B |
+-------+-------+-------+-------+-------+

Active Hazards:
- RAW: Register $2 (Execute → Decode)
- Control: Branch at PC 0x00000014
```

## Performance Metrics

The simulator collects and reports various performance metrics to help analyze the efficiency of the simulated processor configuration:

-   **Cycles Per Instruction (CPI)**: Average number of clock cycles required to execute one instruction.
-   **Pipeline Stall Cycles**: Breakdown of stalls due to data, control, and structural hazards.
-   **Branch Prediction Accuracy**: Percentage of correctly predicted branches.
-   **Cache Hit/Miss Rates**: For each level of the cache hierarchy.
-   **Average Memory Access Time**: The average time taken to access memory, considering cache hits and misses.
-   **Functional Unit Utilization**: For Tomasulo's algorithm, shows how busy each functional unit is.

Example statistics output:

```text
Pipeline Statistics:
  Total Instructions: 1024
  Total Cycles: 1253
  Cycles Per Instruction (CPI): 1.22
  Total Stalls: 229
    Data Hazard Stalls: 142
    Control Hazard Stalls: 76
    Structural Hazard Stalls: 11
  Branch Mispredictions: 24

Cache Statistics:
  L1 Instruction Cache:
    Hit Rate: 95.2%
    Miss Rate: 4.8%
  L1 Data Cache:
    Hit Rate: 92.7%
    Miss Rate: 7.3%
```

## Example Usage Scenarios

### Pipeline Behavior Analysis

Run a program with different pipeline configurations (e.g., with/without forwarding) and observe the changes in CPI and stall cycles.

```bash
cargo run --bin vmips_rust timing my_program.bin --no-forwarding
```

### Branch Prediction Study

Experiment with different branch prediction algorithms and analyze their impact on misprediction rates and overall performance.

```bash
cargo run --bin vmips_rust timing my_program.bin --branch-predictor TwoBit
```

### Cache Optimization

Vary cache parameters (size, associativity, block size) and analyze the resulting hit/miss rates and average memory access times to find optimal configurations for specific workloads.

```bash
cargo run --bin vmips_rust timing my_program.bin --l1d-cache 8192 8 32
```

### Out-of-Order Execution Analysis

Enable Tomasulo's algorithm and observe how instructions are reordered, how dependencies are resolved via the CDB, and the utilization of reservation stations and functional units.

```bash
cargo run --bin vmips_rust timing my_program.bin --tomasulo
```

## Next Steps

-   Explore the [Instruction Set](instruction-set.md) for a complete list of supported MIPS instructions.
-   Refer to the [Architecture Overview](architecture.md) for a high-level understanding of the simulator's design.
-   Check out the [Examples](examples.md) directory for sample MIPS programs designed to illustrate various architectural concepts.
