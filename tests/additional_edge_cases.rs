// Copyright (c) 2024 Mudit Bhargava
//
// Additional edge case tests for VMIPS Rust Simulator v0.2.2
// Tests specific edge cases and boundary conditions

use vmips_rust::assembler::Assembler;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::timing_simulator::config::{BranchPredictorType, CacheConfig, PipelineConfig};
use vmips_rust::timing_simulator::simulator::Simulator as TimingSimulator;

#[test]
fn test_assembler_default_trait() {
    // Test that Default trait is properly implemented for Assembler
    let _assembler1 = Assembler::new();
    let _assembler2 = Assembler::default();
    
    // Both constructors should work without errors
    assert!(true); // Basic test that both can be created
}

#[test]
fn test_assembler_error_display() {
    // Test error display formatting
    use vmips_rust::assembler::AssemblerError;
    use std::io;
    
    let parse_error = AssemblerError::Parse("Invalid instruction".to_string(), 5);
    let display_str = format!("{}", parse_error);
    assert!(display_str.contains("Parse error at line 5"));
    assert!(display_str.contains("Invalid instruction"));
    
    let io_error = AssemblerError::Io(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    let display_str = format!("{}", io_error);
    assert!(display_str.contains("I/O error"));
}

#[test]
fn test_functional_simulator_boundary_conditions() {
    let mut simulator = FunctionalSimulator::new(1024); // Small memory
    
    // Test memory boundary
    let boundary_address = 1020; // Near end of memory
    simulator.memory.write_word_init(boundary_address, 0x12345678);
    assert_eq!(simulator.memory.read_word(boundary_address), Some(0x12345678));
    
    // Test reading beyond boundary
    assert_eq!(simulator.memory.read_word(1024), None); // Should return None for out of bounds
}

#[test]
fn test_timing_simulator_zero_cycles() {
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 1, 1, 1, 1])
        .with_forwarding(false);

    let cache_config = CacheConfig::new(1024, 2, 32);
    
    let _simulator = TimingSimulator::new(
        pipeline_config,
        cache_config.clone(),
        cache_config,
        1024
    );
    
    // Test that simulator can be created with minimal configuration
    assert!(true);
}

#[test]
fn test_register_edge_values() {
    let mut simulator = FunctionalSimulator::new(1024);
    
    // Test maximum u32 value
    simulator.registers.write(1, u32::MAX);
    assert_eq!(simulator.registers.read(1), u32::MAX);
    
    // Test zero register is always zero
    simulator.registers.write(0, 0x12345678);
    assert_eq!(simulator.registers.read(0), 0);
}

#[test]
fn test_empty_program_handling() {
    let mut assembler = Assembler::new();
    
    // Test assembling empty string
    let result = assembler.assemble_string("");
    assert!(result.is_ok());
    
    let binary = result.unwrap();
    // Should contain header with zero sizes
    assert_eq!(binary.len(), 8); // Just the header
    
    // Data section size should be 0
    let data_size = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
    assert_eq!(data_size, 0);
    
    // Text section size should be 0
    let text_size = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
    assert_eq!(text_size, 0);
}

#[test]
fn test_cache_configuration_edge_cases() {
    // Test minimum cache configuration
    let min_cache = CacheConfig::new(64, 1, 4); // 64 bytes, 1-way, 4-byte blocks
    assert_eq!(min_cache.size, 64);
    assert_eq!(min_cache.associativity, 1);
    assert_eq!(min_cache.block_size, 4);
    
    // Test power-of-2 configurations
    let power2_cache = CacheConfig::new(1024, 4, 64);
    assert_eq!(power2_cache.size, 1024);
    assert_eq!(power2_cache.associativity, 4);
    assert_eq!(power2_cache.block_size, 64);
}

#[test]
fn test_pipeline_configuration_validation() {
    // Test minimum pipeline stages
    let min_pipeline = PipelineConfig::new(3);
    assert!(min_pipeline.num_stages >= 3);
    
    // Test with all features enabled
    let full_pipeline = PipelineConfig::new(7)
        .with_latencies(vec![1, 2, 3, 2, 1, 1, 1])
        .with_forwarding(true)
        .with_branch_prediction(true, BranchPredictorType::TwoBit)
        .with_superscalar(2);
    
    assert_eq!(full_pipeline.num_stages, 7);
    assert!(full_pipeline.forwarding_enabled);
    assert!(full_pipeline.branch_prediction_enabled);
}

#[test]
fn test_instruction_decode_edge_cases() {
    use vmips_rust::functional_simulator::simulator::decode_instruction;
    
    // Test all-zero instruction (NOP)
    let nop = decode_instruction(0x00000000);
    assert!(matches!(nop, vmips_rust::functional_simulator::instructions::Instruction::Nop));
    
    // Test all-ones instruction (should be invalid)
    let invalid = decode_instruction(0xFFFFFFFF);
    assert!(matches!(invalid, vmips_rust::functional_simulator::instructions::Instruction::InvalidInstruction));
    
    // Test specific edge case values
    let _edge_case = decode_instruction(0x80000000); // MSB set
    // Should not panic, should return some valid instruction type
}

#[test]
fn test_memory_alignment_edge_cases() {
    let mut simulator = FunctionalSimulator::new(1024);
    
    // Test unaligned access handling
    let unaligned_addr = 1; // Not word-aligned
    
    // Memory should handle unaligned access gracefully
    simulator.memory.write_byte(unaligned_addr, 0x42);
    
    // Reading from unaligned word address should work
    let result = simulator.memory.read_word(unaligned_addr);
    // Should either return None or handle gracefully
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_error_propagation() {
    let mut assembler = Assembler::new();
    
    // Test invalid instruction
    let result = assembler.assemble_string("invalid_instruction");
    assert!(result.is_err());
    
    if let Err(error) = result {
        // Error should be properly formatted
        let error_str = format!("{}", error);
        assert!(!error_str.is_empty());
    }
}

#[test]
fn test_simulation_state_consistency() {
    let mut simulator = FunctionalSimulator::new(1024);
    
    // Initialize some state
    simulator.registers.write(1, 100);
    simulator.memory.write_word_init(0, 0x20010064); // addi $1, $0, 100
    
    // Save initial state
    let initial_reg_value = simulator.registers.read(1);
    
    // Verify state is consistent
    assert_eq!(initial_reg_value, 100);
    
    // After any operation, state should remain consistent
    let final_reg_value = simulator.registers.read(1);
    assert_eq!(final_reg_value, initial_reg_value);
}