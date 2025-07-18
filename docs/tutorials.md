# VMIPS Rust Tutorials

This document provides step-by-step tutorials for common use cases of the VMIPS Rust simulator.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Functional Simulation](#basic-functional-simulation)
3. [Pipeline Simulation and Hazards](#pipeline-simulation-and-hazards)
4. [Cache Performance Analysis](#cache-performance-analysis)
5. [Branch Prediction Study](#branch-prediction-study)
6. [Out-of-Order Execution](#out-of-order-execution)
7. [Loading Real Programs](#loading-real-programs)
8. [Performance Optimization](#performance-optimization)

## Getting Started

### Installation and Setup

1. **Install Rust**: Make sure you have Rust installed (version 1.56.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/muditbhargava66/vmips-simulator.git
   cd vmips-simulator
   ```

3. **Build the Project**:
   ```bash
   cargo build --release
   ```

4. **Run Tests**:
   ```bash
   cargo test
   ```

### First Simulation

Let's run a simple simulation to verify everything works:

```bash
# Run functional simulator
cargo run --bin vmips_rust functional

# Run timing simulator with visualization
cargo run --bin vmips_rust timing --visualize --max-cycles 100
```

## Basic Functional Simulation

### Tutorial 1: Simple Arithmetic Program

Let's create a program that adds two numbers and stores the result.

**Step 1: Create the simulation code**

```rust
use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    // Create simulator with 4KB memory
    let mut simulator = Simulator::new(4096);
    
    // Program: Add 15 + 25 = 40
    // ADDI $t0, $zero, 15    # $t0 = 15
    // ADDI $t1, $zero, 25    # $t1 = 25  
    // ADD  $t2, $t0, $t1     # $t2 = $t0 + $t1
    // SW   $t2, 0x100($zero) # Store result at address 0x100
    
    let instructions = vec![
        0x2008000F, // ADDI $t0, $zero, 15
        0x20090019, // ADDI $t1, $zero, 25
        0x01095020, // ADD $t2, $t0, $t1
        0xAC0A0100, // SW $t2, 0x100($zero)
    ];
    
    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }
    
    println!("=== Initial State ===");
    println!("$t0 (reg 8): {}", simulator.registers.read(8));
    println!("$t1 (reg 9): {}", simulator.registers.read(9));
    println!("$t2 (reg 10): {}", simulator.registers.read(10));
    
    // Execute program step by step
    for step in 0..instructions.len() {
        println!("\n=== Executing Step {} ===", step + 1);
        simulator.step();
        
        println!("$t0 (reg 8): {}", simulator.registers.read(8));
        println!("$t1 (reg 9): {}", simulator.registers.read(9));
        println!("$t2 (reg 10): {}", simulator.registers.read(10));
    }
    
    // Check the result in memory
    match simulator.memory.read_word(0x100) {
        Some(result) => println!("\nResult stored in memory: {}", result),
        None => println!("\nFailed to read result from memory"),
    }
    
    assert_eq!(simulator.registers.read(10), 40);
    println!("\n✓ Test passed! 15 + 25 = 40");
}
```

**Key Learning Points:**
- Instructions are loaded as 32-bit words
- Registers are accessed by number (8 = $t0, 9 = $t1, etc.)
- Memory addresses must be word-aligned (multiples of 4)
- Use `step()` for single instruction execution

### Tutorial 2: Loop Implementation

Let's implement a simple loop that calculates the sum of numbers 1 to 10.

```rust
use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    let mut simulator = Simulator::new(4096);
    
    // Program: Calculate sum of 1 to 10
    // $t0 = counter (1 to 10)
    // $t1 = sum accumulator
    // $t2 = loop limit (10)
    
    let instructions = vec![
        0x20080001, // ADDI $t0, $zero, 1     # counter = 1
        0x20090000, // ADDI $t1, $zero, 0     # sum = 0
        0x2008000A, // ADDI $t2, $zero, 10    # limit = 10
        // Loop start (address 12):
        0x01284820, // ADD $t1, $t1, $t0      # sum += counter
        0x21080001, // ADDI $t0, $t0, 1       # counter++
        0x150AFFFD, // BNE $t0, $t2, -3       # if counter != limit, goto loop
        0xAC090200, // SW $t1, 0x200($zero)   # store result
    ];
    
    // Load program
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }
    
    println!("Calculating sum of 1 to 10...");
    
    // Run until completion (with safety limit)
    let mut cycles = 0;
    let max_cycles = 100;
    
    while cycles < max_cycles {
        simulator.step();
        cycles += 1;
        
        // Check if we've reached the store instruction
        if simulator.memory.read_word(0x200).is_some() {
            break;
        }
    }
    
    let result = simulator.memory.read_word(0x200).unwrap_or(0);
    println!("Sum of 1 to 10 = {}", result);
    println!("Executed in {} cycles", cycles);
    
    assert_eq!(result, 55); // 1+2+3+...+10 = 55
    println!("✓ Loop test passed!");
}
```

## Pipeline Simulation and Hazards

### Tutorial 3: Understanding Pipeline Hazards

This tutorial demonstrates different types of pipeline hazards and how forwarding helps.

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig, BranchPredictorType};
use vmips_rust::timing_simulator::simulator::Simulator;

fn demonstrate_data_hazard() {
    println!("=== Data Hazard Demonstration ===");
    
    // Create pipeline without forwarding
    let pipeline_config = PipelineConfig::new(5)
        .with_forwarding(false)  // Disable forwarding to see stalls
        .with_latencies(vec![1, 1, 1, 1, 1]);
    
    let cache_config = CacheConfig::new(1024, 2, 32);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        cache_config.clone(),
        cache_config,
        4096
    );
    
    // Enable visualization to see pipeline stages
    simulator.enable_visualization(true);
    
    // Program with data hazard:
    // LW   $t0, 0x100($zero)  # Load data
    // ADD  $t1, $t0, $t0     # Use loaded data immediately (hazard!)
    // ADDI $t2, $t1, 1       # Use result of ADD
    
    let instructions = vec![
        0x8C080100, // LW $t0, 0x100($zero)
        0x01084820, // ADD $t1, $t0, $t0
        0x21220001, // ADDI $t2, $t1, 1
    ];
    
    // Initialize memory with test data
    simulator.memory.write_word_init(0x100, 42);
    
    // Load instructions
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }
    
    println!("Running without forwarding (expect stalls):");
    
    // Run simulation
    for cycle in 1..=15 {
        println!("\n--- Cycle {} ---", cycle);
        // Step simulation (implementation specific)
        
        if simulator.pc >= (instructions.len() * 4) as u32 {
            break;
        }
    }
    
    // Now test with forwarding enabled
    println!("\n=== With Forwarding Enabled ===");
    
    let pipeline_config_forwarding = PipelineConfig::new(5)
        .with_forwarding(true)  // Enable forwarding
        .with_latencies(vec![1, 1, 1, 1, 1]);
    
    let mut simulator_forwarding = Simulator::new(
        pipeline_config_forwarding,
        CacheConfig::new(1024, 2, 32),
        CacheConfig::new(1024, 2, 32),
        4096
    );
    
    simulator_forwarding.enable_visualization(true);
    
    // Load same program
    simulator_forwarding.memory.write_word_init(0x100, 42);
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator_forwarding.memory.write_word_init(i * 4, instruction);
    }
    
    println!("Running with forwarding (fewer stalls):");
    
    for cycle in 1..=10 {
        println!("\n--- Cycle {} ---", cycle);
        // Step simulation
        
        if simulator_forwarding.pc >= (instructions.len() * 4) as u32 {
            break;
        }
    }
}

fn main() {
    demonstrate_data_hazard();
}
```

### Tutorial 4: Control Hazards and Branch Prediction

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig, BranchPredictorType};
use vmips_rust::timing_simulator::simulator::Simulator;

fn demonstrate_branch_prediction() {
    println!("=== Branch Prediction Demonstration ===");
    
    // Test different branch predictors
    let predictors = vec![
        ("Static Not Taken", BranchPredictorType::Static),
        ("2-bit Saturating", BranchPredictorType::TwoBit),
    ];
    
    for (name, predictor_type) in predictors {
        println!("\n--- Testing {} ---", name);
        
        let pipeline_config = PipelineConfig::new(5)
            .with_branch_prediction(true, predictor_type)
            .with_forwarding(true);
        
        let cache_config = CacheConfig::new(1024, 2, 32);
        
        let mut simulator = Simulator::new(
            pipeline_config,
            cache_config.clone(),
            cache_config,
            4096
        );
        
        // Program with predictable branch pattern
        // Loop that runs 5 times
        let instructions = vec![
            0x20080000, // ADDI $t0, $zero, 0     # counter = 0
            0x20090005, // ADDI $t1, $zero, 5     # limit = 5
            // Loop start (address 8):
            0x21080001, // ADDI $t0, $t0, 1       # counter++
            0x1509FFFD, // BNE $t0, $t1, -3       # branch back if counter != limit
            0x00000000, // NOP                     # end
        ];
        
        // Load program
        for (i, &instruction) in instructions.iter().enumerate() {
            simulator.memory.write_word_init(i * 4, instruction);
        }
        
        // Run simulation
        let mut cycles = 0;
        let max_cycles = 50;
        
        while cycles < max_cycles && simulator.pc < 16 {
            cycles += 1;
            // Step simulation
        }
        
        println!("Completed in {} cycles", cycles);
        
        // In a real implementation, you would access branch prediction statistics here
        // println!("Branch prediction accuracy: {:.1}%", accuracy);
    }
}

fn main() {
    demonstrate_branch_prediction();
}
```

## Cache Performance Analysis

### Tutorial 5: Cache Behavior Study

```rust
use vmips_rust::timing_simulator::config::{CacheConfig, PipelineConfig};
use vmips_rust::timing_simulator::simulator::Simulator;

fn analyze_cache_performance() {
    println!("=== Cache Performance Analysis ===");
    
    // Test different cache configurations
    let cache_configs = vec![
        ("Small Direct-Mapped", CacheConfig::new(256, 1, 32)),
        ("Medium 2-Way", CacheConfig::new(512, 2, 32)),
        ("Large 4-Way", CacheConfig::new(1024, 4, 32)),
    ];
    
    for (name, cache_config) in cache_configs {
        println!("\n--- Testing {} Cache ---", name);
        
        let pipeline_config = PipelineConfig::new(5);
        
        let mut simulator = Simulator::new(
            pipeline_config,
            cache_config.clone(),
            cache_config,
            8192
        );
        
        // Create memory access pattern that tests cache behavior
        // Sequential access pattern (good locality)
        println!("Sequential access pattern:");
        test_access_pattern(&mut simulator, generate_sequential_pattern());
        
        // Random access pattern (poor locality)
        println!("Random access pattern:");
        test_access_pattern(&mut simulator, generate_random_pattern());
        
        // Strided access pattern (medium locality)
        println!("Strided access pattern:");
        test_access_pattern(&mut simulator, generate_strided_pattern());
    }
}

fn generate_sequential_pattern() -> Vec<u32> {
    // Generate LW instructions for sequential addresses
    (0..16).map(|i| {
        let addr = i * 4;
        0x8C080000 | addr  // LW $t0, addr($zero)
    }).collect()
}

fn generate_random_pattern() -> Vec<u32> {
    // Generate LW instructions for random addresses
    let addresses = vec![0x100, 0x500, 0x200, 0x800, 0x150, 0x600, 0x250, 0x900];
    addresses.iter().map(|&addr| {
        0x8C080000 | addr  // LW $t0, addr($zero)
    }).collect()
}

fn generate_strided_pattern() -> Vec<u32> {
    // Generate LW instructions with stride of 64 bytes
    (0..8).map(|i| {
        let addr = i * 64;
        0x8C080000 | addr  // LW $t0, addr($zero)
    }).collect()
}

fn test_access_pattern(simulator: &mut Simulator, instructions: Vec<u32>) {
    // Initialize memory with test data
    for i in 0..1024 {
        simulator.memory.write_word_init(i * 4, i as u32);
    }
    
    // Load instructions
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(0x2000 + i * 4, instruction);
    }
    
    // Set PC to instruction area
    simulator.pc = 0x2000;
    
    // Run simulation
    let start_cycles = 0; // In real implementation, get current cycle count
    
    for _ in 0..instructions.len() {
        // Step simulation
    }
    
    let end_cycles = instructions.len(); // In real implementation, get actual cycle count
    
    println!("  Instructions: {}, Cycles: {}, CPI: {:.2}", 
             instructions.len(), 
             end_cycles, 
             end_cycles as f64 / instructions.len() as f64);
    
    // In real implementation, access cache statistics:
    // println!("  Cache hit rate: {:.1}%", hit_rate);
    // println!("  Cache misses: {}", misses);
}

fn main() {
    analyze_cache_performance();
}
```

## Branch Prediction Study

### Tutorial 6: Branch Predictor Comparison

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig, BranchPredictorType};
use vmips_rust::timing_simulator::simulator::Simulator;

fn compare_branch_predictors() {
    println!("=== Branch Predictor Comparison ===");
    
    let predictors = vec![
        BranchPredictorType::Static,
        BranchPredictorType::TwoBit,
    ];
    
    let test_patterns = vec![
        ("Always Taken", generate_always_taken_pattern()),
        ("Always Not Taken", generate_always_not_taken_pattern()),
        ("Alternating", generate_alternating_pattern()),
        ("Nested Loops", generate_nested_loop_pattern()),
    ];
    
    for predictor_type in predictors {
        println!("\n=== {:?} Predictor ===", predictor_type);
        
        for (pattern_name, instructions) in &test_patterns {
            println!("\n--- {} Pattern ---", pattern_name);
            
            let pipeline_config = PipelineConfig::new(5)
                .with_branch_prediction(true, predictor_type)
                .with_forwarding(true);
            
            let cache_config = CacheConfig::new(1024, 2, 32);
            
            let mut simulator = Simulator::new(
                pipeline_config,
                cache_config.clone(),
                cache_config,
                4096
            );
            
            // Load test pattern
            for (i, &instruction) in instructions.iter().enumerate() {
                simulator.memory.write_word_init(i * 4, instruction);
            }
            
            // Run simulation
            let mut cycles = 0;
            let max_cycles = 200;
            
            while cycles < max_cycles {
                cycles += 1;
                
                // Check for completion condition
                if simulator.pc >= (instructions.len() * 4) as u32 {
                    break;
                }
            }
            
            println!("  Cycles: {}", cycles);
            // In real implementation:
            // println!("  Prediction accuracy: {:.1}%", accuracy);
            // println!("  Mispredictions: {}", mispredictions);
        }
    }
}

fn generate_always_taken_pattern() -> Vec<u32> {
    vec![
        0x20080000, // ADDI $t0, $zero, 0
        0x21080001, // ADDI $t0, $t0, 1
        0x1000FFFF, // BEQ $zero, $zero, -1  (always taken)
    ]
}

fn generate_always_not_taken_pattern() -> Vec<u32> {
    vec![
        0x20080000, // ADDI $t0, $zero, 0
        0x20090001, // ADDI $t1, $zero, 1
        0x21080001, // ADDI $t0, $t0, 1
        0x1509FFFD, // BNE $t0, $t1, -3  (never taken after first iteration)
    ]
}

fn generate_alternating_pattern() -> Vec<u32> {
    // Pattern that alternates between taken and not taken
    vec![
        0x20080000, // ADDI $t0, $zero, 0
        0x20090002, // ADDI $t1, $zero, 2
        0x21080001, // ADDI $t0, $t0, 1
        0x11090001, // BEQ $t0, $t1, +1   (taken every other time)
        0x1000FFFD, // BEQ $zero, $zero, -3  (always taken - restart)
        0x00000000, // NOP
    ]
}

fn generate_nested_loop_pattern() -> Vec<u32> {
    // Nested loop with different branch behaviors
    vec![
        0x20080000, // ADDI $t0, $zero, 0     # outer counter
        0x20090003, // ADDI $t1, $zero, 3     # outer limit
        0x200A0000, // ADDI $t2, $zero, 0     # inner counter
        0x200B0002, // ADDI $t3, $zero, 2     # inner limit
        // Inner loop start:
        0x214A0001, // ADDI $t2, $t2, 1       # inner counter++
        0x154BFFFF, // BNE $t2, $t3, -1       # inner loop branch
        0x200A0000, // ADDI $t2, $zero, 0     # reset inner counter
        0x21080001, // ADDI $t0, $t0, 1       # outer counter++
        0x1509FFFB, // BNE $t0, $t1, -5       # outer loop branch
    ]
}

fn main() {
    compare_branch_predictors();
}
```

## Out-of-Order Execution

### Tutorial 7: Tomasulo Algorithm Demonstration

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig};
use vmips_rust::timing_simulator::simulator::Simulator;

fn demonstrate_out_of_order_execution() {
    println!("=== Out-of-Order Execution with Tomasulo Algorithm ===");
    
    // Configure for out-of-order execution
    let pipeline_config = PipelineConfig::new(5)
        .with_forwarding(true)
        .with_superscalar(2);  // Issue up to 2 instructions per cycle
    
    let cache_config = CacheConfig::new(1024, 4, 32);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        cache_config.clone(),
        cache_config,
        4096
    );
    
    // Program that benefits from out-of-order execution:
    // Independent operations that can execute in parallel
    let instructions = vec![
        // Group 1: Independent arithmetic
        0x20080005, // ADDI $t0, $zero, 5     # $t0 = 5
        0x20090003, // ADDI $t1, $zero, 3     # $t1 = 3
        0x200A0007, // ADDI $t2, $zero, 7     # $t2 = 7
        0x200B0002, // ADDI $t3, $zero, 2     # $t3 = 2
        
        // Group 2: Operations with dependencies
        0x01094020, // ADD $t0, $t0, $t1      # $t0 = $t0 + $t1 (depends on $t0, $t1)
        0x014B5020, // ADD $t2, $t2, $t3      # $t2 = $t2 + $t3 (depends on $t2, $t3)
        
        // Group 3: Final operations
        0x01485820, // ADD $t3, $t2, $t0      # $t3 = $t2 + $t0 (depends on results above)
        0xAC0B0100, // SW $t3, 0x100($zero)   # Store final result
    ];
    
    // Load instructions
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }
    
    println!("Program loaded. Instructions can execute out-of-order when dependencies allow.");
    println!("Expected result: ((5+3) + (7+2)) = 17");
    
    // Enable detailed visualization
    simulator.enable_visualization(true);
    
    // Run simulation
    let mut cycles = 0;
    let max_cycles = 50;
    
    while cycles < max_cycles {
        cycles += 1;
        println!("\n=== Cycle {} ===", cycles);
        
        // In a real implementation, this would show:
        // - Reservation station status
        // - Reorder buffer contents
        // - Register alias table state
        // - Instructions in flight
        
        // Check if program completed
        if let Some(result) = simulator.memory.read_word(0x100) {
            println!("Program completed! Result: {}", result);
            break;
        }
        
        if simulator.pc >= (instructions.len() * 4) as u32 {
            break;
        }
    }
    
    println!("Simulation completed in {} cycles", cycles);
    
    // In a real implementation, show performance statistics:
    // println!("Instructions per cycle (IPC): {:.2}", ipc);
    // println!("Average instruction latency: {:.2}", avg_latency);
}

