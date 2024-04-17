// pipeline.rs
use crate::timing_simulator::config::{PipelineConfig, CacheConfig};
use crate::functional_simulator::instructions::Instruction;
use super::components::Cache;

pub struct PipelineStage {
    pub latency: usize,
}

impl PipelineStage {
    pub fn new(latency: usize) -> Self {
        Self { latency }
    }
}

pub struct Pipeline {
    pub stages: Vec<PipelineStage>,
    pub instr_cache: Cache,
    pub data_cache: Cache,
}

impl Pipeline {
    pub fn new(config: &PipelineConfig, instr_cache_config: CacheConfig, data_cache_config: CacheConfig) -> Self {
        let stages = config
            .stage_latencies
            .iter()
            .map(|&latency| PipelineStage::new(latency))
            .collect();
        let instr_cache = Cache::new(instr_cache_config);
        let data_cache = Cache::new(data_cache_config);
        Self {
            stages,
            instr_cache,
            data_cache,
        }
    }

    pub fn execute(&mut self, _instruction: &Instruction) -> usize {
        let mut total_latency = 0;
        for stage in &mut self.stages {
            // Perform stage-specific operations
            // ...
            total_latency += stage.latency;
        }
        total_latency
    }
}
