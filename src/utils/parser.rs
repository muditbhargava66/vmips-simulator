// parser.rs
use crate::functional_simulator::instructions::Instruction;

pub fn parse_instruction(instruction_str: &str) -> Instruction {
    // Parse the instruction string and return the corresponding Instruction variant
    // Example parsing logic:
    let parts: Vec<&str> = instruction_str.split_whitespace().collect();
    match parts[0] {
        "add" => {
            let rd = parse_register(parts[1]);
            let rs = parse_register(parts[2]);
            let rt = parse_register(parts[3]);
            Instruction::Add { rd, rs, rt }
        },
        "addi" => {
            let rt = parse_register(parts[1]);
            let rs = parse_register(parts[2]);
            let imm = parts[3].parse().unwrap();
            Instruction::Addi { rt, rs, imm }
        },
        // Parse more instructions as needed
        _ => panic!("Unsupported instruction: {}", instruction_str),
    }
}

fn parse_register(register_str: &str) -> u32 {
    // Parse the register string and return the corresponding register number
    // Example parsing logic:
    match register_str {
        "$zero" => 0,
        "$at" => 1,
        "$v0" => 2,
        // Parse more registers as needed
        _ => panic!("Unsupported register: {}", register_str),
    }
}