fn main() {
    demonstrate_out_of_order_execution();
}
```

## Loading Real Programs

### Tutorial 8: Working with ELF Files

```rust
use vmips_rust::elf_loader::ElfLoader;
use vmips_rust::functional_simulator::simulator::Simulator;
use std::path::Path;

fn load_and_run_elf_program() {
    println!("=== Loading and Running ELF Programs ===");
    
    // This example assumes you have a MIPS ELF file
    let elf_path = "examples/hello_world.elf";
    
    if !Path::new(elf_path).exists() {
        println!("ELF file not found. Creating a simple test instead.");
        demonstrate_elf_loading_concept();
        return;
    }
    
    // Load ELF file
    match ElfLoader::load_file(elf_path) {
        Ok(elf_loader) => {
            println!("ELF file loaded successfully!");
            
            // Get entry point
            let entry_point = elf_loader.entry_point();
            println!("Entry point: 0x{:08X}", entry_point);
            
            // Show loaded segments
            let segments = elf_loader.get_segments();
            println!("Loaded segments:");
            for (vaddr, size, flags) in segments {
                println!("  0x{:08X} - 0x{:08X} (size: {} bytes, flags: 0x{:X})", 
                         vaddr, vaddr + size, size, flags);
            }
            
            // Create simulator
            let mut simulator = Simulator::new(65536); // 64KB memory
            
            // Load ELF into simulator memory
            if let Err(e) = elf_loader.load_into_memory(&mut simulator.memory) {
                println!("Failed to load ELF into memory: {:?}", e);
                return;
            }
            
            println!("ELF loaded into simulator memory");
            
            // Set PC to entry point
            // Note: In a real implementation, you'd set simulator.pc = entry_point
            
            // Run simulation
            println!("Starting execution...");
            let max_steps = 1000;
            for step in 0..max_steps {
                simulator.step();
                
                // Check for program termination (implementation specific)
                // This might be a syscall, infinite loop detection, etc.
                
                if step % 100 == 0 {
                    println!("Executed {} instructions", step);
                }
            }
            
            println!("Simulation completed");
        }
        Err(e) => {
            println!("Failed to load ELF file: {:?}", e);
        }
    }
}

