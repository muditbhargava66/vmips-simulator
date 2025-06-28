// tomasulo.rs - Tomasulo's Algorithm implementation for out-of-order execution
//
// Tomasulo's algorithm is a hardware algorithm for dynamic scheduling of
// instructions to allow out-of-order execution. This module implements:
//   - Reservation stations for holding instructions
//   - Register renaming with register alias table
//   - Common data bus for result forwarding
//   - Reorder buffer (ROB) for in-order commit

use std::collections::{VecDeque, HashMap};
use crate::functional_simulator::instructions::Instruction;
use crate::functional_simulator::registers::Registers;
use crate::functional_simulator::memory::Memory;
use std::fmt;

/// Status of an instruction in the pipeline
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionStatus {
    Waiting,    // Waiting for operands
    Executing,  // Currently executing
    Completed,  // Execution finished, waiting to commit
    Committed,  // Results written back to architectural state
}

/// Type of functional unit that can execute an instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionalUnitType {
    IntegerALU,
    FPAdder,
    FPMultiplier,
    FPDivider,
    LoadStore,
    Branch,
}

/// Reservation station entry
#[derive(Debug, Clone)]
pub struct ReservationStation {
    pub id: usize,
    pub busy: bool,
    pub instruction: Option<Instruction>,
    pub status: InstructionStatus,
    pub vj: Option<u32>,  // Value of first source operand
    pub vk: Option<u32>,  // Value of second source operand
    pub qj: Option<usize>, // Reservation station producing first operand
    pub qk: Option<usize>, // Reservation station producing second operand
    pub dest: Option<usize>, // Destination ROB entry
    pub address: Option<u32>, // Memory address (for loads/stores)
    pub cycles_remaining: usize,
}

impl ReservationStation {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            busy: false,
            instruction: None,
            status: InstructionStatus::Waiting,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            dest: None,
            address: None,
            cycles_remaining: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.busy && self.qj.is_none() && self.qk.is_none() && self.status == InstructionStatus::Waiting
    }

    pub fn reset(&mut self) {
        self.busy = false;
        self.instruction = None;
        self.status = InstructionStatus::Waiting;
        self.vj = None;
        self.vk = None;
        self.qj = None;
        self.qk = None;
        self.dest = None;
        self.address = None;
        self.cycles_remaining = 0;
    }
    
    pub fn issue(&mut self, instruction: Instruction, vj: Option<u32>, vk: Option<u32>,
                 qj: Option<usize>, qk: Option<usize>, dest: Option<usize>,
                 cycle_count: usize) {
        self.busy = true;
        self.instruction = Some(instruction);
        self.status = InstructionStatus::Waiting;
        self.vj = vj;
        self.vk = vk;
        self.qj = qj;
        self.qk = qk;
        self.dest = dest;
        self.address = None;
        self.cycles_remaining = cycle_count;
    }
    
    pub fn start_execution(&mut self) {
        if self.is_ready() {
            self.status = InstructionStatus::Executing;
        }
    }
    
    pub fn tick(&mut self) {
        if self.status == InstructionStatus::Executing && self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;
            if self.cycles_remaining == 0 {
                self.status = InstructionStatus::Completed;
            }
        }
    }
}

/// Reorder Buffer entry
#[derive(Debug, Clone)]
pub struct ReorderBufferEntry {
    pub id: usize,
    pub busy: bool,
    pub instruction: Option<Instruction>,
    pub status: InstructionStatus,
    pub dest: Option<u32>, // Destination register
    pub value: Option<u32>, // Result value
    pub address: Option<u32>, // Memory address (for loads/stores)
    pub predicted_target: Option<u32>, // For branches, the predicted target
    pub actual_target: Option<u32>,    // For branches, the actual target (if known)
    pub mispredicted: bool,            // Whether branch prediction was incorrect
}

