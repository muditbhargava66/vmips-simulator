// examples/dot_product.rs
use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    let memory_size = 1024;
    let mut simulator = Simulator::new(memory_size);

    // Dot product program
    let program = vec![
        0x00000000, // Load vector A into memory
        0x00000000, // Load vector B into memory
        0x00000000, // Initialize dot product accumulator
        // Perform dot product calculation
        0x00000000, // Load element from vector A
        0x00000000, // Load element from vector B
        0x00000000, // Multiply elements
        0x00000000, // Add result to accumulator
        // ...
        0x00000000, // Store final dot product result
    ];

    simulator.load_program(&program);
    simulator.run();

    // Retrieve the dot product result from memory
    // let dot_product_result = simulator.memory.read_word(/* Address of the result */);
    let dot_product_result = simulator.memory.read_word(0x1000); // Assuming the result is stored at address 0x1000
    println!("Dot Product Result: {}", dot_product_result);
}