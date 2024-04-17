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
        let pipeline = Pipeline::new(&pipeline_config, instr_cache_config, data_cache_config);
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
        loop {
            let instruction = self.fetch_instruction();
            let _latency = self.pipeline.execute(&instruction);
            self.pc += 4;
            // Perform hazard detection and resolution
            // ...
        }
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let instruction_bytes = self.pipeline.instr_cache.read(self.pc as usize);
        let instruction_word = u32::from_le_bytes([
            instruction_bytes[0],
            instruction_bytes[1],
            instruction_bytes[2],
            instruction_bytes[3],
        ]);
        decode_instruction(instruction_word)
    }
}