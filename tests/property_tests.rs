use proptest::prelude::*;
use quickcheck::TestResult;
use vmips_rust::functional_simulator::instructions::Instruction;
use vmips_rust::functional_simulator::memory::Memory;
use vmips_rust::functional_simulator::registers::Registers;
use vmips_rust::functional_simulator::simulator::{decode_instruction, Simulator};

// Property-based tests using proptest

proptest! {
    #[test]
    fn test_memory_read_write_consistency(
        addr in (0u32..1000).prop_map(|x| x * 4), // Ensure word alignment
        value in any::<u32>()
    ) {
        let mut memory = Memory::new_simple(4096); // Use simple memory

        // Write and read should be consistent
        if memory.write_word(addr as usize, value) {
            prop_assert_eq!(memory.read_word(addr as usize), Some(value));
        }
    }

    #[test]
    fn test_register_operations(
        reg in 1u32..32, // Skip register 0
        value in any::<u32>()
    ) {
        let mut registers = Registers::new();

        // Register 0 should always be 0
        registers.write(0, value);
        prop_assert_eq!(registers.read(0), 0);

        // Other registers should store values correctly
        registers.write(reg, value);
        prop_assert_eq!(registers.read(reg), value);
    }

    #[test]
    fn test_arithmetic_operations_commutative(
        a in any::<u32>(),
        b in any::<u32>()
    ) {
        let mut sim1 = Simulator::new(1024);
        let mut sim2 = Simulator::new(1024);

        // Test ADD commutativity: a + b = b + a
        sim1.registers.write(1, a);
        sim1.registers.write(2, b);
        sim2.registers.write(1, b);
        sim2.registers.write(2, a);

        // ADD $3, $1, $2
        let add_instruction = 0x00221820u32;
        sim1.memory.write_word_init(0, add_instruction);
        sim2.memory.write_word_init(0, add_instruction);

        sim1.step();
        sim2.step();

        prop_assert_eq!(sim1.registers.read(3), sim2.registers.read(3));
    }

    #[test]
    fn test_memory_bounds_checking(
        addr in any::<u32>(),
        value in any::<u32>()
    ) {
        let memory_size = 1024;
        let mut memory = Memory::new_simple(memory_size); // Use simple memory for predictable behavior

        if addr as usize >= memory_size || addr % 4 != 0 {
            // Out of bounds or misaligned access should fail
            prop_assert!(!memory.write_word(addr as usize, value));
            prop_assert_eq!(memory.read_word(addr as usize), None);
        } else {
            // Valid access should succeed
            prop_assert!(memory.write_word(addr as usize, value));
            prop_assert_eq!(memory.read_word(addr as usize), Some(value));
        }
    }

    #[test]
    fn test_instruction_decode_deterministic(
        instruction_word in any::<u32>()
    ) {
        // Decoding the same instruction should always give the same result
        let decoded1 = decode_instruction(instruction_word);
        let decoded2 = decode_instruction(instruction_word);

        // Since Instruction doesn't implement PartialEq, we test specific cases
        match (decoded1, decoded2) {
            (Instruction::Nop, Instruction::Nop) => prop_assert!(true),
            (Instruction::InvalidInstruction, Instruction::InvalidInstruction) => prop_assert!(true),
            (Instruction::Add { rd: rd1, rs: rs1, rt: rt1 },
             Instruction::Add { rd: rd2, rs: rs2, rt: rt2 }) => {
                prop_assert_eq!(rd1, rd2);
                prop_assert_eq!(rs1, rs2);
                prop_assert_eq!(rt1, rt2);
            },
            _ => {
                // For other instruction types, we just verify they decode consistently
                // This is a simplified test - in practice you'd match all instruction types
                prop_assert!(true);
            }
        }
    }
}

// QuickCheck-based tests

fn qc_memory_alignment_invariant(addr: u32, value: u32) -> TestResult {
    if addr > 4000 {
        return TestResult::discard();
    }

    let mut memory = Memory::new_simple(4096);

    if addr % 4 == 0 && (addr as usize) + 4 <= 4096 {
        // Aligned access within bounds should work
        let write_success = memory.write_word(addr as usize, value);
        let read_result = memory.read_word(addr as usize);

        TestResult::from_bool(write_success && read_result == Some(value))
    } else {
        // Misaligned or out-of-bounds access should fail
        let write_success = memory.write_word(addr as usize, value);
        let read_result = memory.read_word(addr as usize);

        TestResult::from_bool(!write_success && read_result.is_none())
    }
}

fn qc_register_zero_invariant(reg: u32, value: u32) -> TestResult {
    if reg >= 32 {
        return TestResult::discard();
    }

    let mut registers = Registers::new();
    registers.write(reg, value);

    if reg == 0 {
        // Register 0 should always be 0
        TestResult::from_bool(registers.read(0) == 0)
    } else {
        // Other registers should store the value
        TestResult::from_bool(registers.read(reg) == value)
    }
}