impl ReorderBufferEntry {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            busy: false,
            instruction: None,
            status: InstructionStatus::Waiting,
            dest: None,
            value: None,
            address: None,
            predicted_target: None,
            actual_target: None,
            mispredicted: false,
        }
    }
    
    pub fn issue(&mut self, instruction: Instruction, dest: Option<u32>, 
                predicted_target: Option<u32>) {
        self.busy = true;
        self.instruction = Some(instruction);
        self.status = InstructionStatus::Waiting;
        self.dest = dest;
        self.value = None;
        self.address = None;
        self.predicted_target = predicted_target;
        self.actual_target = None;
        self.mispredicted = false;
    }
    
    pub fn complete(&mut self, value: u32, address: Option<u32>, 
                   actual_target: Option<u32>) {
        self.status = InstructionStatus::Completed;
        self.value = Some(value);
        self.address = address;
        self.actual_target = actual_target;
        
        // For branches, check if prediction was correct
        if let Some(actual) = actual_target {
            if let Some(predicted) = self.predicted_target {
                self.mispredicted = actual != predicted;
            } else {
                // If we didn't predict, but branch was taken
                self.mispredicted = true;
            }
        }
    }
    
    pub fn commit(&mut self) {
        self.status = InstructionStatus::Committed;
    }
    
    pub fn reset(&mut self) {
        self.busy = false;
        self.instruction = None;
        self.status = InstructionStatus::Waiting;
        self.dest = None;
        self.value = None;
        self.address = None;
        self.predicted_target = None;
        self.actual_target = None;
        self.mispredicted = false;
    }
    
    pub fn can_commit(&self) -> bool {
        self.busy && self.status == InstructionStatus::Completed
    }
}

/// Register Alias Table (RAT) for register renaming
#[derive(Debug, Clone)]
pub struct RegisterAliasTable {
    pub mapping: HashMap<u32, usize>, // Maps physical register -> ROB entry
}

impl RegisterAliasTable {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
    
    pub fn get_mapping(&self, reg: u32) -> Option<usize> {
        self.mapping.get(&reg).copied()
    }
    
    pub fn set_mapping(&mut self, reg: u32, rob_entry: usize) {
        self.mapping.insert(reg, rob_entry);
    }
    
    pub fn clear_mapping(&mut self, reg: u32) {
        self.mapping.remove(&reg);
    }
    
    pub fn clear_all(&mut self) {
        self.mapping.clear();
    }
}

/// Common Data Bus (CDB) for broadcasting results
#[derive(Debug, Clone)]
pub struct CommonDataBus {
    pub data: Option<(usize, u32)>, // (producing_rs, value)
}

impl CommonDataBus {
    pub fn new() -> Self {
        Self {
            data: None,
        }
    }
    
    pub fn broadcast(&mut self, rs_id: usize, value: u32) {
        self.data = Some((rs_id, value));
    }
    
    pub fn clear(&mut self) {
        self.data = None;
    }
}

/// Functional unit for executing instructions
#[derive(Debug, Clone)]
pub struct FunctionalUnit {
    pub unit_type: FunctionalUnitType,
    pub reservation_station: Option<usize>,
    pub busy: bool,
    pub cycles_remaining: usize,
}

impl FunctionalUnit {
    pub fn new(unit_type: FunctionalUnitType) -> Self {
        Self {
            unit_type,
            reservation_station: None,
            busy: false,
            cycles_remaining: 0,
        }
    }
    
    pub fn is_available(&self) -> bool {
        !self.busy
    }
    
    pub fn start_execution(&mut self, rs_id: usize, cycles: usize) {
        self.reservation_station = Some(rs_id);
        self.busy = true;
        self.cycles_remaining = cycles;
    }
    
    pub fn tick(&mut self) -> bool {
        if self.busy && self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;
            if self.cycles_remaining == 0 {
                // Execution completed
                let _rs_id = self.reservation_station;
                self.reset();
                return true; // Signal completion
            }
        }
        false
    }
    
    pub fn reset(&mut self) {
        self.reservation_station = None;
        self.busy = false;
        self.cycles_remaining = 0;
    }
}