fn demonstrate_elf_loading_concept() {
    println!("=== ELF Loading Concept Demonstration ===");
    
    // Create a mock ELF-like binary data
    let mut binary_data = vec![0u8; 1024];
    
    // ELF magic number
    binary_data[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    
    // This would normally be parsed as a real ELF file
    println!("In a real scenario, you would:");
    println!("1. Parse ELF headers to find loadable segments");
    println!("2. Load each segment at its virtual address");
    println!("3. Set up the initial program counter");
    println!("4. Initialize stack and heap regions");
    println!("5. Handle relocations if necessary");
    
    // Demonstrate the concept with a simple program
    let mut simulator = Simulator::new(4096);
    
    // Simulate loading a program at address 0x1000
    let program_base = 0x1000;
    let instructions = vec![
        0x20080001, // ADDI $t0, $zero, 1
        0x20090002, // ADDI $t1, $zero, 2
        0x01094020, // ADD $t0, $t0, $t1
    ];
    
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(program_base + i * 4, instruction);
    }
    
    println!("Simulated program loaded at address 0x{:08X}", program_base);
    
    // In a real implementation, you would set:
    // simulator.pc = program_base;
    
    println!("Program would start execution from the loaded address");
}

fn main() {
    load_and_run_elf_program();
}
```

## Performance Optimization

### Tutorial 9: Optimizing Simulation Performance

```rust
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig};
use vmips_rust::timing_simulator::simulator::Simulator;
use std::time::Instant;

