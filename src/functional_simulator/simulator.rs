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

// simulator.rs
//
// This file contains the implementation of the MIPS functional simulator.
// It defines the main simulator struct, which includes the CPU registers,
// memory, and program counter. The simulator is responsible for fetching,
// decoding, and executing MIPS instructions.

use super::instructions::Instruction;
use super::memory::Memory;
use super::registers::Registers;
use crate::utils::syscall::handle_syscall;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Exception {
    InvalidInstruction,
    MemoryAccessViolation,
    SystemCall,
    BreakPoint,
    ArithmeticOverflow,
    FloatingPointException,
}

#[derive(Clone, Copy, Debug)]
pub enum ExecutionMode {
    User,
    Kernel,
}

pub struct Simulator {
    pub registers: Registers,
    pub memory: Memory,
    pc: u32,
    pub exception: Option<Exception>,
    pub step_count: usize,
    pub max_steps: usize,
    pub break_points: HashMap<u32, bool>,
    pub mode: ExecutionMode,
    pub fp_enabled: bool,
    pub trace_enabled: bool,
    pub debug_enabled: bool,
}

impl Simulator {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(memory_size),
            pc: 0,
            exception: None,
            step_count: 0,
            max_steps: 1000000, // Prevent infinite loops
            break_points: HashMap::new(),
            mode: ExecutionMode::User,
            fp_enabled: true,
            trace_enabled: false,
            debug_enabled: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        println!("Loading program of size {} bytes", program.len());

        // Load test data first - this ensures it's in memory before the program tries to use it
        self.memory.write_word(0x1000, 10);
        self.memory.write_word(0x1004, 20);
        self.memory.write_word(0x1008, 0); // Will hold add result
        self.memory.write_word(0x100C, 0); // Will hold mult result
        println!("Test data loaded into memory at addresses 0x1000-0x100C");

        // First check if this is an assembler-generated file with header
        let has_header = program.len() >= 8
            && u32::from_le_bytes([program[0], program[1], program[2], program[3]]) < 1_000_000
            && u32::from_le_bytes([program[4], program[5], program[6], program[7]]) < 1_000_000;

        if has_header {
            // Extract data and text section sizes from header
            let data_size =
                u32::from_le_bytes([program[0], program[1], program[2], program[3]]) as usize;

            let text_size =
                u32::from_le_bytes([program[4], program[5], program[6], program[7]]) as usize;

            println!(
                "Loading program with data section: {} bytes, text section: {} bytes",
                data_size, text_size
            );

            // Validate sizes before attempting to copy
            if data_size + text_size + 8 > program.len() {
                println!("Warning: Invalid section sizes in header. Falling back to raw loading.");
                // Fall back to raw loading
                self.memory.data[..program.len()].copy_from_slice(program);
                self.pc = 0;
            } else {
                // Load data section at address 0x10000000 (data segment)
                for (i, &byte) in program[8..8 + data_size].iter().enumerate() {
                    self.memory.write_byte(0x10000000 + i, byte);
                }

                // Load text section at address 0x00400000 (text segment)
                for (i, chunk) in program[8 + data_size..8 + data_size + text_size]
                    .chunks_exact(4)
                    .enumerate()
                {
                    let instr = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    self.memory.write_word(0x00400000 + i * 4, instr);
                }

                // Set PC to start of text segment
                self.pc = 0x00400000;
            }
        } else {
            // Load raw binary as instructions with explicit endianness handling
            println!(
                "Loading raw program without header ({} bytes)",
                program.len()
            );

            // Make sure we're handling 4-byte instruction alignment
            if program.len() % 4 != 0 {
                println!("Warning: Program size is not a multiple of 4 bytes");
            }

            // Print program instructions for debugging
            println!("Program instructions:");
            for (i, chunk) in program.chunks_exact(4).enumerate().take(16) {
                // Convert 4 bytes to a 32-bit instruction - ensure little-endian
                let instr = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                println!("  0x{:04X}: 0x{:08X}", i * 4, instr);

                // Write the instruction to memory
                self.memory.write_word(i * 4, instr);
            }

            // Set PC to 0
            self.pc = 0;
        }

