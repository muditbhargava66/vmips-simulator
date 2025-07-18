// tests/timing_simulator.rs
use vmips_rust::timing_simulator::config::{BranchPredictorType, CacheConfig, PipelineConfig};
use vmips_rust::timing_simulator::simulator::Simulator;

/// Test suite for the timing MIPS simulator

/// Helper function to create a basic pipeline configuration
fn create_test_pipeline_config(
    stages: usize,
    forwarding: bool,
    branch_prediction: bool,
) -> PipelineConfig {
    let latencies = vec![1; stages]; // Single cycle per stage

    PipelineConfig::new(stages)
        .with_latencies(latencies)
        .with_forwarding(forwarding)
        .with_branch_prediction(branch_prediction, BranchPredictorType::TwoBit)
        .with_superscalar(1) // Single-issue pipeline
}

/// Helper function to create a cache configuration
fn create_test_cache_config(
    size_bytes: usize,
    associativity: usize,
    block_size: usize,
) -> CacheConfig {
    CacheConfig::new(size_bytes, associativity, block_size)
}

/// Helper function to set up a simulator with memory values
fn setup_simulator(pipeline_config: PipelineConfig, memory_values: &[(usize, u32)]) -> Simulator {
    // Create standard cache configurations
    let instr_cache_config = create_test_cache_config(4096, 2, 64); // 4KB, 2-way, 64B blocks
    let data_cache_config = create_test_cache_config(4096, 4, 64); // 4KB, 4-way, 64B blocks

    // Create simulator with 32KB memory
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        32768, // 32KB memory
    );

    // Disable visualization for tests
    simulator.visualization = None;

    // Set a reasonable step limit
    simulator.set_max_steps(100);

    // Write initial memory values
    for &(addr, value) in memory_values {
        simulator.memory.write_word_init(addr, value);
        println!("Loaded value {} at memory address 0x{:X}", value, addr);
    }

    simulator
}

/// Helper function to load a program into the simulator
fn load_program(simulator: &mut Simulator, program: &[u32]) {
    // Load program instructions into memory
    for (i, instr) in program.iter().enumerate() {
        let addr = i * 4;
        simulator.memory.write_word_init(addr, *instr);
    }

    // Verify the program was correctly loaded
    println!("Program loaded into memory:");
    for (i, _instr) in program.iter().enumerate() {
        let addr = i * 4;
        let word = simulator.memory.read_word(addr).unwrap_or(0);
        println!("  Memory at 0x{:04X}: 0x{:08X}", addr, word);
    }

    // Set PC to start at the beginning of the program
    simulator.pc = 0;
}

#[test]
fn test_basic_pipeline_execution() {
    // Create a simple 3-stage pipeline without forwarding or branch prediction
    let pipeline_config = create_test_pipeline_config(3, false, false);

    // Set up memory values
    let memory_values = [
        (0x100, 20), // First operand
        (0x104, 22), // Second operand
    ];

    // Create a simple test program
    let program = vec![
        0x8C020100u32, // lw $2, 0x0100($0)  - Load 20 from 0x100
        0x8C030104u32, // lw $3, 0x0104($0)  - Load 22 from 0x104
        0x00431020u32, // add $2, $2, $3     - Add to get 42
        0x00000000u32, // nop - end program
    ];

    let mut simulator = setup_simulator(pipeline_config, &memory_values);
    load_program(&mut simulator, &program);

    // Run the simulation
    simulator.run();

    // Verify final register values
    println!(
        "Final register values: $2={}, $3={}",
        simulator.registers.read(2),
        simulator.registers.read(3)
    );

    assert_eq!(
        simulator.registers.read(2),
        42,
        "Register 2 should contain 42"
    );
    assert_eq!(
        simulator.registers.read(3),
        22,
        "Register 3 should contain 22"
    );

    // Verify PC ended at the right place
    assert!(
        simulator.pc >= 12,
        "PC should have advanced beyond the program"
    );
}

#[test]
fn test_pipeline_with_forwarding() {
    // Create a 5-stage pipeline with forwarding enabled
    let pipeline_config = create_test_pipeline_config(5, true, false);

    // Set up memory values and program to test data forwarding
    let memory_values = [
        (0x200, 5),  // First operand
        (0x204, 10), // Second operand
    ];

    // Program to test forwarding:
    // 1. Load values into registers
    // 2. Perform an add that depends immediately on those loads
    // 3. Perform another operation that depends on the previous add
    let program = vec![
        0x8C020200u32, // lw $2, 0x0200($0)  - Load 5 from 0x200
        0x8C030204u32, // lw $3, 0x0204($0)  - Load 10 from 0x204
        0x00431020u32, // add $2, $2, $3     - Add to get 15 in $2
        0x00421820u32, // add $3, $2, $2     - Add to get 30 in $3
        0x00000000u32, // nop - end program
    ];

    let mut simulator = setup_simulator(pipeline_config, &memory_values);
    load_program(&mut simulator, &program);

    // Run the simulation
    simulator.run();

    // Verify final register values - with forwarding, this should work correctly
    println!(
        "Final register values with forwarding: $2={}, $3={}",
        simulator.registers.read(2),
        simulator.registers.read(3)
    );

    assert_eq!(
        simulator.registers.read(2),
        15,
        "Register 2 should contain 15"
    );
    assert_eq!(
        simulator.registers.read(3),
        30,
        "Register 3 should contain 30"
    );
}