/// Tomasulo processor implementing out-of-order execution
pub struct TomasuloProcessor {
    // Hardware resources
    pub reservation_stations: Vec<ReservationStation>,
    pub reorder_buffer: VecDeque<ReorderBufferEntry>,
    pub register_alias_table: RegisterAliasTable,
    pub common_data_bus: CommonDataBus,
    pub functional_units: Vec<FunctionalUnit>,
    
    // Configuration
    pub num_reservation_stations: usize,
    pub rob_size: usize,
    
    // Mapping of functional unit types to latencies
    pub latency_map: HashMap<FunctionalUnitType, usize>,
    
    // Current architectural state
    pub registers: Registers,
    pub memory: Memory,
    
    // Performance counters
    pub instructions_issued: usize,
    pub instructions_executed: usize,
    pub instructions_committed: usize,
    pub cycles: usize,
    pub branch_mispredictions: usize,
}

impl TomasuloProcessor {
    pub fn new(num_rs: usize, rob_size: usize, registers: Registers, memory: Memory) -> Self {
        // Create reservation stations
        let mut reservation_stations = Vec::with_capacity(num_rs);
        for i in 0..num_rs {
            reservation_stations.push(ReservationStation::new(i));
        }
        
        // Create reorder buffer
        let mut reorder_buffer = VecDeque::with_capacity(rob_size);
        for i in 0..rob_size {
            reorder_buffer.push_back(ReorderBufferEntry::new(i));
        }
        
        // Create functional units
        let functional_units = vec![
            FunctionalUnit::new(FunctionalUnitType::IntegerALU),
            FunctionalUnit::new(FunctionalUnitType::IntegerALU),
            FunctionalUnit::new(FunctionalUnitType::FPAdder),
            FunctionalUnit::new(FunctionalUnitType::FPMultiplier),
            FunctionalUnit::new(FunctionalUnitType::FPDivider),
            FunctionalUnit::new(FunctionalUnitType::LoadStore),
            FunctionalUnit::new(FunctionalUnitType::Branch),
        ];
        
        // Set up latency map
        let mut latency_map = HashMap::new();
        latency_map.insert(FunctionalUnitType::IntegerALU, 1);
        latency_map.insert(FunctionalUnitType::FPAdder, 3);
        latency_map.insert(FunctionalUnitType::FPMultiplier, 5);
        latency_map.insert(FunctionalUnitType::FPDivider, 10);
        latency_map.insert(FunctionalUnitType::LoadStore, 2);
        latency_map.insert(FunctionalUnitType::Branch, 1);
        
        Self {
            reservation_stations,
            reorder_buffer,
            register_alias_table: RegisterAliasTable::new(),
            common_data_bus: CommonDataBus::new(),
            functional_units,
            num_reservation_stations: num_rs,
            rob_size,
            latency_map,
            registers,
            memory,
            instructions_issued: 0,
            instructions_executed: 0,
            instructions_committed: 0,
            cycles: 0,
            branch_mispredictions: 0,
        }
    }
    
    /// Main processor cycle
    pub fn tick(&mut self) {
        self.cycles += 1;
        
        // 1. Process common data bus broadcasts from previous cycle
        self.process_cdb();
        
        // 2. Execute ready instructions
        self.execute_instructions();
        
        // 3. Commit completed instructions
        self.commit_instructions();
        
        // 4. Clear CDB for next cycle
        self.common_data_bus.clear();
    }
    
    /// Process broadcasts on the common data bus
    fn process_cdb(&mut self) {
        if let Some((producing_rs, value)) = self.common_data_bus.data {
            // Update any reservation stations waiting on this result
            for rs in &mut self.reservation_stations {
                if rs.busy {
                    if rs.qj == Some(producing_rs) {
                        rs.vj = Some(value);
                        rs.qj = None;
                    }
                    if rs.qk == Some(producing_rs) {
                        rs.vk = Some(value);
                        rs.qk = None;
                    }
                }
            }
            
            // Update ROB entry with the result
            let producing_rob = self.get_rob_entry_for_rs(producing_rs);
            if let Some(rob_idx) = producing_rob {
                if rob_idx < self.reorder_buffer.len() {
                    let entry = &mut self.reorder_buffer[rob_idx];
                    entry.value = Some(value);
                    entry.status = InstructionStatus::Completed;
                }
            }
        }
    }
    
