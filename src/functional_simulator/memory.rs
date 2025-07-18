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

// memory.rs
//
// This file contains the memory implementation for the MIPS simulator.
// It defines the Memory struct, which manages the simulated memory space,
// including memory-mapped devices and memory regions with different
// access permissions.

use std::collections::HashMap;

/// Configuration options for memory behavior
#[derive(Clone, Copy, Debug)]
pub struct MemoryConfig {
    pub strict_alignment: bool,   // Enforce strict alignment checks
    pub verbose_errors: bool,     // Print detailed error messages
    pub enable_permissions: bool, // Enable memory region permissions
    pub enable_translation: bool, // Enable virtual address translation
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            strict_alignment: false,  // Allow unaligned access for compatibility
            verbose_errors: false,    // Reduce noise in tests
            enable_permissions: true, // Enable permissions by default
            enable_translation: true, // Enable translation by default
        }
    }
}

/// Advanced memory implementation with virtual address translation and memory regions
pub struct Memory {
    pub data: Vec<u8>,
    pub size: usize,
    heap_top: usize,
    memory_regions: Vec<MemoryRegion>,
    mapped_devices: HashMap<usize, Box<dyn MemoryMappedDevice>>,
    config: MemoryConfig,
}

impl Clone for Memory {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size,
            heap_top: self.heap_top,
            memory_regions: self.memory_regions.clone(),
            mapped_devices: HashMap::new(), // Empty on clone
            config: self.config,
        }
    }
}

// Define a trait for memory-mapped devices
pub trait MemoryMappedDevice: Send + Sync {
    fn read_byte(&self, offset: usize) -> u8;
    fn write_byte(&mut self, offset: usize, value: u8);
    fn read_word(&self, offset: usize) -> u32;
    fn write_word(&mut self, offset: usize, value: u32);
}

// Define memory regions with different access permissions
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub priority: u8, // Higher priority regions are checked first
}

impl Memory {
    /// Create a new memory instance with default configuration
    pub fn new(size: usize) -> Self {
        Self::with_config(size, MemoryConfig::default())
    }

    /// Create a new memory instance with custom configuration
    pub fn with_config(size: usize, config: MemoryConfig) -> Self {
        let mut memory_regions = Self::create_default_regions(size);

        // Sort regions by priority (higher priority first)
        memory_regions.sort_by(|a, b| b.priority.cmp(&a.priority));

        Self {
            data: vec![0; size],
            size,
            heap_top: std::cmp::min(size / 2, 0x00200000), // Start heap at middle or 2MB
            mapped_devices: HashMap::new(),
            memory_regions,
            config,
        }
    }

    /// Create a simple memory instance for testing (no complex regions)
    pub fn new_simple(size: usize) -> Self {
        let simple_region = vec![MemoryRegion {
            start: 0x00000000,
            end: size,
            readable: true,
            writable: true,
            executable: true,
            priority: 255, // Highest priority
        }];

        Self {
            data: vec![0; size],
            size,
            heap_top: size / 2,
            mapped_devices: HashMap::new(),
            memory_regions: simple_region,
            config: MemoryConfig {
                strict_alignment: false,
                verbose_errors: false,
                enable_permissions: false, // Disable permissions for simple mode
                enable_translation: false, // Disable translation for simple mode
            },
        }
    }

    /// Create default memory regions based on memory size
    fn create_default_regions(size: usize) -> Vec<MemoryRegion> {
        let mut regions = Vec::new();

        // Always create a base region that covers the entire memory for fallback
        regions.push(MemoryRegion {
            start: 0x00000000,
            end: size,
            readable: true,
            writable: true,
            executable: true,
            priority: 0, // Lowest priority - fallback
        });

        // Add specific regions only if memory is large enough
        if size >= 0x10000 {
            // Text segment (first 64KB)
            regions.push(MemoryRegion {
                start: 0x00000000,
                end: 0x00010000,
                readable: true,
                writable: true, // Allow writes for program loading
                executable: true,
                priority: 10,
            });
        }

        if size >= 0x100000 {
            // Data segment
            regions.push(MemoryRegion {
                start: 0x00010000,
                end: std::cmp::min(0x00100000, size),
                readable: true,
                writable: true,
                executable: false,
                priority: 5,
            });
        }

        if size >= 0x400000 {
            // Extended data segment
            regions.push(MemoryRegion {
                start: 0x00100000,
                end: std::cmp::min(0x00400000, size),
                readable: true,
                writable: true,
                executable: false,
                priority: 3,
            });
        }

        regions
    }

