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

// components.rs
//
// This file contains the implementation of the cache and cache hierarchy
// components for the timing simulator. It defines the Cache, CacheSet, and
// CacheLine structs, as well as the CacheHierarchy, which manages the
// interaction between different levels of cache.

use super::config::{CacheConfig, PrefetchStrategy as ConfigPrefetchStrategy, ReplacementPolicy};
use crate::functional_simulator::memory::Memory;
use std::collections::VecDeque;
use std::time::Instant;

// Helper function to create a prefetcher from a cache config
fn create_prefetcher(config: &CacheConfig) -> Option<Prefetcher> {
    if config.prefetch_enabled {
        // Convert from ConfigPrefetchStrategy to ComponentPrefetchStrategy
        match config.prefetch_strategy {
            ConfigPrefetchStrategy::NextNBlocks(n) => {
                Some(Prefetcher::new(ComponentPrefetchStrategy::NextNBlocks(n)))
            },
            ConfigPrefetchStrategy::AdjacentSets(n) => {
                Some(Prefetcher::new(ComponentPrefetchStrategy::AdjacentSets(n)))
            },
            ConfigPrefetchStrategy::Stride(s) => {
                Some(Prefetcher::new(ComponentPrefetchStrategy::Stride(s)))
            },
            ConfigPrefetchStrategy::Custom => None, // Custom not supported in ComponentPrefetchStrategy
        }
    } else {
        None
    }
}

#[derive(Clone)]
pub struct CacheLine {
    pub valid: bool,
    pub dirty: bool,
    pub tag: usize,
    pub data: Vec<u8>,
    pub last_access: Instant,
    pub access_count: usize,
}

impl CacheLine {
    pub fn new(block_size: usize) -> Self {
        Self {
            valid: false,
            dirty: false,
            tag: 0,
            data: vec![0; block_size],
            last_access: Instant::now(),
            access_count: 0,
        }
    }
}

#[derive(Clone)]
pub struct CacheSet {
    pub lines: Vec<CacheLine>,
    pub replacement_policy: ReplacementPolicy,
    pub lru_queue: VecDeque<usize>, // Tracks LRU order of lines in set
}

impl CacheSet {
    pub fn new(associativity: usize, block_size: usize, policy: ReplacementPolicy) -> Self {
        let mut lines = Vec::with_capacity(associativity);
        let mut lru_queue = VecDeque::with_capacity(associativity);

        for i in 0..associativity {
            lines.push(CacheLine::new(block_size));
            lru_queue.push_back(i);
        }

        Self {
            lines,
            replacement_policy: policy, // Use policy parameter instead of undefined variable
            lru_queue,
        }
    }

    // Find a line in the set with the given tag
    pub fn find_line(&self, tag: usize) -> Option<usize> {
        for (i, line) in self.lines.iter().enumerate() {
            if line.valid && line.tag == tag {
                return Some(i);
            }
        }
        None
    }

    // Update the access information for a line
    pub fn update_access(&mut self, line_idx: usize) {
        let line = &mut self.lines[line_idx];
        line.last_access = Instant::now();
        line.access_count += 1;

        // Update LRU queue - move this line to the back (most recently used)
        if let Some(pos) = self.lru_queue.iter().position(|&i| i == line_idx) {
            self.lru_queue.remove(pos);
        }
        self.lru_queue.push_back(line_idx);
    }

