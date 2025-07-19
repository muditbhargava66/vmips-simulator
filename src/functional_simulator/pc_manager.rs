use crate::errors::SimulatorError;

pub struct PcManager {
    pub pc: u32,
    pub next_pc: u32,
    pub branch_delay_slot: bool,
    pub branch_target: Option<u32>,
    pub memory_size: usize,
}

impl PcManager {
    pub fn new(memory_size: usize) -> Self {
        Self {
            pc: 0,
            next_pc: 4,
            branch_delay_slot: false,
            branch_target: None,
            memory_size,
        }
    }

    pub fn advance(&mut self) {
        self.pc = self.next_pc;

        if self.branch_delay_slot {
            // We're in a branch delay slot, so next PC should be the branch target
            if let Some(target) = self.branch_target {
                self.next_pc = target;
                self.branch_delay_slot = false;
                self.branch_target = None;
            } else {
                // This shouldn't happen, but just in case
                self.next_pc = self.pc.wrapping_add(4);
            }
        } else {
            // Normal sequential execution
            self.next_pc = self.pc.wrapping_add(4);
        }
    }

    pub fn set_branch_target(&mut self, target: u32) -> Result<(), SimulatorError> {
        // Validate branch target
        if target as usize >= self.memory_size {
            return Err(SimulatorError::InvalidBranchTarget(target));
        }

        if target % 4 != 0 {
            return Err(SimulatorError::MemoryMisaligned(target));
        }

        // Set up branch delay slot
        self.branch_delay_slot = true;
        self.branch_target = Some(target);
        Ok(())
    }

    pub fn calculate_branch_target(&self, offset: i16) -> Result<u32, SimulatorError> {
        // PC-relative addressing: PC + 4 + (offset * 4)
        let pc_plus_4 = self.pc.wrapping_add(4);
        let offset_bytes = (offset as i32) * 4;
        let target = pc_plus_4.wrapping_add(offset_bytes as u32);

        // Validate target address
        if target as usize >= self.memory_size {
            return Err(SimulatorError::InvalidBranchTarget(target));
        }

        if target % 4 != 0 {
            return Err(SimulatorError::MemoryMisaligned(target));
        }

        Ok(target)
    }

    pub fn calculate_jump_target(&self, target: u32) -> Result<u32, SimulatorError> {
        // Jump target: (PC & 0xF0000000) | (target << 2)
        let jump_addr = (self.pc & 0xF000_0000) | ((target & 0x03FF_FFFF) << 2);

        // Validate target address
        if jump_addr as usize >= self.memory_size {
            return Err(SimulatorError::InvalidBranchTarget(jump_addr));
        }

        Ok(jump_addr)
    }

    pub fn is_valid_address(&self, address: u32) -> bool {
        (address as usize) < self.memory_size && address % 4 == 0
    }

    pub fn get_current_pc(&self) -> u32 {
        self.pc
    }

    pub fn get_next_pc(&self) -> u32 {
        self.next_pc
    }

    pub fn is_in_delay_slot(&self) -> bool {
        self.branch_delay_slot
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.next_pc = 4;
        self.branch_delay_slot = false;
        self.branch_target = None;
    }

    pub fn set_pc(&mut self, new_pc: u32) -> Result<(), SimulatorError> {
        if !self.is_valid_address(new_pc) {
            return Err(SimulatorError::InvalidBranchTarget(new_pc));
        }

        self.pc = new_pc;
        self.next_pc = new_pc.wrapping_add(4);
        self.branch_delay_slot = false;
        self.branch_target = None;

        Ok(())
    }
}
