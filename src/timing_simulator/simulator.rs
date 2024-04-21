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
        let mut data_hazard = false;
        let mut control_hazard = false;

        loop {
            if stall_cycles > 0 {
                stall_cycles -= 1;
                continue;
            }

            let instruction = self.fetch_instruction();

            // Check for data hazards
            data_hazard = self.check_data_hazard(&instruction);
            if data_hazard {
                // Stall the pipeline for data hazard
                stall_cycles = self.pipeline.stages.len() - 1;
                continue;
            }

            // Check for control hazards
            control_hazard = self.check_control_hazard(&instruction);
            if control_hazard {
                // Flush the pipeline for control hazard
                self.pipeline.flush();
            }

            let _latency = self.pipeline.execute(&instruction, &self.registers, self.pc);
            self.pc += 4;

            // Update the registers and memory based on the executed instruction
            self.update_state(&instruction);
        }
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