#[test]
fn test_branch_prediction() {
    // Create a 5-stage pipeline with branch prediction
    let pipeline_config = create_test_pipeline_config(5, true, true);

    // Program to test branch prediction:
    // 1. Initialize $2 = 3
    // 2. Branch if $2 == 0 (not taken first times)
    // 3. Decrement $2
    // 4. Jump back to branch check
    // 5. When branch taken, set $3 = 42
    let program = vec![
        0x24020003u32, // addi $2, $0, 3     ; Initialize $2 = 3
        0x10400002u32, // beq $2, $0, 2      ; Branch to instruction 4 (addi $3, $0, 42) if $2 == 0
        0x2442FFFFu32, // addi $2, $2, -1    ; Decrement $2
        0x08000001u32, // j 0x00000004       ; Jump back to instruction 1 (beq)
        0x2403002Au32, // addi $3, $0, 42    ; This runs when $2 == 0
        0x00000000u32, // nop - end program
    ];

    let mut simulator = setup_simulator(pipeline_config, &[]);
    load_program(&mut simulator, &program);

    // Set a larger step limit for the loop
    simulator.set_max_steps(200);

    // Run the simulation
    simulator.run();

    // Verify results - the loop should execute 3 times and then set $3 to 42
    assert_eq!(
        simulator.registers.read(2),
        0,
        "Register $2 should be decremented to 0"
    );
    assert_eq!(
        simulator.registers.read(3),
        42,
        "Register $3 should be set to 42 after branching"
    );
}

#[test]
fn test_memory_access_patterns() {
    // Create a pipeline with cache-friendly configuration
    let pipeline_config = create_test_pipeline_config(5, true, true);

    // Create memory values - spread across cache lines to test cache behavior
    let mut memory_values = Vec::new();
    for i in 0..8 {
        memory_values.push((0x1000 + i * 64, i as u32 * 10)); // Values at different cache lines
    }

    // Create a program that accesses the memory locations sequentially
    let mut program = Vec::new();
    for i in 0..8 {
        // lw $reg, addr($0)
        let reg = 2 + i as u32;
        let addr = 0x1000 + i * 64;
        let load_instr = 0x8C000000u32 | (reg << 16) | (addr as u32);
        program.push(load_instr);
    }

    // Add an instruction to sum the first two values
    program.push(0x00431020u32); // add $2, $2, $3

    // Add NOP to end the program
    program.push(0x00000000u32);

    let mut simulator = setup_simulator(pipeline_config, &memory_values);
    load_program(&mut simulator, &program);

    // Run the simulation
    simulator.run();

    // Verify the correct values were loaded
    for i in 0..8 {
        let reg = 2 + i as u32;
        let expected_value = if i == 0 { 10 } else { i as u32 * 10 }; // $2 was modified by add
        assert_eq!(
            simulator.registers.read(reg),
            expected_value,
            "Register ${} should contain {}",
            reg,
            expected_value
        );
    }
}

#[test]
fn test_timing_simulator() {
    // Create a simple pipeline configuration for this test
    let pipeline_config = PipelineConfig::new(3) // Use fewer stages for simpler testing
        .with_latencies(vec![1, 1, 1]) // Single cycle per stage
        .with_forwarding(false) // Disable forwarding for simpler testing
        .with_branch_prediction(false, BranchPredictorType::TwoBit) // Disable branch prediction
        .with_superscalar(1); // Single-issue pipeline

    // Create cache configurations with larger sizes
    let instr_cache_config = CacheConfig::new(4096, 2, 64);
    let data_cache_config = CacheConfig::new(4096, 4, 64);

    // Create a larger memory
    let memory_size = 8192; // 8KB memory

    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        memory_size,
    );

    // Critical: Add test data to memory BEFORE creating and loading the program
    simulator.memory.write_word_init(0x100, 20); // First value
    simulator.memory.write_word_init(0x104, 22); // Second value

    println!("Test values loaded into memory at 0x100 and 0x104");

    // Create a simple test program:
    // 1. Load 20 into register 2
    // 2. Load 22 into register 3
    // 3. Add them to get 42 in register 2
    let program = vec![
        0x8C020100u32, // lw $2, 0x0100($0)  - Load 20 from 0x100
        0x8C030104u32, // lw $3, 0x0104($0)  - Load 22 from 0x104
        0x00431020u32, // add $2, $2, $3     - Add to get 42
        0x00000000u32, // nop - end program
    ];

    // Load program into memory using word-by-word writing for clarity
    simulator.memory.write_word_init(0, program[0]); // lw $2, 0x0100($0)
    simulator.memory.write_word_init(4, program[1]); // lw $3, 0x0104($0)
    simulator.memory.write_word_init(8, program[2]); // add $2, $2, $3
    simulator.memory.write_word_init(12, program[3]); // nop

    println!("Program loaded into memory");

    // Verify the program was correctly loaded
    for i in 0..4 {
        let addr = i * 4;
        let word = simulator.memory.read_word(addr).unwrap_or(0);
        println!("Memory at 0x{:04X}: 0x{:08X}", addr, word);
    }

    // Disable visualization for test - important for cleaner output
    simulator.visualization = None;

    // Set PC to start at 0
    simulator.pc = 0;

    // Set a smaller max_steps for test
    simulator.set_max_steps(50);

    // Run the simulation
    simulator.run();

    // Verify final register values match expected results
    println!(
        "Final register values: $2={}, $3={}",
        simulator.registers.read(2),
        simulator.registers.read(3)
    );

    assert_eq!(
        simulator.registers.read(2),
        42,
        "Register 2 should contain 42"
    );
    assert_eq!(
        simulator.registers.read(3),
        22,
        "Register 3 should contain 22"
    );

    // Verify PC ended at the right place (after executing program)
    assert!(
        simulator.pc >= 12,
        "PC should have advanced beyond the program"
    );
}
