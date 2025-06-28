// examples/matrix_multiply.rs

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    let memory_size = 16384;
    let mut simulator = Simulator::new(memory_size);

    // Define matrix dimensions
    let rows_a = 2;
    let cols_a = 3;
    let rows_b = 3;
    let cols_b = 2;
    
    // Matrix A address: 0x2000
    let matrix_a_addr = 0x2000;
    simulator.memory.write_word(matrix_a_addr, rows_a);
    simulator.memory.write_word(matrix_a_addr + 4, cols_a);
    // Matrix A data: [1, 2, 3, 4, 5, 6]
    simulator.memory.write_word(matrix_a_addr + 8, 1);
    simulator.memory.write_word(matrix_a_addr + 12, 2);
    simulator.memory.write_word(matrix_a_addr + 16, 3);
    simulator.memory.write_word(matrix_a_addr + 20, 4);
    simulator.memory.write_word(matrix_a_addr + 24, 5);
    simulator.memory.write_word(matrix_a_addr + 28, 6);
    
    // Matrix B address: 0x3000
    let matrix_b_addr = 0x3000;
    simulator.memory.write_word(matrix_b_addr, rows_b);
    simulator.memory.write_word(matrix_b_addr + 4, cols_b);
    // Matrix B data: [7, 8, 9, 10, 11, 12]
    simulator.memory.write_word(matrix_b_addr + 8, 7);
    simulator.memory.write_word(matrix_b_addr + 12, 8);
    simulator.memory.write_word(matrix_b_addr + 16, 9);
    simulator.memory.write_word(matrix_b_addr + 20, 10);
    simulator.memory.write_word(matrix_b_addr + 24, 11);
    simulator.memory.write_word(matrix_b_addr + 28, 12);
    
    // Result matrix address: 0x4000
    let result_addr = 0x4000;
    simulator.memory.write_word(result_addr, rows_a);
    simulator.memory.write_word(result_addr + 4, cols_b);
    
    // Initialize registers with matrix addresses - Convert usize to u32
    simulator.registers.write(2, matrix_a_addr as u32);  // $2 = address of matrix A
    simulator.registers.write(3, matrix_b_addr as u32);  // $3 = address of matrix B
    simulator.registers.write(4, result_addr as u32);    // $4 = address of result matrix
    
    // Build matrix multiplication program (direct register use, no lui instructions)
    let program = vec![
        // Load matrix dimensions from memory
        0x8C420000u32, // lw $2, 0($2)         # $2 = rows_a
        0x8C430000u32, // lw $3, 0($3)         # $3 = rows_b
        0x8C440004u32, // lw $4, 4($4)         # $4 = cols_b 
        
        // Initialize our matrix pointers again
        0x24050000u32, // addiu $5, $0, 0      # $5 = 0 (i: row counter for A)
        0x3C062000u32, // lui $6, 0x2000       # $6 = matrix A address
        0x24C60008u32, // addiu $6, $6, 8      # Skip dimensions
        0x3C073000u32, // lui $7, 0x3000       # $7 = matrix B address
        0x24E70008u32, // addiu $7, $7, 8      # Skip dimensions
        0x3C084000u32, // lui $8, 0x4000       # $8 = result matrix address
        0x25080008u32, // addiu $8, $8, 8      # Skip dimensions
        
        // Calculate total elements in A row
        0x24090003u32, // addiu $9, $0, 3      # $9 = cols_a (3)
        
        // Outer loop (for each row of A)
        0x0005502Au32, // slt $10, $0, $5      # if (i < rows_a)
        0x11400013u32, // beq $10, $0, end     # If i >= rows_a, end
        0x240A0000u32, // addiu $10, $0, 0     # $10 = 0 (j: column counter for B)
        
        // Middle loop (for each column of B)
        0x014A582Au32, // slt $11, $10, $10    # if (j < cols_b)
        0x1160000Fu32, // beq $11, $0, endmid  # If j >= cols_b, end middle
        0x240B0000u32, // addiu $11, $0, 0     # $11 = 0 (sum accumulator)
        0x240C0000u32, // addiu $12, $0, 0     # $12 = 0 (k: inner loop)
        
        // Inner loop (sum of A[i][k] * B[k][j])
        0x018C682Au32, // slt $13, $12, $9     # if (k < cols_a)
        0x11600009u32, // beq $13, $0, endinner # If k >= cols_a, end inner
        
        // Calculate A[i][k] address and load value
        0x00096080u32, // sll $12, $9, 2       # $12 = 4 * cols_a
        0x00AC6820u32, // add $13, $5, $12     # $13 = i * cols_a
        0x000D6880u32, // sll $13, $13, 2      # $13 = i * cols_a * 4
        0x00CD6820u32, // add $13, $6, $13     # $13 = baseA + i * cols_a * 4
        0x8DAD0000u32, // lw $13, 0($13)       # $13 = A[i][k]
        
        // Calculate B[k][j] address and load value
        0x00096880u32, // sll $13, $9, 2       # $13 = 4 * cols_a
        0x01AC7020u32, // add $14, $13, $12    # $14 = k * cols_a
        0x000E7080u32, // sll $14, $14, 2      # $14 = k * cols_a * 4
        0x00EE7020u32, // add $14, $7, $14     # $14 = baseB + k * cols_a * 4  
        0x8DCE0000u32, // lw $14, 0($14)       # $14 = B[k][j]
        
        // Multiply and accumulate
        0x01AE0018u32, // mult $13, $14        # $hi:$lo = A[i][k] * B[k][j]
        0x00007012u32, // mflo $14             # $14 = product
        0x016E5820u32, // add $11, $11, $14    # sum += product
        
        // Increment k and loop back
        0x258C0001u32, // addiu $12, $12, 1    # k++
        0x08000013u32, // j inner_loop         # repeat inner loop
        
        // End inner loop, store result
        0x00094880u32, // sll $9, $9, 2        # $9 = cols_a * 4
        0x00A94820u32, // add $9, $5, $9       # $9 = i * cols_a
        0x00094880u32, // sll $9, $9, 2        # $9 = i * cols_a * 4
        0x01094820u32, // add $9, $8, $9       # $9 = baseResult + i * cols_a * 4
        0xAD2B0000u32, // sw $11, 0($9)        # result[i][j] = sum
        
        // Increment j and loop back
        0x254A0001u32, // addiu $10, $10, 1    # j++
        0x0800000Eu32, // j middle_loop        # repeat middle loop
        
        // End middle loop, increment i
        0x24A50001u32, // addiu $5, $5, 1      # i++
        0x0800000Bu32, // j outer_loop         # repeat outer loop
        
        // End program
        0x00000000u32, // nop
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
    simulator.run();

    // Print the result matrix
    println!("Matrix A ({}x{}):", rows_a, cols_a);
    for i in 0..rows_a {
        for j in 0..cols_a {
            let offset = (i * cols_a + j) * 4 + 8;
            let value = simulator.memory.read_word(matrix_a_addr as usize + offset as usize).unwrap_or(0);
            print!("{} ", value);
        }
        println!();
    }

    println!("\nMatrix B ({}x{}):", rows_b, cols_b);
    for i in 0..rows_b {
        for j in 0..cols_b {
            let offset = (i * cols_b + j) * 4 + 8;
            let value = simulator.memory.read_word(matrix_b_addr as usize + offset as usize).unwrap_or(0);
            print!("{} ", value);
        }
        println!();
    }

    println!("\nResult Matrix ({}x{}):", rows_a, cols_b);
    for i in 0..rows_a {
        for j in 0..cols_b {
            let offset = (i * cols_b + j) * 4 + 8;
            let value = simulator.memory.read_word((result_addr as usize) + offset as usize).unwrap_or(0);
            print!("{} ", value);
        }
        println!();
    }
}

// Expected output:
// Matrix A (2x3):
// 1 2 3 
// 4 5 6 
// 
// Matrix B (3x2):
// 7 8 
// 9 10 
// 11 12 
// 
// Result Matrix (2x2):
// 58 64 
// 139 154