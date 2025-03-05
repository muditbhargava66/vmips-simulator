// tests/functional_simulator.rs
use vmips_rust::functional_simulator::simulator::Simulator;

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

    // Convert program to bytes
    let program_bytes = unsafe {
        std::slice::from_raw_parts(
            program.as_ptr() as *const u8,
            program.len() * std::mem::size_of::<u32>(),
        )
    };
    
    println!("Program created: {} instructions", program.len());
    
    // Load program into simulator
    simulator.load_program(program_bytes);
    
    println!("Program loaded, starting simulation");

    // Run the functional simulator
    simulator.run();

    // Print results for debugging
    println!("Simulation completed");
    println!("Register $1 = {}", simulator.registers.read(1));
    println!("Register $2 = {}", simulator.registers.read(2));
    println!("Register $3 = {}", simulator.registers.read(3));
    println!("Memory at 0x1008 = {:?}", simulator.memory.read_word(0x1008));

    // Assert that the results are as expected
    assert_eq!(simulator.registers.read(2), 42, "Register $2 should contain 42");
    assert_eq!(simulator.memory.read_word(0x1008), Some(42), "Memory at 0x1008 should contain 42");
    
    println!("All assertions passed!");
}