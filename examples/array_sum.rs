// examples/array_sum.rs
//
// This example demonstrates calculating the sum of an array of integers
// using the VMIPS functional simulator. It sums the array [10, 20, 30, 40, 50]
// Expected result: 10 + 20 + 30 + 40 + 50 = 150

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS Array Sum Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Initialize array in memory: [10, 20, 30, 40, 50]
    let array_base = 0x1000;
    let array_size = 5;

    simulator.memory.write_word_init(array_base, 10);
    simulator.memory.write_word_init(array_base + 4, 20);
    simulator.memory.write_word_init(array_base + 8, 30);
    simulator.memory.write_word_init(array_base + 12, 40);
    simulator.memory.write_word_init(array_base + 16, 50);

    println!("Array: [10, 20, 30, 40, 50]");
    println!("Expected sum: 10 + 20 + 30 + 40 + 50 = 150\n");

    // Array sum program - calculate sum step by step
    let instructions = vec![
        // Load and sum array elements: 10 + 20 + 30 + 40 + 50 = 150
        0x8C021000u32, // lw $2, 0x1000($0)     # $2 = array[0] = 10
        0x8C031004u32, // lw $3, 0x1004($0)     # $3 = array[1] = 20
        0x00431020u32, // add $2, $2, $3        # $2 = 10 + 20 = 30
        0x8C031008u32, // lw $3, 0x1008($0)     # $3 = array[2] = 30
        0x00431020u32, // add $2, $2, $3        # $2 = 30 + 30 = 60
        0x8C03100Cu32, // lw $3, 0x100C($0)     # $3 = array[3] = 40
        0x00431020u32, // add $2, $2, $3        # $2 = 60 + 40 = 100
        0x8C031010u32, // lw $3, 0x1010($0)     # $3 = array[4] = 50
        0x00431020u32, // add $2, $2, $3        # $2 = 100 + 50 = 150
        // Store result
        0xAC021100u32, // sw $2, 0x1100($0)     # Store sum = 150
        0x00000000u32, // nop                   # End program
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running array sum calculation...");
    simulator.run();

    // Get the result
    let result = simulator.memory.read_word(0x1100).unwrap_or(0);

    // Calculate expected sum
    let expected: u32 = (0..array_size)
        .map(|i| simulator.memory.read_word(array_base + i * 4).unwrap_or(0))
        .sum();

    println!("\nArray sum result: {}", result);
    println!("Expected result: {}", expected);

    if result == expected {
        println!("✓ Array sum calculation successful!");
    } else {
        println!(
            "✗ Calculation failed. Expected {}, got {}",
            expected, result
        );

        // Debug information
        println!("\nDebug - Array elements:");
        for i in 0..array_size {
            let value = simulator.memory.read_word(array_base + i * 4).unwrap_or(0);
            println!("  array[{}] = {}", i, value);
        }

        println!("\nDebug - Final register values:");
        for i in 2..8 {
            println!("${}: {}", i, simulator.registers.read(i));
        }
    }

    println!("\nThis example demonstrates:");
    println!("- Array traversal with loops");
    println!("- Address calculation (base + index * element_size)");
    println!("- Accumulation patterns");
    println!("- Loop control with counters and conditions");
    println!("- Memory addressing and data loading");
}