    /// Execute instructions in functional units and broadcast results
    fn execute_instructions(&mut self) {
        // Update existing executions
        for fu in &mut self.functional_units {
            if fu.busy {
                let completed = fu.tick();
                if completed {
                    // Execution finished, get the reservation station
                    if let Some(rs_id) = fu.reservation_station {
                        if rs_id < self.reservation_stations.len() {
                            // Extract instruction before computing result to avoid double borrow
                            let instruction_clone = self.reservation_stations[rs_id].instruction.clone();
                            
                            // Update status
                            self.reservation_stations[rs_id].status = InstructionStatus::Completed;
                            
                            // Compute result based on instruction type
                            if let Some(instr) = instruction_clone {
                                // Compute result without borrowing 'self' twice
                                let result = match instr {
                                    Instruction::Add { .. } => {
                                        let vj = self.reservation_stations[rs_id].vj.unwrap_or(0);
                                        let vk = self.reservation_stations[rs_id].vk.unwrap_or(0);
                                        vj.wrapping_add(vk)
                                    },
                                    Instruction::Sub { .. } => {
                                        let vj = self.reservation_stations[rs_id].vj.unwrap_or(0);
                                        let vk = self.reservation_stations[rs_id].vk.unwrap_or(0);
                                        vj.wrapping_sub(vk)
                                    },
                                    Instruction::And { .. } => {
                                        let vj = self.reservation_stations[rs_id].vj.unwrap_or(0);
                                        let vk = self.reservation_stations[rs_id].vk.unwrap_or(0);
                                        vj & vk
                                    },
                                    Instruction::Or { .. } => {
                                        let vj = self.reservation_stations[rs_id].vj.unwrap_or(0);
                                        let vk = self.reservation_stations[rs_id].vk.unwrap_or(0);
                                        vj | vk
                                    },
                                    Instruction::Slt { .. } => {
                                        let vj = self.reservation_stations[rs_id].vj.unwrap_or(0) as i32;
                                        let vk = self.reservation_stations[rs_id].vk.unwrap_or(0) as i32;
                                        (vj < vk) as u32
                                    },
                                    // Add more instruction types as needed
                                    _ => 0, // Default
                                };
                                
                                // Broadcast result on CDB
                                self.common_data_bus.broadcast(rs_id, result);
                                
                                self.instructions_executed += 1;
                            }
                        }
                    }
                    fu.reset();
                }
            }
        }
        
        // Start new executions
        for rs_idx in 0..self.reservation_stations.len() {
            let rs = &self.reservation_stations[rs_idx];
            if rs.is_ready() {
                // Find an available functional unit of the right type
                if let Some(ref instr) = rs.instruction {
                    let fu_type = self.get_functional_unit_type(instr);
                    
                    // Find an available FU of the right type
                    for fu in &mut self.functional_units {
                        if fu.is_available() && fu.unit_type == fu_type {
                            // Start execution
                            let latency = self.latency_map[&fu_type];
                            fu.start_execution(rs_idx, latency);
                            
                            // Update RS status
                            let rs = &mut self.reservation_stations[rs_idx];
                            rs.status = InstructionStatus::Executing;
                            rs.cycles_remaining = latency;
                            
                            break;
                        }
                    }
                }
            }
        }
    }
    
