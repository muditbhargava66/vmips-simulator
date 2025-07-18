// examples/factorial.rs
//
// This example demonstrates calculating the factorial of a number
// using the VMIPS functional simulator. It calculates 6! = 720
// using an iterative approach with a loop.

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS Factorial Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Calculate factorial of 6
    let n = 6;
    println!("Calculating {}! (factorial of {})", n, n);
    println!("Expected result: 6! = 6 × 5 × 4 × 3 × 2 × 1 = 720\n");

    // Factorial calculation program - calculate 6! step by step
    let instructions = vec![
        // Calculate 6! = 6 * 5 * 4 * 3 * 2 * 1 = 720
        0x24020006u32, // addiu $2, $0, 6       # $2 = 6
        0x24030005u32, // addiu $3, $0, 5       # $3 = 5
        0x00430018u32, // mult $2, $3           # LO = 6 * 5 = 30
        0x00001012u32, // mflo $2               # $2 = 30
        0x24030004u32, // addiu $3, $0, 4       # $3 = 4
        0x00430018u32, // mult $2, $3           # LO = 30 * 4 = 120
        0x00001012u32, // mflo $2               # $2 = 120
        0x24030003u32, // addiu $3, $0, 3       # $3 = 3
        0x00430018u32, // mult $2, $3           # LO = 120 * 3 = 360
        0x00001012u32, // mflo $2               # $2 = 360
        0x24030002u32, // addiu $3, $0, 2       # $3 = 2
        0x00430018u32, // mult $2, $3           # LO = 360 * 2 = 720
        0x00001012u32, // mflo $2               # $2 = 720
        0x24030001u32, // addiu $3, $0, 1       # $3 = 1
        0x00430018u32, // mult $2, $3           # LO = 720 * 1 = 720
        0x00001012u32, // mflo $2               # $2 = 720
        // Store result
        0xAC021000u32, // sw $2, 0x1000($0)     # Store result at 0x1000
        0x00000000u32, // nop                   # End program
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running factorial calculation...");
    simulator.run();

    // Get the result
    let result = simulator.memory.read_word(0x1000).unwrap_or(0);

    // Calculate expected factorial
    let mut expected = 1u32;
    for i in 1..=n {
        expected *= i;
    }

    println!("\nFactorial result: {}", result);
    println!("Expected result: {}", expected);

    if result == expected {
        println!("✓ Factorial calculation successful!");
    } else {
        println!(
            "✗ Calculation failed. Expected {}, got {}",
            expected, result
        );

        // Debug information
        println!("\nDebug - Final register values:");
        println!("$2 (counter): {}", simulator.registers.read(2));
        println!("$3 (result): {}", simulator.registers.read(3));
    }

    println!("\nThis example demonstrates:");
    println!("- Iterative loops with branch instructions");
    println!("- Multiplication using MULT/MFLO instructions");
    println!("- Conditional branching with BGTZ");
    println!("- Jump instructions for loop control");
}
