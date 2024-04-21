// tests/functional_simulator.rs
use vmips_rust::functional_simulator::simulator::Simulator;

#[test]
fn test_functional_simulator() {
    let memory_size = 8192;
    let mut simulator = Simulator::new(memory_size);

    // Load the expected values into memory
    simulator.memory.write_word(0x1000, 21);
    simulator.memory.write_word(0x1004, 21);

    // Load a test program into memory
    let program = vec![
        0x00000000u32, // nop
        0x00000000u32, // nop
        0x8C020000u32, // lw $2, 0($0)
        0x8C030004u32, // lw $3, 4($0)
        0x00430820u32, // add $1, $2, $3
        0xAC010008u32, // sw $1, 8($0)
    ];
    let program_bytes = unsafe {
        std::slice::from_raw_parts(
            program.as_ptr() as *const u8,
            program.len() * std::mem::size_of::<u32>(),
        )
    };
    simulator.load_program(program_bytes);

    // Run the functional simulator
    simulator.run();

    // Add your test assertions here
    assert_eq!(simulator.registers.read(1), 42);
    assert_eq!(simulator.memory.read_word(0x1008), Some(42));
}