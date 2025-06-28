// pipeline.rs
use super::components::CacheHierarchy;
use crate::functional_simulator::instructions::Instruction;
use crate::functional_simulator::memory::Memory;
use crate::functional_simulator::registers::Registers;
use crate::timing_simulator::config::{CacheConfig, PipelineConfig};

// Pipeline stage types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipelineStageType {
    Fetch,
    Decode,
    Execute,
    Memory,
    Writeback,
}

// Pipeline stage status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipelineStageStatus {
    Empty,
    Busy,
    Stalled,
    Flushed,
}

// Forwarding paths
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForwardingPath {
    None,
    ExToEx,
    MemToEx,
    MemToMem,
    WbToEx,
    WbToMem,
}

// Data hazard types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HazardType {
    None,
    RAW,  // Read After Write
    WAR,  // Write After Read
    WAW,  // Write After Write
    Control,
    Structural,
}

#[derive(Clone)]
pub struct PipelineStage {
    pub stage_type: PipelineStageType,
    pub latency: usize,
    pub cycles_remaining: usize,
    pub instruction: Option<Instruction>,
    pub pc: u32,
    pub status: PipelineStageStatus,
    pub data: Option<u32>,
    pub target_register: Option<u32>,
    pub memory_address: Option<u32>,
}

