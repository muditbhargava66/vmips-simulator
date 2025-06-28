# Timing Simulator

The Timing Simulator models a pipelined MIPS processor architecture, providing detailed information about performance, hazards, and pipeline behavior.

## Overview

Unlike the Functional Simulator which focuses solely on instruction execution, the Timing Simulator models the microarchitectural details:

- 5-stage pipeline (Fetch, Decode, Execute, Memory, Writeback)
- Hazard detection and handling
- Data forwarding
- Branch prediction
- Memory hierarchy with caches
- Performance analysis

This component is ideal for:
- Understanding pipeline dynamics
- Analyzing program performance
- Studying hazards and their resolution
- Exploring cache behavior

## Pipeline Architecture

### Pipeline Stages

The Timing Simulator implements a classic 5-stage RISC pipeline:

1. **Fetch (IF)**: Fetches the instruction from memory
2. **Decode (ID)**: Decodes the instruction and reads register operands
3. **Execute (EX)**: Performs ALU operations and address calculations
4. **Memory (MEM)**: Accesses data memory for loads and stores
5. **Writeback (WB)**: Writes results back to the register file

### Hazard Handling

The simulator models three types of hazards:

1. **Data Hazards**: Occur when an instruction depends on the result of a previous instruction
   - Handled through forwarding when possible
   - Stalls when forwarding cannot resolve the hazard

2. **Control Hazards**: Occur with branches and jumps
   - Branch prediction to minimize stalls
   - Pipeline flushing when prediction is incorrect

3. **Structural Hazards**: Occur when multiple instructions compete for the same resource
   - Stalling to resolve resource conflicts

### Branch Prediction

The simulator includes configurable branch prediction:

- Static prediction (always taken/not taken)
- 1-bit dynamic prediction
- 2-bit saturating counter prediction
- Branch target buffer (BTB)

### Memory Hierarchy

The memory system models:

- Separate L1 instruction and data caches
- Optional unified L2 cache
- Configurable cache parameters (size, associativity, block size)
- Various replacement policies (LRU, FIFO, Random)

## Usage

### Running the Timing Simulator

```bash
./target/release/vmips_rust timing [memory_size]
```

Where `memory_size` is an optional parameter specifying the memory size in bytes (default: 8192).

### Configuration Options

The timing simulator can be configured through code or command-line parameters:

- Pipeline configuration (stages, latencies)
- Cache parameters (size, associativity, block size)
- Branch prediction methods
- Forwarding enable/disable
- Visualization options

## Visualization

The Timing Simulator includes visualization capabilities:

- Pipeline stage contents for each cycle
- Hazard detection and resolution
- Cache hit/miss patterns
- Register and data forwarding

Example visualization output:

```
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

The simulator collects and reports various performance metrics:

- Cycles Per Instruction (CPI)
- Pipeline stall cycles
- Branch prediction accuracy
- Cache hit/miss rates
- Memory access latency

Example statistics:

```
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

## Advanced Features

### Out-of-Order Execution

The Timing Simulator can be configured to model out-of-order execution with Tomasulo's algorithm:

- Reservation stations
- Register renaming
- Reorder buffer
- Speculative execution

### Superscalar Execution

For advanced study, the simulator can model superscalar execution:

- Multiple instructions per cycle
- Multiple functional units
- Instruction scheduling

## Example Usage Scenarios

### Pipeline Behavior Analysis

```bash
# Run with basic configuration
./target/release/vmips_rust timing

# Examine pipeline visualization and stall patterns
```

### Branch Prediction Study

```bash
# Edit config to try different branch predictors
# Run the same program with each predictor
# Compare branch misprediction rates
```

### Cache Optimization

```bash
# Vary cache parameters
# Compare hit/miss rates for different configurations
# Analyze impact on overall performance
```

## Limitations

The Timing Simulator has some limitations:

- Simplified memory model
- Limited out-of-order capabilities
- No speculation recovery mechanisms
- No dynamic branch prediction adaptation

## Next Steps

After exploring the Timing Simulator, you might want to check out:

- [Functional Simulator](functional-simulator.md) for simpler instruction execution
- [Architecture Overview](architecture.md) for system design details
- [Examples](examples.md) for sample programs to study pipeline behavior