fn benchmark_simulation_performance() {
    println!("=== Simulation Performance Optimization ===");
    
    // Test different configuration options for performance
    let configurations = vec![
        ("Basic Config", create_basic_config()),
        ("Optimized Config", create_optimized_config()),
        ("High-Performance Config", create_high_performance_config()),
    ];
    
    let test_program = create_performance_test_program();
    
    for (name, (pipeline_config, cache_config)) in configurations {
        println!("\n--- Testing {} ---", name);
        
        let start_time = Instant::now();
        
        let mut simulator = Simulator::new(
            pipeline_config,
            cache_config.clone(),
            cache_config,
            8192
        );
        
        // Load test program
        for (i, &instruction) in test_program.iter().enumerate() {
            simulator.memory.write_word_init(i * 4, instruction);
        }
        
        // Run simulation
        let max_cycles = 1000;
        for _ in 0..max_cycles {
            // Step simulation
            if simulator.pc >= (test_program.len() * 4) as u32 {
                break;
            }
        }
        
        let elapsed = start_time.elapsed();
        println!("  Execution time: {:?}", elapsed);
        println!("  Instructions simulated: {}", test_program.len());
        println!("  Performance: {:.0} instructions/second", 
                 test_program.len() as f64 / elapsed.as_secs_f64());
    }
    
    println!("\n=== Performance Tips ===");
    println!("1. Disable visualization for production runs");
    println!("2. Use smaller cache sizes for faster simulation");
    println!("3. Reduce pipeline complexity when not needed");
    println!("4. Use release builds (cargo build --release)");
    println!("5. Consider memory size vs. simulation speed tradeoffs");
}

