// instructions.rs
use super::registers::Registers;
use super::memory::Memory;

pub enum Instruction {
    Add { rd: u32, rs: u32, rt: u32 },
    Sub { rd: u32, rs: u32, rt: u32 },
    And { rd: u32, rs: u32, rt: u32 },
    Or { rd: u32, rs: u32, rt: u32 },
    Slt { rd: u32, rs: u32, rt: u32 },
    Addi { rt: u32, rs: u32, imm: i16 },
    Lw { rt: u32, base: u32, offset: i16 },
    Sw { rt: u32, base: u32, offset: i16 },
    Beq { rs: u32, rt: u32, offset: i16 },
    J { target: u32 },
    // Add more instructions as needed
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
            Instruction::Addi { rt, rs, imm } => {
                let rs_value = registers.read(*rs);
                let result = rs_value.wrapping_add(*imm as u32);
                registers.write(*rt, result);
                None
            }
            Instruction::Lw { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                let value = memory.read_word(address as usize);
                registers.write(*rt, value);
                None
            }
            Instruction::Sw { rt, base, offset } => {
                let base_value = registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                let value = registers.read(*rt);
                memory.write_word(address as usize, value);
                None
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
            // Add more instruction execution logic as needed
        }
    }
}
