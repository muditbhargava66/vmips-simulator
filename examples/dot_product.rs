// examples/dot_product.rs

use vmips_rust::functional_simulator::simulator::Simulator;

// This function is used to validate our simulator implementation
fn dot_product(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut result = 0;
    for (a_val, b_val) in a.iter().zip(b.iter()) {
        result += a_val * b_val;
    }
    result
}

fn main() {
    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Load the vectors into memory
    // Use smaller values to avoid issues with multiplication overflow
    simulator.memory.write_word(0x1000, 2);
    simulator.memory.write_word(0x1004, 3);
    simulator.memory.write_word(0x1008, 4);
    simulator.memory.write_word(0x100C, 5);

    // Simplified dot product program - just calculate 2*3 + 4*5 = 6 + 20 = 26
    let program = vec![
        // Initialize registers
        0x00001020u32, // add $2, $0, $0 - initialize result register to 0
        
        // Load first pair and multiply
        0x8C061000u32, // lw $6, 0x1000($0) - load value 2
        0x8C071004u32, // lw $7, 0x1004($0) - load value 3
        0x00C70018u32, // mult $6, $7       - multiply 2*3=6
        0x00001012u32, // mflo $2           - move result to $2
        
        // Save the first result
        0x00401820u32, // add $3, $2, $0    - save first result to $3
        
        // Load second pair and multiply
        0x8C061008u32, // lw $6, 0x1008($0) - load value 4
        0x8C07100Cu32, // lw $7, 0x100C($0) - load value 5
        0x00C70018u32, // mult $6, $7       - multiply 4*5=20
        0x00001012u32, // mflo $2           - move result to $2
        
        // Add the two products
        0x00621020u32, // add $2, $3, $2    - add 6 + 20 = 26
        
        // Store the final result
        0xAC021010u32, // sw $2, 0x1010($0) - store result at 0x1010
    ];

    let program_bytes = unsafe {
        std::slice::from_raw_parts(
            program.as_ptr() as *const u8,
            program.len() * std::mem::size_of::<u32>(),
        )
    };
    simulator.load_program(program_bytes);

    simulator.run();

    // Retrieve the dot product result from memory
    let dot_product_address = 0x1010;
    match simulator.memory.read_word(dot_product_address) {
        Some(dot_product_result) => {
            println!("Dot Product Result: {}", dot_product_result);
            
            // Verify with the native function
            let a = vec![2, 4];
            let b = vec![3, 5];
            let expected = dot_product(a, b);
            println!("Expected Result: {}", expected);
            
            if dot_product_result as i32 == expected {
                println!("✓ Simulator result matches expected result!");
            } else {
                println!("✗ Results don't match. Simulator might have an issue.");
                
                // Print register values for debugging
                println!("Final register values:");
                for i in 1..8 {
                    println!("${}: {}", i, simulator.registers.read(i));
                }
            }
        }
        None => {
            println!("Failed to retrieve the dot product result from memory.");
        }
    }
}