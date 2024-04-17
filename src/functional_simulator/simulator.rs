// simulator.rs
use super::registers::Registers;
use super::memory::Memory;
use super::instructions::Instruction;

pub enum Exception {
    InvalidInstruction,
    MemoryAccessViolation,
    // Add more exception types as needed
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
            let pc_offset = self.execute_instruction(instruction);
            match pc_offset {
                Some(offset) => {
                    self.pc = self.pc.wrapping_add(offset << 2);
                }
                None => {
                    if let Some(exception) = &self.exception {
                        // Handle the exception
                        match exception {
                            Exception::InvalidInstruction => {
                                println!("Invalid instruction encountered at PC: {}", self.pc);
                                break;
                            }
                            Exception::MemoryAccessViolation => {
                                println!("Memory access violation occurred at PC: {}", self.pc);
                                break;
                            }
                            // Handle more exception types as needed
                        }
                    } else {
                        self.pc += 4;
                    }
                }
            }
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        let instruction_word = self.memory.read_word(self.pc as usize);
        decode_instruction(instruction_word)
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Option<u32> {
        instruction.execute(&mut self.registers, &mut self.memory)
    }

    // Add system call handling and user mode/privileged mode functionality as needed
}

pub fn decode_instruction(instruction_word: u32) -> Instruction {
    // Decode the instruction based on the instruction format
    // and return the corresponding Instruction variant
    // Example decoding logic for different instruction formats:
    let opcode = instruction_word >> 26;
    match opcode {
        0 => {
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let rd = (instruction_word >> 11) & 0x1F;
            let funct = instruction_word & 0x3F;
            match funct {
                0x20 => Instruction::Add { rd, rs, rt },
                0x22 => Instruction::Sub { rd, rs, rt },
                0x24 => Instruction::And { rd, rs, rt },
                0x25 => Instruction::Or { rd, rs, rt },
                0x2A => Instruction::Slt { rd, rs, rt },
                // Add more R-type instructions as needed
                _ => panic!("Unsupported R-type instruction"),
            }
        }
        0x08 => {
            // I-type instruction (e.g., Addi)
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as i16;
            Instruction::Addi { rt, rs, imm }
        }
        0x23 => {
            // I-type instruction (e.g., Lw)
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        }
        0x2B => {
            // I-type instruction (e.g., Sw)
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        }
        0x04 => {
            // I-type instruction (e.g., Beq)
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Beq { rs, rt, offset }
        }
        0x02 => {
            // J-type instruction (e.g., J)
            let target = instruction_word & 0x03FFFFFF;
            Instruction::J { target }
        }
        // Decode more instruction formats as needed
        _ => panic!("Unsupported instruction format"),
    }
}