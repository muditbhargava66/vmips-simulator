// tests/functional_simulator.rs
use vmips_rust::functional_simulator::simulator::Simulator;

/// Test suite for the functional MIPS simulator

/// Helper function to create a simple test program
fn create_test_program(memory_values: &[(usize, u32)]) -> Vec<u32> {
    // Create a program that loads values, adds them, and stores the result
    let mut program = Vec::new();
    
    // Load each value into consecutive registers starting from $2
    for (i, (addr, _)) in memory_values.iter().enumerate() {
        // lw $reg, addr($0) - Load from specified address
        let reg = 2 + i as u32;
        let load_instr = 0x8C000000u32 | (reg << 16) | (*addr as u32);
        program.push(load_instr);
    }
    
    // If we have values to add
    if memory_values.len() >= 2 {
        // add $2, $2, $3 - Add first two registers
        program.push(0x00431020u32);
        
        // Store the result at the result address (if provided)
        if memory_values.len() > 2 {
            let result_addr = memory_values[2].0;
            // sw $2, result_addr($0) - Store at result address
            let store_instr = 0xAC000000u32 | (2 << 16) | (result_addr as u32);
            program.push(store_instr);
        }
    }
    
    // Add NOP to end the program
    program.push(0x00000000u32);
    
    program
}

/// Helper function to load a program and memory values
fn setup_simulator(memory_values: &[(usize, u32)], program: &[u32]) -> Simulator {
    let memory_size = 32768; // 32KB memory
    let mut simulator = Simulator::new(memory_size);
    
    // Write initial memory values
    for &(addr, value) in memory_values {
        simulator.memory.write_word(addr, value);
        println!("Loaded value {} at memory address 0x{:X}", value, addr);
    }
    
    // Convert program to bytes
    let mut program_bytes = Vec::with_capacity(program.len() * 4);
    for &word in program {
        program_bytes.extend_from_slice(&word.to_le_bytes());
    }
    
    // Load program
    simulator.load_program(&program_bytes);
    println!("Loaded program with {} instructions", program.len());
    
    // Ensure memory values are preserved by rewriting them
    for &(addr, value) in memory_values {
        simulator.memory.write_word(addr, value);
    }
    
    simulator
}

#[test]
fn test_basic_arithmetic() {
    // Test adding two numbers
    let memory_values = [
        (0x1000, 21),   // First operand
        (0x1004, 21),   // Second operand
        (0x1008, 0),    // Result location
    ];
    
    let program = create_test_program(&memory_values);
    let mut simulator = setup_simulator(&memory_values, &program);
    
    // Print initial state
    println!("Memory before execution:");
    for &(addr, _) in &memory_values {
        println!("  0x{:X}: {:?}", addr, simulator.memory.read_word(addr));
    }
    
    // Run simulation
    simulator.run();
    
    // Print final state
    println!("Registers after execution:");
    for i in 1..4 {
        println!("  ${} = {}", i, simulator.registers.read(i));
    }
    
    println!("Memory after execution:");
    for &(addr, _) in &memory_values {
        println!("  0x{:X}: {:?}", addr, simulator.memory.read_word(addr));
    }
    
    // Verify results
    assert_eq!(simulator.registers.read(2), 42, "Register $2 should be 21 + 21 = 42");
    assert_eq!(simulator.memory.read_word(0x1008), Some(42), "Memory at 0x1008 should contain 42");
}

#[test]
fn test_load_store_operations() {
    // Set up memory values
    let memory_values = [
        (0x2000, 0x12345678),  // Value to load
        (0x2004, 0),           // Location to store
    ];
    
    // Create a simple load/store program
    let program = vec![
        0x8C022000u32,  // lw $2, 0x2000($0) - Load from address 0x2000
        0xAC022004u32,  // sw $2, 0x2004($0) - Store to address 0x2004
        0x00000000u32,  // nop - end program
    ];
    
    let mut simulator = setup_simulator(&memory_values, &program);
    
    // Run simulation
    simulator.run();
    
    // Verify results
    assert_eq!(simulator.registers.read(2), 0x12345678, "Register $2 should contain 0x12345678");
    assert_eq!(simulator.memory.read_word(0x2004), Some(0x12345678), "Memory at 0x2004 should contain 0x12345678");
}