    /// Address translation function to map virtual addresses to physical addresses
    pub fn translate_address(&self, address: usize) -> usize {
        if !self.config.enable_translation {
            return address;
        }

        // Handle high virtual addresses (typically from LUI instructions)
        if address >= 0x10000000 {
            // Map high addresses to lower physical space
            let physical_addr = address & (self.size - 1).max(0xFFFFF);

            if self.config.verbose_errors && physical_addr >= self.size {
                println!("Warning: Translated address 0x{:08X} (from 0x{:08X}) still out of bounds (size: {})", 
                        physical_addr, address, self.size);
            }

            physical_addr
        } else {
            // For lower addresses, use them directly
            address
        }
    }

    /// Check if an address is valid for the given operation
    fn is_valid_access(&self, address: usize, size: usize) -> bool {
        let physical_addr = self.translate_address(address);

        // Check bounds
        if physical_addr >= self.size || physical_addr + size > self.size {
            return false;
        }

        // Always check alignment for simple memory (when permissions are disabled)
        // This ensures predictable behavior for property tests
        if !self.config.enable_permissions || self.config.strict_alignment {
            match size {
                1 => true,             // Byte access - no alignment required
                2 => address % 2 == 0, // Halfword - 2-byte aligned
                4 => address % 4 == 0, // Word - 4-byte aligned
                _ => false,            // Invalid size
            }
        } else {
            // In non-strict mode with permissions enabled, allow unaligned access but warn
            if size > 1 && address % size != 0 && self.config.verbose_errors {
                println!(
                    "Warning: Unaligned {}-byte access at address 0x{:08X}",
                    size, address
                );
            }
            true
        }
    }

    /// Direct write method that bypasses permission checks (for initialization)
    pub fn write_word_init(&mut self, address: usize, value: u32) -> bool {
        if !self.is_valid_access(address, 4) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!("Memory write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            }
            return false;
        }

        let physical_addr = self.translate_address(address);
        let bytes = value.to_le_bytes();
        self.data[physical_addr..physical_addr + 4].copy_from_slice(&bytes);
        true
    }

