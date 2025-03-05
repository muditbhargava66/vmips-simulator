// instructions.rs

use super::registers::Registers;
use super::memory::Memory;

#[derive(Debug)]
pub enum Instruction {
    Add { rd: u32, rs: u32, rt: u32 },
    Sub { rd: u32, rs: u32, rt: u32 },
    And { rd: u32, rs: u32, rt: u32 },
    Or { rd: u32, rs: u32, rt: u32 },
    Slt { rd: u32, rs: u32, rt: u32 },
    Sll { rd: u32, rt: u32, shamt: u32 },
    Srl { rd: u32, rt: u32, shamt: u32 },
    Addi { rt: u32, rs: u32, imm: i16 },
    Lw { rt: u32, base: u32, offset: i16 },
    Sw { rt: u32, base: u32, offset: i16 },
    Beq { rs: u32, rt: u32, offset: i16 },
    J { target: u32 },
    // Additional instruction variants
    Lui { rt: u32, imm: u16 },        // Load Upper Immediate
    Ori { rt: u32, rs: u32, imm: u16 }, // OR Immediate
    Mult { rs: u32, rt: u32 },         // Multiply
    Mflo { rd: u32 },                  // Move from LO register
    Addiu { rt: u32, rs: u32, imm: i16 }, // Add Immediate Unsigned
    Bne { rs: u32, rt: u32, offset: i16 }, // Branch if Not Equal
    Jr { rs: u32 },
    InvalidInstruction,
}

impl Instruction {
    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) -> Option<u32> {
        match self {
            Instruction::Add { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value.wrapping_add(rt_value);
                registers.write(*rd, result);
                None
            }
            Instruction::Sub { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value.wrapping_sub(rt_value);
                registers.write(*rd, result);
                None
            }
            Instruction::And { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value & rt_value;
                registers.write(*rd, result);
                None
            }
            Instruction::Or { rd, rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value | rt_value;
                registers.write(*rd, result);
                None
            }
            Instruction::Slt { rd, rs, rt } => {
                let rs_value = registers.read(*rs) as i32;
                let rt_value = registers.read(*rt) as i32;
                let result = (rs_value < rt_value) as u32;
                registers.write(*rd, result);
                None
            }
            Instruction::Sll { rd, rt, shamt } => {
                let rt_value = registers.read(*rt);
                let result = rt_value << shamt;
                registers.write(*rd, result);
                None
            }
            Instruction::Srl { rd, rt, shamt } => {
                let rt_value = registers.read(*rt);
                let result = rt_value >> shamt;
                registers.write(*rd, result);
                None
            }
            Instruction::Addi { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value.wrapping_add(*imm as u32);
                registers.write(*rt, result);
                None
            }
            Instruction::Lw { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                match memory.read_word(address as usize) {
                    Some(value) => {
                        registers.write(*rt, value);
                        None
                    }
                    None => Some(address),
                }
            }
            Instruction::Sw { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                let value = registers.read(*rt);
                if memory.write_word(address as usize, value) {
                    None
                } else {
                    Some(address)
                }
            }
            Instruction::Beq { rs, rt, offset } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                if rs_value == rt_value {
                    Some(*offset as u32)
                } else {
                    None
                }
            }
            Instruction::J { target } => Some(*target),
            // New instruction implementations
            Instruction::Lui { rt, imm } => {
                let value = (*imm as u32) << 16;
                registers.write(*rt, value);
                None
            }
            Instruction::Ori { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value | (*imm as u32);
                registers.write(*rt, result);
                None
            }
            Instruction::Mult { rs, rt } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                let result = rs_value.wrapping_mul(rt_value);
                
                // Ensure we have space for LO register
                if registers.data.len() <= 32 {
                    registers.data.resize(33, 0);
                }
                registers.data[32] = result; // Store in LO register (index 32)
                None
            }
            Instruction::Mflo { rd } => {
                // Get value from LO register
                let lo_value = if registers.data.len() > 32 {
                    registers.data[32]
                } else {
                    0
                };
                registers.write(*rd, lo_value);
                None
            }
            Instruction::Addiu { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value.wrapping_add(*imm as u32);
                registers.write(*rt, result);
                None
            }
            Instruction::Bne { rs, rt, offset } => {
                let rs_value = registers.read(*rs);
                let rt_value = registers.read(*rt);
                if rs_value != rt_value {
                    Some(*offset as u32)
                } else {
                    None
                }
            }
            Instruction::Jr { rs } => {
                // Jump to the address stored in register rs
                let target_address = registers.read(*rs);
                println!("JR: Jumping to address in register ${}: 0x{:08X}", rs, target_address);
                Some(target_address >> 2) // Return the target address divided by 4
            }
            Instruction::InvalidInstruction => None,
        }
    }

    pub fn get_address(&self, registers: &Registers, pc: u32) -> u32 {
        match self {
            Instruction::Lw { base, offset, .. } |
            Instruction::Sw { base, offset, .. } => {
                let base_value = registers.read(*base);
                base_value.wrapping_add(*offset as u32)
            }
            Instruction::Beq { offset, .. } |
            Instruction::Bne { offset, .. } => {
                // PC-relative addressing for branch instructions
                // Note: In MIPS, the branch target is PC + 4 + (offset << 2)
                // But since we're incrementing PC by 4 elsewhere, we use:
                pc.wrapping_add((*offset as u32) << 2)
            }
            Instruction::J { target } => {
                // In MIPS, J-type instructions use the upper 4 bits of the current PC
                // combined with the 26-bit target shifted left by 2
                (pc & 0xF0000000) | (*target << 2)
            }
            Instruction::Jr { rs } => {
                // Get the target address from the register
                registers.read(*rs)
            }
            Instruction::Lui { .. } |
            Instruction::Ori { .. } |
            Instruction::Mflo { .. } => 0, // These don't directly access memory
            _ => 0,
        }
    }
}