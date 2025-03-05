// simulator.rs

use crate::functional_simulator::registers::Registers;
use crate::functional_simulator::memory::Memory;
use crate::functional_simulator::simulator::decode_instruction;
use crate::functional_simulator::instructions::Instruction;
use super::config::{CacheConfig, PipelineConfig};
use super::pipeline::Pipeline;

pub struct Simulator {
    pub pipeline: Pipeline,
    pub registers: Registers,
    pub memory: Memory,
    pub pc: u32,
}

impl Simulator {
    pub fn new(pipeline_config: PipelineConfig, instr_cache_config: CacheConfig, data_cache_config: CacheConfig, memory_size: usize) -> Self {
        let pipeline = Pipeline::new(&pipeline_config, instr_cache_config, data_cache_config, Memory::new(memory_size));
        let registers = Registers::new();
        let memory = Memory::new(memory_size);
        Self {
            pipeline,
            registers,
            memory,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        let mut stall_cycles = 0;
        let mut error_count = 0;     // Counter to track consecutive errors
        const MAX_ERRORS: usize = 5; // Maximum consecutive errors before ending simulation
        
        loop {
            if stall_cycles > 0 {
                stall_cycles -= 1;
                continue;
            }
    
            let instruction = self.fetch_instruction();
            
            // Check if we hit an invalid instruction
            if let Instruction::InvalidInstruction = instruction {
                println!("Invalid instruction encountered at PC: 0x{:08X}", self.pc);
                break;
            }
    
            // Check for data hazards
            if self.check_data_hazard(&instruction) {
                // Stall the pipeline for data hazard
                stall_cycles = self.pipeline.stages.len() - 1;
                continue;
            }
    
            // Check for control hazards
            if self.check_control_hazard(&instruction) {
                // Flush the pipeline for control hazard
                self.pipeline.flush();
            }
    
            // Execute the instruction and check for errors
            let result = self.pipeline.execute(&instruction, &self.registers, self.pc);
            
            // If we get a very high latency, it's likely due to a cache miss or error
            if result > 20 {
                error_count += 1;
                if error_count >= MAX_ERRORS {
                    println!("Ending simulation after {} consecutive cache/memory errors", MAX_ERRORS);
                    println!("Last instruction at PC: 0x{:08X}", self.pc);
                    break;
                }
            } else {
                // Reset error count on successful execution
                error_count = 0;
            }
            
            self.pc += 4;
    
            // Prevent runaway execution - if PC gets too large, exit
            if self.pc >= self.memory.size as u32 {
                println!("PC exceeded memory size bounds. Ending simulation.");
                break;
            }
    
            // Update the registers and memory based on the executed instruction
            self.update_state(&instruction);
        }
        
        println!("Simulation complete. Final PC: 0x{:08X}", self.pc);
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let instruction_bytes = self.pipeline.instr_cache.read(self.pc as usize);
        match instruction_bytes {
            Some(bytes) => {
                let instruction_word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                decode_instruction(instruction_word)
            }
            None => Instruction::InvalidInstruction,
        }
    }

    fn check_data_hazard(&self, instruction: &Instruction) -> bool {
        match instruction {
            Instruction::Add { rs, rt, .. } |
            Instruction::Sub { rs, rt, .. } |
            Instruction::And { rs, rt, .. } |
            Instruction::Or { rs, rt, .. } |
            Instruction::Slt { rs, rt, .. } => {
                self.pipeline.is_register_being_written(*rs) || self.pipeline.is_register_being_written(*rt)
            }
            Instruction::Addi { rs, .. } => {
                self.pipeline.is_register_being_written(*rs)
            }
            Instruction::Lw { base, .. } => {
                self.pipeline.is_register_being_written(*base)
            }
            Instruction::Sw { base, rt, .. } => {
                self.pipeline.is_register_being_written(*base) || self.pipeline.is_register_being_written(*rt)
            }
            _ => false,
        }
    }

    fn check_control_hazard(&self, instruction: &Instruction) -> bool {
        match instruction {
            Instruction::Beq { .. } |
            Instruction::J { .. } => true,
            _ => false,
        }
    }

    fn update_state(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Add { rd, rs, rt } => {
                let rs_value = self.registers.read(*rs);
                let rt_value = self.registers.read(*rt);
                let result = rs_value.wrapping_add(rt_value);
                self.registers.write(*rd, result);
            }
            Instruction::Sub { rd, rs, rt } => {
                let rs_value = self.registers.read(*rs);
                let rt_value = self.registers.read(*rt);
                let result = rs_value.wrapping_sub(rt_value);
                self.registers.write(*rd, result);
            }
            Instruction::And { rd, rs, rt } => {
                let rs_value = self.registers.read(*rs);
                let rt_value = self.registers.read(*rt);
                let result = rs_value & rt_value;
                self.registers.write(*rd, result);
            }
            Instruction::Or { rd, rs, rt } => {
                let rs_value = self.registers.read(*rs);
                let rt_value = self.registers.read(*rt);
                let result = rs_value | rt_value;
                self.registers.write(*rd, result);
            }
            Instruction::Slt { rd, rs, rt } => {
                let rs_value = self.registers.read(*rs) as i32;
                let rt_value = self.registers.read(*rt) as i32;
                let result = (rs_value < rt_value) as u32;
                self.registers.write(*rd, result);
            }
            Instruction::Addi { rt, rs, imm } => {
                let rs_value = self.registers.read(*rs);
                let result = rs_value.wrapping_add(*imm as u32);
                self.registers.write(*rt, result);
            }
            Instruction::Lw { rt, base, offset } => {
                let base_value = self.registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                match self.memory.read_word(address as usize) {
                    Some(value) => self.registers.write(*rt, value),
                    None => {} // Handle memory access error
                }
            }
            Instruction::Sw { rt, base, offset } => {
                let base_value = self.registers.read(*base);
                let address = base_value.wrapping_add(*offset as u32);
                let value = self.registers.read(*rt);
                match self.memory.write_word(address as usize, value) {
                    true => {} // Success
                    false => {} // Handle memory access error
                }
            }
            Instruction::Beq { rs, rt, offset } => {
                let rs_value = self.registers.read(*rs);
                let rt_value = self.registers.read(*rt);
                if rs_value == rt_value {
                    let new_pc = self.pc.wrapping_add(*offset as u32);
                    self.pc = new_pc;
                }
            }
            Instruction::J { target } => {
                let new_pc = (*target) << 2;
                self.pc = new_pc;
            }
            _ => {}
        }
    }
}