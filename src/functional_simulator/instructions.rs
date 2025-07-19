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

// instructions.rs
//
// This file contains the instruction definitions and execution logic for the
// MIPS functional simulator. It defines the Instruction enum, which represents
// all supported MIPS instructions, and the execute method, which implements
// the behavior of each instruction.

use super::memory::Memory;
use super::registers::Registers;
use crate::utils::syscall::handle_syscall;

#[derive(Debug, Clone)]
pub enum Instruction {
    // Original R-type instructions
    Add { rd: u32, rs: u32, rt: u32 },
    Sub { rd: u32, rs: u32, rt: u32 },
    And { rd: u32, rs: u32, rt: u32 },
    Or { rd: u32, rs: u32, rt: u32 },
    Slt { rd: u32, rs: u32, rt: u32 },
    Sll { rd: u32, rt: u32, shamt: u32 },
    Srl { rd: u32, rt: u32, shamt: u32 },

    // Original I-type instructions
    Addi { rt: u32, rs: u32, imm: i16 },
    Lw { rt: u32, base: u32, offset: i16 },
    Sw { rt: u32, base: u32, offset: i16 },
    Beq { rs: u32, rt: u32, offset: i16 },

    // Original J-type instruction
    J { target: u32 },

    // Additional instruction variants previously implemented
    Lui { rt: u32, imm: u16 },
    Ori { rt: u32, rs: u32, imm: u16 },
    Mult { rs: u32, rt: u32 },
    Mflo { rd: u32 },
    Addiu { rt: u32, rs: u32, imm: i16 },
    Bne { rs: u32, rt: u32, offset: i16 },
    Jr { rs: u32 },

    // New R-type instructions
    Sra { rd: u32, rt: u32, shamt: u32 },
    Sllv { rd: u32, rt: u32, rs: u32 },
    Srlv { rd: u32, rt: u32, rs: u32 },
    Srav { rd: u32, rt: u32, rs: u32 },
    Div { rs: u32, rt: u32 },
    Divu { rs: u32, rt: u32 },
    Xor { rd: u32, rs: u32, rt: u32 },
    Nor { rd: u32, rs: u32, rt: u32 },
    Mfhi { rd: u32 },
    Mthi { rs: u32 },
    Mtlo { rs: u32 },

    // New I-type instructions
    Andi { rt: u32, rs: u32, imm: u16 },
    Xori { rt: u32, rs: u32, imm: u16 },
    Slti { rt: u32, rs: u32, imm: i16 },
    Sltiu { rt: u32, rs: u32, imm: i16 },
    Lb { rt: u32, base: u32, offset: i16 },
    Lh { rt: u32, base: u32, offset: i16 },
    Lbu { rt: u32, base: u32, offset: i16 },
    Lhu { rt: u32, base: u32, offset: i16 },
    Sb { rt: u32, base: u32, offset: i16 },
    Sh { rt: u32, base: u32, offset: i16 },

    // Branch instructions
    Bgtz { rs: u32, offset: i16 },
    Blez { rs: u32, offset: i16 },
    Bltz { rs: u32, offset: i16 },
    Bgez { rs: u32, offset: i16 },

    // Jump instruction variants
    Jal { target: u32 },
    Jalr { rd: u32, rs: u32 },

    // Floating-point instructions
    AddS { fd: u32, fs: u32, ft: u32 },
    SubS { fd: u32, fs: u32, ft: u32 },
    MulS { fd: u32, fs: u32, ft: u32 },
    DivS { fd: u32, fs: u32, ft: u32 },
    AbsS { fd: u32, fs: u32 },
    NegS { fd: u32, fs: u32 },
    MovS { fd: u32, fs: u32 },
    CvtSW { fd: u32, fs: u32 },
    CvtWS { fd: u32, fs: u32 },
    CmpS { fs: u32, ft: u32, cond: u32 },
    LwC1 { ft: u32, base: u32, offset: i16 },
    SwC1 { ft: u32, base: u32, offset: i16 },
    BC1T { offset: i16 },
    BC1F { offset: i16 },

    // Special instructions
    Syscall,
    Break { code: u32 },
    Nop,

    InvalidInstruction,
}