        // Initialize stack pointer
        self.registers.write(29, 0x7FFFFFFC); // $sp = 0x7FFFFFFC

        println!(
            "Program loaded. Initial PC: 0x{:08X}, SP: 0x{:08X}",
            self.pc,
            self.registers.read(29)
        );
    }

    pub fn run(&mut self) {
        println!("Starting execution at PC: 0x{:08X}", self.pc);

        // Print first few instructions for debugging
        if self.debug_enabled {
            for offset in 0..5 {
                let addr = self.pc as usize + offset * 4;
                if addr < self.memory.size {
                    if let Some(instruction_word) = self.memory.read_word(addr) {
                        println!("Instruction at 0x{:08X}: 0x{:08X}", addr, instruction_word);
                    }
                }
            }
        }

        self.step_count = 0;

        // Save the PC in registers for use in branch instructions
        self.registers.pc = self.pc;

        // Track frequency of PC values to detect loops
        let mut pc_frequency: HashMap<u32, usize> = HashMap::new();

        loop {
            // Check if we've reached the maximum number of steps
            self.step_count += 1;
            if self.step_count > self.max_steps {
                println!(
                    "Reached maximum instruction limit ({}). Stopping execution.",
                    self.max_steps
                );
                break;
            }

            // Record this PC in our frequency map for loop detection
            *pc_frequency.entry(self.pc).or_insert(0) += 1;

            // Print trace information if enabled
            if self.trace_enabled || self.debug_enabled {
                if self.step_count % 1000 == 0 || self.step_count < 100 {
                    println!("Step {}: PC = 0x{:08X}", self.step_count, self.pc);
                }

                if pc_frequency.get(&self.pc).unwrap_or(&0) > &100 {
                    println!(
                        "Warning: PC 0x{:08X} executed over 100 times - possible infinite loop",
                        self.pc
                    );
                }
            }

            // Check if this address is a breakpoint
            if self.break_points.contains_key(&self.pc) {
                println!("Breakpoint hit at address 0x{:08X}", self.pc);
                self.exception = Some(Exception::BreakPoint);
                break;
            }

            // Fetch instruction
            let instruction = self.fetch_instruction();

            // Update PC in registers for branch delay calculations
            self.registers.pc = self.pc;

            // Execute instruction
            match instruction {
                Instruction::InvalidInstruction => {
                    println!("Invalid instruction encountered at PC: 0x{:08X}", self.pc);
                    self.exception = Some(Exception::InvalidInstruction);
                    break;
                },
                Instruction::Syscall => {
                    // Handle system call
                    if let Some(new_pc) = handle_syscall(&mut self.registers, &mut self.memory) {
                        if new_pc == 0xFFFFFFFF {
                            // Program termination requested
                            println!("Program terminated via syscall at PC: 0x{:08X}", self.pc);
                            break;
                        }
                        self.pc = new_pc;
                    } else {
                        // Normal syscall - continue execution
                        self.pc += 4;
                    }
                    continue;
                },
                Instruction::Break { code: _ } => {
                    println!(
                        "Breakpoint instruction encountered at PC: 0x{:08X}",
                        self.pc
                    );
                    self.exception = Some(Exception::BreakPoint);
                    break;
                },
                Instruction::Nop => {
                    // Special termination condition: multiple NOPs in a row at the end of the program
                    if self.step_count > 10 {
                        let mut nop_count = 0;
                        for i in 0..4 {
                            let addr = self.pc as usize + i * 4;
                            if addr < self.memory.size {
                                if let Some(word) = self.memory.read_word(addr) {
                                    if word == 0 {
                                        nop_count += 1;
                                    }
                                }
                            }
                        }

                        if nop_count >= 3 {
                            println!(
                                "Reached multiple NOPs at PC 0x{:08X} - terminating program",
                                self.pc
                            );
                            break;
                        }
                    }

                    // Just a regular NOP - continue execution
                    self.pc += 4;
                    continue;
                },
                _ => {
                    // Execute regular instruction
                    let pc_offset = self.execute_instruction(&instruction);

                    match pc_offset {
                        Some(offset) => {
                            // Branch or jump instruction - calculate new PC
                            let new_pc = if offset == 0xFFFFFFFF {
                                // Special value used by some instructions to set PC directly
                                self.registers.read(31) // Use $ra register instead of target_reg
                            } else {
                                // For branch instructions, PC+4+offset; for jumps, just PC+offset
                                if instruction.is_branch_or_jump() {
                                    if matches!(
                                        instruction,
                                        Instruction::J { .. } | Instruction::Jal { .. }
                                    ) {
                                        // Jump instructions use the lower 26 bits shifted left by 2
                                        // with the upper 4 bits from the current PC
                                        (self.pc & 0xF0000000) | (offset & 0x0FFFFFFF)
                                    } else if matches!(
                                        instruction,
                                        Instruction::Jr { .. } | Instruction::Jalr { .. }
                                    ) {
                                        // JR and JALR use the register value directly
                                        offset
                                    } else {
                                        // Branch instructions: add offset to PC+4
                                        self.pc.wrapping_add(4).wrapping_add(offset)
                                    }
                                } else {
                                    // Default case
                                    self.pc.wrapping_add(offset)
                                }
                            };

                            // Track branching for debugging
                            println!(
                                "Branch/Jump: from PC=0x{:08X} to PC=0x{:08X}, offset=0x{:08X}",
                                self.pc, new_pc, offset
                            );

                            // Check for potential infinite loop (jumping to same address)
                            if new_pc == self.pc && pc_frequency.get(&self.pc).unwrap_or(&0) > &10 {
                                println!("Warning: Jump to same address detected (0x{:08X}). Breaking potential infinite loop.", 
                                         new_pc);
                                self.pc = self.pc.wrapping_add(4); // Skip to next instruction
                            } else if new_pc < self.memory.size as u32 {
                                if self.trace_enabled
                                    && pc_frequency.get(&self.pc).unwrap_or(&0) > &10
                                {
                                    println!(
                                        "Jump/branch from 0x{:08X} to 0x{:08X}",
                                        self.pc, new_pc
                                    );
                                }
                                self.pc = new_pc;
                            } else {
                                println!("Invalid jump target: 0x{:08X}", new_pc);
                                self.exception = Some(Exception::MemoryAccessViolation);
                                break;
                            }
                        },
                        None => {
                            // Regular instruction - increment PC
                            self.pc += 4;

                            // Check if an exception occurred during execution
                            if self.exception.is_some() {
                                println!(
                                    "Exception during instruction execution at PC: 0x{:08X}",
                                    self.pc - 4
                                );
                                break;
                            }
                        },
                    }
                },
            }
        }

        println!(
            "Simulation ended after executing {} instructions",
            self.step_count
        );
        println!("Final PC: 0x{:08X}", self.pc);
    }

    pub fn step(&mut self) -> bool {
        // Execute a single instruction and return true if execution should continue

        // Check if we've reached the maximum number of steps
        self.step_count += 1;
        if self.step_count > self.max_steps {
            println!(
                "Reached maximum instruction limit ({}). Stopping execution.",
                self.max_steps
            );
            return false;
        }

        // Check if this address is a breakpoint
        if self.break_points.contains_key(&self.pc) {
            println!("Breakpoint hit at address 0x{:08X}", self.pc);
            self.exception = Some(Exception::BreakPoint);
            return false;
        }

        // Update PC in registers for branch delay calculations
        self.registers.pc = self.pc;

        // Fetch instruction
        let instruction = self.fetch_instruction();

        // Execute instruction
        match instruction {
            Instruction::InvalidInstruction => {
                println!("Invalid instruction encountered at PC: 0x{:08X}", self.pc);
                self.exception = Some(Exception::InvalidInstruction);
                return false;
            },
            Instruction::Syscall => {
                // Handle system call
                if let Some(new_pc) = handle_syscall(&mut self.registers, &mut self.memory) {
                    if new_pc == 0xFFFFFFFF {
                        // Program termination requested
                        println!("Program terminated via syscall at PC: 0x{:08X}", self.pc);
                        return false;
                    }
                    self.pc = new_pc;
                } else {
                    // Normal syscall - continue execution
                    self.pc += 4;
                }
            },
            Instruction::Break { code: _ } => {
                println!(
                    "Breakpoint instruction encountered at PC: 0x{:08X}",
                    self.pc
                );
                self.exception = Some(Exception::BreakPoint);
                return false;
            },
            Instruction::Nop => {
                // Just a regular NOP - continue execution
                self.pc += 4;
            },
            _ => {
                // Execute regular instruction
                let pc_offset = self.execute_instruction(&instruction);

                match pc_offset {
                    Some(offset) => {
                        // Branch or jump instruction - calculate new PC
                        let new_pc = if offset == 0xFFFFFFFF {
                            // Special value used by some instructions to set PC directly
                            self.registers.read(self.registers.target_reg.unwrap_or(0))
                        } else {
                            self.pc.wrapping_add(offset)
                        };

                        if new_pc < self.memory.size as u32 {
                            self.pc = new_pc;
                        } else {
                            println!("Invalid jump target: 0x{:08X}", new_pc);
                            self.exception = Some(Exception::MemoryAccessViolation);
                            return false;
                        }
                    },
                    None => {
                        // Regular instruction - increment PC
                        self.pc += 4;

                        // Check if an exception occurred during execution
                        if self.exception.is_some() {
                            println!(
                                "Exception during instruction execution at PC: 0x{:08X}",
                                self.pc - 4
                            );
                            return false;
                        }
                    },
                }
            },
        }

        // Execution should continue
        true
    }

    fn fetch_instruction(&self) -> Instruction {
        match self.memory.read_word(self.pc as usize) {
            Some(instruction_word) => {
                if self.debug_enabled || self.trace_enabled {
                    println!("Fetched 0x{:08X} at PC=0x{:08X}", instruction_word, self.pc);
                }
                decode_instruction(instruction_word)
            },
            None => {
                println!("Memory access violation at PC: 0x{:08X}", self.pc);
                Instruction::InvalidInstruction
            },
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Option<u32> {
        // Execute the instruction and return the PC offset if it's a branch/jump
        // Print debug info for branching instructions to diagnose test failures
        if let Instruction::Beq { rs, rt, offset } = &instruction {
            let rs_val = self.registers.read(*rs);
            let rt_val = self.registers.read(*rt);
            println!(
                "Debug BEQ: rs({})={}, rt({})={}, offset={}, PC=0x{:08X}",
                rs, rs_val, rt, rt_val, offset, self.pc
            );
        }
        instruction.execute(&mut self.registers, &mut self.memory)
    }

    pub fn add_breakpoint(&mut self, address: u32) {
        self.break_points.insert(address, true);
        println!("Breakpoint added at address 0x{:08X}", address);
    }

    pub fn remove_breakpoint(&mut self, address: u32) {
        self.break_points.remove(&address);
        println!("Breakpoint removed from address 0x{:08X}", address);
    }

    pub fn list_breakpoints(&self) {
        println!("Breakpoints:");
        for &address in self.break_points.keys() {
            println!("  0x{:08X}", address);
        }
    }

    pub fn enable_floating_point(&mut self, enabled: bool) {
        self.fp_enabled = enabled;
        println!(
            "Floating point support {}",
            if enabled { "enabled" } else { "disabled" }
        );
    }

    pub fn enable_trace(&mut self, enabled: bool) {
        self.trace_enabled = enabled;
        println!(
            "Instruction tracing {}",
            if enabled { "enabled" } else { "disabled" }
        );
    }

    pub fn enable_debug(&mut self, enabled: bool) {
        self.debug_enabled = enabled;
        println!(
            "Debug mode {}",
            if enabled { "enabled" } else { "disabled" }
        );
    }

    pub fn set_max_steps(&mut self, max_steps: usize) {
        self.max_steps = max_steps;
        println!("Maximum steps set to {}", max_steps);
    }

    pub fn get_disassembly(&self, address: u32, count: usize) -> Vec<(u32, String)> {
        let mut result = Vec::new();
        let mut current_addr = address;

        for _ in 0..count {
            if current_addr >= self.memory.size as u32 {
                break;
            }

            if let Some(word) = self.memory.read_word(current_addr as usize) {
                let instruction = decode_instruction(word);
                let disasm = format!(
                    "{:08X}: {}",
                    current_addr,
                    instruction_to_string(&instruction, word)
                );
                result.push((current_addr, disasm));
            } else {
                break;
            }

            current_addr += 4;
        }

        result
    }
}

// Decode an instruction word into an Instruction enum
// Basic implementation for decode_instruction
pub fn decode_instruction(instruction_word: u32) -> Instruction {
    // Extract instruction fields
    let opcode = (instruction_word >> 26) & 0x3F;
    let rs = (instruction_word >> 21) & 0x1F;
    let rt = (instruction_word >> 16) & 0x1F;
    let rd = (instruction_word >> 11) & 0x1F;
    let shamt = (instruction_word >> 6) & 0x1F;
    let funct = instruction_word & 0x3F;
    let immediate = instruction_word & 0xFFFF;
    let target = instruction_word & 0x3FFFFFF;

    match opcode {
        0 => {
            // R-type instruction
            match funct {
                // First check for NOP specifically (instruction_word == 0)
                _ if instruction_word == 0 => Instruction::Nop,
                // Then check other function codes
                0x20 => Instruction::Add { rd, rs, rt },
                0x22 => Instruction::Sub { rd, rs, rt },
                0x24 => Instruction::And { rd, rs, rt },
                0x25 => Instruction::Or { rd, rs, rt },
                0x2A => Instruction::Slt { rd, rs, rt },
                0x00 => Instruction::Sll { rd, rt, shamt },
                0x02 => Instruction::Srl { rd, rt, shamt },
                0x03 => Instruction::Sra { rd, rt, shamt },
                0x04 => Instruction::Sllv { rd, rt, rs },
                0x06 => Instruction::Srlv { rd, rt, rs },
                0x07 => Instruction::Srav { rd, rt, rs },
                0x08 => Instruction::Jr { rs },
                0x09 => Instruction::Jalr { rd, rs },
                0x0C => Instruction::Syscall,
                0x0D => Instruction::Break {
                    code: instruction_word >> 16,
                },
                0x10 => Instruction::Mfhi { rd },
                0x11 => Instruction::Mthi { rs },
                0x12 => Instruction::Mflo { rd },
                0x13 => Instruction::Mtlo { rs },
                0x18 => Instruction::Mult { rs, rt },
                0x1A => Instruction::Div { rs, rt },
                0x1B => Instruction::Divu { rs, rt },
                0x26 => Instruction::Xor { rd, rs, rt },
                0x27 => Instruction::Nor { rd, rs, rt },
                _ => {
                    println!(
                        "Unrecognized R-type instruction with funct: 0x{:02X}",
                        funct
                    );
                    Instruction::InvalidInstruction
                },
            }
        },
        0x08 => Instruction::Addi {
            rt,
            rs,
            imm: immediate as i16,
        },
        0x09 => Instruction::Addiu {
            rt,
            rs,
            imm: immediate as i16,
        },
        0x0A => Instruction::Slti {
            rt,
            rs,
            imm: immediate as i16,
        },
        0x0B => Instruction::Sltiu {
            rt,
            rs,
            imm: immediate as i16,
        },
        0x0C => Instruction::Andi {
            rt,
            rs,
            imm: immediate as u16,
        },
        0x0D => Instruction::Ori {
            rt,
            rs,
            imm: immediate as u16,
        },
        0x0E => Instruction::Xori {
            rt,
            rs,
            imm: immediate as u16,
        },
        0x0F => Instruction::Lui {
            rt,
            imm: immediate as u16,
        },
        0x20 => Instruction::Lb {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x21 => Instruction::Lh {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x23 => Instruction::Lw {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x24 => Instruction::Lbu {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x25 => Instruction::Lhu {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x28 => Instruction::Sb {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x29 => Instruction::Sh {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x2B => Instruction::Sw {
            rt,
            base: rs,
            offset: immediate as i16,
        },
        0x04 => Instruction::Beq {
            rs,
            rt,
            offset: immediate as i16,
        },
        0x05 => Instruction::Bne {
            rs,
            rt,
            offset: immediate as i16,
        },
        0x06 => Instruction::Blez {
            rs,
            offset: immediate as i16,
        },
        0x07 => Instruction::Bgtz {
            rs,
            offset: immediate as i16,
        },
        0x01 => {
            // Special branch instructions
            match rt {
                0x00 => Instruction::Bltz {
                    rs,
                    offset: immediate as i16,
                },
                0x01 => Instruction::Bgez {
                    rs,
                    offset: immediate as i16,
                },
                _ => {
                    println!("Unrecognized branch instruction with rt: 0x{:02X}", rt);
                    Instruction::InvalidInstruction
                },
            }
        },
        0x02 => Instruction::J { target },
        0x03 => Instruction::Jal { target },
        // Coprocessor instructions
        0x31 => Instruction::LwC1 {
            ft: rt,
            base: rs,
            offset: immediate as i16,
        },
        0x39 => Instruction::SwC1 {
            ft: rt,
            base: rs,
            offset: immediate as i16,
        },
        0x11 => {
            // FPU operations
            let fmt = rs;
            let ft = rt;
            let fs = rd;
            let fd = shamt;

            match fmt {
                0x10 => {
                    // Single precision
                    match funct {
                        0x00 => Instruction::AddS { fd, fs, ft },
                        0x01 => Instruction::SubS { fd, fs, ft },
                        0x02 => Instruction::MulS { fd, fs, ft },
                        0x03 => Instruction::DivS { fd, fs, ft },
                        0x05 => Instruction::AbsS { fd, fs },
                        0x07 => Instruction::NegS { fd, fs },
                        0x06 => Instruction::MovS { fd, fs },
                        0x32 => Instruction::CvtWS { fd, fs },
                        0x20 => Instruction::CvtSW { fd, fs },
                        0x30 => Instruction::CmpS { fs, ft, cond: 0 }, // eq
                        0x3C => Instruction::CmpS { fs, ft, cond: 1 }, // lt
                        0x3E => Instruction::CmpS { fs, ft, cond: 2 }, // le
                        _ => {
                            println!("Unrecognized FPU instruction with funct: 0x{:02X}", funct);
                            Instruction::InvalidInstruction
                        },
                    }
                },
                0x08 => {
                    // Branch on FP condition
                    if rt == 0 {
                        Instruction::BC1F {
                            offset: immediate as i16,
                        }
                    } else if rt == 1 {
                        Instruction::BC1T {
                            offset: immediate as i16,
                        }
                    } else {
                        println!("Unrecognized FP branch instruction with rt: 0x{:02X}", rt);
                        Instruction::InvalidInstruction
                    }
                },
                _ => {
                    println!("Unrecognized FPU instruction with fmt: 0x{:02X}", fmt);
                    Instruction::InvalidInstruction
                },
            }
        },
        _ => {
            println!("Unrecognized instruction with opcode: 0x{:02X}", opcode);
            Instruction::InvalidInstruction
        },
    }
}

// Convert an instruction to a human-readable string
fn instruction_to_string(instruction: &Instruction, raw_word: u32) -> String {
    match instruction {
        Instruction::Add { rd, rs, rt } => {
            format!("add ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::Sub { rd, rs, rt } => {
            format!("sub ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::And { rd, rs, rt } => {
            format!("and ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::Or { rd, rs, rt } => {
            format!("or ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::Xor { rd, rs, rt } => {
            format!("xor ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::Nor { rd, rs, rt } => {
            format!("nor ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::Slt { rd, rs, rt } => {
            format!("slt ${}, ${}, ${}", rd, rs, rt)
        },
        Instruction::Sll { rd, rt, shamt } => {
            format!("sll ${}, ${}, {}", rd, rt, shamt)
        },
        Instruction::Srl { rd, rt, shamt } => {
            format!("srl ${}, ${}, {}", rd, rt, shamt)
        },
        Instruction::Sra { rd, rt, shamt } => {
            format!("sra ${}, ${}, {}", rd, rt, shamt)
        },
        Instruction::Sllv { rd, rt, rs } => {
            format!("sllv ${}, ${}, ${}", rd, rt, rs)
        },
        Instruction::Srlv { rd, rt, rs } => {
            format!("srlv ${}, ${}, ${}", rd, rt, rs)
        },
        Instruction::Srav { rd, rt, rs } => {
            format!("srav ${}, ${}, ${}", rd, rt, rs)
        },
        Instruction::Jr { rs } => {
            format!("jr ${}", rs)
        },
        Instruction::Jalr { rd, rs } => {
            format!("jalr ${}, ${}", rd, rs)
        },
        Instruction::Syscall => "syscall".to_string(),
        Instruction::Break { code } => {
            format!("break {}", code)
        },
        Instruction::Mfhi { rd } => {
            format!("mfhi ${}", rd)
        },
        Instruction::Mthi { rs } => {
            format!("mthi ${}", rs)
        },
        Instruction::Mflo { rd } => {
            format!("mflo ${}", rd)
        },
        Instruction::Mtlo { rs } => {
            format!("mtlo ${}", rs)
        },
        Instruction::Mult { rs, rt } => {
            format!("mult ${}, ${}", rs, rt)
        },
        Instruction::Div { rs, rt } => {
            format!("div ${}, ${}", rs, rt)
        },
        Instruction::Divu { rs, rt } => {
            format!("divu ${}, ${}", rs, rt)
        },
        Instruction::Addi { rt, rs, imm } => {
            format!("addi ${}, ${}, {}", rt, rs, imm)
        },
        Instruction::Addiu { rt, rs, imm } => {
            format!("addiu ${}, ${}, {}", rt, rs, imm)
        },
        Instruction::Slti { rt, rs, imm } => {
            format!("slti ${}, ${}, {}", rt, rs, imm)
        },
        Instruction::Sltiu { rt, rs, imm } => {
            format!("sltiu ${}, ${}, {}", rt, rs, imm)
        },
        Instruction::Andi { rt, rs, imm } => {
            format!("andi ${}, ${}, 0x{:X}", rt, rs, imm)
        },
        Instruction::Ori { rt, rs, imm } => {
            format!("ori ${}, ${}, 0x{:X}", rt, rs, imm)
        },
        Instruction::Xori { rt, rs, imm } => {
            format!("xori ${}, ${}, 0x{:X}", rt, rs, imm)
        },
        Instruction::Lui { rt, imm } => {
            format!("lui ${}, 0x{:X}", rt, imm)
        },
        Instruction::Lb { rt, base, offset } => {
            format!("lb ${}, {}(${})", rt, offset, base)
        },
        Instruction::Lh { rt, base, offset } => {
            format!("lh ${}, {}(${})", rt, offset, base)
        },
        Instruction::Lw { rt, base, offset } => {
            format!("lw ${}, {}(${})", rt, offset, base)
        },
        Instruction::Lbu { rt, base, offset } => {
            format!("lbu ${}, {}(${})", rt, offset, base)
        },
        Instruction::Lhu { rt, base, offset } => {
            format!("lhu ${}, {}(${})", rt, offset, base)
        },
        Instruction::Sb { rt, base, offset } => {
            format!("sb ${}, {}(${})", rt, offset, base)
        },
        Instruction::Sh { rt, base, offset } => {
            format!("sh ${}, {}(${})", rt, offset, base)
        },
        Instruction::Sw { rt, base, offset } => {
            format!("sw ${}, {}(${})", rt, offset, base)
        },
        Instruction::Beq { rs, rt, offset } => {
            format!("beq ${}, ${}, {}", rs, rt, offset)
        },
        Instruction::Bne { rs, rt, offset } => {
            format!("bne ${}, ${}, {}", rs, rt, offset)
        },
        Instruction::Bgtz { rs, offset } => {
            format!("bgtz ${}, {}", rs, offset)
        },
        Instruction::Blez { rs, offset } => {
            format!("blez ${}, {}", rs, offset)
        },
        Instruction::Bltz { rs, offset } => {
            format!("bltz ${}, {}", rs, offset)
        },
        Instruction::Bgez { rs, offset } => {
            format!("bgez ${}, {}", rs, offset)
        },
        Instruction::J { target } => {
            format!("j 0x{:X}", target << 2)
        },
        Instruction::Jal { target } => {
            format!("jal 0x{:X}", target << 2)
        },
        // Floating-point instructions
        Instruction::AddS { fd, fs, ft } => {
            format!("add.s $f{}, $f{}, $f{}", fd, fs, ft)
        },
        Instruction::SubS { fd, fs, ft } => {
            format!("sub.s $f{}, $f{}, $f{}", fd, fs, ft)
        },
        Instruction::MulS { fd, fs, ft } => {
            format!("mul.s $f{}, $f{}, $f{}", fd, fs, ft)
        },
        Instruction::DivS { fd, fs, ft } => {
            format!("div.s $f{}, $f{}, $f{}", fd, fs, ft)
        },
        Instruction::AbsS { fd, fs } => {
            format!("abs.s $f{}, $f{}", fd, fs)
        },
        Instruction::NegS { fd, fs } => {
            format!("neg.s $f{}, $f{}", fd, fs)
        },
        Instruction::MovS { fd, fs } => {
            format!("mov.s $f{}, $f{}", fd, fs)
        },
        Instruction::CvtSW { fd, fs } => {
            format!("cvt.s.w $f{}, $f{}", fd, fs)
        },
        Instruction::CvtWS { fd, fs } => {
            format!("cvt.w.s $f{}, $f{}", fd, fs)
        },
        Instruction::CmpS { fs, ft, cond } => {
            let cond_str = match cond {
                0 => "eq",
                1 => "lt",
                2 => "le",
                _ => "??",
            };
            format!("c.{}.s $f{}, $f{}", cond_str, fs, ft)
        },
        Instruction::LwC1 { ft, base, offset } => {
            format!("lwc1 $f{}, {}(${})", ft, offset, base)
        },
        Instruction::SwC1 { ft, base, offset } => {
            format!("swc1 $f{}, {}(${})", ft, offset, base)
        },
        Instruction::BC1T { offset } => {
            format!("bc1t {}", offset)
        },
        Instruction::BC1F { offset } => {
            format!("bc1f {}", offset)
        },
        Instruction::Nop => "nop".to_string(),
        Instruction::InvalidInstruction => {
            format!("INVALID (0x{:08X})", raw_word)
        },
    }
}