    /// Direct byte write method (for initialization)
    pub fn write_byte_init(&mut self, address: usize, value: u8) -> bool {
        if !self.is_valid_access(address, 1) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!(
                    "Memory byte write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds",
                    address, physical_addr
                );
            }
            return false;
        }

        let physical_addr = self.translate_address(address);
        self.data[physical_addr] = value;
        true
    }

    /// Read a single byte from memory
    pub fn read_byte(&self, address: usize) -> Option<u8> {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device(address) {
            let offset = address - base_addr;
            return Some(device.read_byte(offset));
        }

        if !self.is_valid_access(address, 1) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!("Memory read failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})",
                         address, physical_addr, self.size);
            }
            return None;
        }

        if self.config.enable_permissions && !self.check_permission(address, true, false, false) {
            return None;
        }

        let physical_addr = self.translate_address(address);
        Some(self.data[physical_addr])
    }

    /// Write a single byte to memory
    pub fn write_byte(&mut self, address: usize, value: u8) -> bool {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device_mut(address) {
            let offset = address - base_addr;
            device.write_byte(offset, value);
            return true;
        }

        if !self.is_valid_access(address, 1) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!("Memory write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                        address, physical_addr, self.size);
            }
            return false;
        }

        if self.config.enable_permissions && !self.check_permission(address, false, true, false) {
            return false;
        }

        let physical_addr = self.translate_address(address);
        self.data[physical_addr] = value;
        true
    }

    /// Read a 32-bit word from memory
    pub fn read_word(&self, address: usize) -> Option<u32> {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device(address) {
            let offset = address - base_addr;
            return Some(device.read_word(offset));
        }

        if !self.is_valid_access(address, 4) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!("Memory read failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            }
            return None;
        }

        if self.config.enable_permissions
            && (!self.check_permission(address, true, false, false)
                || !self.check_permission(address + 3, true, false, false))
        {
            return None;
        }

        let physical_addr = self.translate_address(address);
        let bytes = &self.data[physical_addr..physical_addr + 4];
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Write a 32-bit word to memory
    pub fn write_word(&mut self, address: usize, value: u32) -> bool {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device_mut(address) {
            let offset = address - base_addr;
            device.write_word(offset, value);
            return true;
        }

        if !self.is_valid_access(address, 4) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!("Memory write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            }
            return false;
        }

        if self.config.enable_permissions
            && (!self.check_permission(address, false, true, false)
                || !self.check_permission(address + 3, false, true, false))
        {
            if self.config.verbose_errors {
                println!(
                    "Memory write failed: address 0x{:08X} permission denied",
                    address
                );
            }
            return false;
        }

        let physical_addr = self.translate_address(address);
        let bytes = value.to_le_bytes();
        self.data[physical_addr..physical_addr + 4].copy_from_slice(&bytes);
        true
    }

    /// Read a 16-bit halfword from memory
    pub fn read_halfword(&self, address: usize) -> Option<u16> {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device(address) {
            let offset = address - base_addr;
            return Some((device.read_word(offset) & 0xFFFF) as u16);
        }

        if !self.is_valid_access(address, 2) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!("Memory read failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            }
            return None;
        }

        if self.config.enable_permissions
            && (!self.check_permission(address, true, false, false)
                || !self.check_permission(address + 1, true, false, false))
        {
            return None;
        }

        let physical_addr = self.translate_address(address);
        let bytes = &self.data[physical_addr..physical_addr + 2];
        Some(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Write a 16-bit halfword to memory
    pub fn write_halfword(&mut self, address: usize, value: u16) -> bool {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device_mut(address) {
            let offset = address - base_addr;
            device.write_word(offset, value as u32);
            return true;
        }

        if !self.is_valid_access(address, 2) {
            if self.config.verbose_errors {
                let physical_addr = self.translate_address(address);
                println!(
                    "Memory halfword write failed: address 0x{:08X} (physical: 0x{:08X})",
                    address, physical_addr
                );
            }
            return false;
        }

        if self.config.enable_permissions
            && (!self.check_permission(address, false, true, false)
                || !self.check_permission(address + 1, false, true, false))
        {
            return false;
        }

        let physical_addr = self.translate_address(address);
        let bytes = value.to_le_bytes();
        self.data[physical_addr..physical_addr + 2].copy_from_slice(&bytes);
        true
    }

    // Memory-mapped device management
    pub fn map_device(&mut self, base_address: usize, device: Box<dyn MemoryMappedDevice>) {
        self.mapped_devices.insert(base_address, device);
    }

    pub fn unmap_device(&mut self, base_address: usize) {
        self.mapped_devices.remove(&base_address);
    }

    fn get_mapped_device(&self, address: usize) -> Option<(usize, &dyn MemoryMappedDevice)> {
        for (&base_addr, device) in &self.mapped_devices {
            // Assuming each device has a fixed size of 4KB
            if address >= base_addr && address < base_addr + 4096 {
                return Some((base_addr, device.as_ref()));
            }
        }
        None
    }

    fn get_mapped_device_mut(
        &mut self,
        address: usize,
    ) -> Option<(usize, &mut dyn MemoryMappedDevice)> {
        let mut found_base = None;
        for &base_addr in self.mapped_devices.keys() {
            // Assuming each device has a fixed size of 4KB
            if address >= base_addr && address < base_addr + 4096 {
                found_base = Some(base_addr);
                break;
            }
        }

        if let Some(base_addr) = found_base {
            if let Some(device) = self.mapped_devices.get_mut(&base_addr) {
                return Some((base_addr, device.as_mut()));
            }
        }
        None
    }

    /// Memory region management
    pub fn add_memory_region(&mut self, region: MemoryRegion) {
        self.memory_regions.push(region);
        // Re-sort by priority
        self.memory_regions
            .sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Remove all memory regions and add a new one
    pub fn set_memory_regions(&mut self, regions: Vec<MemoryRegion>) {
        self.memory_regions = regions;
        self.memory_regions
            .sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Check memory access permissions
    fn check_permission(&self, address: usize, read: bool, write: bool, execute: bool) -> bool {
        // Check regions in priority order (highest first)
        for region in &self.memory_regions {
            if address >= region.start && address < region.end {
                let permitted = (!read || region.readable)
                    && (!write || region.writable)
                    && (!execute || region.executable);

                if !permitted && self.config.verbose_errors {
                    if write && !region.writable {
                        println!("Permission denied: Can't write to address 0x{:08X} in region 0x{:08X}-0x{:08X}", 
                                 address, region.start, region.end);
                    }
                    if read && !region.readable {
                        println!("Permission denied: Can't read from address 0x{:08X} in region 0x{:08X}-0x{:08X}", 
                                 address, region.start, region.end);
                    }
                    if execute && !region.executable {
                        println!("Permission denied: Can't execute from address 0x{:08X} in region 0x{:08X}-0x{:08X}", 
                                 address, region.start, region.end);
                    }
                }

                return permitted;
            }
        }

        // If no region is defined for this address, log it for debugging
        if self.config.verbose_errors && (read || write || execute) {
            println!("No memory region defined for address 0x{:08X}", address);
        }

        // Default to allowing access for backward compatibility
        true
    }

    // Heap management
    pub fn heap_end(&self) -> usize {
        self.heap_top
    }

    pub fn set_heap_end(&mut self, new_end: usize) {
        if new_end < self.size {
            self.heap_top = new_end;
        }
    }

    // Debug functions
    pub fn dump_memory(&self, start: usize, length: usize) -> String {
        let mut result = String::new();
        let end = std::cmp::min(start + length, self.size);

        for i in (start..end).step_by(16) {
            result.push_str(&format!("{:08x}:  ", i));

            for j in 0..16 {
                if i + j < end {
                    result.push_str(&format!("{:02x} ", self.data[i + j]));
                } else {
                    result.push_str("   ");
                }

                if j == 7 {
                    result.push(' ');
                }
            }

            result.push_str(" |");
            for j in 0..16 {
                if i + j < end {
                    let c = self.data[i + j];
                    if c >= 32 && c <= 126 {
                        result.push(c as char);
                    } else {
                        result.push('.');
                    }
                } else {
                    result.push(' ');
                }
            }
            result.push_str("|\n");
        }

        result
    }

    // New debug function to dump specific memory regions
    pub fn debug_dump(&self, start: usize, count: usize) {
        println!(
            "Memory dump from 0x{:08X} to 0x{:08X}:",
            start,
            start + count * 4 - 1
        );
        for i in 0..count {
            let addr = start + i * 4;
            if addr < self.size {
                let word = self.read_word(addr);
                println!("  0x{:08X}: {:?}", addr, word);
            }
        }
    }

    // Print memory region information
    pub fn print_memory_regions(&self) {
        println!("Memory Regions:");
        for (i, region) in self.memory_regions.iter().enumerate() {
            println!(
                "  Region {}: 0x{:08X}-0x{:08X} (r:{}, w:{}, x:{})",
                i, region.start, region.end, region.readable, region.writable, region.executable
            );
        }
    }

    /// Check if an address range is writable
    pub fn is_range_writable(&self, start: usize, length: usize) -> bool {
        if !self.config.enable_permissions {
            return self.is_valid_access(start, length);
        }

        let end = start + length;
        for addr in start..end {
            if !self.check_permission(addr, false, true, false) {
                return false;
            }
        }
        true
    }

    /// Get current memory configuration
    pub fn get_config(&self) -> MemoryConfig {
        self.config
    }

    /// Update memory configuration
    pub fn set_config(&mut self, config: MemoryConfig) {
        self.config = config;
    }

    /// Enable or disable verbose error messages
    pub fn set_verbose_errors(&mut self, verbose: bool) {
        self.config.verbose_errors = verbose;
    }

    /// Enable or disable strict alignment checking
    pub fn set_strict_alignment(&mut self, strict: bool) {
        self.config.strict_alignment = strict;
    }

    /// Enable or disable memory permissions
    pub fn set_permissions_enabled(&mut self, enabled: bool) {
        self.config.enable_permissions = enabled;
    }

    /// Enable or disable address translation
    pub fn set_translation_enabled(&mut self, enabled: bool) {
        self.config.enable_translation = enabled;
    }

    /// Get memory statistics
    pub fn get_statistics(&self) -> MemoryStatistics {
        MemoryStatistics {
            total_size: self.size,
            heap_top: self.heap_top,
            num_regions: self.memory_regions.len(),
            num_mapped_devices: self.mapped_devices.len(),
        }
    }

    /// Clear all memory (set to zero)
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    /// Fill memory range with a specific value
    pub fn fill_range(&mut self, start: usize, length: usize, value: u8) -> bool {
        if !self.is_valid_access(start, length) {
            return false;
        }

        let physical_start = self.translate_address(start);
        let end = physical_start + length;

        if end <= self.size {
            self.data[physical_start..end].fill(value);
            true
        } else {
            false
        }
    }

    /// Copy data from one memory location to another
    pub fn copy_range(&mut self, src: usize, dst: usize, length: usize) -> bool {
        if !self.is_valid_access(src, length) || !self.is_valid_access(dst, length) {
            return false;
        }

        let physical_src = self.translate_address(src);
        let physical_dst = self.translate_address(dst);

        if physical_src + length <= self.size && physical_dst + length <= self.size {
            // Use a temporary buffer to handle overlapping ranges
            let temp: Vec<u8> = self.data[physical_src..physical_src + length].to_vec();
            self.data[physical_dst..physical_dst + length].copy_from_slice(&temp);
            true
        } else {
            false
        }
    }
}

/// Memory statistics structure
#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    pub total_size: usize,
    pub heap_top: usize,
    pub num_regions: usize,
    pub num_mapped_devices: usize,
}
