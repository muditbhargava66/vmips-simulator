use std::collections::{HashMap, HashSet};

pub struct RegisterAllocator {
    pub used_registers: HashSet<u32>,
    pub register_values: HashMap<u32, u32>,
    pub register_lifetimes: HashMap<u32, (u32, u32)>, // (first_use, last_use)
    pub spilled_registers: HashMap<u32, u32>,         // reg -> memory_addr
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            used_registers: HashSet::new(),
            register_values: HashMap::new(),
            register_lifetimes: HashMap::new(),
            spilled_registers: HashMap::new(),
        }
    }

    pub fn analyze_register_usage(&mut self, instructions: &[u32]) {
        self.used_registers.clear();
        self.register_lifetimes.clear();

        for (i, &instruction) in instructions.iter().enumerate() {
            let pc = i as u32 * 4;

            // Extract registers used in this instruction
            let opcode = instruction >> 26;

            if opcode == 0 {
                // R-type instruction
                let rs = (instruction >> 21) & 0x1F;
                let rt = (instruction >> 16) & 0x1F;
                let rd = (instruction >> 11) & 0x1F;

                self.record_register_usage(rs, pc);
                self.record_register_usage(rt, pc);
                self.record_register_usage(rd, pc);
            } else {
                // I-type or J-type instruction
                let rs = (instruction >> 21) & 0x1F;
                let rt = (instruction >> 16) & 0x1F;

                self.record_register_usage(rs, pc);
                self.record_register_usage(rt, pc);
            }
        }
    }

    fn record_register_usage(&mut self, reg: u32, pc: u32) {
        if reg == 0 {
            // $zero register is not tracked
            return;
        }

        self.used_registers.insert(reg);

        if let Some((first_use, _)) = self.register_lifetimes.get(&reg) {
            // Update last use
            self.register_lifetimes.insert(reg, (*first_use, pc));
        } else {
            // First use
            self.register_lifetimes.insert(reg, (pc, pc));
        }
    }

    pub fn allocate_registers(&mut self) -> HashMap<u32, u32> {
        // Simple register allocation strategy
        // In a real implementation, this would be more sophisticated
        let mut allocation = HashMap::new();
        let available_registers: Vec<u32> = (8..16).collect(); // $t0-$t7

        let mut register_priority: Vec<(u32, (u32, u32))> = self
            .register_lifetimes
            .iter()
            .map(|(&reg, &lifetime)| (reg, lifetime))
            .collect();

        // Sort by lifetime span (shorter lifetimes get priority)
        register_priority.sort_by_key(|(_, (first, last))| last - first);

        for (i, (reg, _)) in register_priority.iter().enumerate() {
            if i < available_registers.len() {
                allocation.insert(*reg, available_registers[i]);
            } else {
                // Need to spill this register to memory
                let memory_addr = 0x7000 + (i as u32 - available_registers.len() as u32) * 4;
                self.spilled_registers.insert(*reg, memory_addr);
            }
        }

        allocation
    }

    pub fn get_register_pressure(&self) -> f32 {
        // Calculate register pressure as a percentage
        let total_available = 24; // $t0-$t7, $s0-$s7, $a0-$a3, $v0-$v1
        let used_count = self.used_registers.len();

        (used_count as f32 / total_available as f32) * 100.0
    }

    pub fn suggest_optimizations(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        if self.get_register_pressure() > 80.0 {
            suggestions.push(
                "High register pressure detected. Consider reducing variable scope.".to_string(),
            );
        }

        if !self.spilled_registers.is_empty() {
            suggestions.push(format!(
                "Register spilling detected for {} registers. Consider code restructuring.",
                self.spilled_registers.len()
            ));
        }

        // Analyze register lifetime overlaps
        let mut overlapping_lifetimes = 0;
        let lifetimes: Vec<_> = self.register_lifetimes.values().collect();

        for i in 0..lifetimes.len() {
            for j in (i + 1)..lifetimes.len() {
                let (start1, end1) = lifetimes[i];
                let (start2, end2) = lifetimes[j];

                // Check if lifetimes overlap
                if start1 <= end2 && start2 <= end1 {
                    overlapping_lifetimes += 1;
                }
            }
        }

        if overlapping_lifetimes > 10 {
            suggestions.push(
                "Many overlapping register lifetimes detected. Consider reordering operations."
                    .to_string(),
            );
        }

        suggestions
    }
}

impl Default for RegisterAllocator {
    fn default() -> Self {
        Self::new()
    }
}
