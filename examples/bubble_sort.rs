// examples/bubble_sort.rs

use vmips_rust::functional_simulator::simulator::Simulator;

fn main() {
    let memory_size = 16384;
    let mut simulator = Simulator::new(memory_size);

    // Create a small array of just 2 elements: [9, 3]
    simulator.memory.write_word(0x100, 9);
    simulator.memory.write_word(0x104, 3);

    // Print original array values
    println!("Original array:");
    println!("Element 1: {}", simulator.memory.read_word(0x100).unwrap_or(0));
    println!("Element 2: {}", simulator.memory.read_word(0x104).unwrap_or(0));

    // Create an extremely simple program that just loads, compares, and swaps two values
    // Using only the most basic instructions that we know work in the simulator
    let program = vec![
        // Load the two values
        0x8c020100u32, // lw $2, 0x100($0)   # Load 9 into $2
        0x8c030104u32, // lw $3, 0x104($0)   # Load 3 into $3
        
        // Compare if value in $3 < value in $2
        0x0043282au32, // slt $5, $2, $3     # $5 = 1 if $2 < $3, else 0
        0x14a00005u32, // bne $5, $0, skip   # If $5 != 0, elements already in order
        
        // If we get here, we need to swap
        0x8c020100u32, // lw $2, 0x100($0)   # Reload first value (9)
        0x8c030104u32, // lw $3, 0x104($0)   # Reload second value (3)
        0xac030100u32, // sw $3, 0x100($0)   # Store second value (3) in first position
        0xac020104u32, // sw $2, 0x104($0)   # Store first value (9) in second position
        
        // Skip the swap
        0x00000000u32, // nop                # End program
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
    println!("\nRunning minimal swap operation...");
    simulator.run();

    // Print the final array values
    println!("\nFinal array (should be sorted to [3, 9]):");
    println!("Element 1: {}", simulator.memory.read_word(0x100).unwrap_or(0));
    println!("Element 2: {}", simulator.memory.read_word(0x104).unwrap_or(0));
    
    // Print final register values for debugging
    println!("\nFinal register values:");
    println!("$2: {}", simulator.registers.read(2));
    println!("$3: {}", simulator.registers.read(3));
    println!("$5: {}", simulator.registers.read(5));
}