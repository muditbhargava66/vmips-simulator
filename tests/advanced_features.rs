// tests/advanced_features.rs
// Advanced tests for MIPS simulator features

use vmips_rust::timing_simulator::config::{CacheConfig, PipelineConfig, BranchPredictorType};
use vmips_rust::timing_simulator::simulator::Simulator;

/// Helper function to create a simple simulator
#[allow(dead_code)]
fn create_simple_simulator() -> Simulator {
    // Create a basic pipeline configuration
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 1, 1, 1, 1])
        .with_forwarding(true)
        .with_branch_prediction(true, BranchPredictorType::TwoBit);
    
    // Use standard cache configurations
    let instr_cache_config = CacheConfig::new(4096, 2, 64);
    let data_cache_config = CacheConfig::new(4096, 4, 64);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        32768, // 32KB
    );
    
    // Disable visualization for tests
    simulator.visualization = None;
    
    simulator
}

/// Test multi-cycle execution
#[test]
fn test_multi_cycle_execution() {
    // Create a pipeline with variable cycle latencies
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 2, 3, 2, 1]) // Different latencies for each stage
        .with_forwarding(true)
        .with_branch_prediction(false, BranchPredictorType::TwoBit);
    
    let instr_cache_config = CacheConfig::new(4096, 2, 64);
    let data_cache_config = CacheConfig::new(4096, 4, 64);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        32768,
    );
    
    // Disable visualization
    simulator.visualization = None;
    
    // Set memory values for test
    simulator.memory.write_word_init(0x100, 5);
    simulator.memory.write_word_init(0x104, 10);
    
    // Program with sequential operations
    let program = vec![
        0x8C020100u32, // lw $2, 0x0100($0)   - Load 5 from 0x100
        0x8C030104u32, // lw $3, 0x0104($0)   - Load 10 from 0x104
        0x00431020u32, // add $2, $2, $3      - Add $2 + $3 = 5 + 10 = 15
        0x00000000u32, // nop - end program
    ];
    
    // Load program
    let program_base = 0x1000;
    for (i, &instr) in program.iter().enumerate() {
        let addr = program_base + i * 4;
        simulator.memory.write_word_init(addr, instr);
    }
    
    // Set PC to start
    simulator.pc = program_base as u32;
    
    // Set reasonable step limit
    simulator.set_max_steps(50);
    
    // Run simulation
    simulator.run();
    
    // Verify results
    assert_eq!(simulator.registers.read(2), 15, "Register $2 should contain 15");
    assert_eq!(simulator.registers.read(3), 10, "Register $3 should contain 10");
}

/// Test cache miss handling
#[test]
fn test_cache_miss_handling() {
    // Create pipeline config
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 1, 1, 1, 1])
        .with_forwarding(true);
    
    // Small cache with small block size will have more misses
    let instr_cache_config = CacheConfig::new(512, 1, 16);
    let data_cache_config = CacheConfig::new(512, 1, 16);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        32768,
    );
    
    // Disable visualization
    simulator.visualization = None;
    
    // Create memory pattern that will cause cache misses
    // Place values far apart to be in different cache lines
    for i in 0..10 {
        simulator.memory.write_word_init(i * 128, i as u32 * 10);
    }
    
    // Program that accesses memory in a pattern causing cache misses
    let mut program = Vec::new();
    
    // First, load values in sequence (should populate cache)
    for i in 0..5 {
        let reg = 10 + i as u32;
        let addr = i * 128;
        program.push(0x8C000000u32 | (reg << 16) | (addr as u32)); // lw $reg, addr($0)
    }
    
    // Then, access the second batch (should cause evictions)
    for i in 5..10 {
        let reg = 15 + (i-5) as u32;
        let addr = i * 128;
        program.push(0x8C000000u32 | (reg << 16) | (addr as u32)); // lw $reg, addr($0)
    }
    
    // Then, go back to the first batch (should miss again)
    for i in 0..5 {
        let reg = 20 + i as u32;
        let addr = i * 128;
        program.push(0x8C000000u32 | (reg << 16) | (addr as u32)); // lw $reg, addr($0)
    }
    
    // Add NOP to end
    program.push(0x00000000u32);
    
    // Load program
    let program_base = 0x1000;
    for (i, &instr) in program.iter().enumerate() {
        let addr = program_base + i * 4;
        simulator.memory.write_word_init(addr, instr);
    }
    
    // Set PC to start
    simulator.pc = program_base as u32;
    
    // Set longer step limit
    simulator.set_max_steps(200);
    
    // Run simulation
    simulator.run();
    
    // Verify that registers have expected values despite cache misses
    for i in 0..5 {
        // First batch load
        let reg = 10 + i as u32;
        assert_eq!(simulator.registers.read(reg), i as u32 * 10, 
                  "Register ${} should contain {}", reg, i * 10);
        
        // Second load of first batch
        let reg2 = 20 + i as u32;
        assert_eq!(simulator.registers.read(reg2), i as u32 * 10, 
                  "Register ${} should contain {}", reg2, i * 10);
    }
    
    // Check second batch
    for i in 5..10 {
        let reg = 15 + (i-5) as u32;
        assert_eq!(simulator.registers.read(reg), i as u32 * 10, 
                  "Register ${} should contain {}", reg, i * 10);
    }
}

