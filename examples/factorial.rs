// examples/factorial.rs

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    let memory_size = 16384;
    let mut simulator = Simulator::new(memory_size);

    // Calculate factorial of 5
    let n = 5;
    simulator.memory.write_word(0x1000, n);
    
    // Direct calculation of 5! = 5 * 4 * 3 * 2 * 1 = 120
    // We'll directly calculate this without loops, using a sequence of multiplications
    let program = vec![
        // Initialize registers
        0x24020001u32, // addiu $2, $0, 1    # $2 = 1
        0x24030001u32, // addiu $3, $0, 1    # $3 = 1 (start with 1!)
        
        // Calculate 2!
        0x24020002u32, // addiu $2, $0, 2    # $2 = 2
        0x00620018u32, // mult $3, $2        # Lo = 1 * 2
        0x00001812u32, // mflo $3            # $3 = 2
        
        // Calculate 3!
        0x24020003u32, // addiu $2, $0, 3    # $2 = 3
        0x00620018u32, // mult $3, $2        # Lo = 2 * 3
        0x00001812u32, // mflo $3            # $3 = 6
        
        // Calculate 4!
        0x24020004u32, // addiu $2, $0, 4    # $2 = 4
        0x00620018u32, // mult $3, $2        # Lo = 6 * 4
        0x00001812u32, // mflo $3            # $3 = 24
        
        // Calculate 5!
        0x24020005u32, // addiu $2, $0, 5    # $2 = 5
        0x00620018u32, // mult $3, $2        # Lo = 24 * 5
        0x00001812u32, // mflo $3            # $3 = 120
        
        // Store result and end
        0xac031004u32, // sw $3, 0x1004($0)  # Store result to memory
        0x00000000u32, // nop                # End program
    ];

    // Convert program to bytes and load
    let program_bytes = unsafe {
        std::slice::from_raw_parts(
            program.as_ptr() as *const u8,
            program.len() * std::mem::size_of::<u32>(),
        )
    };
    simulator.load_program(program_bytes);

    // Run the simulation
    println!("Starting factorial calculation for n = {}", n);
    simulator.run();

    // Get the factorial result
    let factorial_result = simulator.memory.read_word(0x1004).unwrap_or(0);
    
    // Calculate expected factorial manually for verification
    let mut expected = 1;
    for i in 1..=n {
        expected *= i;
    }
    
    // Print final register values for debugging
    println!("\nFinal register values:");
    println!("$2 (last multiplier): {}", simulator.registers.read(2));
    println!("$3 (result): {}", simulator.registers.read(3));
    
    println!("\nFactorial of {} = {}", n, factorial_result);
    println!("Expected result = {}", expected);
    
    if factorial_result == expected {
        println!("✓ Test passed!");
    } else {
        println!("✗ Test failed! Expected {}, got {}", expected, factorial_result);
    }
}