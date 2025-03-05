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
        // Calculate the number of cache lines needed
        // Make sure we have at least one line
        let num_sets = config.size / (config.associativity * config.block_size);
        let total_lines = if num_sets > 0 {
            num_sets * config.associativity
        } else {
            config.associativity // At least one set
        };
        
        println!("Creating cache with {} sets, {} lines total", num_sets, total_lines);
        let lines = vec![CacheLine::new(config.block_size); total_lines];
        Self { config, lines, memory }
    }

    pub fn read(&mut self, address: usize) -> Option<&[u8]> {
        // Early check: if address is beyond memory size, return None immediately
        if address >= self.memory.size {
            return None;
        }
    
        let (tag, index, offset) = self.decode_address(address);
        
        // Ensure the calculated index is within bounds of our cache
        if index >= (self.lines.len() / self.config.associativity) {
            println!("Cache index {} out of bounds (max: {})", 
                     index, (self.lines.len() / self.config.associativity) - 1);
            return None;
        }
        
        let start_index = index * self.config.associativity;
        let end_index = start_index + self.config.associativity;
    
        if end_index > self.lines.len() {
            println!("Cache access out of bounds: start_index={}, end_index={}, total lines={}",
                    start_index, end_index, self.lines.len());
            return None;
        }
    
        let lines = &mut self.lines[start_index..end_index];
    
        if let Some(line_index) = lines.iter().position(|line| line.valid && line.tag == tag) {
            // Cache hit
            let line = &mut lines[line_index];
            line.timestamp = 0; // Update timestamp for replacement policy
            
            // Make sure we don't go out of bounds on the cache line data
            if offset + 4 <= line.data.len() {
                Some(&line.data[offset..offset + 4])
            } else {
                println!("Cache line data access out of bounds: offset={}, size={}", offset, line.data.len());
                None
            }
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
            
            // Make sure we don't exceed memory bounds
            let end_address = std::cmp::min(block_address + self.config.block_size, self.memory.size);
            
            for i in 0..(end_address - block_address) {
                let mem_address = block_address + i;
                if mem_address < self.memory.size {
                    replace_line.data[i] = self.memory.data[mem_address];
                }
            }
    
            // Make sure we don't go out of bounds on the cache line data
            if offset + 4 <= replace_line.data.len() {
                Some(&replace_line.data[offset..offset + 4])
            } else {
                println!("Cache line data access out of bounds after miss: offset={}, size={}", 
                         offset, replace_line.data.len());
                None
            }
        }
    }

    fn decode_address(&self, address: usize) -> (usize, usize, usize) {
        let offset_bits = (self.config.block_size as f64).log2() as usize;
        
        // Calculate number of sets in the cache (total_lines / associativity)
        let num_sets = self.lines.len() / self.config.associativity;
        // Calculate index bits based on the actual number of sets
        let index_bits = if num_sets > 0 {
            (num_sets as f64).log2().floor() as usize
        } else {
            0
        };
        
        // Create masks for each part of the address
        let offset_mask = (1 << offset_bits) - 1;
        let index_mask = ((1 << index_bits) - 1) << offset_bits;
        let tag_mask = !offset_mask & !index_mask;
        
        // Extract parts of the address
        let offset = address & offset_mask;
        
        // Calculate index ensuring it's within bounds
        let raw_index = (address & index_mask) >> offset_bits;
        let index = if num_sets > 0 { 
            raw_index % num_sets 
        } else { 
            0 
        };
        
        let tag = (address & tag_mask) >> (offset_bits + index_bits);
        
        (tag, index, offset)
    }
}