    /// Compute result of an instruction execution
    #[allow(dead_code)]
    fn compute_result(&mut self, rs: &ReservationStation, instr: &Instruction) -> u32 {
        match instr {
            Instruction::Add { .. } => {
                let vj = rs.vj.unwrap_or(0);
                let vk = rs.vk.unwrap_or(0);
                vj.wrapping_add(vk)
            },
            Instruction::Sub { .. } => {
                let vj = rs.vj.unwrap_or(0);
                let vk = rs.vk.unwrap_or(0);
                vj.wrapping_sub(vk)
            },
            Instruction::And { .. } => {
                let vj = rs.vj.unwrap_or(0);
                let vk = rs.vk.unwrap_or(0);
                vj & vk
            },
            Instruction::Or { .. } => {
                let vj = rs.vj.unwrap_or(0);
                let vk = rs.vk.unwrap_or(0);
                vj | vk
            },
            Instruction::Slt { .. } => {
                let vj = rs.vj.unwrap_or(0) as i32;
                let vk = rs.vk.unwrap_or(0) as i32;
                (vj < vk) as u32
            },
            // Add more instruction types as needed
            _ => 0, // Default
        }
    }
    
    /// Commit completed instructions from ROB
    fn commit_instructions(&mut self) {
        // Check head of ROB for instructions ready to commit
        let mut committed = 0;
        while !self.reorder_buffer.is_empty() && committed < 4 { // Commit up to 4 instructions per cycle
            let can_commit = {
                let entry = &self.reorder_buffer[0];
                entry.busy && entry.status == InstructionStatus::Completed
            };
            
            if can_commit {
                // Get entry at head of ROB
                let mut entry = self.reorder_buffer.pop_front().unwrap();
                
                // Check for branch misprediction
                if entry.mispredicted {
                    self.branch_mispredictions += 1;
                    self.handle_branch_misprediction();
                    
                    // We need to stop committing after a misprediction
                    break;
                }
                
                // Commit instruction by updating architectural state
                if let Some(dest_reg) = entry.dest {
                    if let Some(value) = entry.value {
                        self.registers.write(dest_reg, value);
                    }
                    
                    // Update RAT if this entry is still mapped
                    if let Some(mapped_entry) = self.register_alias_table.get_mapping(dest_reg) {
                        if mapped_entry == entry.id {
                            self.register_alias_table.clear_mapping(dest_reg);
                        }
                    }
                }
                
                entry.status = InstructionStatus::Committed;
                self.instructions_committed += 1;
                committed += 1;
                
                // Add the now-free entry back to the end of the ROB
                entry.reset();
                self.reorder_buffer.push_back(entry);
            } else {
                // If we can't commit the head, we can't commit anything
                break;
            }
        }
    }
    
    /// Handle a branch misprediction by flushing the pipeline
    fn handle_branch_misprediction(&mut self) {
        // Clear all reservation stations
        for rs in &mut self.reservation_stations {
            rs.reset();
        }
        
        // Clear all functional units
        for fu in &mut self.functional_units {
            fu.reset();
        }
        
        // Clear ROB entries after the mispredicted branch
        while self.reorder_buffer.len() > 1 { // Keep the entry at the head
            self.reorder_buffer.pop_back();
        }
        
        // Clear register alias table
        self.register_alias_table.clear_all();
        
        // Clear CDB
        self.common_data_bus.clear();
    }
    
