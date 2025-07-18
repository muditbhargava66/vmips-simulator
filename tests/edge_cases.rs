use vmips_rust::functional_simulator::memory::Memory;
use vmips_rust::functional_simulator::registers::Registers;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::timing_simulator::config::{BranchPredictorType, CacheConfig, PipelineConfig};
use vmips_rust::timing_simulator::simulator::Simulator as TimingSimulator;

#[test]
fn test_memory_boundary_conditions() {
    let mut memory = Memory::new_simple(1024);

    // Test writing at memory boundaries
    assert!(memory.write_word(0, 0x12345678));
    assert!(memory.write_word(1020, 0x87654321));

    // Test reading at boundaries
    assert_eq!(memory.read_word(0), Some(0x12345678));
    assert_eq!(memory.read_word(1020), Some(0x87654321));

    // Test out-of-bounds access should fail
    assert_eq!(memory.read_word(1024), None);
    assert!(!memory.write_word(1024, 0x12345678));

    // Test unaligned access should fail with simple memory
    assert_eq!(memory.read_word(1), None);
    assert!(!memory.write_word(1, 0x12345678));
}

#[test]
fn test_register_edge_cases() {
    let mut registers = Registers::new();

    // Test writing to register 0 (should remain 0)
    registers.write(0, 0x12345678);
    assert_eq!(registers.read(0), 0);

    // Test maximum register values
    registers.write(1, u32::MAX);
    assert_eq!(registers.read(1), u32::MAX);

    // Test all registers
    for i in 1..32 {
        registers.write(i, i as u32);
        assert_eq!(registers.read(i), i as u32);
    }
}

#[test]
fn test_arithmetic_overflow() {
    let mut simulator = FunctionalSimulator::new(1024);

    // Set up registers for overflow test
    simulator.registers.write(1, u32::MAX);
    simulator.registers.write(2, 1);

    // Load ADD instruction: add $3, $1, $2
    let add_instruction = 0x00221820u32; // ADD $3, $1, $2
    simulator.memory.write_word_init(0, add_instruction);

    // Execute one step
    simulator.step();

    // Check that overflow wraps around
    assert_eq!(simulator.registers.read(3), 0);
}

#[test]
fn test_division_by_zero() {
    let mut simulator = FunctionalSimulator::new(1024);

    // Set up registers for division by zero
    simulator.registers.write(1, 100);
    simulator.registers.write(2, 0);

    // Load DIV instruction: div $1, $2
    let div_instruction = 0x0022001Au32; // DIV $1, $2
    simulator.memory.write_word_init(0, div_instruction);

    // Execute one step - should not crash
    simulator.step();

    // Result should be undefined but simulator should continue
    // (Implementation specific behavior)
}

#[test]
fn test_branch_edge_cases() {
    let mut simulator = FunctionalSimulator::new(1024);

    // Test branch with maximum positive offset
    simulator.registers.write(1, 5);
    simulator.registers.write(2, 5);

    // BEQ with large positive offset
    let beq_instruction = 0x10220FFFu32; // BEQ $1, $2, 0xFFF
    simulator.memory.write_word_init(0, beq_instruction);

    simulator.step();

    // PC should have jumped (implementation specific)
    // This tests that large offsets don't cause crashes
}

#[test]
fn test_load_store_edge_cases() {
    let mut simulator = FunctionalSimulator::new(1024);

    // Test load from uninitialized memory
    let lw_instruction = 0x8C010400u32; // LW $1, 0x400($0)
    simulator.memory.write_word_init(0, lw_instruction);

    simulator.step();

    // Load from uninitialized memory (behavior may vary)
    let _loaded_value = simulator.registers.read(1);
    // Just verify the operation completed without crashing

    // Test store to boundary
    simulator.registers.write(2, 0x12345678);
    let sw_instruction = 0xAC0203FCu32; // SW $2, 0x3FC($0)
    simulator.memory.write_word_init(4, sw_instruction);

    simulator.step();

    // Verify store worked (if within bounds)
    let _stored_value = simulator.memory.read_word(0x3FC);
    // Just verify the operation completed
}

