// simulator.rs
use super::registers::Registers;
use super::memory::Memory;
use super::instructions::Instruction;
use std::collections::HashMap;

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
        // Load the program at memory address 0
        self.memory.data[..program.len()].copy_from_slice(program);
        
        // Reset PC to 0 to start execution from the beginning of the program
        self.pc = 0;
        
        // Debug output
        println!("Program loaded. Initial PC: 0x{:08X}", self.pc);
    }
    
    pub fn run(&mut self) {
        println!("Starting execution at PC: 0x{:08X}", self.pc);
        
        // Print first few instructions for debugging
        for offset in 0..5 {
            let addr = self.pc as usize + offset * 4;
            if addr < self.memory.size {
                if let Some(instruction_word) = self.memory.read_word(addr) {
                    println!("Instruction at 0x{:08X}: 0x{:08X}", addr, instruction_word);
                }
            }
        }
        
        // Add instruction counter and limit
        let max_instructions = 1000; // Prevent infinite loops
        let mut instruction_count = 0;
        
        // Track frequency of PC values to detect loops
        let mut pc_frequency: HashMap<u32, usize> = HashMap::new();
        
        loop {
            // Check instruction limit
            instruction_count += 1;
            if instruction_count > max_instructions {
                println!("Reached maximum instruction limit ({}). Possible infinite loop.", max_instructions);
                
                // Find the most common PC values (likely loop)
                let mut pc_vec: Vec<_> = pc_frequency.into_iter().collect();
                pc_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by frequency (descending)
                
                println!("Most frequent PC values (possible loop locations):");
                for (pc, count) in pc_vec.iter().take(5) {
                    println!("  PC 0x{:08X}: executed {} times", pc, count);
                    
                    // Print the instruction at this PC
                    if let Some(instruction_word) = self.memory.read_word(*pc as usize) {
                        println!("    Instruction: 0x{:08X}", instruction_word);
                    }
                }
                
                break;
            }
            
            // Record this PC in our frequency map
            *pc_frequency.entry(self.pc).or_insert(0) += 1;
            
            if instruction_count % 100 == 0 {
                println!("Executed {} instructions, current PC: 0x{:08X}", 
                         instruction_count, self.pc);
            }
            
            let instruction = self.fetch_instruction();
            
            // Check for special termination condition: all zeros at the end of the program
            if let Some(instruction_word) = self.memory.read_word(self.pc as usize) {
                if instruction_word == 0 && instruction_count > 10 {
                    // NOP at the end of the program - treat as termination
                    println!("Reached NOP instruction (0x00000000) at PC 0x{:08X} - terminating", self.pc);
                    break;
                }
            }
            
            match instruction {
                Instruction::InvalidInstruction => {
                    println!("Invalid instruction encountered at PC: 0x{:08X}", self.pc);
                    break;
                }
                _ => {
                    // Debug output for the current instruction
                    if pc_frequency.get(&self.pc).unwrap_or(&0) > &10 {
                        if let Some(instruction_word) = self.memory.read_word(self.pc as usize) {
                            println!("Frequently executed: PC 0x{:08X}, Instruction: 0x{:08X}", 
                                     self.pc, instruction_word);
                        }
                    }
                    
                    let pc_offset = self.execute_instruction(instruction);
                    match pc_offset {
                        Some(offset) => {
                            let new_pc = self.pc.wrapping_add(offset << 2);
                            
                            // Check for potential infinite loop (jumping to same address)
                            if new_pc == self.pc {
                                println!("Warning: Jump to same address detected (0x{:08X}). Breaking potential infinite loop.", new_pc);
                                self.pc = self.pc.wrapping_add(4); // Skip to next instruction
                            } else if new_pc < self.memory.size as u32 {
                                if pc_frequency.get(&self.pc).unwrap_or(&0) > &10 {
                                    println!("Jump/branch from 0x{:08X} to 0x{:08X}", self.pc, new_pc);
                                }
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
        
        println!("Simulation ended after executing {} instructions", instruction_count);
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
    // Check for special case: instruction word is 0
    if instruction_word == 0 {
        return Instruction::Add { rd: 0, rs: 0, rt: 0 }; // NOP implemented as add $0, $0, $0
    }

    let opcode = instruction_word >> 26;
    match opcode {
        0 => {
            // R-type instructions
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let rd = (instruction_word >> 11) & 0x1F;
            let shamt = (instruction_word >> 6) & 0x1F;
            let funct = instruction_word & 0x3F;
            
            match funct {
                0x00 => Instruction::Sll { rd, rt, shamt },
                0x02 => Instruction::Srl { rd, rt, shamt },
                0x03 => {
                    // SRA - Shift Right Arithmetic (handle as SRL for simplicity)
                    println!("Note: SRA instruction at 0x{:08X} treated as SRL", instruction_word);
                    Instruction::Srl { rd, rt, shamt }
                },
                0x04 => {
                    // SLLV - Shift Left Logical Variable (handle as SLL)
                    println!("Note: SLLV instruction at 0x{:08X} treated as SLL", instruction_word);
                    Instruction::Sll { rd, rt, shamt: rs & 0x1F } // Use rs as shift amount
                },
                0x06 => {
                    // SRLV - Shift Right Logical Variable (handle as SRL)
                    println!("Note: SRLV instruction at 0x{:08X} treated as SRL", instruction_word);
                    Instruction::Srl { rd, rt, shamt: rs & 0x1F } // Use rs as shift amount
                },
                0x07 => {
                    // SRAV - Shift Right Arithmetic Variable (handle as SRL)
                    println!("Note: SRAV instruction at 0x{:08X} treated as SRL", instruction_word);
                    Instruction::Srl { rd, rt, shamt: rs & 0x1F } // Use rs as shift amount
                },
                0x08 => {
                    // JR - Jump Register (needs special handling)
                    println!("Note: JR instruction at PC 0x{:08X}", instruction_word);
                    // Create a new instruction type specifically for JR
                    Instruction::Jr { rs }
                },
                0x09 => {
                    // JALR - Jump and Link Register (handle as J for simplicity)
                    println!("Note: JALR instruction at 0x{:08X} treated as J", instruction_word);
                    Instruction::J { target: rs } // Using rs as target
                },
                0x0A => {
                    // MOVZ - Move if Zero (treat as simple OR for now)
                    println!("Note: MOVZ instruction at 0x{:08X} treated as OR", instruction_word);
                    Instruction::Or { rd, rs, rt }
                },
                0x0B => {
                    // MOVN - Move if Not Zero (treat as simple OR for now)
                    println!("Note: MOVN instruction at 0x{:08X} treated as OR", instruction_word);
                    Instruction::Or { rd, rs, rt }
                },
                0x10 => {
                    // MFHI - Move From HI (similar to MFLO)
                    println!("Note: MFHI instruction at 0x{:08X} treated as MFLO", instruction_word);
                    Instruction::Mflo { rd }
                },
                0x1E => {
                    // DSRL - Doubleword Shift Right Logical (handle as SRL for simplicity)
                    println!("Note: DSRL instruction at 0x{:08X} treated as SRL", instruction_word);
                    Instruction::Srl { rd, rt, shamt }
                },
                0x12 => Instruction::Mflo { rd },
                0x14 => {
                    // DSLLV - Doubleword Shift Left Logical Variable (handle as SLL)
                    println!("Note: DSLLV instruction at 0x{:08X} treated as SLL", instruction_word);
                    Instruction::Sll { rd, rt, shamt: rs & 0x1F } // Use rs as shift amount
                },
                0x18 => Instruction::Mult { rs, rt },
                0x19 => {
                    // MULTU - Multiply Unsigned (same as MULT for our simulator)
                    Instruction::Mult { rs, rt }
                },
                0x1A => {
                    // DIV - Division (handle as MULT for simplicity)
                    println!("Note: DIV instruction at 0x{:08X} treated as MULT", instruction_word);
                    Instruction::Mult { rs, rt }
                },
                0x1B => {
                    // DIVU - Division Unsigned (handle as MULT for simplicity)
                    println!("Note: DIVU instruction at 0x{:08X} treated as MULT", instruction_word);
                    Instruction::Mult { rs, rt }
                },
                0x20 => Instruction::Add { rd, rs, rt },
                0x21 => {
                    // ADDU - Add Unsigned (same as ADD for our simulator)
                    Instruction::Add { rd, rs, rt }
                },
                0x22 => Instruction::Sub { rd, rs, rt },
                0x23 => {
                    // SUBU - Subtract Unsigned (same as SUB for our simulator)
                    Instruction::Sub { rd, rs, rt }
                },
                0x24 => Instruction::And { rd, rs, rt },
                0x25 => Instruction::Or { rd, rs, rt },
                0x26 => {
                    // XOR - Exclusive OR (handle as OR for simplicity)
                    println!("Note: XOR instruction at 0x{:08X} treated as OR", instruction_word);
                    Instruction::Or { rd, rs, rt }
                },
                0x27 => {
                    // NOR - Not OR (handle as OR for simplicity)
                    println!("Note: NOR instruction at 0x{:08X} treated as OR", instruction_word);
                    Instruction::Or { rd, rs, rt }
                },
                0x2A => Instruction::Slt { rd, rs, rt },
                0x2B => {
                    // SLTU - Set Less Than Unsigned (same as SLT for our simulator)
                    Instruction::Slt { rd, rs, rt }
                },
                _ => {
                    println!("Unrecognized function code: 0x{:02X} in instruction 0x{:08X}", 
                             funct, instruction_word);
                    Instruction::InvalidInstruction
                }
            }
        },
        // Rest of the opcode handling remains the same
        0x08 => {
            // ADDI
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as i16;
            Instruction::Addi { rt, rs, imm }
        },
        0x09 => {
            // ADDIU
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as i16;
            Instruction::Addiu { rt, rs, imm }
        },
        0x0A => {
            // SLTI - Set Less Than Immediate (handle as ADDI)
            println!("Note: SLTI instruction at 0x{:08X} treated as ADDI", instruction_word);
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as i16;
            Instruction::Addi { rt, rs, imm }
        },
        0x0B => {
            // SLTIU - Set Less Than Immediate Unsigned (handle as ADDI)
            println!("Note: SLTIU instruction at 0x{:08X} treated as ADDI", instruction_word);
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as i16;
            Instruction::Addi { rt, rs, imm }
        },
        0x0C => {
            // ANDI - AND Immediate (handle as ORI)
            println!("Note: ANDI instruction at 0x{:08X} treated as ORI", instruction_word);
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as u16;
            Instruction::Ori { rt, rs, imm }
        },
        0x0D => {
            // ORI
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as u16;
            Instruction::Ori { rt, rs, imm }
        },
        0x0E => {
            // XORI - XOR Immediate (handle as ORI)
            println!("Note: XORI instruction at 0x{:08X} treated as ORI", instruction_word);
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as u16;
            Instruction::Ori { rt, rs, imm }
        },
        0x0F => {
            // LUI
            let rt = (instruction_word >> 16) & 0x1F;
            let imm = (instruction_word & 0xFFFF) as u16;
            Instruction::Lui { rt, imm }
        },
        0x23 => {
            // LW
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        },
        0x20 => {
            // LB - Load Byte (handle as LW for simplicity)
            println!("Note: LB instruction at 0x{:08X} treated as LW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        },
        0x21 => {
            // LH - Load Halfword (handle as LW for simplicity)
            println!("Note: LH instruction at 0x{:08X} treated as LW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        },
        0x22 => {
            // LWL - Load Word Left (handle as LW for simplicity)
            println!("Note: LWL instruction at 0x{:08X} treated as LW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        },
        0x26 => {
            // LWR - Load Word Right (handle as LW for simplicity)
            println!("Note: LWR instruction at 0x{:08X} treated as LW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Lw { rt, base, offset }
        },
        0x2B => {
            // SW
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        },
        0x28 => {
            // SB - Store Byte (handle as SW for simplicity)
            println!("Note: SB instruction at 0x{:08X} treated as SW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        },
        0x29 => {
            // SH - Store Halfword (handle as SW for simplicity)
            println!("Note: SH instruction at 0x{:08X} treated as SW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        },
        0x2A => {
            // SWL - Store Word Left (handle as SW for simplicity)
            println!("Note: SWL instruction at 0x{:08X} treated as SW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        },
        0x2E => {
            // SWR - Store Word Right (handle as SW for simplicity)
            println!("Note: SWR instruction at 0x{:08X} treated as SW", instruction_word);
            let base = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Sw { rt, base, offset }
        },
        0x04 => {
            // BEQ
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Beq { rs, rt, offset }
        },
        0x05 => {
            // BNE
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Bne { rs, rt, offset }
        },
        0x06 => {
            // BLEZ - Branch if Less Than or Equal to Zero (handle as BEQ)
            println!("Note: BLEZ instruction at 0x{:08X} treated as BEQ", instruction_word);
            let rs = (instruction_word >> 21) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Beq { rs, rt: 0, offset } // Compare with $0
        },
        0x07 => {
            // BGTZ - Branch if Greater Than Zero (handle as BNE)
            println!("Note: BGTZ instruction at 0x{:08X} treated as BNE", instruction_word);
            let rs = (instruction_word >> 21) & 0x1F;
            let offset = (instruction_word & 0xFFFF) as i16;
            Instruction::Bne { rs, rt: 0, offset } // Compare with $0
        },
        0x01 => {
            // BGEZ/BLTZ/etc. - Branch variants (handle as BEQ/BNE)
            let rs = (instruction_word >> 21) & 0x1F;
            let rt = (instruction_word >> 16) & 0x1F; // rt field is used as branch type
            let offset = (instruction_word & 0xFFFF) as i16;
            
            match rt {
                0x00 => { // BLTZ
                    println!("Note: BLTZ instruction at 0x{:08X} treated as BEQ", instruction_word);
                    Instruction::Beq { rs, rt: 0, offset }
                },
                0x01 => { // BGEZ
                    println!("Note: BGEZ instruction at 0x{:08X} treated as BNE", instruction_word);
                    Instruction::Bne { rs, rt: 0, offset }
                },
                _ => {
                    println!("Unrecognized branch variant: 0x{:02X} in instruction 0x{:08X}", 
                             rt, instruction_word);
                    Instruction::InvalidInstruction
                }
            }
        },
        0x02 => {
            // J
            let target = instruction_word & 0x03FFFFFF;
            Instruction::J { target }
        },
        0x03 => {
            // JAL - Jump and Link (handle as J for simplicity)
            let target = instruction_word & 0x03FFFFFF;
            println!("Note: JAL instruction at 0x{:08X} treated as J", instruction_word);
            Instruction::J { target }
        },
        _ => {
            println!("Unrecognized opcode: 0x{:02X} in instruction 0x{:08X}", 
                     opcode, instruction_word);
            Instruction::InvalidInstruction
        }
    }
}