    /// Issue an instruction to the processor
    pub fn issue(&mut self, instruction: Instruction, pc: u32) -> bool {
        // Check if ROB has space
        let rob_entries_used = self.reorder_buffer.iter().filter(|e| e.busy).count();
        if rob_entries_used >= self.rob_size {
            return false; // ROB full
        }
        
        // Find a free reservation station
        let rs_idx = self.find_free_reservation_station();
        if rs_idx.is_none() {
            return false; // No free reservation station
        }
        let rs_idx = rs_idx.unwrap();
        
        // Find a free ROB entry
        let rob_idx = self.find_free_rob_entry();
        if rob_idx.is_none() {
            return false; // No free ROB entry
        }
        let rob_idx = rob_idx.unwrap();
        
        // Get source and destination registers
        let src_regs = instruction.get_source_registers();
        let dest_reg = instruction.get_destination_register();
        
        // Get values or dependencies for source registers
        let mut vj = None;
        let mut vk = None;
        let mut qj = None;
        let mut qk = None;
        
        // Handle source operands
        if !src_regs.is_empty() {
            // First source register
            let rs1 = src_regs[0];
            if let Some(rob_entry) = self.register_alias_table.get_mapping(rs1) {
                // Register is mapped to an ROB entry
                if self.reorder_buffer[rob_entry].status == InstructionStatus::Completed {
                    // Value is available
                    vj = self.reorder_buffer[rob_entry].value;
                } else {
                    // Value is not available, set dependency
                    qj = Some(rob_entry);
                }
            } else {
                // Register value is available in register file
                vj = Some(self.registers.read(rs1));
            }
            
            // Second source register (if any)
            if src_regs.len() > 1 {
                let rs2 = src_regs[1];
                if let Some(rob_entry) = self.register_alias_table.get_mapping(rs2) {
                    // Register is mapped to an ROB entry
                    if self.reorder_buffer[rob_entry].status == InstructionStatus::Completed {
                        // Value is available
                        vk = self.reorder_buffer[rob_entry].value;
                    } else {
                        // Value is not available, set dependency
                        qk = Some(rob_entry);
                    }
                } else {
                    // Register value is available in register file
                    vk = Some(self.registers.read(rs2));
                }
            }
        }
        
        // Predict branch target if this is a branch/jump
        let predicted_target = if instruction.is_branch_or_jump() {
            instruction.get_immediate_target().map(|t| t + pc)
        } else {
            None
        };
        
        // Issue to reservation station
        self.reservation_stations[rs_idx].issue(
            instruction.clone(),
            vj, vk, qj, qk,
            Some(rob_idx),
            0
        );
        
        // Add to reorder buffer
        self.reorder_buffer[rob_idx].issue(
            instruction,
            dest_reg,
            predicted_target
        );
        
        // Update register alias table if instruction has a destination register
        if let Some(dr) = dest_reg {
            self.register_alias_table.set_mapping(dr, rob_idx);
        }
        
        self.instructions_issued += 1;
        true
    }
    
    /// Find a free reservation station
    fn find_free_reservation_station(&self) -> Option<usize> {
        for (i, rs) in self.reservation_stations.iter().enumerate() {
            if !rs.busy {
                return Some(i);
            }
        }
        None
    }
    
    /// Find a free ROB entry
    fn find_free_rob_entry(&self) -> Option<usize> {
        for (i, entry) in self.reorder_buffer.iter().enumerate() {
            if !entry.busy {
                return Some(i);
            }
        }
        None
    }
    
    /// Get the ROB entry associated with a reservation station
    fn get_rob_entry_for_rs(&self, rs_id: usize) -> Option<usize> {
        if rs_id < self.reservation_stations.len() {
            self.reservation_stations[rs_id].dest
        } else {
            None
        }
    }
    
    /// Get the appropriate functional unit type for an instruction
    fn get_functional_unit_type(&self, instruction: &Instruction) -> FunctionalUnitType {
        match *instruction {
            Instruction::Add { .. } |
            Instruction::Sub { .. } |
            Instruction::And { .. } |
            Instruction::Or { .. } |
            Instruction::Xor { .. } |
            Instruction::Slt { .. } |
            Instruction::Addi { .. } |
            Instruction::Addiu { .. } => FunctionalUnitType::IntegerALU,
            
            Instruction::Lw { .. } |
            Instruction::Sw { .. } |
            Instruction::Lb { .. } |
            Instruction::Sb { .. } => FunctionalUnitType::LoadStore,
            
            Instruction::AddS { .. } |
            Instruction::SubS { .. } => FunctionalUnitType::FPAdder,
            
            Instruction::MulS { .. } |
            Instruction::Mult { .. } => FunctionalUnitType::FPMultiplier,
            
            Instruction::DivS { .. } |
            Instruction::Div { .. } |
            Instruction::Divu { .. } => FunctionalUnitType::FPDivider,
            
            Instruction::Beq { .. } |
            Instruction::Bne { .. } |
            Instruction::J { .. } |
            Instruction::Jal { .. } |
            Instruction::Jr { .. } => FunctionalUnitType::Branch,
            
            _ => FunctionalUnitType::IntegerALU, // Default
        }
    }
    
