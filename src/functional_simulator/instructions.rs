// instructions.rs

use super::registers::Registers;
use super::memory::Memory;

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
            Instruction::Beq { offset, .. } => pc.wrapping_add(*offset as u32),
            Instruction::J { target } => *target,
            _ => 0,
        }
    }
}