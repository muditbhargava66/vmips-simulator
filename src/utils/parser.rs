// Copyright (c) 2024 Mudit Bhargava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// parser.rs
//
// This file contains a simple parser for MIPS assembly instructions.
// It provides functions to parse instruction strings and register names.

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
