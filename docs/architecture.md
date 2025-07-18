# Architecture Overview

The VMIPS Rust simulator is designed with a modular architecture, allowing for flexible simulation of various MIPS processor configurations. It primarily consists of a **Functional Simulator** and a **Timing Simulator**, complemented by an **Assembler** and **Visualization Tools**.

## High-Level System Flow

```
+-----------------+       +-----------------+
| MIPS Assembly   |       | VMIPS Rust      |
| Code (.s files) |------>| Assembler       |
+-----------------+       | (main_assembler)|
                          +--------+--------+
                                   |
                                   v
                          +-----------------+
                          | Binary Program  |
                          | (.bin file)     |
                          +--------+--------+
                                   |
                                   v
                 +-------------------------------------+
                 |          VMIPS Rust Simulator       |
                 |          (vmips_rust executable)    |
                 +-------------------------------------+
                 |                                     |
        +--------v--------+                   +--------v--------+
        | Functional      |                   | Timing Simulator|
        | Simulator       |                   | (Pipelined,     |
        | (Correctness)   |                   |  Performance)   |
        +-----------------+                   +-----------------+
                 |                                     |
        +--------v----------------+--------------------v--------+
        |              Visualization & Debugging                |
        |              (Logs, Dumps, Statistics)                |
        +-------------------------------------------------------+
```

## Component Breakdown

### 1. Assembler (`src/assembler/`)

The built-in assembler converts MIPS assembly code (`.s` files) into machine code (binary programs). It supports a wide range of MIPS instructions, directives (e.g., `.data`, `.text`, `.word`, `.byte`, `.half`, `.ascii`, `.asciiz`, `.space`, `.align`), and pseudo-instructions (e.g., `move`, `li`, `la`, `b`). It performs two passes to handle labels and symbol resolution.

### 2. Functional Simulator (`src/functional_simulator/`)

The functional simulator focuses solely on the correct execution of MIPS instructions and the accurate update of the architectural state (registers and memory). It does not model any timing aspects or pipeline behavior. It's primarily used for verifying the correctness of instruction implementations and for simple program execution.

**Key Sub-components:**
-   **Simulator Core**: Manages the simulation loop, instruction fetch-decode-execute cycle, PC management, and exception handling.
-   **Instruction Set**: Defines and implements the behavior of each MIPS instruction.
-   **Memory System**: Models byte-addressable memory with word, halfword, and byte access modes, including alignment checks.
-   **Register File**: Implements 32 general-purpose registers, HI/LO registers, and 32 floating-point registers.

### 3. Timing Simulator (`src/timing_simulator/`)

The timing simulator is a more advanced component that models the cycle-by-cycle behavior of a MIPS processor, including various microarchitectural features:

-   **Pipelined Execution**: Implements a configurable pipeline (default 5-stage: Fetch, Decode, Execute, Memory, Writeback) with adjustable stage latencies.
-   **Hazard Detection and Resolution**: Detects and handles data hazards (RAW, WAR, WAW) and control hazards (branches, jumps) through stalling and forwarding mechanisms.
-   **Data Forwarding**: Implements data forwarding paths to reduce stalls caused by data dependencies.
-   **Cache Hierarchy**: Models a multi-level cache system (L1 instruction cache, L1 data cache, and optional L2 cache) with configurable parameters such as size, associativity, block size, replacement policies (LRU, FIFO, Random, LFU), write policies (write-through, write-back), and allocation policies (write-allocate, no-write-allocate). It also supports prefetching strategies.
-   **Branch Prediction**: Incorporates advanced branch prediction techniques, including a 2-bit saturating counter for dynamic prediction and a Branch Target Buffer (BTB) for predicting branch targets.
-   **Tomasulo's Algorithm (Out-of-Order Execution)**: A key feature enabling out-of-order execution. This implementation includes:
    -   **Reservation Stations**: Buffers for holding instructions and their operands.
    -   **Reorder Buffer (ROB)**: Ensures in-order commitment of instructions.
    -   **Common Data Bus (CDB)**: Broadcasts results from functional units.
    -   **Register Renaming**: Eliminates WAR and WAW hazards.
-   **Superscalar Execution**: Can be configured to simulate a superscalar processor, allowing it to issue multiple independent instructions per cycle.

### 4. Visualization and Debugging Tools

The simulator provides extensive tools for understanding and debugging program execution:

-   **Cycle-by-Cycle Pipeline Visualization**: Displays the state of each pipeline stage, showing instruction flow, stalls, and flushes.
-   **Cache Hierarchy Visualization**: Provides statistics on cache accesses, hits, misses, and hit rates for each level of the cache.
-   **Register and Memory Dumps**: Allows inspection of the architectural state at any point during simulation.
-   **Instruction Tracing**: Detailed logs of instruction fetch, decode, execute, and writeback stages.
-   **Performance Statistics**: Calculates key metrics such as Instructions Per Cycle (IPC), branch misprediction rates, and functional unit utilization.

This modular and feature-rich architecture makes VMIPS Rust a powerful tool for both learning and research in computer architecture.
