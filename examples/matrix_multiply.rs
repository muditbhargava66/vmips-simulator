// examples/matrix_multiply.rs
//
// This example demonstrates matrix multiplication using the VMIPS functional simulator.
// It multiplies two 2x2 matrices:
// A = [[1, 2], [3, 4]]  ×  B = [[5, 6], [7, 8]]  =  C = [[19, 22], [43, 50]]

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    println!("=== VMIPS Matrix Multiplication Example ===\n");

    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Initialize 2x2 matrices in memory
    // Matrix A: [[1, 2], [3, 4]] at address 0x1000
    simulator.memory.write_word_init(0x1000, 1); // A[0][0]
    simulator.memory.write_word_init(0x1004, 2); // A[0][1]
    simulator.memory.write_word_init(0x1008, 3); // A[1][0]
    simulator.memory.write_word_init(0x100C, 4); // A[1][1]

    // Matrix B: [[5, 6], [7, 8]] at address 0x1100
    simulator.memory.write_word_init(0x1100, 5); // B[0][0]
    simulator.memory.write_word_init(0x1104, 6); // B[0][1]
    simulator.memory.write_word_init(0x1108, 7); // B[1][0]
    simulator.memory.write_word_init(0x110C, 8); // B[1][1]

    println!("Matrix A:");
    println!("[[1, 2],");
    println!(" [3, 4]]");
    println!("\nMatrix B:");
    println!("[[5, 6],");
    println!(" [7, 8]]");
    println!("\nExpected result C = A × B:");
    println!("C[0][0] = 1×5 + 2×7 = 5 + 14 = 19");
    println!("C[0][1] = 1×6 + 2×8 = 6 + 16 = 22");
    println!("C[1][0] = 3×5 + 4×7 = 15 + 28 = 43");
    println!("C[1][1] = 3×6 + 4×8 = 18 + 32 = 50\n");

    // Matrix multiplication program for 2x2 matrices
    let instructions = vec![
        // Calculate C[0][0] = A[0][0]*B[0][0] + A[0][1]*B[1][0] = 1*5 + 2*7 = 19
        0x8C021000u32, // lw $2, 0x1000($0)     # $2 = A[0][0] = 1
        0x8C031100u32, // lw $3, 0x1100($0)     # $3 = B[0][0] = 5
        0x00430018u32, // mult $2, $3           # LO = 1 * 5 = 5
        0x00002012u32, // mflo $4               # $4 = 5
        0x8C021004u32, // lw $2, 0x1004($0)     # $2 = A[0][1] = 2
        0x8C031108u32, // lw $3, 0x1108($0)     # $3 = B[1][0] = 7
        0x00430018u32, // mult $2, $3           # LO = 2 * 7 = 14
        0x00002812u32, // mflo $5               # $5 = 14
        0x00851020u32, // add $2, $4, $5        # $2 = 5 + 14 = 19
        0xAC021200u32, // sw $2, 0x1200($0)     # Store C[0][0] = 19
        // Calculate C[0][1] = A[0][0]*B[0][1] + A[0][1]*B[1][1] = 1*6 + 2*8 = 22
        0x8C021000u32, // lw $2, 0x1000($0)     # $2 = A[0][0] = 1
        0x8C031104u32, // lw $3, 0x1104($0)     # $3 = B[0][1] = 6
        0x00430018u32, // mult $2, $3           # LO = 1 * 6 = 6
        0x00002012u32, // mflo $4               # $4 = 6
        0x8C021004u32, // lw $2, 0x1004($0)     # $2 = A[0][1] = 2
        0x8C03110Cu32, // lw $3, 0x110C($0)     # $3 = B[1][1] = 8
        0x00430018u32, // mult $2, $3           # LO = 2 * 8 = 16
        0x00002812u32, // mflo $5               # $5 = 16
        0x00851020u32, // add $2, $4, $5        # $2 = 6 + 16 = 22
        0xAC021204u32, // sw $2, 0x1204($0)     # Store C[0][1] = 22
        // Calculate C[1][0] = A[1][0]*B[0][0] + A[1][1]*B[1][0] = 3*5 + 4*7 = 43
        0x8C021008u32, // lw $2, 0x1008($0)     # $2 = A[1][0] = 3
        0x8C031100u32, // lw $3, 0x1100($0)     # $3 = B[0][0] = 5
        0x00430018u32, // mult $2, $3           # LO = 3 * 5 = 15
        0x00002012u32, // mflo $4               # $4 = 15
        0x8C02100Cu32, // lw $2, 0x100C($0)     # $2 = A[1][1] = 4
        0x8C031108u32, // lw $3, 0x1108($0)     # $3 = B[1][0] = 7
        0x00430018u32, // mult $2, $3           # LO = 4 * 7 = 28
        0x00002812u32, // mflo $5               # $5 = 28
        0x00851020u32, // add $2, $4, $5        # $2 = 15 + 28 = 43
        0xAC021208u32, // sw $2, 0x1208($0)     # Store C[1][0] = 43
        // Calculate C[1][1] = A[1][0]*B[0][1] + A[1][1]*B[1][1] = 3*6 + 4*8 = 50
        0x8C021008u32, // lw $2, 0x1008($0)     # $2 = A[1][0] = 3
        0x8C031104u32, // lw $3, 0x1104($0)     # $3 = B[0][1] = 6
        0x00430018u32, // mult $2, $3           # LO = 3 * 6 = 18
        0x00002012u32, // mflo $4               # $4 = 18
        0x8C02100Cu32, // lw $2, 0x100C($0)     # $2 = A[1][1] = 4
        0x8C03110Cu32, // lw $3, 0x110C($0)     # $3 = B[1][1] = 8
        0x00430018u32, // mult $2, $3           # LO = 4 * 8 = 32
        0x00002812u32, // mflo $5               # $5 = 32
        0x00851020u32, // add $2, $4, $5        # $2 = 18 + 32 = 50
        0xAC02120Cu32, // sw $2, 0x120C($0)     # Store C[1][1] = 50
        // End program
        0x00000000u32, // nop
    ];

    // Load instructions into memory
    for (i, &instruction) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instruction);
    }

    println!("Running matrix multiplication...");
    simulator.run();

    // Read and display the result matrix
    let c00 = simulator.memory.read_word(0x1200).unwrap_or(0);
    let c01 = simulator.memory.read_word(0x1204).unwrap_or(0);
    let c10 = simulator.memory.read_word(0x1208).unwrap_or(0);
    let c11 = simulator.memory.read_word(0x120C).unwrap_or(0);

    println!("\nResult Matrix C:");
    println!("[[{}, {}],", c00, c01);
    println!(" [{}, {}]]", c10, c11);

    // Verify results
    let expected = [[19, 22], [43, 50]];
    let actual = [[c00, c01], [c10, c11]];

    let mut correct = true;
    for i in 0..2 {
        for j in 0..2 {
            if actual[i][j] != expected[i][j] {
                correct = false;
                break;
            }
        }
    }

    if correct {
        println!("\n✓ Matrix multiplication successful!");
    } else {
        println!("\n✗ Matrix multiplication failed!");
        println!("Expected: [[19, 22], [43, 50]]");
        println!("Got:      [[{}, {}], [{}, {}]]", c00, c01, c10, c11);
    }

    println!("\nThis example demonstrates:");
    println!("- Loading matrix elements from memory");
    println!("- Matrix multiplication algorithm");
    println!("- Multiple multiplication and addition operations");
    println!("- Storing results back to memory");
}
