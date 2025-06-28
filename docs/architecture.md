# VMIPS Simulator Architecture

This document describes the high-level architecture of the VMIPS Simulator, explaining its components and design philosophy.

## System Overview

![VMIPS Architecture Diagram](images/architecture.png)

The VMIPS Simulator consists of several major components:

1. **Functional Simulator** - A cycle-accurate functional model of a MIPS processor
2. **Timing Simulator** - A pipelined model with performance analysis capabilities
3. **Assembler** - Converts MIPS assembly to machine code
4. **Memory System** - Models memory hierarchy with caches
5. **Register File** - Maintains processor state
6. **Utilities** - Includes logging, parsing, and visualization tools

## Component Details

### Functional Simulator

Located in `src/functional_simulator/`, this component:

- Implements all core MIPS instructions
- Maintains accurate processor state
- Performs instruction fetch, decode, and execute cycles
- Models memory and register accesses

Key files:
- `simulator.rs` - Main simulation loop and instruction handling
- `instructions.rs` - Instruction definitions and execution
- `memory.rs` - Memory access and management
- `registers.rs` - Register file implementation

### Timing Simulator

Located in `src/timing_simulator/`, this component:

- Models a 5-stage pipeline (Fetch, Decode, Execute, Memory, Writeback)
- Detects and handles data, control, and structural hazards
- Implements forwarding for performance optimization
- Includes branch prediction for control hazard mitigation
- Provides performance metrics and visualization

Key files:
- `simulator.rs` - Pipeline coordination and execution
- `pipeline.rs` - Pipeline stage implementation
- `components.rs` - Cache and other architectural components
- `config.rs` - Configuration options for the simulator
- `visualization.rs` - Pipeline state visualization

### Memory System

The memory system models:

- Main memory with configurable size
- Instruction and data caches (L1)
- Optional L2 unified cache
- Cache hierarchies with various replacement policies

### Assembler

Located in `src/assembler/`, this component:

- Parses MIPS assembly syntax
- Converts assembly to machine code
- Handles labels and pseudo-instructions
- Generates executable binary files

## Design Philosophy

The VMIPS Simulator is designed with the following principles:

1. **Accuracy** - Correctly models the behavior of MIPS instructions
2. **Modularity** - Components are separated with clean interfaces
3. **Extensibility** - Easy to add new instructions or features
4. **Performance** - Efficient implementation for handling complex programs
5. **Educational Value** - Clear visualization and metrics for learning

## Memory Map

The simulator uses the following memory map:

- `0x00000000 - 0x00FFFFFF`: Text segment (Code)
- `0x10000000 - 0x10FFFFFF`: Data segment
- `0x7FFFFFFF - 0x7FFFFFFC`: Stack (grows downward)

## Data Flow

1. Program is loaded into memory
2. Instructions are fetched sequentially (or according to control flow)
3. Each instruction is decoded and executed
4. Results are written back to registers or memory
5. Performance statistics are collected and displayed

## Future Extensions

The architecture is designed to support future extensions including:

- Out-of-order execution
- More advanced branch prediction
- Superscalar capabilities
- Floating-point unit
- Memory management unit (MMU)