/// Test memory access patterns
#[test]
fn test_memory_access_patterns() {
    // Create pipeline
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 1, 1, 1, 1]);
    
    // Cache configuration
    let instr_cache_config = CacheConfig::new(1024, 2, 32);
    let data_cache_config = CacheConfig::new(1024, 2, 32);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        32768,
    );
    
    // Disable visualization
    simulator.visualization = None;
    
    // Create sequential memory values
    for i in 0..20 {
        simulator.memory.write_word_init(0x1000 + i * 4, i as u32);
    }
    
    // Program that accesses memory sequentially
    let program: Vec<u32> = (0..10).map(|i| {
        let reg = 2 + i;
        let addr = 0x1000 + i * 4;
        0x8C000000u32 | (reg << 16) | (addr as u32)
    }).collect();
    
    // Load program
    let program_base = 0x2000;
    for (i, &instr) in program.iter().enumerate() {
        let addr = program_base + i * 4;
        simulator.memory.write_word_init(addr, instr);
    }
    
    // Set PC to start
    simulator.pc = program_base as u32;
    
    // Run simulation
    simulator.set_max_steps(100);
    simulator.run();
    
    // Verify all values were correctly loaded
    for i in 0..10 {
        let reg = 2 + i as u32;
        assert_eq!(simulator.registers.read(reg), i as u32, 
                  "Register ${} should contain {}", reg, i);
    }
}

/// Test superscalar execution
#[test]
fn test_superscalar_execution() {
    // Create pipeline with superscalar execution (can execute 2 instructions per cycle)
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 1, 1, 1, 1])
        .with_superscalar(2); // Can issue 2 instructions per cycle
    
    let instr_cache_config = CacheConfig::new(4096, 2, 64);
    let data_cache_config = CacheConfig::new(4096, 4, 64);
    
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        32768,
    );
    
    // Disable visualization
    simulator.visualization = None;
    
    // Program with independent instructions that can be executed in parallel
    let program = vec![
        0x20020001u32, // addi $2, $0, 1     - Set $2 = 1
        0x20030002u32, // addi $3, $0, 2     - Set $3 = 2
        0x20040003u32, // addi $4, $0, 3     - Set $4 = 3
        0x20050004u32, // addi $5, $0, 4     - Set $5 = 4
        0x20060005u32, // addi $6, $0, 5     - Set $6 = 5
        0x20070006u32, // addi $7, $0, 6     - Set $7 = 6
        0x20080007u32, // addi $8, $0, 7     - Set $8 = 7
        0x20090008u32, // addi $9, $0, 8     - Set $9 = 8
        0x00000000u32, // nop - end program
    ];
    
    // Load program
    let program_base = 0x1000;
    for (i, &instr) in program.iter().enumerate() {
        let addr = program_base + i * 4;
        simulator.memory.write_word_init(addr, instr);
    }
    
    // Set PC to start
    simulator.pc = program_base as u32;
    
    // Run simulation
    simulator.set_max_steps(50);
    simulator.run();
    
    // Verify all registers have expected values
    for i in 1..9 {
        let reg = 1 + i as u32;
        assert_eq!(simulator.registers.read(reg), i as u32, 
                  "Register ${} should contain {}", reg, i);
    }
}