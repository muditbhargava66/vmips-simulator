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

// config.rs
//
// This file contains the configuration structs for the timing simulator.
// It defines the configuration for the pipeline, caches, and branch predictor.

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub size: usize,          // Cache size in bytes
    pub associativity: usize, // Number of ways (lines per set)
    pub block_size: usize,    // Size of each cache line in bytes
    pub replacement_policy: ReplacementPolicy,
    pub hit_latency: usize,     // Cache hit latency in cycles
    pub miss_penalty: usize,    // Additional latency on cache miss
    pub write_back: bool,       // true = write-back, false = write-through
    pub write_allocate: bool,   // true = write-allocate, false = no-write-allocate
    pub prefetch_enabled: bool, // Whether prefetching is enabled
    pub prefetch_strategy: PrefetchStrategy,
}

impl CacheConfig {
    pub fn new(size: usize, associativity: usize, block_size: usize) -> Self {
        // Validate configuration
        assert!(size > 0, "Cache size must be positive");
        assert!(associativity > 0, "Associativity must be positive");
        assert!(block_size > 0, "Block size must be positive");
        assert!(
            size % (associativity * block_size) == 0,
            "Cache size must be divisible by (associativity * block_size)"
        );

        Self {
            size,
            associativity,
            block_size,
            replacement_policy: ReplacementPolicy::LRU,
            hit_latency: 1,
            miss_penalty: 10,
            write_back: true,
            write_allocate: true,
            prefetch_enabled: false,
            prefetch_strategy: PrefetchStrategy::NextNBlocks(1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReplacementPolicy {
    LRU,    // Least Recently Used
    FIFO,   // First In First Out
    Random, // Random replacement
    LFU,    // Least Frequently Used
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrefetchStrategy {
    NextNBlocks(usize),  // Prefetch the next N sequential blocks
    AdjacentSets(usize), // Prefetch blocks from N adjacent sets
    Stride(isize),       // Prefetch with a specific stride pattern
    Custom,              // Custom prefetch strategy
}

pub struct PipelineConfig {
    pub num_stages: usize,
    pub stage_latencies: Vec<usize>,
    pub forwarding_enabled: bool,
    pub branch_prediction_enabled: bool,
    pub branch_predictor_type: BranchPredictorType,
    /// Tomasulo out-of-order execution settings
    pub tomasulo_config: Option<TomasuloConfig>,
    pub superscalar_width: usize,
}

impl PipelineConfig {
    pub fn new(num_stages: usize) -> Self {
        // Default to 1 cycle per stage
        let stage_latencies = vec![1; num_stages];

        Self {
            num_stages,
            stage_latencies,
            forwarding_enabled: true,
            branch_prediction_enabled: true,
            branch_predictor_type: BranchPredictorType::TwoBit,
            tomasulo_config: None,
            superscalar_width: 1,
        }
    }

    pub fn with_latencies(mut self, latencies: Vec<usize>) -> Self {
        assert_eq!(
            latencies.len(),
            self.num_stages,
            "Number of latencies must match number of stages"
        );
        self.stage_latencies = latencies;
        self
    }

    pub fn with_forwarding(mut self, enabled: bool) -> Self {
        self.forwarding_enabled = enabled;
        self
    }

    pub fn with_branch_prediction(
        mut self,
        enabled: bool,
        predictor_type: BranchPredictorType,
    ) -> Self {
        self.branch_prediction_enabled = enabled;
        self.branch_predictor_type = predictor_type;
        self
    }

    /// Enable Tomasulo's Algorithm for out-of-order execution
    pub fn with_tomasulo(mut self, enabled: bool, config: TomasuloConfig) -> Self {
        if enabled {
            self.tomasulo_config = Some(config);
        } else {
            self.tomasulo_config = None;
        }
        self
    }

    pub fn with_superscalar(mut self, width: usize) -> Self {
        assert!(width > 0, "Superscalar width must be positive");
        self.superscalar_width = width;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchPredictorType {
    Static,      // Always predict taken or not taken
    OneBit,      // Remember last outcome
    TwoBit,      // 2-bit saturating counter
    Correlating, // Use history of recent branches
    Tournament,  // Combine multiple predictors
}

/// Configuration for Tomasulo's algorithm
#[derive(Debug, Clone)]
pub struct TomasuloConfig {
    /// Number of reservation stations
    pub num_reservation_stations: usize,
    /// Size of the reorder buffer
    pub rob_size: usize,
    /// Number of functional units of each type
    pub num_alu_units: usize,
    pub num_fpu_units: usize,
    pub num_load_store_units: usize,
    pub num_branch_units: usize,
    /// Commit width (instructions committed per cycle)
    pub commit_width: usize,
    /// Issue width (instructions issued per cycle)
    pub issue_width: usize,
}

impl TomasuloConfig {
    pub fn new() -> Self {
        Self {
            num_reservation_stations: 16,
            rob_size: 32,
            num_alu_units: 2,
            num_fpu_units: 2,
            num_load_store_units: 1,
            num_branch_units: 1,
            commit_width: 4,
            issue_width: 2,
        }
    }

    pub fn with_reservation_stations(mut self, num: usize) -> Self {
        self.num_reservation_stations = num;
        self
    }

    pub fn with_rob_size(mut self, size: usize) -> Self {
        self.rob_size = size;
        self
    }

    pub fn with_alu_units(mut self, num: usize) -> Self {
        self.num_alu_units = num;
        self
    }

    pub fn with_fpu_units(mut self, num: usize) -> Self {
        self.num_fpu_units = num;
        self
    }

    pub fn with_load_store_units(mut self, num: usize) -> Self {
        self.num_load_store_units = num;
        self
    }

    pub fn with_branch_units(mut self, num: usize) -> Self {
        self.num_branch_units = num;
        self
    }

    pub fn with_commit_width(mut self, width: usize) -> Self {
        self.commit_width = width;
        self
    }

    pub fn with_issue_width(mut self, width: usize) -> Self {
        self.issue_width = width;
        self
    }
}

pub struct SimulatorConfig {
    pub memory_size: usize,
    pub max_instructions: usize,
    pub pipeline_config: PipelineConfig,
    pub l1_cache_config: CacheConfig,
    pub l2_cache_config: Option<CacheConfig>,
    pub debug_enabled: bool,
    pub trace_enabled: bool,
    pub statistics_enabled: bool,
}

impl SimulatorConfig {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory_size,
            max_instructions: 1000000,
            pipeline_config: PipelineConfig::new(5),
            l1_cache_config: CacheConfig::new(32768, 4, 64),
            l2_cache_config: None,
            debug_enabled: false,
            trace_enabled: false,
            statistics_enabled: true,
        }
    }

    pub fn with_pipeline(mut self, pipeline_config: PipelineConfig) -> Self {
        self.pipeline_config = pipeline_config;
        self
    }

    pub fn with_l1_cache(mut self, cache_config: CacheConfig) -> Self {
        self.l1_cache_config = cache_config;
        self
    }

    pub fn with_l2_cache(mut self, cache_config: CacheConfig) -> Self {
        self.l2_cache_config = Some(cache_config);
        self
    }

    pub fn with_max_instructions(mut self, max_instructions: usize) -> Self {
        self.max_instructions = max_instructions;
        self
    }

    pub fn with_debug(mut self, enabled: bool) -> Self {
        self.debug_enabled = enabled;
        self
    }

    pub fn with_trace(mut self, enabled: bool) -> Self {
        self.trace_enabled = enabled;
        self
    }

    pub fn with_statistics(mut self, enabled: bool) -> Self {
        self.statistics_enabled = enabled;
        self
    }
}
