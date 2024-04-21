// simulator.rs
use super::registers::Registers;
use super::memory::Memory;
use super::instructions::Instruction;

pub enum Exception {
    InvalidInstruction,
    MemoryAccessViolation,
}

pub struct Simulator {
    pub registers: Registers,
    pub memory: Memory,
    pc: u32,
    pub exception: Option<Exception>,
}

impl Simulator {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(memory_size),
            pc: 0,
            exception: None,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.data[..program.len()].copy_from_slice(program);
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch_instruction();
            match instruction {
                Instruction::InvalidInstruction => {
                    println!("Invalid instruction encountered at PC: 0x{:08X}", self.pc);
                    break;
                }
                _ => {
                    let pc_offset = self.execute_instruction(instruction);
                    match pc_offset {
                        Some(offset) => {
                            let new_pc = self.pc.wrapping_add(offset << 2);
                            if new_pc < self.memory.size as u32 {
                                self.pc = new_pc;
                            } else {
                                println!("Invalid jump target: 0x{:08X}", new_pc);
                                break;
                            }
                        }
                        None => {
                            if let Some(exception) = &self.exception {
                                match exception {
                                    Exception::InvalidInstruction => {
                                        println!("Invalid instruction encountered at PC: 0x{:08X}", self.pc);
                                    }
                                    Exception::MemoryAccessViolation => {
                                        println!("Memory access violation occurred at PC: 0x{:08X}", self.pc);
                                    }
                                }
                                break;
                            } else {
                                self.pc = self.pc.wrapping_add(4);
                            }
                        }
                    }
                }
            }
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        match self.memory.read_word(self.pc as usize) {
            Some(instruction_word) => decode_instruction(instruction_word),
            None => Instruction::InvalidInstruction,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Option<u32> {
        instruction.execute(&mut self.registers, &mut self.memory)
    }
}

pub fn decode_instruction(instruction_word: u32) -> Instruction {
    let opcode = instruction_word >> 26;
    match opcode {
        0 => {
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let rd = (instruction_word >> 11) & 0x1F;
            let funct = instruction_word & 0x3F;
            match funct {
                0x00 => Instruction::Sll { rd, rt, shamt: (instruction_word >> 6) & 0x1F },
                0x02 => Instruction::Srl { rd, rt, shamt: (instruction_word >> 6) & 0x1F },
                0x20 => Instruction::Add { rd, rs, rt },
                0x22 => Instruction::Sub { rd, rs, rt },
                0x24 => Instruction::And { rd, rs, rt },
                0x25 => Instruction::Or { rd, rs, rt },
                0x2A => Instruction::Slt { rd, rs, rt },
                _ => Instruction::InvalidInstruction,
            }
        }
        0x08 => {
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as i16;
            Instruction::Addi { rt, rs, imm }
        }
        0x23 => {
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        }
        0x2B => {
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        }
        0x04 => {
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Beq { rs, rt, offset }
        }
        0x02 => {
            let target = instruction_word & 0x03FFFFFF;
            Instruction::J { target }
        }
        _ => Instruction::InvalidInstruction,
    }
}