    /// Get the processor's performance statistics
    pub fn get_stats(&self) -> TomasuloStats {
        TomasuloStats {
            cycles: self.cycles,
            instructions_issued: self.instructions_issued,
            instructions_executed: self.instructions_executed,
            instructions_committed: self.instructions_committed,
            branch_mispredictions: self.branch_mispredictions,
            ipc: if self.cycles > 0 {
                self.instructions_committed as f32 / self.cycles as f32
            } else {
                0.0
            },
            reservation_station_utilization: self.reservation_stations.iter()
                .filter(|rs| rs.busy).count() as f32 / self.num_reservation_stations as f32,
            rob_utilization: self.reorder_buffer.iter()
                .filter(|entry| entry.busy).count() as f32 / self.rob_size as f32,
        }
    }
    
    /// Get a formatted dump of the processor state
    pub fn dump_state(&self) -> String {
        let mut result = String::new();
        
        // Add ROB state
        result.push_str("=== Reorder Buffer ===\n");
        for (i, entry) in self.reorder_buffer.iter().enumerate() {
            if entry.busy {
                result.push_str(&format!("[{}] {}: {:?} Dest: {:?} Value: {:?} Status: {:?}\n",
                    i,
                    if i == 0 { "HEAD" } else { "    " },
                    entry.instruction,
                    entry.dest,
                    entry.value,
                    entry.status
                ));
            }
        }
        
        // Add Reservation Station state
        result.push_str("\n=== Reservation Stations ===\n");
        for rs in &self.reservation_stations {
            if rs.busy {
                result.push_str(&format!("[{}] {:?} Vj: {:?} Vk: {:?} Qj: {:?} Qk: {:?} Dest: {:?} Status: {:?}\n",
                    rs.id,
                    rs.instruction,
                    rs.vj,
                    rs.vk,
                    rs.qj,
                    rs.qk,
                    rs.dest,
                    rs.status
                ));
            }
        }
        
        // Add Functional Unit state
        result.push_str("\n=== Functional Units ===\n");
        for (i, fu) in self.functional_units.iter().enumerate() {
            result.push_str(&format!("[{}] {:?} Busy: {} RS: {:?} Cycles Left: {}\n",
                i,
                fu.unit_type,
                fu.busy,
                fu.reservation_station,
                fu.cycles_remaining
            ));
        }
        
        // Add Register Alias Table
        result.push_str("\n=== Register Alias Table ===\n");
        for (reg, rob_entry) in &self.register_alias_table.mapping {
            result.push_str(&format!("r{} -> ROB[{}]\n", reg, rob_entry));
        }
        
        result
    }
}

/// Performance statistics for Tomasulo's algorithm
pub struct TomasuloStats {
    pub cycles: usize,
    pub instructions_issued: usize,
    pub instructions_executed: usize,
    pub instructions_committed: usize,
    pub branch_mispredictions: usize,
    pub ipc: f32,
    pub reservation_station_utilization: f32,
    pub rob_utilization: f32,
}

impl fmt::Display for TomasuloStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tomasulo's Algorithm Statistics:\n")?;
        write!(f, "  Cycles: {}\n", self.cycles)?;
        write!(f, "  Instructions Issued: {}\n", self.instructions_issued)?;
        write!(f, "  Instructions Executed: {}\n", self.instructions_executed)?;
        write!(f, "  Instructions Committed: {}\n", self.instructions_committed)?;
        write!(f, "  Instructions Per Cycle (IPC): {:.2}\n", self.ipc)?;
        write!(f, "  Branch Mispredictions: {}\n", self.branch_mispredictions)?;
        write!(f, "  Reservation Station Utilization: {:.2}%\n", self.reservation_station_utilization * 100.0)?;
        write!(f, "  Reorder Buffer Utilization: {:.2}%\n", self.rob_utilization * 100.0)
    }
}
