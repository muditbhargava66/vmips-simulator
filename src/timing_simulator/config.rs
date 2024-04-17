// config.rs
pub struct CacheConfig {
    pub size: usize,
    pub associativity: usize,
    pub block_size: usize,
    pub replacement_policy: ReplacementPolicy,
}

pub enum ReplacementPolicy {
    LRU,
    // Add more replacement policies as needed
}

pub struct PipelineConfig {
    pub num_stages: usize,
    pub stage_latencies: Vec<usize>,
}