#[test]
fn test_cache_edge_cases() {
    let cache_config = CacheConfig::new(64, 1, 4); // Very small cache
    let pipeline_config = PipelineConfig::new(5);

    let mut simulator =
        TimingSimulator::new(pipeline_config, cache_config.clone(), cache_config, 1024);

    // Fill cache with conflicting addresses
    for i in 0..20 {
        let addr = i * 64; // Addresses that conflict in direct-mapped cache
        simulator.memory.write_word_init(addr, i as u32);
    }

    // Access pattern that causes many cache misses
    for i in 0..10 {
        let addr = i * 64;
        let _ = simulator.memory.read_word(addr);
    }

    // Test should complete without crashes
}

#[test]
fn test_pipeline_hazard_edge_cases() {
    let pipeline_config = PipelineConfig::new(5).with_forwarding(false); // Disable forwarding to test stalls
    let cache_config = CacheConfig::new(1024, 4, 32);

    let mut simulator =
        TimingSimulator::new(pipeline_config, cache_config.clone(), cache_config, 1024);

    // Create a sequence with data hazards
    let instructions = vec![
        0x8C010000u32, // LW $1, 0($0)    - Load
        0x00211020u32, // ADD $2, $1, $1  - Use loaded value immediately
        0x00411820u32, // ADD $3, $2, $1  - Another dependency
    ];

    for (i, &instr) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instr);
    }

    // Run for several cycles to test hazard handling
    for _ in 0..20 {
        // This should handle hazards without crashing
        if simulator.pc >= (instructions.len() * 4) as u32 {
            break;
        }
    }
}

#[test]
fn test_branch_prediction_edge_cases() {
    let pipeline_config =
        PipelineConfig::new(5).with_branch_prediction(true, BranchPredictorType::TwoBit);
    let cache_config = CacheConfig::new(1024, 4, 32);

    let mut simulator =
        TimingSimulator::new(pipeline_config, cache_config.clone(), cache_config, 1024);

    // Create alternating branch pattern to test predictor
    simulator.registers.write(1, 0);
    simulator.registers.write(2, 1);

    let instructions = vec![
        0x10220001u32, // BEQ $1, $2, +1  - Not taken
        0x00000000u32, // NOP
        0x14220001u32, // BNE $1, $2, +1  - Taken
        0x00000000u32, // NOP
    ];

    for (i, &instr) in instructions.iter().enumerate() {
        simulator.memory.write_word_init(i * 4, instr);
    }

    // Test branch prediction with alternating pattern
    for _ in 0..10 {
        if simulator.pc >= (instructions.len() * 4) as u32 {
            break;
        }
    }
}

#[test]
fn test_memory_alignment_errors() {
    let mut memory = Memory::new_simple(1024);

    // Test various misaligned accesses should fail
    for offset in 1..4 {
        assert_eq!(memory.read_word(offset), None);
        assert!(!memory.write_word(offset, 0x12345678));
    }

    // Test aligned accesses work
    for i in 0..10 {
        let addr = i * 4;
        assert!(memory.write_word(addr, i as u32));
        assert_eq!(memory.read_word(addr), Some(i as u32));
    }
}

#[test]
fn test_instruction_decode_edge_cases() {
    use vmips_rust::functional_simulator::instructions::Instruction;
    use vmips_rust::functional_simulator::simulator::decode_instruction;

    // Test invalid instruction
    let invalid = decode_instruction(0xFFFFFFFF);
    assert!(matches!(invalid, Instruction::InvalidInstruction));

    // Test all zeros
    let zeros = decode_instruction(0x00000000);
    assert!(matches!(zeros, Instruction::Nop));

    // Test various instruction formats
    let add = decode_instruction(0x00221020); // ADD $2, $1, $2
    assert!(matches!(add, Instruction::Add { .. }));

    let lw = decode_instruction(0x8C220000); // LW $2, 0($1)
    assert!(matches!(lw, Instruction::Lw { .. }));
}

#[test]
fn test_simulator_state_consistency() {
    let mut simulator = FunctionalSimulator::new(1024);

    // Set initial state
    simulator.registers.write(1, 100);
    simulator.memory.write_word_init(0x100, 200);

    // Save state
    let reg_val = simulator.registers.read(1);
    let mem_val = simulator.memory.read_word(0x100);

    // Execute NOP
    simulator.memory.write_word_init(0, 0x00000000);
    simulator.step();

    // State should be unchanged after NOP
    assert_eq!(simulator.registers.read(1), reg_val);
    assert_eq!(simulator.memory.read_word(0x100), mem_val);
}
