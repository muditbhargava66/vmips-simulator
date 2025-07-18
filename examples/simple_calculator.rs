// examples/simple_calculator.rs
//
// This example demonstrates a simple calculator that performs basic arithmetic
// operations using the VMIPS functional simulator. It calculates:
// (15 + 25) * 3 - 10 / 2 = 40 * 3 - 5 = 120 - 5 = 115

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS Simple Calculator Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Initialize operands in memory
    simulator.memory.write_word_init(0x1000, 15); // First operand
    simulator.memory.write_word_init(0x1004, 25); // Second operand
    simulator.memory.write_word_init(0x1008, 3); // Third operand
    simulator.memory.write_word_init(0x100C, 10); // Fourth operand
    simulator.memory.write_word_init(0x1010, 2); // Fifth operand

    println!("Calculating: (15 + 25) * 3 - 10 / 2");
    println!("Expected result: (40) * 3 - 5 = 120 - 5 = 115\n");

    // Calculator program
    let instructions = vec![
        // Load operands
        0x8C021000u32, // lw $2, 0x1000($0)     # $2 = 15
        0x8C031004u32, // lw $3, 0x1004($0)     # $3 = 25
        0x8C041008u32, // lw $4, 0x1008($0)     # $4 = 3
        0x8C05100Cu32, // lw $5, 0x100C($0)     # $5 = 10
        0x8C061010u32, // lw $6, 0x1010($0)     # $6 = 2
        // Step 1: Calculate 15 + 25 = 40
        0x00431020u32, // add $2, $2, $3        # $2 = 15 + 25 = 40
        // Step 2: Calculate 40 * 3 = 120
        0x00440018u32, // mult $2, $4           # LO = 40 * 3 = 120
        0x00001012u32, // mflo $2               # $2 = 120
        // Step 3: Calculate 10 / 2 = 5
        0x00A6001Au32, // div $5, $6            # LO = 10 / 2 = 5
        0x00001812u32, // mflo $3               # $3 = 5
        // Step 4: Calculate 120 - 5 = 115
        0x00431022u32, // sub $2, $2, $3        # $2 = 120 - 5 = 115
        // Store final result
        0xAC021100u32, // sw $2, 0x1100($0)     # Store result at 0x1100
        // Store intermediate results for verification
        0xAC041104u32, // sw $4, 0x1104($0)     # Store multiplication result
        0xAC031108u32, // sw $3, 0x1108($0)     # Store division result
        // End program
        0x00000000u32, // nop
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running calculator program...");
    simulator.run();

    // Get results
    let final_result = simulator.memory.read_word(0x1100).unwrap_or(0);
    let division_result = simulator.memory.read_word(0x1108).unwrap_or(0);

    println!("\nCalculation steps:");
    println!("Step 1: 15 + 25 = 40");
    println!("Step 2: 40 × 3 = 120");
    println!("Step 3: 10 ÷ 2 = {}", division_result);
    println!("Step 4: 120 - {} = {}", division_result, final_result);

    // Verify the result
    let expected = (15 + 25) * 3 - 10 / 2;

    println!("\nFinal result: {}", final_result);
    println!("Expected result: {}", expected);

    if final_result == expected {
        println!("✓ Calculator computation successful!");
    } else {
        println!(
            "✗ Calculation failed. Expected {}, got {}",
            expected, final_result
        );

        // Debug information
        println!("\nDebug - Final register values:");
        for i in 2..7 {
            println!("${}: {}", i, simulator.registers.read(i));
        }
    }

    println!("\nThis example demonstrates:");
    println!("- Basic arithmetic operations (add, subtract, multiply, divide)");
    println!("- Using MULT/MFLO for multiplication");
    println!("- Using DIV/MFLO for division");
    println!("- Order of operations in complex expressions");
    println!("- Storing intermediate and final results");
}
