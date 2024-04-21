// examples/dot_product.rs

use vmips_rust::functional_simulator::simulator::Simulator;

fn dot_product(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut result = 0.0;
    for (a_val, b_val) in a.iter().zip(b.iter()) {
        result += a_val * b_val;
    }
    result
}

fn main() {
    let memory_size = 8192; // Increase the memory size as needed
    let mut simulator = Simulator::new(memory_size);

    // Load the vectors into memory
    simulator.memory.write_word(0x1000, 2);
    simulator.memory.write_word(0x1004, 3);
    simulator.memory.write_word(0x1008, 4);
    simulator.memory.write_word(0x100C, 5);

    // Dot product program
    let program = vec![
        0x3C040000u32, // lui $4, 0x0000
        0x34841000u32, // ori $4, $4, 0x1000
        0x3C050000u32, // lui $5, 0x0000
        0x34A51004u32, // ori $5, $5, 0x1004
        0x00001020u32, // add $2, $0, $0
        0x00001820u32, // add $3, $0, $0
        0x8C860000u32, // lw $6, 0($4)
        0x8CA70000u32, // lw $7, 0($5)
        0x00E6C018u32, // mult $6, $7
        0x00001012u32, // mflo $2
        0x00621020u32, // add $2, $3, $2
        0x00401820u32, // add $3, $2, $0
        0x24840004u32, // addiu $4, $4, 4
        0x24A50004u32, // addiu $5, $5, 4
        0x1480FFF5u32, // bne $4, $0, -11
        0xAC021010u32, // sw $2, 0x1010($0)
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
    let dot_product_address = 0x1010; // Assuming the result is stored at address 0x1010
    match simulator.memory.read_word(dot_product_address as usize) {
        Some(dot_product_result) => {
            println!("Dot Product Result: {}", dot_product_result);
        }
        None => {
            println!("Failed to retrieve the dot product result from memory.");
        }
    }
}