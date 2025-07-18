// examples/string_length.rs
//
// This example demonstrates calculating the length of a null-terminated string
// using the VMIPS functional simulator. It counts characters until it finds a null byte.

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS String Length Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Initialize a null-terminated string "HELLO" in memory
    // Each character is stored as a byte, but we'll use word operations
    let string_base = 0x1000;

    // Store "HELLO\0" - each character in a separate word for simplicity
    simulator.memory.write_word_init(string_base, b'H' as u32); // H
    simulator
        .memory
        .write_word_init(string_base + 4, b'E' as u32); // E
    simulator
        .memory
        .write_word_init(string_base + 8, b'L' as u32); // L
    simulator
        .memory
        .write_word_init(string_base + 12, b'L' as u32); // L
    simulator
        .memory
        .write_word_init(string_base + 16, b'O' as u32); // O
    simulator.memory.write_word_init(string_base + 20, 0); // \0 (null terminator)

    println!("String: \"HELLO\" (null-terminated)");
    println!("Expected length: 5\n");

    // String length calculation program - count characters step by step
    let instructions = vec![
        // Count characters in "HELLO" = 5 characters
        0x24020000u32, // addiu $2, $0, 0       # $2 = length = 0
        // Check character 0: 'H' (72)
        0x8C031000u32, // lw $3, 0x1000($0)     # $3 = char[0] = 'H'
        0x10600001u32, // beq $3, $0, skip1     # if char == 0, skip
        0x24420001u32, // addiu $2, $2, 1       # length++
        // Check character 1: 'E' (69)
        0x8C031004u32, // lw $3, 0x1004($0)     # $3 = char[1] = 'E'
        0x10600001u32, // beq $3, $0, skip2     # if char == 0, skip
        0x24420001u32, // addiu $2, $2, 1       # length++
        // Check character 2: 'L' (76)
        0x8C031008u32, // lw $3, 0x1008($0)     # $3 = char[2] = 'L'
        0x10600001u32, // beq $3, $0, skip3     # if char == 0, skip
        0x24420001u32, // addiu $2, $2, 1       # length++
        // Check character 3: 'L' (76)
        0x8C03100Cu32, // lw $3, 0x100C($0)     # $3 = char[3] = 'L'
        0x10600001u32, // beq $3, $0, skip4     # if char == 0, skip
        0x24420001u32, // addiu $2, $2, 1       # length++
        // Check character 4: 'O' (79)
        0x8C031010u32, // lw $3, 0x1010($0)     # $3 = char[4] = 'O'
        0x10600001u32, // beq $3, $0, skip5     # if char == 0, skip
        0x24420001u32, // addiu $2, $2, 1       # length++
        // Check character 5: '\0' (0) - should stop here
        0x8C031014u32, // lw $3, 0x1014($0)     # $3 = char[5] = '\0'
        0x10600001u32, // beq $3, $0, end       # if char == 0, end
        0x24420001u32, // addiu $2, $2, 1       # length++ (shouldn't execute)
        // Store result
        0xAC021100u32, // sw $2, 0x1100($0)     # Store length
        0x00000000u32, // nop                   # End program
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running string length calculation...");
    simulator.run();

    // Get the result
    let result = simulator.memory.read_word(0x1100).unwrap_or(0);

    // Verify by reading the string characters
    println!("\nString characters:");
    let mut expected_length = 0;
    for i in 0..10 {
        // Check up to 10 characters
        let char_value = simulator.memory.read_word(string_base + i * 4).unwrap_or(0);
        if char_value == 0 {
            break;
        }
        let ch = char_value as u8 as char;
        println!("  Position {}: '{}' (ASCII {})", i, ch, char_value);
        expected_length += 1;
    }

    println!("\nString length result: {}", result);
    println!("Expected length: {}", expected_length);

    if result == expected_length {
        println!("✓ String length calculation successful!");
    } else {
        println!(
            "✗ Calculation failed. Expected {}, got {}",
            expected_length, result
        );

        // Debug information
        println!("\nDebug - Final register values:");
        println!("$2 (final address): 0x{:X}", simulator.registers.read(2));
        println!("$3 (length): {}", simulator.registers.read(3));
        println!("$4 (last character): {}", simulator.registers.read(4));
    }

    println!("\nThis example demonstrates:");
    println!("- String processing with null termination");
    println!("- Character-by-character traversal");
    println!("- Loop termination based on data content");
    println!("- Pointer arithmetic for string navigation");
    println!("- Working with ASCII character values");
}
