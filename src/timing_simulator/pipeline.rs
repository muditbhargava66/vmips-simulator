// pipeline.rs

use crate::timing_simulator::config::{PipelineConfig, CacheConfig};
use crate::functional_simulator::instructions::Instruction;
use crate::functional_simulator::memory::Memory;
use crate::functional_simulator::registers::Registers;
use super::components::Cache;

pub struct PipelineStage {
    pub latency: usize,
    pub instruction: Option<Instruction>,
}

impl PipelineStage {
    pub fn new(latency: usize) -> Self {
        Self {
            latency,
            instruction: None,
        }
    }
}

pub struct Pipeline {
    pub stages: Vec<PipelineStage>,
    pub instr_cache: Cache,
    pub data_cache: Cache,
}

impl Pipeline {
    pub fn new(config: &PipelineConfig, instr_cache_config: CacheConfig, data_cache_config: CacheConfig, memory: Memory) -> Self {
        let stages = config
            .stage_latencies
            .iter()
            .map(|&latency| PipelineStage::new(latency))
            .collect();
        let instr_cache = Cache::new(instr_cache_config, memory.clone());
        let data_cache = Cache::new(data_cache_config, memory);
        Self {
            stages,
            instr_cache,
            data_cache,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction, registers: &Registers, pc: u32) -> usize {
        let mut total_latency = 0;
        
        // Fetch stage
        let fetch_latency = self.instr_cache.read(instruction.get_address(registers, pc) as usize).map(|_| 1).unwrap_or(10);
        total_latency += fetch_latency;

        // Decode stage
        let decode_latency = 1;
        total_latency += decode_latency;

        // Execute stage
        let execute_latency = match instruction {
            Instruction::Add { .. } |
            Instruction::Sub { .. } |
            Instruction::And { .. } |
            Instruction::Or { .. } |
            Instruction::Slt { .. } |
            Instruction::Addi { .. } => 1,
            Instruction::Lw { .. } |
            Instruction::Sw { .. } => 2,
            Instruction::Beq { .. } |
            Instruction::J { .. } => 1,
            _ => 1,
        };
        total_latency += execute_latency;

        // Memory stage
        if let Instruction::Lw { .. } | Instruction::Sw { .. } = instruction {
            let memory_latency = self.data_cache.read(instruction.get_address(registers, pc) as usize).map(|_| 2).unwrap_or(20);
            total_latency += memory_latency;
        }

        // Writeback stage
        let writeback_latency = 1;
        total_latency += writeback_latency;

        total_latency
    }

    pub fn flush(&mut self) {
        for stage in &mut self.stages {
            stage.instruction = None;
        }
    }

    pub fn is_register_being_written(&self, reg_num: u32) -> bool {
        for stage in &self.stages {
            if let Some(instruction) = &stage.instruction {
                match instruction {
                    Instruction::Add { rd, .. } |
                    Instruction::Sub { rd, .. } |
                    Instruction::And { rd, .. } |
                    Instruction::Or { rd, .. } |
                    Instruction::Slt { rd, .. } if *rd == reg_num => {
                        return true;
                    }
                    Instruction::Addi { rt, .. } |
                    Instruction::Lw { rt, .. } if *rt == reg_num => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }
}