impl Instruction {
    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) -> Option<u32> {
        match self {
            // Original R-type instructions
            Instruction::Add { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value.wrapping_add(rt_value);
                registers.write(*rd, result);
                None
            },
            Instruction::Sub { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value.wrapping_sub(rt_value);
                registers.write(*rd, result);
                None
            },
            Instruction::And { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value & rt_value;
                registers.write(*rd, result);
                None
            },
            Instruction::Or { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value | rt_value;
                registers.write(*rd, result);
                None
            },
            Instruction::Slt { rd, rs, rt } => {
                let rs_value = registers.read(*rs) as i32;
                let rt_value = registers.read(*rt) as i32;
                let result = (rs_value < rt_value) as u32;
                registers.write(*rd, result);
                None
            },
            Instruction::Sll { rd, rt, shamt } => {
                let rt_value = registers.read(*rt);
                let result = rt_value << shamt;
                registers.write(*rd, result);
                None
            },
            Instruction::Srl { rd, rt, shamt } => {
                let rt_value = registers.read(*rt);
                let result = rt_value >> shamt;
                registers.write(*rd, result);
                None
            },

            // Original I-type instructions
            Instruction::Addi { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value.wrapping_add(*imm as u32);
                registers.write(*rt, result);
                None
            },
            Instruction::Lw { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);

                // Check alignment - MIPS requires word accesses to be aligned
                if address % 4 != 0 {
                    println!(
                        "Memory alignment exception: address 0x{:08x} not aligned for word access",
                        address
                    );
                    return Some(address); // Return address causing the exception
                }

                match memory.read_word(address as usize) {
                    Some(value) => {
                        registers.write(*rt, value);
                        None
                    },
                    None => {
                        println!(
                            "Memory access exception: address 0x{:08x} out of bounds",
                            address
                        );
                        Some(address)
                    },
                }
            },
            Instruction::Sw { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);

                // Check alignment - MIPS requires word accesses to be aligned
                if address % 4 != 0 {
                    println!(
                        "Memory alignment exception: address 0x{:08x} not aligned for word access",
                        address
                    );
                    return Some(address); // Return address causing the exception
                }

                let value = registers.read(*rt);
                if memory.write_word(address as usize, value) {
                    None
                } else {
                    println!(
                        "Memory access exception: address 0x{:08x} out of bounds",
                        address
                    );
                    Some(address)
                }
            },
            Instruction::Beq { rs, rt, offset } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);

                if rs_value == rt_value {
                    println!(
                        "BEQ: (${} == ${}): {} == {} - Branch taken",
                        rs, rt, rs_value, rt_value
                    );
                    Some((*offset as u32) << 2)
                } else {
                    println!(
                        "BEQ: (${} == ${}): {} != {} - Branch NOT taken",
                        rs, rt, rs_value, rt_value
                    );
                    None
                }
            },

            // Original J-type instruction
            Instruction::J { target } => Some(*target),

            // Previously implemented instructions
            Instruction::Lui { rt, imm } => {
                let value = (*imm as u32) << 16;
                registers.write(*rt, value);
                None
            },
            Instruction::Ori { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value | (*imm as u32);
                registers.write(*rt, result);
                None
            },
            Instruction::Mult { rs, rt } => {
                let rs_value = registers.read(*rs) as i32;
                let rt_value = registers.read(*rt) as i32;
                let result = rs_value.wrapping_mul(rt_value) as i64;

                registers.set_lo((result & 0xFFFFFFFF) as u32);
                registers.set_hi(((result >> 32) & 0xFFFFFFFF) as u32);
                None
            },
            Instruction::Mflo { rd } => {
                let lo_value = registers.get_lo();
                registers.write(*rd, lo_value);
                None
            },
            Instruction::Addiu { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value.wrapping_add(*imm as u32);
                registers.write(*rt, result);
                None
            },
            Instruction::Bne { rs, rt, offset } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                if rs_value != rt_value {
                    Some((*offset as u32) << 2)
                } else {
                    None
                }
            },
            Instruction::Jr { rs } => {
                let target_address = registers.read(*rs);
                Some(target_address)
            },

            // New R-type instructions implementations
            Instruction::Sra { rd, rt, shamt } => {
                let rt_value = registers.read(*rt) as i32;
                let result = rt_value >> shamt;
                registers.write(*rd, result as u32);
                None
            },
            Instruction::Sllv { rd, rt, rs } => {
                let rt_value = registers.read(*rt);
                let rs_value = registers.read(*rs) & 0x1F; // Only use lower 5 bits of rs
                let result = rt_value << rs_value;
                registers.write(*rd, result);
                None
            },
            Instruction::Srlv { rd, rt, rs } => {
                let rt_value = registers.read(*rt);
                let rs_value = registers.read(*rs) & 0x1F; // Only use lower 5 bits of rs
                let result = rt_value >> rs_value;
                registers.write(*rd, result);
                None
            },
            Instruction::Srav { rd, rt, rs } => {
                let rt_value = registers.read(*rt) as i32;
                let rs_value = registers.read(*rs) & 0x1F; // Only use lower 5 bits of rs
                let result = rt_value >> rs_value;
                registers.write(*rd, result as u32);
                None
            },
            Instruction::Div { rs, rt } => {
                let rs_value = registers.read(*rs) as i32;
                let rt_value = registers.read(*rt) as i32;

                // Check for division by zero
                if rt_value == 0 {
                    // In hardware this would be undefined behavior
                    // For simulator, set LO and HI to 0
                    registers.set_lo(0);
                    registers.set_hi(0);
                } else {
                    let quotient = rs_value / rt_value;
                    let remainder = rs_value % rt_value;
                    registers.set_lo(quotient as u32);
                    registers.set_hi(remainder as u32);
                }
                None
            },
            Instruction::Divu { rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);

                // Check for division by zero
                if rt_value == 0 {
                    registers.set_lo(0);
                    registers.set_hi(0);
                } else {
                    let quotient = rs_value / rt_value;
                    let remainder = rs_value % rt_value;
                    registers.set_lo(quotient);
                    registers.set_hi(remainder);
                }
                None
            },
            Instruction::Xor { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value ^ rt_value;
                registers.write(*rd, result);
                None
            },
            Instruction::Nor { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = !(rs_value | rt_value);
                registers.write(*rd, result);
                None
            },
            Instruction::Mfhi { rd } => {
                let hi_value = registers.get_hi();
                registers.write(*rd, hi_value);
                None
            },
            Instruction::Mthi { rs } => {
                let rs_value = registers.read(*rs);
                registers.set_hi(rs_value);
                None
            },
            Instruction::Mtlo { rs } => {
                let rs_value = registers.read(*rs);
                registers.set_lo(rs_value);
                None
            },

            // New I-type instruction implementations
            Instruction::Andi { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value & (*imm as u32);
                registers.write(*rt, result);
                None
            },
            Instruction::Xori { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value ^ (*imm as u32);
                registers.write(*rt, result);
                None
            },
            Instruction::Slti { rt, rs, imm } => {
                let rs_value = registers.read(*rs) as i32;
                let imm_value = *imm as i32;
                let result = (rs_value < imm_value) as u32;
                registers.write(*rt, result);
                None
            },
            Instruction::Sltiu { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let imm_value = *imm as u32;
                let result = (rs_value < imm_value) as u32;
                registers.write(*rt, result);
                None
            },
            Instruction::Lb { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                match memory.read_byte(address as usize) {
                    Some(value) => {
                        // Sign extend
                        let sign_extended = ((value as i8) as i32) as u32;
                        registers.write(*rt, sign_extended);
                        None
                    },
                    None => Some(address),
                }
            },
            Instruction::Lbu { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                match memory.read_byte(address as usize) {
                    Some(value) => {
                        // Zero extend
                        registers.write(*rt, value as u32);
                        None
                    },
                    None => Some(address),
                }
            },
            Instruction::Lh { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);

                // Check alignment - MIPS requires halfword accesses to be aligned
                if address % 2 != 0 {
                    println!("Memory alignment exception: address 0x{:08x} not aligned for halfword access", address);
                    return Some(address); // Return address causing the exception
                }

                if address as usize + 1 < memory.size {
                    let low_byte = memory.read_byte(address as usize).unwrap_or(0);
                    let high_byte = memory.read_byte((address as usize) + 1).unwrap_or(0);
                    let halfword = ((high_byte as u16) << 8) | (low_byte as u16);
                    // Sign extend
                    let sign_extended = ((halfword as i16) as i32) as u32;
                    registers.write(*rt, sign_extended);
                    None
                } else {
                    println!(
                        "Memory access exception: address 0x{:08x} out of bounds",
                        address
                    );
                    Some(address)
                }
            },
            Instruction::Lhu { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);

                // Check alignment - MIPS requires halfword accesses to be aligned
                if address % 2 != 0 {
                    println!("Memory alignment exception: address 0x{:08x} not aligned for halfword access", address);
                    return Some(address); // Return address causing the exception
                }

                if address as usize + 1 < memory.size {
                    let low_byte = memory.read_byte(address as usize).unwrap_or(0);
                    let high_byte = memory.read_byte((address as usize) + 1).unwrap_or(0);
                    let halfword = ((high_byte as u16) << 8) | (low_byte as u16);
                    // Zero extend
                    registers.write(*rt, halfword as u32);
                    None
                } else {
                    println!(
                        "Memory access exception: address 0x{:08x} out of bounds",
                        address
                    );
                    Some(address)
                }
            },
            Instruction::Sb { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                let value = registers.read(*rt) as u8;
                if memory.write_byte(address as usize, value) {
                    None
                } else {
                    println!(
                        "Memory access exception: address 0x{:08x} out of bounds",
                        address
                    );
                    Some(address)
                }
            },
            Instruction::Sh { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);

                // Check alignment - MIPS requires halfword accesses to be aligned
                if address % 2 != 0 {
                    println!("Memory alignment exception: address 0x{:08x} not aligned for halfword access", address);
                    return Some(address); // Return address causing the exception
                }

                let value = registers.read(*rt) as u16;
                if address as usize + 1 < memory.size {
                    let low_byte = value as u8;
                    let high_byte = (value >> 8) as u8;
                    memory.write_byte(address as usize, low_byte);
                    memory.write_byte((address as usize) + 1, high_byte);
                    None
                } else {
                    Some(address)
                }
            },

            // Branch instructions
            Instruction::Bgtz { rs, offset } => {
                let rs_value = registers.read(*rs) as i32;
                if rs_value > 0 {
                    Some(*offset as u32)
                } else {
                    None
                }
            },
            Instruction::Blez { rs, offset } => {
                let rs_value = registers.read(*rs) as i32;
                if rs_value <= 0 {
                    Some(*offset as u32)
                } else {
                    None
                }
            },
            Instruction::Bltz { rs, offset } => {
                let rs_value = registers.read(*rs) as i32;
                if rs_value < 0 {
                    Some(*offset as u32)
                } else {
                    None
                }
            },
            Instruction::Bgez { rs, offset } => {
                let rs_value = registers.read(*rs) as i32;
                if rs_value >= 0 {
                    Some(*offset as u32)
                } else {
                    None
                }
            },

            // Jump instruction variants
            Instruction::Jal { target } => {
                // Store return address in $ra (register 31)
                registers.write(31, registers.pc + 4);
                Some(*target)
            },
            Instruction::Jalr { rd, rs } => {
                // Store return address in rd
                registers.write(*rd, registers.pc + 4);
                // Jump to address in rs
                Some(registers.read(*rs))
            },

            // Floating-point instructions
            Instruction::AddS { fd, fs, ft } => {
                let fs_value = registers.read_float(*fs);
                let ft_value = registers.read_float(*ft);
                let result = fs_value + ft_value;
                registers.write_float(*fd, result);
                None
            },
            Instruction::SubS { fd, fs, ft } => {
                let fs_value = registers.read_float(*fs);
                let ft_value = registers.read_float(*ft);
                let result = fs_value - ft_value;
                registers.write_float(*fd, result);
                None
            },
            Instruction::MulS { fd, fs, ft } => {
                let fs_value = registers.read_float(*fs);
                let ft_value = registers.read_float(*ft);
                let result = fs_value * ft_value;
                registers.write_float(*fd, result);
                None
            },
            Instruction::DivS { fd, fs, ft } => {
                let fs_value = registers.read_float(*fs);
                let ft_value = registers.read_float(*ft);
                if ft_value != 0.0 {
                    let result = fs_value / ft_value;
                    registers.write_float(*fd, result);
                } else {
                    // Division by zero - set to infinity or NaN depending on numerator
                    if fs_value == 0.0 {
                        registers.write_float(*fd, f32::NAN);
                    } else if fs_value > 0.0 {
                        registers.write_float(*fd, f32::INFINITY);
                    } else {
                        registers.write_float(*fd, f32::NEG_INFINITY);
                    }
                    // Set appropriate flags in FCSR
                    registers.fcsr |= 0x8; // Division by zero flag
                }
                None
            },
            Instruction::AbsS { fd, fs } => {
                let fs_value = registers.read_float(*fs);
                let result = fs_value.abs();
                registers.write_float(*fd, result);
                None
            },
            Instruction::NegS { fd, fs } => {
                let fs_value = registers.read_float(*fs);
                let result = -fs_value;
                registers.write_float(*fd, result);
                None
            },
            Instruction::MovS { fd, fs } => {
                let fs_value = registers.read_float(*fs);
                registers.write_float(*fd, fs_value);
                None
            },
            Instruction::CvtSW { fd, fs } => {
                let fs_value = registers.read(*fs) as i32 as f32;
                registers.write_float(*fd, fs_value);
                None
            },
            Instruction::CvtWS { fd, fs } => {
                let fs_value = registers.read_float(*fs);
                let result = fs_value as i32 as u32;
                registers.write(*fd, result);
                None
            },
            Instruction::CmpS { fs, ft, cond } => {
                let fs_value = registers.read_float(*fs);
                let ft_value = registers.read_float(*ft);
                let mut condition_bit = false;

                match cond {
                    0 => condition_bit = fs_value == ft_value, // EQ
                    1 => condition_bit = fs_value < ft_value,  // LT
                    2 => condition_bit = fs_value <= ft_value, // LE
                    // Add other condition codes as needed
                    _ => {},
                }

                // Set condition flag in fcsr
                if condition_bit {
                    registers.fcsr |= 0x800000; // Set condition bit
                } else {
                    registers.fcsr &= !0x800000; // Clear condition bit
                }
                None
            },
            Instruction::LwC1 { ft, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                match memory.read_word(address as usize) {
                    Some(value) => {
                        // Convert raw bits to float
                        let float_value = f32::from_bits(value);
                        registers.write_float(*ft, float_value);
                        None
                    },
                    None => Some(address),
                }
            },
            Instruction::SwC1 { ft, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                let value = registers.read_float(*ft).to_bits();
                if memory.write_word(address as usize, value) {
                    None
                } else {
                    Some(address)
                }
            },
            Instruction::BC1T { offset } => {
                // Branch if FP condition flag is true (bit 23 of fcsr)
                if (registers.fcsr & 0x800000) != 0 {
                    Some(*offset as u32)
                } else {
                    None
                }
            },
            Instruction::BC1F { offset } => {
                // Branch if FP condition flag is false (bit 23 of fcsr)
                if (registers.fcsr & 0x800000) == 0 {
                    Some(*offset as u32)
                } else {
                    None
                }
            },

            // Special instructions
            Instruction::Syscall => handle_syscall(registers, memory),
            Instruction::Break { code: _ } => {
                // Normally would trigger debugger, but for our simulator we'll just print a message
                println!("Breakpoint encountered at PC: 0x{:08X}", registers.pc);
                None
            },
            Instruction::Nop => None,

            Instruction::InvalidInstruction => None,
        }
    }

    pub fn get_address(&self, registers: &Registers, pc: u32) -> u32 {
        match self {
            Instruction::Lw { base, offset, .. }
            | Instruction::Sw { base, offset, .. }
            | Instruction::Lb { base, offset, .. }
            | Instruction::Lh { base, offset, .. }
            | Instruction::Lbu { base, offset, .. }
            | Instruction::Lhu { base, offset, .. }
            | Instruction::Sb { base, offset, .. }
            | Instruction::Sh { base, offset, .. }
            | Instruction::LwC1 { base, offset, .. }
            | Instruction::SwC1 { base, offset, .. } => {
                let base_value = registers.read(*base);
                base_value.wrapping_add(*offset as u32)
            },
            Instruction::Beq { offset, .. }
            | Instruction::Bne { offset, .. }
            | Instruction::Bgtz { offset, .. }
            | Instruction::Blez { offset, .. }
            | Instruction::Bltz { offset, .. }
            | Instruction::Bgez { offset, .. }
            | Instruction::BC1T { offset }
            | Instruction::BC1F { offset } => {
                // PC-relative addressing: PC + 4 + (offset << 2)
                pc.wrapping_add(4).wrapping_add((*offset as u32) << 2)
            },
            Instruction::J { target } | Instruction::Jal { target } => {
                // Jump format: (PC & 0xF0000000) | (target << 2)
                (pc & 0xF0000000) | (*target << 2)
            },
            Instruction::Jr { rs } | Instruction::Jalr { rs, .. } => {
                // Jump register: address in rs
                registers.read(*rs)
            },
            _ => 0, // Other instructions don't directly access memory
        }
    }

    pub fn generates_result(&self) -> bool {
        matches!(
            self,
            Instruction::Add { .. }
                | Instruction::Sub { .. }
                | Instruction::And { .. }
                | Instruction::Or { .. }
                | Instruction::Xor { .. }
                | Instruction::Nor { .. }
                | Instruction::Slt { .. }
                | Instruction::Slti { .. }
                | Instruction::Sltiu { .. }
                | Instruction::Sll { .. }
                | Instruction::Srl { .. }
                | Instruction::Sra { .. }
                | Instruction::Sllv { .. }
                | Instruction::Srlv { .. }
                | Instruction::Srav { .. }
                | Instruction::Addi { .. }
                | Instruction::Addiu { .. }
                | Instruction::Lui { .. }
                | Instruction::Ori { .. }
                | Instruction::Andi { .. }
                | Instruction::Xori { .. }
                | Instruction::Lw { .. }
                | Instruction::Lb { .. }
                | Instruction::Lbu { .. }
                | Instruction::Lh { .. }
                | Instruction::Lhu { .. }
                | Instruction::Mflo { .. }
                | Instruction::Mfhi { .. }
                | Instruction::LwC1 { .. }
                | Instruction::AddS { .. }
                | Instruction::SubS { .. }
                | Instruction::MulS { .. }
                | Instruction::DivS { .. }
                | Instruction::AbsS { .. }
                | Instruction::NegS { .. }
                | Instruction::MovS { .. }
                | Instruction::CvtSW { .. }
                | Instruction::CvtWS { .. }
        )
    }

    pub fn get_destination_register(&self) -> Option<u32> {
        match self {
            Instruction::Add { rd, .. }
            | Instruction::Sub { rd, .. }
            | Instruction::And { rd, .. }
            | Instruction::Or { rd, .. }
            | Instruction::Xor { rd, .. }
            | Instruction::Nor { rd, .. }
            | Instruction::Slt { rd, .. }
            | Instruction::Sll { rd, .. }
            | Instruction::Srl { rd, .. }
            | Instruction::Sra { rd, .. }
            | Instruction::Sllv { rd, .. }
            | Instruction::Srlv { rd, .. }
            | Instruction::Srav { rd, .. }
            | Instruction::Mflo { rd }
            | Instruction::Mfhi { rd }
            | Instruction::Jalr { rd, .. } => Some(*rd),

            Instruction::Addi { rt, .. }
            | Instruction::Addiu { rt, .. }
            | Instruction::Slti { rt, .. }
            | Instruction::Sltiu { rt, .. }
            | Instruction::Andi { rt, .. }
            | Instruction::Ori { rt, .. }
            | Instruction::Xori { rt, .. }
            | Instruction::Lui { rt, .. }
            | Instruction::Lw { rt, .. }
            | Instruction::Lb { rt, .. }
            | Instruction::Lbu { rt, .. }
            | Instruction::Lh { rt, .. }
            | Instruction::Lhu { rt, .. } => Some(*rt),

            Instruction::Jal { .. } => Some(31), // $ra

            // FP instructions
            Instruction::AddS { fd, .. }
            | Instruction::SubS { fd, .. }
            | Instruction::MulS { fd, .. }
            | Instruction::DivS { fd, .. }
            | Instruction::AbsS { fd, .. }
            | Instruction::NegS { fd, .. }
            | Instruction::MovS { fd, .. }
            | Instruction::CvtSW { fd, .. }
            | Instruction::CvtWS { fd, .. } => Some(*fd),

            Instruction::LwC1 { ft, .. } => Some(*ft), // FP registers are accessed directly

            _ => None,
        }
    }

    pub fn get_source_registers(&self) -> Vec<u32> {
        match self {
            Instruction::Add { rs, rt, .. }
            | Instruction::Sub { rs, rt, .. }
            | Instruction::And { rs, rt, .. }
            | Instruction::Or { rs, rt, .. }
            | Instruction::Xor { rs, rt, .. }
            | Instruction::Nor { rs, rt, .. }
            | Instruction::Slt { rs, rt, .. } => vec![*rs, *rt],

            Instruction::Sll { rt, .. }
            | Instruction::Srl { rt, .. }
            | Instruction::Sra { rt, .. } => vec![*rt],

            Instruction::Sllv { rs, rt, .. }
            | Instruction::Srlv { rs, rt, .. }
            | Instruction::Srav { rs, rt, .. } => vec![*rs, *rt],

            Instruction::Addi { rs, .. }
            | Instruction::Addiu { rs, .. }
            | Instruction::Slti { rs, .. }
            | Instruction::Sltiu { rs, .. }
            | Instruction::Andi { rs, .. }
            | Instruction::Ori { rs, .. }
            | Instruction::Xori { rs, .. } => vec![*rs],

            Instruction::Lw { base, .. }
            | Instruction::Lb { base, .. }
            | Instruction::Lbu { base, .. }
            | Instruction::Lh { base, .. }
            | Instruction::Lhu { base, .. }
            | Instruction::LwC1 { base, .. } => vec![*base],

            Instruction::Sw { rt, base, .. } => {
                if *base == 0 {
                    vec![*rt] // Special case for storing to absolute address
                } else {
                    vec![*rt, *base]
                }
            },
            Instruction::Sb { rt, base, .. } => {
                if *base == 0 {
                    vec![*rt] // Special case for storing to absolute address
                } else {
                    vec![*rt, *base]
                }
            },
            Instruction::Sh { rt, base, .. } => {
                if *base == 0 {
                    vec![*rt] // Special case for storing to absolute address
                } else {
                    vec![*rt, *base]
                }
            },
            Instruction::SwC1 { ft, base, .. } => {
                if *base == 0 {
                    vec![*ft] // Special case for storing to absolute address
                } else {
                    vec![*ft, *base]
                }
            },

            Instruction::Beq { rs, rt, .. } | Instruction::Bne { rs, rt, .. } => vec![*rs, *rt],

            Instruction::Bgtz { rs, .. }
            | Instruction::Blez { rs, .. }
            | Instruction::Bltz { rs, .. }
            | Instruction::Bgez { rs, .. } => vec![*rs],

            Instruction::Jr { rs }
            | Instruction::Jalr { rs, .. }
            | Instruction::Mthi { rs }
            | Instruction::Mtlo { rs } => vec![*rs],

            Instruction::Mult { rs, rt }
            | Instruction::Div { rs, rt }
            | Instruction::Divu { rs, rt } => vec![*rs, *rt],

            // FP instructions
            Instruction::AddS { fs, ft, .. }
            | Instruction::SubS { fs, ft, .. }
            | Instruction::MulS { fs, ft, .. }
            | Instruction::DivS { fs, ft, .. }
            | Instruction::CmpS { fs, ft, .. } => vec![*fs, *ft], // FP registers are accessed directly

            Instruction::AbsS { fs, .. }
            | Instruction::NegS { fs, .. }
            | Instruction::MovS { fs, .. }
            | Instruction::CvtSW { fs, .. } => vec![*fs],

            Instruction::CvtWS { fs, .. } => vec![*fs],

            Instruction::Syscall => {
                // Syscall uses multiple registers for parameters
                vec![2, 4, 5, 6, 7]
            },

            _ => vec![],
        }
    }

    pub fn is_branch_or_jump(&self) -> bool {
        matches!(
            self,
            Instruction::Beq { .. }
                | Instruction::Bne { .. }
                | Instruction::Bgtz { .. }
                | Instruction::Blez { .. }
                | Instruction::Bltz { .. }
                | Instruction::Bgez { .. }
                | Instruction::J { .. }
                | Instruction::Jal { .. }
                | Instruction::Jr { .. }
                | Instruction::Jalr { .. }
                | Instruction::BC1T { .. }
                | Instruction::BC1F { .. }
        )
    }

    pub fn is_memory_access(&self) -> bool {
        matches!(
            self,
            Instruction::Lw { .. }
                | Instruction::Sw { .. }
                | Instruction::Lb { .. }
                | Instruction::Lbu { .. }
                | Instruction::Lh { .. }
                | Instruction::Lhu { .. }
                | Instruction::Sb { .. }
                | Instruction::Sh { .. }
                | Instruction::LwC1 { .. }
                | Instruction::SwC1 { .. }
        )
    }

    pub fn is_load(&self) -> bool {
        matches!(
            self,
            Instruction::Lw { .. }
                | Instruction::Lb { .. }
                | Instruction::Lbu { .. }
                | Instruction::Lh { .. }
                | Instruction::Lhu { .. }
                | Instruction::LwC1 { .. }
        )
    }

    pub fn is_store(&self) -> bool {
        matches!(
            self,
            Instruction::Sw { .. }
                | Instruction::Sb { .. }
                | Instruction::Sh { .. }
                | Instruction::SwC1 { .. }
        )
    }

    pub fn is_fp_instruction(&self) -> bool {
        matches!(
            self,
            Instruction::AddS { .. }
                | Instruction::SubS { .. }
                | Instruction::MulS { .. }
                | Instruction::DivS { .. }
                | Instruction::AbsS { .. }
                | Instruction::NegS { .. }
                | Instruction::MovS { .. }
                | Instruction::CvtSW { .. }
                | Instruction::CvtWS { .. }
                | Instruction::CmpS { .. }
                | Instruction::LwC1 { .. }
                | Instruction::SwC1 { .. }
                | Instruction::BC1T { .. }
                | Instruction::BC1F { .. }
        )
    }

    /// Gets immediate target offset for branch and jump instructions
    /// For branch instructions, returns the offset that should be added to PC+4
    /// For jump instructions, returns the target address (shifted)
    /// Returns None for register-based jumps (like JR, JALR) or non-branch/jump instructions
    pub fn get_immediate_target(&self) -> Option<u32> {
        match self {
            Instruction::J { target } | Instruction::Jal { target } => {
                // J-type instructions: target << 2 (target is the 26-bit address)
                // This is an absolute address (combined with upper PC bits in actual execution)
                Some(*target << 2)
            },
            Instruction::Beq { offset, .. }
            | Instruction::Bne { offset, .. }
            | Instruction::Bgtz { offset, .. }
            | Instruction::Blez { offset, .. }
            | Instruction::Bltz { offset, .. }
            | Instruction::Bgez { offset, .. }
            | Instruction::BC1T { offset }
            | Instruction::BC1F { offset } => {
                // PC-relative branches: returns offset to be added to PC+4
                // The caller is responsible for adding this to the appropriate PC value
                Some((*offset as u32) << 2)
            },
            _ => None,
        }
    }

    /// Calculates the actual branch target address given the current PC
    /// This is a helper function that properly combines PC with branch offsets
    pub fn calculate_branch_target(&self, current_pc: u32) -> Option<u32> {
        match self {
            Instruction::J { target } | Instruction::Jal { target } => {
                // J-type instructions: (PC+4 & 0xF0000000) | (target << 2)
                let pc_plus_4 = current_pc.wrapping_add(4);
                Some((pc_plus_4 & 0xF000_0000) | ((*target & 0x03FF_FFFF) << 2))
            },
            Instruction::Beq { offset, .. }
            | Instruction::Bne { offset, .. }
            | Instruction::Bgtz { offset, .. }
            | Instruction::Blez { offset, .. }
            | Instruction::Bltz { offset, .. }
            | Instruction::Bgez { offset, .. }
            | Instruction::BC1T { offset }
            | Instruction::BC1F { offset } => {
                // PC-relative branches: PC + 4 + (offset << 2)
                let pc_plus_4 = current_pc.wrapping_add(4);
                Some(pc_plus_4.wrapping_add(((*offset as i32) << 2) as u32))
            },
            _ => None,
        }
    }
}
