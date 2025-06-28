// branch_predictor.rs

use std::collections::HashMap;

/// Branch prediction states using a 2-bit saturating counter scheme
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PredictionState {
    StronglyNotTaken = 0,
    WeaklyNotTaken = 1,
    WeaklyTaken = 2,
    StronglyTaken = 3,
}

impl PredictionState {
    pub fn is_taken(&self) -> bool {
        match self {
            PredictionState::WeaklyTaken | PredictionState::StronglyTaken => true,
            _ => false,
        }
    }
    
    pub fn update(&self, taken: bool) -> Self {
        match (self, taken) {
            (PredictionState::StronglyNotTaken, false) => PredictionState::StronglyNotTaken,
            (PredictionState::StronglyNotTaken, true) => PredictionState::WeaklyNotTaken,
            (PredictionState::WeaklyNotTaken, false) => PredictionState::StronglyNotTaken,
            (PredictionState::WeaklyNotTaken, true) => PredictionState::WeaklyTaken,
            (PredictionState::WeaklyTaken, false) => PredictionState::WeaklyNotTaken,
            (PredictionState::WeaklyTaken, true) => PredictionState::StronglyTaken,
            (PredictionState::StronglyTaken, false) => PredictionState::WeaklyTaken,
            (PredictionState::StronglyTaken, true) => PredictionState::StronglyTaken,
        }
    }
}

/// Enhanced branch predictor implementation that uses a 2-bit saturating counter
/// for more accurate predictions and implements both local and global history
pub struct BranchPredictor {
    /// Branch history table - maps PC to prediction state
    branch_history_table: HashMap<u32, PredictionState>,
    
    /// Global branch history register
    global_history: u8,
    
    /// Global pattern history table - indexed by global_history
    global_predictor: [PredictionState; 16],
    
    /// Branch target buffer - caches branch target addresses
    branch_target_buffer: HashMap<u32, u32>,
    
    /// Statistics
    predictions: usize,
    correct_predictions: usize,
}

impl BranchPredictor {
    pub fn new() -> Self {
        Self {
            branch_history_table: HashMap::new(),
            global_history: 0,
            global_predictor: [PredictionState::WeaklyNotTaken; 16],
            branch_target_buffer: HashMap::new(),
            predictions: 0,
            correct_predictions: 0,
        }
    }

    pub fn predict(&mut self, pc: u32) -> bool {
        self.predictions += 1;
        
        // Try local prediction first
        if let Some(&state) = self.branch_history_table.get(&pc) {
            return state.is_taken();
        }
        
        // Fall back to global prediction
        let index = (self.global_history & 0xF) as usize;
        self.global_predictor[index].is_taken()
    }

    pub fn update(&mut self, pc: u32, taken: bool, actual_target: u32) {
        // Update branch target buffer
        if taken {
            self.branch_target_buffer.insert(pc, actual_target);
        }
        
        // Update local predictor
        let local_state = self.branch_history_table
            .get(&pc)
            .cloned()
            .unwrap_or(PredictionState::WeaklyNotTaken);
            
        // Check if prediction was correct
        if local_state.is_taken() == taken {
            self.correct_predictions += 1;
        }
        
        // Update the state
        let new_state = local_state.update(taken);
        self.branch_history_table.insert(pc, new_state);
        
        // Update global history (shift left and add new outcome)
        self.global_history = ((self.global_history << 1) | (taken as u8)) & 0xF;
        
        // Update global predictor
        let index = (self.global_history & 0xF) as usize;
        self.global_predictor[index] = self.global_predictor[index].update(taken);
    }
    
    /// Get the predicted target address for a branch
    pub fn get_target(&self, pc: u32) -> Option<u32> {
        self.branch_target_buffer.get(&pc).cloned()
    }
    
    /// Get prediction accuracy statistics
    pub fn get_accuracy(&self) -> f32 {
        if self.predictions == 0 {
            return 0.0;
        }
        
        (self.correct_predictions as f32) / (self.predictions as f32)
    }
}