impl PipelineStage {
    pub fn new(stage_type: PipelineStageType, latency: usize) -> Self {
        Self {
            stage_type,
            latency,
            cycles_remaining: 0,
            instruction: None,
            pc: 0,
            status: PipelineStageStatus::Empty,
            data: None,
            target_register: None,
            memory_address: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.status == PipelineStageStatus::Busy && self.cycles_remaining == 0
    }

    pub fn tick(&mut self) {
        if self.status == PipelineStageStatus::Busy && self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.cycles_remaining = 0;
        self.instruction = None;
        self.pc = 0;
        self.status = PipelineStageStatus::Empty;
        self.data = None;
        self.target_register = None;
        self.memory_address = None;
    }

    pub fn start_instruction(&mut self, instruction: Instruction, pc: u32) {
        self.instruction = Some(instruction);
        self.pc = pc;
        self.status = PipelineStageStatus::Busy;
        self.cycles_remaining = self.latency;

        // For execute and memory stages, determine target register
        if self.stage_type == PipelineStageType::Execute || self.stage_type == PipelineStageType::Memory {
            if let Some(ref instr) = self.instruction {
                self.target_register = instr.get_destination_register();
            }
        }
    }

    pub fn stall(&mut self) {
        self.status = PipelineStageStatus::Stalled;
    }

    pub fn unstall(&mut self) {
        if self.status == PipelineStageStatus::Stalled {
            self.status = PipelineStageStatus::Busy;
        }
    }

    pub fn flush(&mut self) {
        self.status = PipelineStageStatus::Flushed;
        self.instruction = None;
    }
}

pub struct Pipeline {
    pub stages: Vec<PipelineStage>,
    pub cache_hierarchy: CacheHierarchy,
    pub branch_predictor: BranchPredictor,
    pub forwarding_enabled: bool,
    pub stall_cycles: usize,
    pub branch_mispredictions: usize,
    pub instruction_count: usize,
    pub cycle_count: usize,
    pub stall_count: usize,
    pub cache_miss_stalls: usize,
    pub data_hazard_stalls: usize,
    pub control_hazard_stalls: usize,
    pub structural_hazard_stalls: usize,
    pub hazard_stats: Vec<(HazardType, usize)>,
    pub register_file_accesses: usize,
    pub memory_accesses: usize,
    pub forwarding_used: usize,
}

impl Pipeline {
    pub fn new(
        config: &PipelineConfig,
        instr_cache_config: CacheConfig,
        data_cache_config: CacheConfig,
        memory: Memory,
    ) -> Self {
        // Create pipeline stages
        let stage_types = vec![
            PipelineStageType::Fetch,
            PipelineStageType::Decode,
            PipelineStageType::Execute,
            PipelineStageType::Memory,
            PipelineStageType::Writeback,
        ];

        let mut stages = Vec::new();
        for (i, &stage_type) in stage_types.iter().enumerate() {
            let latency = if i < config.stage_latencies.len() {
                config.stage_latencies[i]
            } else {
                1
            };
            stages.push(PipelineStage::new(stage_type, latency));
        }

        // Create a cache hierarchy with L1 instruction and data caches
        let cache_hierarchy = CacheHierarchy::new(
            memory,
            data_cache_config,     // L1 data cache config
            instr_cache_config,   // L1 instruction cache config
            None                  // No L2 cache for now
        );

        // Initialize hazard statistics
        let hazard_stats = vec![
            (HazardType::RAW, 0),
            (HazardType::WAR, 0),
            (HazardType::WAW, 0),
            (HazardType::Control, 0),
            (HazardType::Structural, 0),
        ];

        Self {
            stages,
            cache_hierarchy,
            branch_predictor: BranchPredictor::new(),
            forwarding_enabled: true,
            stall_cycles: 0,
            branch_mispredictions: 0,
            instruction_count: 0,
            cycle_count: 0,
            stall_count: 0,
            cache_miss_stalls: 0,
            data_hazard_stalls: 0,
            control_hazard_stalls: 0,
            structural_hazard_stalls: 0,
            hazard_stats,
            register_file_accesses: 0,
            memory_accesses: 0,
            forwarding_used: 0,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction, _registers: &Registers, pc: u32) -> usize {
        self.instruction_count += 1;
        
        // Track how many cycles it takes to execute this instruction
        let start_cycle = self.cycle_count;
        
        // Insert instruction into fetch stage
        self.stages[0].start_instruction(instruction.clone(), pc);
        
        // Run the pipeline until this instruction completes
        let mut completed = false;
        let mut cycles = 0;
        
        while !completed && cycles < 100 { // Limit to prevent infinite loops
            self.tick();
            cycles += 1;
            
            // Check if the instruction has reached Writeback stage and completed
            if self.stages.last().unwrap().instruction.is_some() && 
               self.stages.last().unwrap().is_ready() {
                completed = true;
            }
        }
        
        let total_cycles = self.cycle_count - start_cycle;
        total_cycles
    }

    pub fn tick(&mut self) {
        self.cycle_count += 1;
        
        // First, handle stalls
        if self.stall_cycles > 0 {
            self.stall_cycles -= 1;
            self.stall_count += 1;
            return;
        }
        
        // Process each stage in reverse order (to prevent data loss)
        for i in (0..self.stages.len()).rev() {
            self.stages[i].tick();
        }
        
        // Check for hazards
        let hazards = self.detect_hazards();
        if !hazards.is_empty() {
            // Handle hazards
            self.handle_hazards(&hazards);
        } else {
            // No hazards, advance the pipeline
            self.advance_pipeline();
        }
    }

    fn detect_hazards(&self) -> Vec<(HazardType, usize)> {
        let mut hazards = Vec::new();
        
        // Check for data hazards
        for i in 0..self.stages.len() - 1 {
            let current_stage = &self.stages[i];
            if current_stage.status != PipelineStageStatus::Busy {
                continue;
            }
            
            if let Some(ref current_instr) = current_stage.instruction {
                // Get source registers for this instruction
                let source_regs = current_instr.get_source_registers();
                
                // Check if any of the source registers are being written by later stages
                for j in i + 1..self.stages.len() {
                    let later_stage = &self.stages[j];
                    if later_stage.status != PipelineStageStatus::Busy {
                        continue;
                    }
                    
                    if let Some(target_reg) = later_stage.target_register {
                        if source_regs.contains(&target_reg) {
                            // RAW hazard
                            hazards.push((HazardType::RAW, i));
                        }
                    }
                    
                    // Check for WAR and WAW hazards
                    if let Some(current_target_reg) = current_stage.target_register {
                        if let Some(ref later_instr) = later_stage.instruction {
                            let later_source_regs = later_instr.get_source_registers();
                            if later_source_regs.contains(&current_target_reg) {
                                // WAR hazard
                                hazards.push((HazardType::WAR, i));
                            }
                            
                            if let Some(later_target_reg) = later_stage.target_register {
                                if later_target_reg == current_target_reg {
                                    // WAW hazard
                                    hazards.push((HazardType::WAW, i));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Check for control hazards
        for i in 0..self.stages.len() {
            let stage = &self.stages[i];
            if stage.status == PipelineStageStatus::Busy {
                if let Some(ref instr) = stage.instruction {
                    if instr.is_branch_or_jump() {
                        hazards.push((HazardType::Control, i));
                    }
                }
            }
        }
        
        // Check for structural hazards (e.g., multiple memory accesses)
        let mut memory_stages = Vec::new();
        for i in 0..self.stages.len() {
            let stage = &self.stages[i];
            if stage.status == PipelineStageStatus::Busy {
                if let Some(ref instr) = stage.instruction {
                    if instr.is_memory_access() {
                        memory_stages.push(i);
                    }
                }
            }
        }
        
        if memory_stages.len() > 1 {
            // Multiple memory accesses at the same time
            hazards.push((HazardType::Structural, memory_stages[0]));
        }
        
        hazards
    }

    fn handle_hazards(&mut self, hazards: &[(HazardType, usize)]) {
        for &(hazard_type, stage_idx) in hazards {
            match hazard_type {
                HazardType::RAW => {
                    // Try forwarding first
                    if self.forwarding_enabled {
                        if self.try_forwarding(stage_idx) {
                            // Forwarding successful, no need to stall
                            self.forwarding_used += 1;
                            continue;
                        }
                    }
                    
                    // Forwarding failed or disabled, need to stall
                    self.stall_pipeline(stage_idx);
                    self.data_hazard_stalls += 1;
                    
                    // Update hazard statistics
                    for i in 0..self.hazard_stats.len() {
                        if self.hazard_stats[i].0 == HazardType::RAW {
                            self.hazard_stats[i].1 += 1;
                            break;
                        }
                    }
                },
                HazardType::WAR | HazardType::WAW => {
                    // Stall the pipeline
                    self.stall_pipeline(stage_idx);
                    self.data_hazard_stalls += 1;
                    
                    // Update hazard statistics
                    for i in 0..self.hazard_stats.len() {
                        if self.hazard_stats[i].0 == hazard_type {
                            self.hazard_stats[i].1 += 1;
                            break;
                        }
                    }
                },
                HazardType::Control => {
                    // Handle branch hazard
                    let branch_taken = self.predict_branch(stage_idx);
                    
                    if branch_taken {
                        // Flush earlier pipeline stages
                        self.flush_pipeline(0, stage_idx);
                    }
                    
                    self.control_hazard_stalls += 1;
                    
                    // Update hazard statistics
                    for i in 0..self.hazard_stats.len() {
                        if self.hazard_stats[i].0 == HazardType::Control {
                            self.hazard_stats[i].1 += 1;
                            break;
                        }
                    }
                },
                HazardType::Structural => {
                    // Stall the pipeline
                    self.stall_pipeline(stage_idx);
                    self.structural_hazard_stalls += 1;
                    
                    // Update hazard statistics
                    for i in 0..self.hazard_stats.len() {
                        if self.hazard_stats[i].0 == HazardType::Structural {
                            self.hazard_stats[i].1 += 1;
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn advance_pipeline(&mut self) {
        // Move instructions from one stage to the next
        for i in (1..self.stages.len()).rev() {
            let prev_idx = i - 1;
            
            if self.stages[prev_idx].is_ready() && self.stages[i].status == PipelineStageStatus::Empty {
                // Transfer instruction to next stage
                let instr = self.stages[prev_idx].instruction.clone();
                let pc = self.stages[prev_idx].pc;
                let data = self.stages[prev_idx].data;
                let target_reg = self.stages[prev_idx].target_register;
                let memory_addr = self.stages[prev_idx].memory_address;
                
                self.stages[i].start_instruction(instr.unwrap(), pc);
                self.stages[i].data = data;
                self.stages[i].target_register = target_reg;
                self.stages[i].memory_address = memory_addr;
                
                // Reset previous stage
                self.stages[prev_idx].reset();
            }
        }
        
        // Fetch new instruction
        if self.stages[0].status == PipelineStageStatus::Empty {
            // In a real implementation, this would fetch from memory
            // This is handled elsewhere in our simulator
        }
    }

    fn try_forwarding(&mut self, stage_idx: usize) -> bool {
        if stage_idx >= self.stages.len() {
            return false;
        }
        
        let current_stage = &self.stages[stage_idx];
        
        if let Some(ref instr) = current_stage.instruction {
            let source_regs = instr.get_source_registers();
            
            // Check if any later stage has the data we need
            for i in stage_idx + 1..self.stages.len() {
                let later_stage = &self.stages[i];
                
                if later_stage.status != PipelineStageStatus::Busy || later_stage.data.is_none() {
                    continue;
                }
                
                if let Some(target_reg) = later_stage.target_register {
                    if source_regs.contains(&target_reg) {
                        // We can forward the data!
                        return true;
                    }
                }
            }
        }
        
        false
    }

    fn stall_pipeline(&mut self, from_stage: usize) {
        // Stall this stage and all earlier stages
        for i in 0..=from_stage {
            self.stages[i].stall();
        }
        
        self.stall_cycles += 1;
    }

    fn flush_pipeline(&mut self, from_stage: usize, to_stage: usize) {
        // Flush stages between from_stage and to_stage (exclusive)
        for i in from_stage..to_stage {
            self.stages[i].flush();
        }
    }

    fn predict_branch(&mut self, stage_idx: usize) -> bool {
        let stage = &self.stages[stage_idx];
        
        if let Some(ref instr) = stage.instruction {
            if instr.is_branch_or_jump() {
                // Use branch predictor to predict if branch is taken
                let prediction = self.branch_predictor.predict(stage.pc);
                
                // If we predict taken, we also need a predicted target address
                if prediction {
                    if let Some(_target) = self.branch_predictor.get_target(stage.pc) {
                        // We have a predicted target in our BTB
                        return true;
                    } else if let Some(_target) = instr.get_immediate_target() {
                        // For immediate targets (e.g., J instruction), we can compute the target
                        // This is a simplified example - in reality, we would compute the proper target
                        return true;
                    }
                }
                
                return prediction;
            }
        }
        
        false
    }

    pub fn flush(&mut self) {
        for stage in &mut self.stages {
            stage.flush();
        }
    }

    pub fn is_register_being_written(&self, reg_num: u32) -> bool {
        for stage in &self.stages {
            if stage.status == PipelineStageStatus::Busy || stage.status == PipelineStageStatus::Stalled {
                if let Some(target_reg) = stage.target_register {
                    if target_reg == reg_num {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn print_statistics(&self) -> String {
        let mut stats = String::new();
        
        stats.push_str(&format!("Pipeline Statistics:\n"));
        stats.push_str(&format!("  Total Instructions: {}\n", self.instruction_count));
        stats.push_str(&format!("  Total Cycles: {}\n", self.cycle_count));
        
        if self.instruction_count > 0 {
            let cpi = self.cycle_count as f32 / self.instruction_count as f32;
            stats.push_str(&format!("  Cycles Per Instruction (CPI): {:.2}\n", cpi));
        }
        
        stats.push_str(&format!("  Total Stalls: {}\n", self.stall_count));
        stats.push_str(&format!("    Data Hazard Stalls: {}\n", self.data_hazard_stalls));
        stats.push_str(&format!("    Control Hazard Stalls: {}\n", self.control_hazard_stalls));
        stats.push_str(&format!("    Structural Hazard Stalls: {}\n", self.structural_hazard_stalls));
        stats.push_str(&format!("    Cache Miss Stalls: {}\n", self.cache_miss_stalls));
        
        stats.push_str(&format!("  Branch Mispredictions: {}\n", self.branch_mispredictions));
        
        if self.forwarding_enabled {
            stats.push_str(&format!("  Forwarding Used: {} times\n", self.forwarding_used));
        }
        
        stats.push_str(&format!("  Register File Accesses: {}\n", self.register_file_accesses));
        stats.push_str(&format!("  Memory Accesses: {}\n", self.memory_accesses));
        
        stats.push_str(&format!("\nHazard Statistics:\n"));
        for &(hazard_type, count) in &self.hazard_stats {
            stats.push_str(&format!("  {:?}: {}\n", hazard_type, count));
        }
        
        // Add cache hierarchy statistics
        stats.push_str("\n");
        stats.push_str(&self.cache_hierarchy.print_stats());
        
        stats
    }

    pub fn visualize(&self, cycle: usize) -> String {
        let visualization = crate::timing_simulator::visualization::PipelineVisualization::new();
        visualization.visualize_pipeline(self, cycle)
    }
}

use super::branch_predictor::BranchPredictor;