    // Find a victim line to replace based on replacement policy
    pub fn find_victim(&self) -> usize {
        match self.replacement_policy {
            ReplacementPolicy::LRU => {
                // Return the least recently used line (front of the queue)
                self.lru_queue.front().cloned().unwrap_or(0)
            },
            ReplacementPolicy::Random => {
                // Pick a random line
                use rand::Rng;
                let mut rng = rand::thread_rng(); // Corrected from rand::rng()
                rng.gen_range(0..self.lines.len()) // Corrected from random_range
            },
            ReplacementPolicy::FIFO => {
                // Return the first line that was filled (front of the queue)
                self.lru_queue.front().cloned().unwrap_or(0)
            },
            ReplacementPolicy::LFU => {
                // Return the least frequently used line
                self.lines
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, line)| line.access_count)
                    .map(|(idx, _)| idx)
                    .unwrap_or(0)
            },
        }
    }

    // Find the first invalid line, or return None if all are valid
    pub fn find_invalid_line(&self) -> Option<usize> {
        for (i, line) in self.lines.iter().enumerate() {
            if !line.valid {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct CacheStatistics {
    pub accesses: usize,
    pub hits: usize,
    pub misses: usize,
    pub read_accesses: usize,
    pub read_hits: usize,
    pub write_accesses: usize,
    pub write_hits: usize,
    pub evictions: usize,
    pub writebacks: usize,
    pub total_access_time: u128, // in nanoseconds
}

impl CacheStatistics {
    pub fn new() -> Self {
        Self {
            accesses: 0,
            hits: 0,
            misses: 0,
            read_accesses: 0,
            read_hits: 0,
            write_accesses: 0,
            write_hits: 0,
            evictions: 0,
            writebacks: 0,
            total_access_time: 0,
        }
    }

    pub fn hit_rate(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            self.hits as f64 / self.accesses as f64
        }
    }

    pub fn miss_rate(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            self.misses as f64 / self.accesses as f64
        }
    }

    pub fn read_hit_rate(&self) -> f64 {
        if self.read_accesses == 0 {
            0.0
        } else {
            self.read_hits as f64 / self.read_accesses as f64
        }
    }

    pub fn write_hit_rate(&self) -> f64 {
        if self.write_accesses == 0 {
            0.0
        } else {
            self.write_hits as f64 / self.write_accesses as f64
        }
    }

    pub fn average_access_time(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            self.total_access_time as f64 / self.accesses as f64
        }
    }

    pub fn print_statistics(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("Cache Statistics:\n"));
        result.push_str(&format!("  Accesses: {}\n", self.accesses));
        result.push_str(&format!("  Hits: {}\n", self.hits));
        result.push_str(&format!("  Misses: {}\n", self.misses));
        result.push_str(&format!("  Hit Rate: {:.2}%\n", self.hit_rate() * 100.0));
        result.push_str(&format!("  Miss Rate: {:.2}%\n", self.miss_rate() * 100.0));
        result.push_str(&format!("  Read Accesses: {}\n", self.read_accesses));
        result.push_str(&format!("  Read Hits: {}\n", self.read_hits));
        result.push_str(&format!(
            "  Read Hit Rate: {:.2}%\n",
            self.read_hit_rate() * 100.0
        ));
        result.push_str(&format!("  Write Accesses: {}\n", self.write_accesses));
        result.push_str(&format!("  Write Hits: {}\n", self.write_hits));
        result.push_str(&format!(
            "  Write Hit Rate: {:.2}%\n",
            self.write_hit_rate() * 100.0
        ));
        result.push_str(&format!("  Evictions: {}\n", self.evictions));
        result.push_str(&format!("  Writebacks: {}\n", self.writebacks));
        result.push_str(&format!(
            "  Average Access Time: {:.2} ns\n",
            self.average_access_time()
        ));

        result
    }
}

