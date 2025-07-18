# VMIPS Rust API Documentation

This document provides comprehensive API documentation for the VMIPS Rust simulator library.

## Table of Contents

1. [Core Modules](#core-modules)
2. [Functional Simulator](#functional-simulator)
3. [Timing Simulator](#timing-simulator)
4. [Memory System](#memory-system)
5. [Instruction Set](#instruction-set)
6. [Utilities](#utilities)
7. [Examples](#examples)

## Core Modules

### `vmips_rust::functional_simulator`

The functional simulator provides cycle-accurate simulation of MIPS instruction execution without timing details.

#### Key Components

- `Simulator`: Main functional simulator struct
- `Memory`: Memory subsystem implementation
- `Registers`: Register file implementation
- `Instructions`: MIPS instruction definitions and execution logic

### `vmips_rust::timing_simulator`

The timing simulator provides detailed pipeline simulation with configurable timing parameters.

#### Key Components

- `Simulator`: Main timing simulator struct
- `Pipeline`: Pipeline implementation with hazard detection
- `Cache`: Cache hierarchy simulation
- `BranchPredictor`: Branch prediction mechanisms
- `Tomasulo`: Out-of-order execution engine

## Functional Simulator

### Creating a Functional Simulator

```rust
use vmips_rust::functional_simulator::simulator::Simulator;

// Create simulator with 8KB memory
let mut simulator = Simulator::new(8192);
```

### Loading Programs

```rust
// Load instruction at address 0
simulator.memory.write_word_init(0, 0x20010001); // ADDI $1, $0, 1

// Load data into memory
simulator.memory.write_word_init(0x1000, 42);
```

### Executing Instructions

```rust
// Execute single instruction
simulator.step();

// Run until completion
simulator.run();
```

### Accessing State

```rust
// Read register values
let reg_value = simulator.registers.read(1);

// Read memory
let mem_value = simulator.memory.read_word(0x1000);

// Get program counter (through public interface)
// Note: PC is managed internally
```

## Timing Simulator

### Configuration

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig, BranchPredictorType};
use vmips_rust::timing_simulator::simulator::Simulator;

// Configure pipeline
let pipeline_config = PipelineConfig::new(5)  // 5-stage pipeline
    .with_latencies(vec![1, 1, 1, 1, 1])      // Stage latencies
    .with_forwarding(true)                     // Enable forwarding
    .with_branch_prediction(true, BranchPredictorType::TwoBit);

// Configure caches
let icache_config = CacheConfig::new(32768, 4, 64);  // 32KB, 4-way, 64B blocks
let dcache_config = CacheConfig::new(32768, 4, 64);

// Create simulator
let mut simulator = Simulator::new(
    pipeline_config,
    icache_config,
    dcache_config,
    65536  // 64KB memory
);
```

### Visualization

```rust
// Enable pipeline visualization
simulator.enable_visualization(true);
simulator.configure_visualization(true, true);  // Show pipeline and cache stats

// Set output format
use vmips_rust::timing_simulator::visualization::OutputFormat;
simulator.set_visualization_format(OutputFormat::Text);
```

### Execution Modes

```rust
use vmips_rust::timing_simulator::simulator::ExecutionMode;

// The simulator supports different execution modes:
// - InOrder: Traditional in-order pipeline
// - OutOfOrder: Tomasulo-based out-of-order execution

// Mode is configured automatically based on pipeline configuration
```

## Memory System

### Memory Interface

```rust
use vmips_rust::functional_simulator::memory::Memory;

let mut memory = Memory::new(4096);  // 4KB memory

// Write operations
memory.write_word(0x1000, 0x12345678);      // Normal write
memory.write_word_init(0x1004, 0x87654321); // Initialization write

// Read operations
let value = memory.read_word(0x1000);  // Returns Option<u32>

// Byte operations
memory.write_byte(0x2000, 0xFF);
let byte_val = memory.read_byte(0x2000);
```

### Cache Configuration

```rust
use vmips_rust::timing_simulator::config::CacheConfig;

// Create cache configuration
let cache_config = CacheConfig::new(
    32768,  // Size in bytes
    4,      // Associativity
    64      // Block size in bytes
);

// Cache policies are configured internally:
// - LRU replacement policy
// - Write-back with write-allocate
```

## Instruction Set

### Supported Instructions

The simulator supports a comprehensive MIPS instruction set:

#### R-Type Instructions
- Arithmetic: `ADD`, `SUB`, `MULT`, `DIV`, `DIVU`
- Logical: `AND`, `OR`, `XOR`, `NOR`
- Shift: `SLL`, `SRL`, `SRA`, `SLLV`, `SRLV`, `SRAV`
- Comparison: `SLT`, `SLTU`
- Jump: `JR`, `JALR`
- Move: `MFHI`, `MFLO`, `MTHI`, `MTLO`

#### I-Type Instructions
- Arithmetic: `ADDI`, `ADDIU`
- Logical: `ANDI`, `ORI`, `XORI`
- Comparison: `SLTI`, `SLTIU`
- Memory: `LW`, `SW`, `LB`, `LBU`, `LH`, `LHU`, `SB`, `SH`
- Branch: `BEQ`, `BNE`, `BGTZ`, `BLEZ`, `BLTZ`, `BGEZ`
- Load Upper: `LUI`

#### J-Type Instructions
- Jump: `J`, `JAL`

#### Floating-Point Instructions
- Arithmetic: `ADD.S`, `SUB.S`, `MUL.S`, `DIV.S`
- Conversion: `CVT.S.W`, `CVT.W.S`
- Comparison: `C.EQ.S`, `C.LT.S`, `C.LE.S`
- Move: `MOV.S`, `ABS.S`, `NEG.S`
- Memory: `LWC1`, `SWC1`
- Branch: `BC1T`, `BC1F`

#### Special Instructions
- `NOP`: No operation
- `SYSCALL`: System call
- `BREAK`: Breakpoint

### Instruction Encoding

```rust
use vmips_rust::functional_simulator::instructions::Instruction;
use vmips_rust::functional_simulator::simulator::decode_instruction;

// Decode instruction from 32-bit word
let instruction = decode_instruction(0x20010001);  // ADDI $1, $0, 1

match instruction {
    Instruction::Addi { rt, rs, immediate } => {
        println!("ADDI ${}, ${}, {}", rt, rs, immediate);
    }
    _ => {}
}
```

## Utilities

### Logging

```rust
use vmips_rust::utils::logger::{Logger, LogLevel};

// Create logger
let mut logger = Logger::new(Some("simulation.log"), LogLevel::Debug);

// Log messages
logger.info("Starting simulation");
logger.debug("Detailed debug information");
logger.warning("Warning message");
logger.error("Error occurred");
```

### Assembly Parsing

```rust
use vmips_rust::assembler::Assembler;

// Create assembler
let mut assembler = Assembler::new();

// Assemble MIPS code
let assembly = "
    addi $t0, $zero, 10
    addi $t1, $zero, 20
    add $t2, $t0, $t1
";

let machine_code = assembler.assemble(assembly);
```

### ELF Loading

```rust
use vmips_rust::elf_loader::ElfLoader;

// Load ELF file
let elf_loader = ElfLoader::load_file("program.elf")?;

// Get entry point
let entry_point = elf_loader.entry_point();

// Load into memory
elf_loader.load_into_memory(&mut simulator.memory)?;
```

## Examples

### Basic Functional Simulation

```rust
use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    let mut simulator = Simulator::new(4096);
    
    // Load simple program: add two numbers
    simulator.memory.write_word_init(0, 0x20010005);  // ADDI $1, $0, 5
    simulator.memory.write_word_init(4, 0x2002000A);  // ADDI $2, $0, 10
    simulator.memory.write_word_init(8, 0x00221820);  // ADD $3, $1, $2
    
    // Execute program
    for _ in 0..3 {
        simulator.step();
    }
    
    // Check result
    assert_eq!(simulator.registers.read(3), 15);
}
```

### Timing Simulation with Visualization

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig};
use vmips_rust::timing_simulator::simulator::Simulator;

fn main() {
    let pipeline_config = PipelineConfig::new(5).with_forwarding(true);
    let cache_config = CacheConfig::new(1024, 2, 32);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        cache_config.clone(),
        cache_config,
        4096
    );
    
    // Enable visualization
    simulator.enable_visualization(true);
    
    // Load and run program
    // ... (load instructions)
    
    // Run with cycle limit
    let max_cycles = 100;
    for cycle in 0..max_cycles {
        // Visualization output will be generated automatically
        if simulator.pc >= program_end {
            break;
        }
    }
}
```

### Performance Analysis

```rust
use vmips_rust::timing_simulator::simulator::Simulator;

fn analyze_performance(simulator: &Simulator) {
    // Access performance statistics
    if let Some(stats) = simulator.get_statistics() {
        println!("Instructions executed: {}", stats.instructions_executed);
        println!("Cycles elapsed: {}", stats.cycles_elapsed);
        println!("CPI: {:.2}", stats.cycles_elapsed as f64 / stats.instructions_executed as f64);
        
        // Cache statistics
        println!("I-Cache hit rate: {:.2}%", stats.icache_hit_rate * 100.0);
        println!("D-Cache hit rate: {:.2}%", stats.dcache_hit_rate * 100.0);
        
        // Branch prediction statistics
        println!("Branch prediction accuracy: {:.2}%", stats.branch_prediction_accuracy * 100.0);
    }
}
```

## Error Handling

Most API functions return `Result` types or `Option` types for error handling:

```rust
// Memory operations return Option<T>
match memory.read_word(address) {
    Some(value) => println!("Read value: {}", value),
    None => println!("Invalid memory access"),
}

// File operations return Result<T, E>
match ElfLoader::load_file("program.elf") {
    Ok(loader) => { /* use loader */ },
    Err(e) => println!("Failed to load ELF: {:?}", e),
}
```

## Thread Safety

The simulator components are not thread-safe by default. For multi-threaded usage, wrap components in appropriate synchronization primitives:

```rust
use std::sync::{Arc, Mutex};

let simulator = Arc::new(Mutex::new(Simulator::new(4096)));

// Use from multiple threads
let sim_clone = simulator.clone();
std::thread::spawn(move || {
    let mut sim = sim_clone.lock().unwrap();
    sim.step();
});
```

## Performance Considerations

- Use `write_word_init()` for initial memory setup to bypass permission checks
- Enable compiler optimizations with `--release` flag for production use
- Consider memory size carefully - larger memories increase simulation overhead
- Cache configurations significantly impact timing simulation performance
- Visualization adds overhead - disable for performance-critical simulations

## Version Compatibility

This API documentation is for VMIPS Rust v0.2.0. Breaking changes between versions will be documented in the CHANGELOG.md file.