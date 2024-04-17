// components.rs
use super::config::CacheConfig;

#[derive(Clone)]
pub struct CacheLine {
    pub valid: bool,
    pub tag: usize,
    pub data: Vec<u8>,
    pub timestamp: usize,
}

impl CacheLine {
    pub fn new(block_size: usize) -> Self {
        Self {
            valid: false,
            tag: 0,
            data: vec![0; block_size],
            timestamp: 0,
        }
    }
}

pub struct Cache {
    pub config: CacheConfig,
    pub lines: Vec<CacheLine>,
}

impl Cache {
    pub fn new(config: CacheConfig) -> Self {
        let num_lines = config.size / (config.associativity * config.block_size);
        let lines = vec![CacheLine::new(config.block_size); num_lines];
        Self { config, lines }
    }

    pub fn read(&mut self, address: usize) -> &[u8] {
        let (tag, index, offset) = self.decode_address(address);
        let start_index = index * self.config.associativity;
        let end_index = start_index + self.config.associativity;
        let lines = &mut self.lines[start_index..end_index];
    
        if let Some(line_index) = lines.iter().position(|line| line.valid && line.tag == tag) {
            let line = &mut lines[line_index];
            line.timestamp = 0; // Update timestamp for replacement policy
            return &line.data[offset..offset + 4];
        }
    
        // Cache miss
        let replace_index = lines
            .iter()
            .enumerate()
            .min_by_key(|(_, line)| line.timestamp)
            .map(|(index, _)| index)
            .unwrap_or(0);
    
        let replace_line = &mut lines[replace_index];
        replace_line.valid = true;
        replace_line.tag = tag;
        replace_line.timestamp = 0; // Update timestamp for replacement policy
        // Fetch data from memory and update cache line
        // (Assuming memory is accessible through a separate component)
        // ...
        &replace_line.data[offset..offset + 4]
    }

    fn decode_address(&self, address: usize) -> (usize, usize, usize) {
        let offset_bits = (self.config.block_size as f64).log2() as usize;
        let index_bits = (self.config.size as f64 / self.config.associativity as f64).log2() as usize;
        let offset_mask = (1 << offset_bits) - 1;
        let index_mask = ((1 << index_bits) - 1) << offset_bits;
        let tag_mask = !offset_mask & !index_mask;
        let offset = address & offset_mask;
        let index = (address & index_mask) >> offset_bits;
        let tag = (address & tag_mask) >> (offset_bits + index_bits);
        (tag, index, offset)
    }
}