// A simple cache in the memory hierarchy
pub struct Cache {
    pub config: CacheConfig,
    pub sets: Vec<CacheSet>,
    pub memory: Memory,
    pub stats: CacheStatistics,
    pub write_policy: WritePolicy,
    pub allocation_policy: AllocationPolicy,
    pub prefetcher: Option<Prefetcher>,
    pub next_level: Option<Box<Cache>>, // Pointer to next cache level in hierarchy
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WritePolicy {
    WriteThrough,
    WriteBack,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AllocationPolicy {
    WriteAllocate,
    NoWriteAllocate,
}

impl Cache {
    pub fn new(config: CacheConfig, memory: Memory) -> Self {
        Self::new_with_next_level(config, memory, None)
    }

    pub fn new_with_next_level(
        config: CacheConfig,
        memory: Memory,
        next_level: Option<Box<Cache>>,
    ) -> Self {
        let num_sets = config.size / (config.associativity * config.block_size);
        let mut sets = Vec::with_capacity(num_sets);

        for _ in 0..num_sets {
            sets.push(CacheSet::new(
                config.associativity,
                config.block_size,
                config.replacement_policy.clone(),
            ));
        }

        println!(
            "Creating cache with {} sets, {} lines total, block size: {} bytes",
            num_sets,
            num_sets * config.associativity,
            config.block_size
        );

        Self {
            config: config.clone(),
            sets,
            memory,
            stats: CacheStatistics::new(),
            write_policy: WritePolicy::WriteBack,
            allocation_policy: AllocationPolicy::WriteAllocate,
            prefetcher: create_prefetcher(&config),
            next_level,
        }
    }

    // Read a block from the cache
    pub fn read(&mut self, address: usize) -> Option<(Vec<u8>, usize)> {
        let _start_time = Instant::now();
        self.stats.accesses += 1;
        self.stats.read_accesses += 1;

        let (tag, set_idx, offset) = self.decode_address(address);
        let block_addr = address - offset;

        // Check if address is within bounds
        if set_idx >= self.sets.len() {
            println!(
                "Cache index {} out of bounds (max: {})",
                set_idx,
                self.sets.len() - 1
            );
            return None;
        }

        // Check for a cache hit first
        let hit_index = {
            let set = &self.sets[set_idx];
            set.find_line(tag)
        };

        if let Some(line_idx) = hit_index {
            // Cache hit
            // Update access info
            {
                let set = &mut self.sets[set_idx];
                set.update_access(line_idx);
            }

            // Get the data
            let data = {
                let set = &self.sets[set_idx];
                let cache_line = &set.lines[line_idx];
                if offset + 4 <= cache_line.data.len() {
                    cache_line.data[offset..offset + 4].to_vec()
                } else {
                    println!(
                        "Cache line data access out of bounds: offset={}, size={}",
                        offset,
                        cache_line.data.len()
                    );
                    return None;
                }
            };

            // Update statistics
            self.stats.hits += 1;
            self.stats.read_hits += 1;
            let latency = self.config.hit_latency;
            self.stats.total_access_time += latency as u128;

            return Some((data, latency));
        }

        // Cache miss path
        self.stats.misses += 1;

        // Try next level cache if available
        if let Some(next_cache) = &mut self.next_level {
            // If we have a next-level cache, try to read from it
            if let Some((data, next_latency)) = next_cache.read(address) {
                // Successfully read from next level, add its latency
                return Some((data, self.config.hit_latency + next_latency));
            }
            // Fall through to memory access if next level cache also misses
        }

        // Find a line to replace
        let (victim_idx, need_writeback, dirty_tag) = {
            let set = &self.sets[set_idx];
            let victim_idx = set.find_invalid_line().unwrap_or_else(|| set.find_victim());
            let cache_line = &set.lines[victim_idx];
            let need_writeback = cache_line.valid && cache_line.dirty;
            let dirty_tag = cache_line.tag;
            (victim_idx, need_writeback, dirty_tag)
        };

        // Handle writeback if needed
        if need_writeback {
            self.write_back_line(set_idx, victim_idx, dirty_tag);
            self.stats.writebacks += 1;
        }

        // Load the new block
        self.load_block(block_addr, set_idx, victim_idx, tag);

        // Update access information
        {
            let set = &mut self.sets[set_idx];
            set.update_access(victim_idx);
        }

        // Get the data
        let data = {
            let set = &self.sets[set_idx];
            let cache_line = &set.lines[victim_idx];
            if offset + 4 <= cache_line.data.len() {
                cache_line.data[offset..offset + 4].to_vec()
            } else {
                println!(
                    "Cache line data access out of bounds after miss: offset={}, size={}",
                    offset,
                    cache_line.data.len()
                );
                return None;
            }
        };

        // Update statistics
        let access_time = self.config.miss_penalty;
        self.stats.total_access_time += access_time as u128;

        // Perform prefetching if enabled
        if let Some(prefetcher) = &self.prefetcher {
            let prefetch_addresses =
                prefetcher.get_prefetch_addresses(block_addr, self.config.block_size);

            for prefetch_addr in prefetch_addresses {
                // Clone necessary data for prefetch to avoid borrowing self again
                let (p_tag, p_set_idx, _) = self.decode_address(prefetch_addr);
                let p_block_addr = prefetch_addr - (prefetch_addr % self.config.block_size);
                self.prefetch_single_block(p_block_addr, p_set_idx, p_tag);
            }
        }

        Some((data, access_time))
    }

    // Add a new method to handle a single prefetch without multiple borrows
    fn prefetch_single_block(&mut self, block_addr: usize, set_idx: usize, tag: usize) {
        // Check if address is within bounds
        if set_idx >= self.sets.len() {
            return;
        }

        // Check if block is already in cache
        let is_in_cache = {
            let set = &self.sets[set_idx];
            set.find_line(tag).is_some()
        };

        if is_in_cache {
            return; // Block already in cache
        }

        // Find a line to prefetch into
        let (victim_idx, need_writeback, dirty_tag) = {
            let set = &self.sets[set_idx];
            let victim_idx = set.find_invalid_line().unwrap_or_else(|| set.find_victim());
            let cache_line = &set.lines[victim_idx];
            let need_writeback = cache_line.valid && cache_line.dirty;
            let dirty_tag = cache_line.tag;
            (victim_idx, need_writeback, dirty_tag)
        };

        // Handle writeback if needed
        if need_writeback {
            self.write_back_line(set_idx, victim_idx, dirty_tag);
        }

        // Load the block
        self.load_block(block_addr, set_idx, victim_idx, tag);
    }

    // Write a value to the cache
    pub fn write(&mut self, address: usize, value: &[u8]) -> usize {
        let _start_time = Instant::now();
        self.stats.accesses += 1;
        self.stats.write_accesses += 1;

        let (tag, set_idx, offset) = self.decode_address(address);
        let block_addr = address - offset;

        // Check if address is within bounds
        if set_idx >= self.sets.len() {
            println!(
                "Cache index {} out of bounds (max: {})",
                set_idx,
                self.sets.len() - 1
            );
            return self.config.miss_penalty; // Return miss penalty as access time
        }

        // Check if the block is in the cache
        let hit_index = {
            let set = &self.sets[set_idx];
            set.find_line(tag)
        };

        if let Some(line_idx) = hit_index {
            // Cache hit
            {
                let set = &mut self.sets[set_idx];
                set.update_access(line_idx);
            }

            // Write the value to cache line
            let result = {
                let set = &mut self.sets[set_idx];
                let cache_line = &mut set.lines[line_idx];

                if offset + value.len() <= cache_line.data.len() {
                    cache_line.data[offset..offset + value.len()].copy_from_slice(value);
                    cache_line.dirty = true;

                    // If write-through policy, also write to memory
                    if self.write_policy == WritePolicy::WriteThrough {
                        for (i, &byte) in value.iter().enumerate() {
                            self.memory.write_byte(address + i, byte);
                        }
                        // Reset dirty bit since it's consistent with memory
                        cache_line.dirty = false;
                    }

                    self.config.hit_latency
                } else {
                    println!(
                        "Cache line data write out of bounds: offset={}, size={}, write_size={}",
                        offset,
                        cache_line.data.len(),
                        value.len()
                    );
                    self.config.miss_penalty
                }
            };

            // Update statistics
            self.stats.hits += 1;
            self.stats.write_hits += 1;
            self.stats.total_access_time += result as u128;

            return result;
        }

        // Cache miss
        self.stats.misses += 1;

        // Try next level cache if available for write
        if let Some(next_cache) = &mut self.next_level {
            // If we have a next-level cache and write-allocate policy
            if self.allocation_policy == AllocationPolicy::WriteAllocate {
                let next_latency = next_cache.write(address, value);
                // Return with combined latencies
                return self.config.hit_latency + next_latency;
            }
        }

        match self.allocation_policy {
            AllocationPolicy::WriteAllocate => {
                // Find a line to replace
                let (victim_idx, need_writeback, dirty_tag) = {
                    let set = &self.sets[set_idx];
                    let victim_idx = set.find_invalid_line().unwrap_or_else(|| {
                        // No invalid lines, need to evict one
                        self.stats.evictions += 1;
                        set.find_victim()
                    });
                    let cache_line = &set.lines[victim_idx];
                    let need_writeback = cache_line.valid && cache_line.dirty;
                    let dirty_tag = cache_line.tag;
                    (victim_idx, need_writeback, dirty_tag)
                };

                // Handle writeback if needed
                if need_writeback {
                    self.write_back_line(set_idx, victim_idx, dirty_tag);
                    self.stats.writebacks += 1;
                }

                // Load the block from memory
                self.load_block(block_addr, set_idx, victim_idx, tag);

                // Update access information first
                {
                    let set = &mut self.sets[set_idx];
                    set.update_access(victim_idx);
                }

                // Then handle the cache line operations
                {
                    let set = &mut self.sets[set_idx];
                    let cache_line = &mut set.lines[victim_idx];
                    if offset + value.len() <= cache_line.data.len() {
                        cache_line.data[offset..offset + value.len()].copy_from_slice(value);
                        cache_line.dirty = self.write_policy == WritePolicy::WriteBack;

                        // If write-through policy, also write to memory
                        if self.write_policy == WritePolicy::WriteThrough {
                            for (i, &byte) in value.iter().enumerate() {
                                self.memory.write_byte(address + i, byte);
                            }
                        }
                    }
                }
            },
            AllocationPolicy::NoWriteAllocate => {
                // Write directly to memory without allocating a cache line
                for (i, &byte) in value.iter().enumerate() {
                    self.memory.write_byte(address + i, byte);
                }
            },
        }

        let access_time = self.config.miss_penalty;
        self.stats.total_access_time += access_time as u128;

        access_time
    }

    // Load a block from memory into the cache
    fn load_block(&mut self, block_addr: usize, set_idx: usize, line_idx: usize, tag: usize) {
        let line = &mut self.sets[set_idx].lines[line_idx];
        line.valid = true;
        line.dirty = false;
        line.tag = tag;

        // Fetch data from memory and update cache line
        for i in 0..self.config.block_size {
            let mem_addr = block_addr + i;
            if mem_addr < self.memory.size {
                line.data[i] = self.memory.read_byte(mem_addr).unwrap_or(0);
            } else {
                line.data[i] = 0;
            }
        }
    }

    // Write a dirty cache line back to memory
    pub fn write_back_line(&mut self, set_idx: usize, line_idx: usize, tag: usize) {
        let block_size = self.config.block_size;
        let data = self.sets[set_idx].lines[line_idx].data.clone();

        // Calculate block address
        let bits_per_block = (block_size as f64).log2() as usize;
        let bits_per_set = (self.sets.len() as f64).log2() as usize;
        let block_addr = (tag << (bits_per_set + bits_per_block)) | (set_idx << bits_per_block);

        // Write data back to memory
        for i in 0..block_size {
            let mem_addr = block_addr + i;
            if mem_addr < self.memory.size {
                self.memory.write_byte(mem_addr, data[i]);
            }
        }

        // Clear dirty bit
        self.sets[set_idx].lines[line_idx].dirty = false;
    }

    // Prefetch a block into the cache
    #[allow(dead_code)]
    fn prefetch_block(&mut self, address: usize) {
        let (tag, set_idx, _) = self.decode_address(address);
        let block_addr = address - (address % self.config.block_size);

        // Check if address is within bounds
        if set_idx >= self.sets.len() {
            return;
        }

        let set = &mut self.sets[set_idx];

        // Check if the block is already in the cache
        if set.find_line(tag).is_some() {
            // Block already in cache, no need to prefetch
            return;
        }

        // Find a line to prefetch into
        let line_idx = set.find_invalid_line().unwrap_or_else(|| {
            // No invalid lines, need to evict one
            set.find_victim()
        });

        // If the line is dirty, write it back to memory
        let cache_line = &set.lines[line_idx];
        if cache_line.valid && cache_line.dirty {
            let tag = cache_line.tag; // Get the tag first
            self.write_back_line(set_idx, line_idx, tag);
        }

        // Load the block from memory
        self.load_block(block_addr, set_idx, line_idx, tag);
    }

    // Decode address into tag, set index, and offset
    fn decode_address(&self, address: usize) -> (usize, usize, usize) {
        let offset_bits = (self.config.block_size as f64).log2() as usize;
        let num_sets = self.sets.len();
        let index_bits = (num_sets as f64).log2() as usize;

        // Create masks for each part of the address
        let offset_mask = (1 << offset_bits) - 1;
        let index_mask = ((1 << index_bits) - 1) << offset_bits;
        let tag_mask = !offset_mask & !index_mask;

        // Extract parts of the address
        let offset = address & offset_mask;
        let set_idx = (address & index_mask) >> offset_bits;
        let tag = (address & tag_mask) >> (offset_bits + index_bits);

        (tag, set_idx, offset)
    }

    // Flush the entire cache
    pub fn flush(&mut self) {
        for set_idx in 0..self.sets.len() {
            for line_idx in 0..self.sets[set_idx].lines.len() {
                let (need_writeback, tag) = {
                    let line = &self.sets[set_idx].lines[line_idx];
                    (line.valid && line.dirty, line.tag)
                };

                if need_writeback {
                    self.write_back_line(set_idx, line_idx, tag);
                }

                // Now invalidate the line
                self.sets[set_idx].lines[line_idx].valid = false;
                self.sets[set_idx].lines[line_idx].dirty = false;
            }
        }
    }

    // Get cache statistics
    pub fn get_statistics(&self) -> &CacheStatistics {
        &self.stats
    }

    // Reset cache statistics
    pub fn reset_statistics(&mut self) {
        self.stats = CacheStatistics::new();
    }
}

// Prefetcher for cache
pub struct Prefetcher {
    strategy: ComponentPrefetchStrategy,
    prefetch_distance: usize,
}

pub enum ComponentPrefetchStrategy {
    NextNBlocks(usize),
    AdjacentSets(usize),
    Stride(isize),
    Custom(Box<dyn CustomPrefetcher>),
}

// Define a multi-level cache hierarchy
pub struct CacheHierarchy {
    pub l1_data_cache: Cache,
    pub l1_instr_cache: Cache,
    pub stats: CacheHierarchyStats,
}

// Statistics for the entire cache hierarchy
pub struct CacheHierarchyStats {
    pub miss_latency: usize,
    pub hit_latency: usize,
    pub average_access_time: f64,
}

impl CacheHierarchy {
    pub fn new(
        memory: Memory,
        l1d_config: CacheConfig,
        l1i_config: CacheConfig,
        l2_config: Option<CacheConfig>,
    ) -> Self {
        // Create L2 cache if configured
        let l2_cache = l2_config.map(|config| Box::new(Cache::new(config, memory.clone())));

        // Create L1 data cache with L2 as next level
        let l1_data_cache = Cache::new_with_next_level(l1d_config, memory.clone(), l2_cache);

        // Create L1 instruction cache without next level (since we already created L2)
        let l1_instr_cache = Cache::new(l1i_config, memory);

        Self {
            l1_data_cache,
            l1_instr_cache,
            stats: CacheHierarchyStats {
                miss_latency: 0,
                hit_latency: 0,
                average_access_time: 0.0,
            },
        }
    }

    pub fn read_data(&mut self, address: usize) -> Option<(Vec<u8>, usize)> {
        self.l1_data_cache.read(address)
    }

    pub fn write_data(&mut self, address: usize, value: &[u8]) -> usize {
        self.l1_data_cache.write(address, value)
    }

    pub fn read_instruction(&mut self, address: usize) -> Option<(Vec<u8>, usize)> {
        self.l1_instr_cache.read(address)
    }

    pub fn flush(&mut self) {
        self.l1_data_cache.flush();
        self.l1_instr_cache.flush();
    }

    pub fn update_stats(&mut self) {
        // Calculate average access time for the hierarchy
        let l1d_stats = self.l1_data_cache.get_statistics();
        let l1i_stats = self.l1_instr_cache.get_statistics();

        // Combine stats from both caches
        let total_accesses = l1d_stats.accesses + l1i_stats.accesses;
        if total_accesses > 0 {
            self.stats.average_access_time = (l1d_stats.total_access_time
                + l1i_stats.total_access_time) as f64
                / total_accesses as f64;
        }

        // Set hit and miss latencies
        self.stats.hit_latency = self.l1_data_cache.config.hit_latency;
        self.stats.miss_latency = self.l1_data_cache.config.miss_penalty;
    }

    pub fn print_stats(&self) -> String {
        let mut result = String::new();

        result.push_str("Cache Hierarchy Statistics:\n");
        result.push_str("L1 Data Cache:\n");
        result.push_str(&self.l1_data_cache.get_statistics().print_statistics());
        result.push_str("\nL1 Instruction Cache:\n");
        result.push_str(&self.l1_instr_cache.get_statistics().print_statistics());

        // Print L2 cache stats if available
        if let Some(l2_cache) = &self.l1_data_cache.next_level {
            result.push_str("\nL2 Cache:\n");
            result.push_str(&l2_cache.get_statistics().print_statistics());
        }

        result.push_str(&format!(
            "\nHierarchy Summary:\n  Average Access Time: {:.2} ns\n",
            self.stats.average_access_time
        ));

        result
    }
}

// Define a new trait for prefetching
pub trait CustomPrefetcher: Send + Sync {
    fn get_prefetch_addresses(&self, address: usize, block_size: usize) -> Vec<usize>;
    fn clone_box(&self) -> Box<dyn CustomPrefetcher>;
}

// Implement the trait for closures using a wrapper
pub struct FnPrefetcher<F>(F)
where
    F: Fn(usize, usize) -> Vec<usize> + Send + Sync + Clone + 'static; // Add 'static here

impl<F> CustomPrefetcher for FnPrefetcher<F>
where
    F: Fn(usize, usize) -> Vec<usize> + Send + Sync + Clone + 'static, // Add 'static here
{
    fn get_prefetch_addresses(&self, address: usize, block_size: usize) -> Vec<usize> {
        (self.0)(address, block_size)
    }

    fn clone_box(&self) -> Box<dyn CustomPrefetcher> {
        Box::new(FnPrefetcher(self.0.clone()))
    }
}

impl Prefetcher {
    pub fn new(strategy: ComponentPrefetchStrategy) -> Self {
        Self {
            strategy,
            prefetch_distance: 1,
        }
    }

    pub fn set_prefetch_distance(&mut self, distance: usize) {
        self.prefetch_distance = distance;
    }

    pub fn get_prefetch_addresses(&self, address: usize, block_size: usize) -> Vec<usize> {
        match &self.strategy {
            ComponentPrefetchStrategy::NextNBlocks(n) => {
                // Prefetch the next N blocks
                let mut addresses = Vec::with_capacity(*n);
                for i in 1..=*n {
                    addresses.push(address + block_size * i);
                }
                addresses
            },
            ComponentPrefetchStrategy::AdjacentSets(n) => {
                // Prefetch blocks from adjacent sets
                let mut addresses = Vec::with_capacity(*n * 2);
                for i in 1..=*n {
                    addresses.push(address + block_size * i * self.prefetch_distance);
                    if address >= block_size * i * self.prefetch_distance {
                        addresses.push(address - block_size * i * self.prefetch_distance);
                    }
                }
                addresses
            },
            ComponentPrefetchStrategy::Stride(stride) => {
                // Prefetch with a specific stride pattern
                let mut addresses = Vec::with_capacity(self.prefetch_distance);
                let stride_bytes = *stride * block_size as isize;
                for i in 1..=self.prefetch_distance {
                    let next_addr = address as isize + stride_bytes * i as isize;
                    if next_addr >= 0 {
                        addresses.push(next_addr as usize);
                    }
                }
                addresses
            },
            ComponentPrefetchStrategy::Custom(func) => {
                // Call the method on the trait object
                func.get_prefetch_addresses(address, block_size)
            },
        }
    }
}