#[test]
fn test_branching() {
    let memory_size = 32768;
    let mut simulator = Simulator::new(memory_size);
    
    // Program to test branching
    // 1. Initialize $2 = 10
    // 2. Loop:
    // 3. Decrement $2
    // 4. Branch if $2 != 0 (should take branch 10 times)
    // 5. If branch not taken, set $3 = 42 (this happens when $2 becomes 0)
    let program = vec![
        0x2402000Au32,  // addi $2, $0, 10    ; Initialize $2 = 10
        0x2442FFFFu32,  // addi $2, $2, -1    ; Decrement $2
        0x1440FFFEu32,  // bne $2, $0, -2     ; Branch to instruction 2 if $2 != 0
        0x2403002Au32,  // addi $3, $0, 42    ; This runs when $2 == 0
        0x00000000u32,  // nop - end program
    ];
    
    // Convert program to bytes
    let mut program_bytes = Vec::with_capacity(program.len() * 4);
    for word in program {
        program_bytes.extend_from_slice(&word.to_le_bytes());
    }
    
    // Load program
    simulator.load_program(&program_bytes);
    
    // Run simulation
    simulator.run();
    
    // Verify results - the loop should execute 10 times and then set $3 to 42
    assert_eq!(simulator.registers.read(2), 0, "Register $2 should be decremented to 0");
    assert_eq!(simulator.registers.read(3), 42, "Register $3 should be set to 42 after branching");
}

#[test]
fn test_functional_simulator() {
    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Load the expected values into memory
    simulator.memory.write_word(0x1000, 21);
    simulator.memory.write_word(0x1004, 21);

    println!("Test values loaded into memory at 0x1000 and 0x1004");

    // Create a test program that:
    // 1. Loads values from memory
    // 2. Adds them together
    // 3. Stores the result
    let program = vec![
        0x8C021000u32, // lw $2, 0x1000($0) - load from address 0x1000 (value 21)
        0x8C031004u32, // lw $3, 0x1004($0) - load from address 0x1004 (value 21)
        0x00431020u32, // add $2, $2, $3   - add values (21+21=42)
        0xAC021008u32, // sw $2, 0x1008($0) - store at address 0x1008
        0x00000000u32, // nop - end program
    ];

    // Convert program to bytes safely
    let mut program_bytes = Vec::with_capacity(program.len() * 4);
    for &word in &program {
        program_bytes.extend_from_slice(&word.to_le_bytes());
    }

    println!("Program created: {} instructions", program.len());

    // Load program into simulator
    simulator.load_program(&program_bytes);
    
    // Re-load the test values after loading the program
    // This ensures they're not overwritten
    simulator.memory.write_word(0x1000, 21);
    simulator.memory.write_word(0x1004, 21);
    
    println!("Test values re-loaded after program loading");
    
    // Verify memory values before running
    println!("Memory values before execution:");
    println!("  0x1000: {:?}", simulator.memory.read_word(0x1000));
    println!("  0x1004: {:?}", simulator.memory.read_word(0x1004));

    println!("Starting simulation");

    // Run the functional simulator
    simulator.run();

    // Print results for debugging
    println!("Simulation completed");
    println!("Register $1 = {}", simulator.registers.read(1));
    println!("Register $2 = {}", simulator.registers.read(2));
    println!("Register $3 = {}", simulator.registers.read(3));
    println!(
        "Memory at 0x1008 = {:?}",
        simulator.memory.read_word(0x1008)
    );

    // Assert that the results are as expected
    assert_eq!(
        simulator.registers.read(2),
        42,
        "Register $2 should contain 42"
    );
    assert_eq!(
        simulator.memory.read_word(0x1008),
        Some(42),
        "Memory at 0x1008 should contain 42"
    );

    println!("All assertions passed!");
}