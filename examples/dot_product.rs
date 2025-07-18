// examples/dot_product.rs
//
// This example demonstrates computing the dot product of two vectors
// using the VMIPS functional simulator. The dot product of vectors
// A = [1, 2, 3] and B = [4, 5, 6] is calculated as:
// A·B = (1×4) + (2×5) + (3×6) = 4 + 10 + 18 = 32

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS Dot Product Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Initialize vectors in memory
    // Vector A: [1, 2, 3] at address 0x1000
    simulator.memory.write_word_init(0x1000, 1);
    simulator.memory.write_word_init(0x1004, 2);
    simulator.memory.write_word_init(0x1008, 3);

    // Vector B: [4, 5, 6] at address 0x1100
    simulator.memory.write_word_init(0x1100, 4);
    simulator.memory.write_word_init(0x1104, 5);
    simulator.memory.write_word_init(0x1108, 6);

    println!("Vector A: [1, 2, 3]");
    println!("Vector B: [4, 5, 6]");
    println!("Expected dot product: 1×4 + 2×5 + 3×6 = 4 + 10 + 18 = 32\n");

    // Dot product calculation program
    let instructions = vec![
        // Initialize result accumulator
        0x24020000u32, // addiu $2, $0, 0       # $2 = 0 (result accumulator)
        // Calculate first term: A[0] × B[0] = 1 × 4 = 4
        0x8C031000u32, // lw $3, 0x1000($0)     # $3 = A[0] = 1
        0x8C041100u32, // lw $4, 0x1100($0)     # $4 = B[0] = 4
        0x00640018u32, // mult $3, $4           # LO = 1 × 4 = 4
        0x00002812u32, // mflo $5               # $5 = 4
        0x00451020u32, // add $2, $2, $5        # $2 = 0 + 4 = 4
        // Calculate second term: A[1] × B[1] = 2 × 5 = 10
        0x8C031004u32, // lw $3, 0x1004($0)     # $3 = A[1] = 2
        0x8C041104u32, // lw $4, 0x1104($0)     # $4 = B[1] = 5
        0x00640018u32, // mult $3, $4           # LO = 2 × 5 = 10
        0x00002812u32, // mflo $5               # $5 = 10
        0x00451020u32, // add $2, $2, $5        # $2 = 4 + 10 = 14
        // Calculate third term: A[2] × B[2] = 3 × 6 = 18
        0x8C031008u32, // lw $3, 0x1008($0)     # $3 = A[2] = 3
        0x8C041108u32, // lw $4, 0x1108($0)     # $4 = B[2] = 6
        0x00640018u32, // mult $3, $4           # LO = 3 × 6 = 18
        0x00002812u32, // mflo $5               # $5 = 18
        0x00451020u32, // add $2, $2, $5        # $2 = 14 + 18 = 32
        // Store result in memory
        0xAC021200u32, // sw $2, 0x1200($0)     # Store result at 0x1200
        // End program
        0x00000000u32, // nop
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running dot product calculation...");
    simulator.run();

    // Get the result
    let result = simulator.memory.read_word(0x1200).unwrap_or(0);

    println!("\nDot product result: {}", result);

    // Verify the result
    let expected = 1 * 4 + 2 * 5 + 3 * 6;
    println!("Expected result: {}", expected);

    if result == expected {
        println!("✓ Dot product calculation successful!");
    } else {
        println!(
            "✗ Calculation failed. Expected {}, got {}",
            expected, result
        );

        // Debug information
        println!("\nDebug - Final register values:");
        println!("$2 (result): {}", simulator.registers.read(2));
        println!("$3 (last A value): {}", simulator.registers.read(3));
        println!("$4 (last B value): {}", simulator.registers.read(4));
        println!("$5 (last product): {}", simulator.registers.read(5));
    }

    println!("\nThis example demonstrates:");
    println!("- Loading data from memory");
    println!("- Multiplication using MULT/MFLO instructions");
    println!("- Accumulating results");
    println!("- Storing final results back to memory");
}
