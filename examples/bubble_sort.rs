// examples/bubble_sort.rs
//
// This example demonstrates a simple bubble sort algorithm implementation
// using the VMIPS functional simulator. It sorts a small array of integers.

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS Bubble Sort Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Initialize array data in memory: [5, 2, 8, 1, 9]
    let array_base = 0x1000;
    let array_size = 5;

    simulator.memory.write_word_init(array_base, 5);
    simulator.memory.write_word_init(array_base + 4, 2);
    simulator.memory.write_word_init(array_base + 8, 8);
    simulator.memory.write_word_init(array_base + 12, 1);
    simulator.memory.write_word_init(array_base + 16, 9);

    // Print original array
    println!("Original array:");
    for i in 0..array_size {
        let value = simulator.memory.read_word(array_base + i * 4).unwrap_or(0);
        print!("{} ", value);
    }
    println!("\n");

    // Simple bubble sort program for 5 elements
    // This performs one complete pass through the array
    let instructions = vec![
        // Initialize base address and counters
        0x24021000u32, // addiu $2, $0, 0x1000  # $2 = array base address
        0x24030000u32, // addiu $3, $0, 0       # $3 = outer loop counter (i)
        0x24040004u32, // addiu $4, $0, 4       # $4 = inner loop limit (n-1)
        // Outer loop: for i = 0 to n-2
        // Compare adjacent elements and swap if needed
        0x8C450000u32, // lw $5, 0($2)          # Load arr[0]
        0x8C460004u32, // lw $6, 4($2)          # Load arr[1]
        0x00A6382Au32, // slt $7, $5, $6        # $7 = 1 if arr[0] < arr[1]
        0x14E00003u32, // bne $7, $0, skip1     # Skip swap if in order
        0xAC460000u32, // sw $6, 0($2)          # arr[0] = arr[1]
        0xAC450004u32, // sw $5, 4($2)          # arr[1] = original arr[0]
        // Compare arr[1] and arr[2]
        0x8C450004u32, // lw $5, 4($2)          # Load arr[1]
        0x8C460008u32, // lw $6, 8($2)          # Load arr[2]
        0x00A6382Au32, // slt $7, $5, $6        # $7 = 1 if arr[1] < arr[2]
        0x14E00003u32, // bne $7, $0, skip2     # Skip swap if in order
        0xAC460004u32, // sw $6, 4($2)          # arr[1] = arr[2]
        0xAC450008u32, // sw $5, 8($2)          # arr[2] = original arr[1]
        // Compare arr[2] and arr[3]
        0x8C450008u32, // lw $5, 8($2)          # Load arr[2]
        0x8C46000Cu32, // lw $6, 12($2)         # Load arr[3]
        0x00A6382Au32, // slt $7, $5, $6        # $7 = 1 if arr[2] < arr[3]
        0x14E00003u32, // bne $7, $0, skip3     # Skip swap if in order
        0xAC46000Cu32, // sw $6, 12($2)         # arr[2] = arr[3]
        0xAC450008u32, // sw $5, 8($2)          # arr[3] = original arr[2]
        // Compare arr[3] and arr[4]
        0x8C45000Cu32, // lw $5, 12($2)         # Load arr[3]
        0x8C460010u32, // lw $6, 16($2)         # Load arr[4]
        0x00A6382Au32, // slt $7, $5, $6        # $7 = 1 if arr[3] < arr[4]
        0x14E00003u32, // bne $7, $0, skip4     # Skip swap if in order
        0xAC460010u32, // sw $6, 16($2)         # arr[3] = arr[4]
        0xAC45000Cu32, // sw $5, 12($2)         # arr[4] = original arr[3]
        // End program
        0x00000000u32, // nop
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running bubble sort (one pass)...");
    simulator.run();

    // Print sorted array
    println!("\nArray after one bubble sort pass:");
    for i in 0..array_size {
        let value = simulator.memory.read_word(array_base + i * 4).unwrap_or(0);
        print!("{} ", value);
    }
    println!();

    // Verify the largest element moved to the end
    let last_element = simulator.memory.read_word(array_base + 16).unwrap_or(0);
    println!(
        "\nLargest element (9) should be at the end: {}",
        last_element
    );

    if last_element == 9 {
        println!("✓ Bubble sort pass completed successfully!");
    } else {
        println!("✗ Bubble sort may not have worked correctly.");
    }

    println!("\nNote: This example shows one pass of bubble sort.");
    println!("A complete bubble sort would require multiple passes.");
}