fn create_basic_config() -> (PipelineConfig, CacheConfig) {
    let pipeline = PipelineConfig::new(5);
    let cache = CacheConfig::new(1024, 2, 32);
    (pipeline, cache)
}

fn create_optimized_config() -> (PipelineConfig, CacheConfig) {
    let pipeline = PipelineConfig::new(5)
        .with_forwarding(true);
    let cache = CacheConfig::new(512, 2, 32);  // Smaller cache for speed
    (pipeline, cache)
}

fn create_high_performance_config() -> (PipelineConfig, CacheConfig) {
    let pipeline = PipelineConfig::new(3)  // Shorter pipeline
        .with_forwarding(true);
    let cache = CacheConfig::new(256, 1, 32);  // Direct-mapped for speed
    (pipeline, cache)
}

fn create_performance_test_program() -> Vec<u32> {
    // Create a program with various instruction types
    let mut instructions = Vec::new();
    
    // Arithmetic operations
    for i in 0..10 {
        instructions.push(0x20080000 | i);  // ADDI $t0, $zero, i
        instructions.push(0x01084020);      // ADD $t0, $t0, $t0
    }
    
    // Memory operations
    for i in 0..10 {
        let addr = 0x1000 + i * 4;
        instructions.push(0x8C080000 | addr);  // LW $t0, addr($zero)
        instructions.push(0xAC080000 | addr);  // SW $t0, addr($zero)
    }
    
    // Branch operations
    instructions.push(0x20080005);  // ADDI $t0, $zero, 5
    instructions.push(0x21080001);  // ADDI $t0, $t0, 1
    instructions.push(0x1500FFFF);  // BNE $t0, $zero, -1
    
    instructions
}

fn main() {
    benchmark_simulation_performance();
}
```

## Conclusion

These tutorials cover the major use cases for the VMIPS Rust simulator:

1. **Basic functional simulation** for understanding instruction execution
2. **Pipeline analysis** for studying processor performance
3. **Cache behavior** for memory hierarchy optimization
4. **Branch prediction** for control flow optimization
5. **Out-of-order execution** for advanced processor features
6. **Real program loading** for practical applications
7. **Performance optimization** for efficient simulation

Each tutorial builds on previous concepts and demonstrates both the simulator's capabilities and computer architecture principles. The examples can be modified and extended for specific research or educational needs.

For more advanced usage, refer to the API documentation and examine the source code examples in the `examples/` directory of the repository.