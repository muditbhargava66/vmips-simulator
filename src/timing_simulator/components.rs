// components.rs

use super::config::CacheConfig;
use crate::functional_simulator::memory::Memory;

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
    pub memory: Memory,
}

impl Cache {
    pub fn new(config: CacheConfig, memory: Memory) -> Self {
        let num_lines = config.size / (config.associativity * config.block_size);
        let lines = vec![CacheLine::new(config.block_size); num_lines];
        Self { config, lines, memory }
    }

    pub fn read(&mut self, address: usize) -> Option<&[u8]> {
        let (tag, index, offset) = self.decode_address(address);
        let start_index = index * self.config.associativity;
        let end_index = start_index + self.config.associativity;
    
        if end_index <= self.lines.len() {
            let lines = &mut self.lines[start_index..end_index];
    
            if let Some(line_index) = lines.iter().position(|line| line.valid && line.tag == tag) {
                let line = &mut lines[line_index];
                line.timestamp = 0; // Update timestamp for replacement policy
                Some(&line.data[offset..offset + 4])
            } else {
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
                let block_address = address - (address % self.config.block_size);
                for i in 0..self.config.block_size {
                    let mem_address = block_address + i;
                    if mem_address < self.memory.size {
                        replace_line.data[i] = self.memory.data[mem_address];
                    } else {
                        // Memory access violation
                        println!("Memory access violation occurred while fetching data for cache");
                        println!("Address: 0x{:08X}", mem_address);
                        println!("Cache configuration: {:?}", self.config);
                        println!("Memory size: {}", self.memory.size);
                        return None;
                    }
                }
    
                Some(&replace_line.data[offset..offset + 4])
            }
        } else {
            // Cache access violation
            println!("Cache access violation: address 0x{:08X} is out of bounds", address);
            println!("Cache configuration: {:?}", self.config);
            println!("Number of cache lines: {}", self.lines.len());
            None
        }
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