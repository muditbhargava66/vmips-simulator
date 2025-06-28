# Functional Simulator

The Functional Simulator provides a cycle-accurate model of a MIPS processor that executes instructions sequentially without modeling the pipeline or timing characteristics.

## Overview

The functional simulator focuses on the correct execution of MIPS instructions. It accurately models:

- Register file state
- Memory accesses
- Instruction behavior
- Control flow

This component is useful for:
- Verifying program correctness
- Debugging MIPS assembly code
- Understanding the execution of MIPS programs

## Key Components

### Simulator Core

The main simulator component (`src/functional_simulator/simulator.rs`) implements:

- The simulation loop
- Instruction fetch, decode, execute cycle
- Program counter (PC) management
- Exception handling

### Instruction Set

The instruction set (`src/functional_simulator/instructions.rs`) provides:

- Instruction encoding/decoding
- Execution behavior for each instruction
- Access to register file and memory

### Memory System

The memory component (`src/functional_simulator/memory.rs`) models:

- Byte-addressable memory space
- Word, halfword, and byte access modes
- Memory protection (read/write permissions)

### Register File

The register file (`src/functional_simulator/registers.rs`) implements:

- 32 general-purpose registers
- Special registers (HI, LO)
- Register access control

## Usage

### Running the Functional Simulator

```bash
./target/release/vmips_rust functional [memory_size]
```

Where `memory_size` is an optional parameter specifying the memory size in bytes (default: 8192).

### Example

```bash
# Assemble a program
./target/release/main_assembler input.s -o program.bin

# Run the functional simulator
./target/release/vmips_rust functional
```

## Program Execution

The simulator follows this execution flow:

1. Load program into memory
2. Initialize registers and PC
3. For each instruction:
   - Fetch instruction from memory at PC
   - Decode instruction
   - Execute instruction (update registers/memory)
   - Update PC (next instruction or branch target)
4. Continue until termination condition (e.g., `syscall` exit or end of program)
5. Display final state

## Features

### Debugging Support

The functional simulator includes debugging features:

- Breakpoints (`break` instruction or specified addresses)
- Step-by-step execution
- Register and memory state inspection
- Instruction tracing

### System Calls

The simulator supports MIPS system calls for I/O and program control:

- Console input/output
- File operations
- Dynamic memory allocation
- Program termination

### Exception Handling

The simulator models basic exception conditions:

- Invalid instructions
- Memory access violations
- Arithmetic exceptions (divide by zero)
- System call exceptions

## Limitations

The functional simulator has some limitations:

- No pipeline modeling (use the Timing Simulator for this)
- Limited floating-point support
- No virtual memory or memory management unit
- Limited system call implementation

## Example Program Execution

Here's an example of how a simple program is executed:

```
# Program: Calculate sum of numbers 1 to 10
# Initial state: $0=0, all other registers=0, PC=0

Instruction 1: addi $2, $0, 0    # Initialize sum=0 in $2
  - Update $2 = 0 + 0 = 0
  - PC += 4 (now PC=4)

Instruction 2: addi $3, $0, 1    # Initialize i=1 in $3
  - Update $3 = 0 + 1 = 1
  - PC += 4 (now PC=8)

Instruction 3: addi $4, $0, 11   # Set limit=11 in $4
  - Update $4 = 0 + 11 = 11
  - PC += 4 (now PC=12)

Instruction 4: beq $3, $4, 3     # if i==limit, exit loop
  - Compare $3(1) and $4(11) -> not equal
  - PC += 4 (now PC=16)

Instruction 5: add $2, $2, $3    # sum += i
  - Update $2 = 0 + 1 = 1
  - PC += 4 (now PC=20)

Instruction 6: addi $3, $3, 1    # i++
  - Update $3 = 1 + 1 = 2
  - PC += 4 (now PC=24)

Instruction 7: j 3               # jump back to loop condition
  - PC = (PC & 0xF0000000) | (3 << 2) = 12
  
... (loop continues until i=11) ...

Final state: $2=55 (sum of numbers 1 to 10), $3=11, $4=11
```

## Next Steps

After getting familiar with the Functional Simulator, you might want to explore:

- [Timing Simulator](timing-simulator.md) for pipeline modeling
- [Assembler](assembler.md) for writing your own MIPS programs
- [Examples](examples.md) for sample programs