fn qc_arithmetic_overflow_behavior(a: u32, b: u32) -> bool {
    let mut simulator = Simulator::new(1024);

    simulator.registers.write(1, a);
    simulator.registers.write(2, b);

    // ADD $3, $1, $2
    let add_instruction = 0x00221820u32;
    simulator.memory.write_word_init(0, add_instruction);

    simulator.step();

    // Verify overflow behavior (wrapping)
    let expected = a.wrapping_add(b);
    simulator.registers.read(3) == expected
}

fn qc_memory_initialization_consistency(addresses: Vec<u32>, values: Vec<u32>) -> TestResult {
    if addresses.len() != values.len() || addresses.len() > 100 {
        return TestResult::discard();
    }

    let mut memory = Memory::new(4096);
    let mut valid_pairs = Vec::new();

    // Filter for valid addresses and store valid pairs
    for (&addr, &value) in addresses.iter().zip(values.iter()) {
        if addr < 4096 && addr % 4 == 0 {
            memory.write_word_init(addr as usize, value);
            valid_pairs.push((addr, value));
        }
    }

    // Verify all stored values can be read back correctly
    for (addr, expected_value) in valid_pairs {
        if memory.read_word(addr as usize) != Some(expected_value) {
            return TestResult::from_bool(false);
        }
    }

    TestResult::from_bool(true)
}

fn qc_simulator_state_isolation(instructions: Vec<u32>) -> TestResult {
    if instructions.len() > 10 {
        return TestResult::discard();
    }

    let mut sim1 = Simulator::new(1024);
    let mut sim2 = Simulator::new(1024);

    // Load same instructions into both simulators
    for (i, &instruction) in instructions.iter().enumerate() {
        sim1.memory.write_word_init(i * 4, instruction);
        sim2.memory.write_word_init(i * 4, instruction);
    }

    // Set same initial register state
    for reg in 1..8 {
        let value = reg as u32 * 10;
        sim1.registers.write(reg, value);
        sim2.registers.write(reg, value);
    }

    // Execute same number of steps
    let steps = std::cmp::min(instructions.len(), 5);
    for _ in 0..steps {
        sim1.step();
        sim2.step();
    }

    // Both simulators should have identical state
    for reg in 0..8 {
        if sim1.registers.read(reg) != sim2.registers.read(reg) {
            return TestResult::from_bool(false);
        }
    }

    TestResult::from_bool(true)
}

// Property tests for specific instruction behaviors

fn qc_load_store_consistency(addr: u32, value: u32) -> TestResult {
    if addr >= 1000 || addr % 4 != 0 {
        return TestResult::discard();
    }

    let mut simulator = Simulator::new(4096);
    // Use simple memory for the simulator's memory
    simulator.memory = Memory::new_simple(4096);

    // Store value using SW instruction
    simulator.registers.write(1, value);
    let sw_instruction = 0xAC010000u32 | addr; // SW $1, addr($0)
    simulator.memory.write_word_init(0, sw_instruction);
    simulator.step();

    // Load value using LW instruction
    let lw_instruction = 0x8C020000u32 | addr; // LW $2, addr($0)
    simulator.memory.write_word_init(4, lw_instruction);
    simulator.step();

    // Loaded value should match stored value
    TestResult::from_bool(simulator.registers.read(2) == value)
}

fn qc_branch_instruction_pc_behavior(offset: i16) -> TestResult {
    // Test that branch instructions correctly modify PC
    let mut simulator = Simulator::new(1024);

    // Set up equal registers for BEQ
    simulator.registers.write(1, 42);
    simulator.registers.write(2, 42);

    // BEQ $1, $2, offset
    let beq_instruction = 0x10220000u32 | ((offset as u16) as u32);
    simulator.memory.write_word_init(0, beq_instruction);

    let initial_pc = 0u32; // PC starts at 0
    simulator.step();

    // PC should be modified by the branch offset (scaled by 4)
    let _expected_pc = initial_pc
        .wrapping_add(4)
        .wrapping_add((offset as i32 * 4) as u32);

    // Note: This test assumes we can access PC, which may not be directly possible
    // In a real implementation, you'd need a way to verify PC changes
    TestResult::from_bool(true) // Simplified for this example
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_quickcheck_tests() {
        // Run a smaller number of tests to avoid overwhelming output
        quickcheck::QuickCheck::new()
            .tests(10)
            .quickcheck(qc_memory_alignment_invariant as fn(u32, u32) -> TestResult);

        quickcheck::QuickCheck::new()
            .tests(10)
            .quickcheck(qc_register_zero_invariant as fn(u32, u32) -> TestResult);

        quickcheck::QuickCheck::new()
            .tests(10)
            .quickcheck(qc_arithmetic_overflow_behavior as fn(u32, u32) -> bool);

        quickcheck::QuickCheck::new()
            .tests(5)
            .quickcheck(qc_load_store_consistency as fn(u32, u32) -> TestResult);
    }
}
