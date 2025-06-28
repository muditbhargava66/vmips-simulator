// memory.rs
use std::collections::HashMap;

pub struct Memory {
    pub data: Vec<u8>,
    pub size: usize,
    heap_top: usize,
    memory_regions: Vec<MemoryRegion>,
    mapped_devices: HashMap<usize, Box<dyn MemoryMappedDevice>>,
}

impl Clone for Memory {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size,
            heap_top: self.heap_top,
            memory_regions: self.memory_regions.clone(),
            mapped_devices: HashMap::new(), // Empty on clone
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
#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        // Create default memory regions
        let default_regions = vec![
            // Special test region with highest priority
            MemoryRegion {
                start: 0x00000000, 
                end: 0x00010000,   // First 64KB - includes both program code and test data
                readable: true,
                writable: true,    // Make it writable for tests
                executable: true,
            },
            // Text segment (code) with lower priority
            MemoryRegion {
                start: 0x00010000, // Start after test region
                end: 0x00100000,
                readable: true,
                writable: false,
                executable: true,
            },
            // Data segment - Read and write - Make it larger and ensure it's writable
            MemoryRegion {
                start: 0x00100000,
                end: 0x00400000, // Increased size to cover more addresses
                readable: true,
                writable: true,
                executable: false,
            },
            // Stack segment - Read and write
            MemoryRegion {
                start: 0x7FFF0000,
                end: 0x80000000,
                readable: true,
                writable: true,
                executable: false,
            },
            // Add a general purpose memory region for testing
            // This allows access to lower memory addresses typically used in examples
            MemoryRegion {
                start: 0x00000000, // Start from beginning
                end: 0x00010000,   // Include the first 64KB
                readable: true,
                writable: true,    // Make it writable for tests
                executable: false,
            },
        ];

        Self {
            data: vec![0; size],
            size,
            heap_top: 0x00200000, // Start heap after data segment
            mapped_devices: HashMap::new(),
            memory_regions: default_regions,
        }
    }

    // Address translation function to map high virtual addresses to physical addresses
    pub fn translate_address(&self, address: usize) -> usize {
        // When a high memory address (like 0x10000000) is detected,
        // it's likely from a 'lui' instruction, so map it to a lower physical address
        if address >= 0x10000000 {
            // Take the lower bits to map it within our memory bounds
            let physical_addr = address & 0xFFFFF; // Mask to lower 20 bits (1MB space)
            
            if physical_addr >= self.size {
                println!("Warning: Translated address 0x{:08X} (from 0x{:08X}) still out of bounds (size: {})", 
                        physical_addr, address, self.size);
            }
            
            physical_addr
        } else {
            // For lower addresses, use them directly
            address
        }
    }

    // Direct write method that bypasses permission checks (for initialization)
    pub fn write_word_init(&mut self, address: usize, value: u32) -> bool {
        // Check alignment (address should be a multiple of 4)
        if address % 4 != 0 {
            println!("Warning: Unaligned word write at address 0x{:08x}", address);
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Only check if the address is within memory bounds
        if physical_addr + 3 < self.size {
            let bytes = value.to_le_bytes();
            self.data[physical_addr..physical_addr + 4].copy_from_slice(&bytes);
            true
        } else {
            println!("Memory write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                     address, physical_addr, self.size);
            false
        }
    }

    // Direct byte write method (for initialization)
    pub fn write_byte_init(&mut self, address: usize, value: u8) -> bool {
        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);
        
        // Only check if the address is within memory bounds
        if physical_addr < self.size {
            self.data[physical_addr] = value;
            true
        } else {
            println!("Memory byte write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds", 
                    address, physical_addr);
            false
        }
    }

    pub fn read_byte(&self, address: usize) -> Option<u8> {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device(address) {
            let offset = address - base_addr;
            return Some(device.read_byte(offset));
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Check if the access is within memory bounds and has read permission
        if physical_addr < self.size && self.check_permission(address, true, false, false) {
            Some(self.data[physical_addr])
        } else {
            if physical_addr >= self.size {
                println!("Memory read failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})",
                         address, physical_addr, self.size);
            }
            None
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) -> bool {
        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device_mut(address) {
            let offset = address - base_addr;
            device.write_byte(offset, value);
            return true;
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Check if the access is within memory bounds and has write permission
        if physical_addr < self.size && self.check_permission(address, false, true, false) {
            self.data[physical_addr] = value;
            true
        } else {
            if physical_addr >= self.size {
                println!("Memory write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                        address, physical_addr, self.size);
            }
            false
        }
    }

    pub fn read_word(&self, address: usize) -> Option<u32> {
        // Check alignment (address should be a multiple of 4)
        if address % 4 != 0 {
            println!("Warning: Unaligned word read at address 0x{:08x}", address);
        }

        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device(address) {
            let offset = address - base_addr;
            return Some(device.read_word(offset));
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Check if all bytes are within memory bounds and have read permission
        if physical_addr + 3 < self.size && 
           self.check_permission(address, true, false, false) &&
           self.check_permission(address + 3, true, false, false) {
            let bytes = &self.data[physical_addr..physical_addr + 4];
            Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            if physical_addr + 3 >= self.size {
                println!("Memory read failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            }
            None
        }
    }

    pub fn write_word(&mut self, address: usize, value: u32) -> bool {
        // Check alignment (address should be a multiple of 4)
        if address % 4 != 0 {
            println!("Warning: Unaligned word write at address 0x{:08x}", address);
        }

        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device_mut(address) {
            let offset = address - base_addr;
            device.write_word(offset, value);
            return true;
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Check if all bytes are within memory bounds and have write permission
        if physical_addr + 3 < self.size && 
           self.check_permission(address, false, true, false) &&
           self.check_permission(address + 3, false, true, false) {
            let bytes = value.to_le_bytes();
            self.data[physical_addr..physical_addr + 4].copy_from_slice(&bytes);
            true
        } else {
            if physical_addr + 3 >= self.size {
                println!("Memory write failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            } else {
                println!("Memory write failed: address 0x{:08X} permission denied", address);
            }
            false
        }
    }

    pub fn read_halfword(&self, address: usize) -> Option<u16> {
        // Check alignment (address should be a multiple of 2)
        if address % 2 != 0 {
            println!("Warning: Unaligned halfword read at address 0x{:08x}", address);
        }

        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device(address) {
            let offset = address - base_addr;
            return Some((device.read_word(offset) & 0xFFFF) as u16);
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Check if all bytes are within memory bounds and have read permission
        if physical_addr + 1 < self.size && 
           self.check_permission(address, true, false, false) &&
           self.check_permission(address + 1, true, false, false) {
            let bytes = &self.data[physical_addr..physical_addr + 2];
            Some(u16::from_le_bytes([bytes[0], bytes[1]]))
        } else {
            if physical_addr + 1 >= self.size {
                println!("Memory read failed: address 0x{:08X} (physical: 0x{:08X}) out of bounds (size: {})", 
                         address, physical_addr, self.size);
            }
            None
        }
    }

    pub fn write_halfword(&mut self, address: usize, value: u16) -> bool {
        // Check alignment (address should be a multiple of 2)
        if address % 2 != 0 {
            println!("Warning: Unaligned halfword write at address 0x{:08x}", address);
        }

        // Check if address is mapped to a device
        if let Some((base_addr, device)) = self.get_mapped_device_mut(address) {
            let offset = address - base_addr;
            device.write_word(offset, value as u32);
            return true;
        }

        // Translate virtual address to physical address
        let physical_addr = self.translate_address(address);

        // Check if all bytes are within memory bounds and have write permission
        if physical_addr + 1 < self.size && 
           self.check_permission(address, false, true, false) &&
           self.check_permission(address + 1, false, true, false) {
            let bytes = value.to_le_bytes();
            self.data[physical_addr..physical_addr + 2].copy_from_slice(&bytes);
            true
        } else {
            println!("Memory halfword write failed: address 0x{:08X} (physical: 0x{:08X})", 
                     address, physical_addr);
            false
        }
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

    fn get_mapped_device_mut(&mut self, address: usize) -> Option<(usize, &mut dyn MemoryMappedDevice)> {
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

    // Memory region management
    pub fn add_memory_region(&mut self, region: MemoryRegion) {
        self.memory_regions.push(region);
    }

    fn check_permission(&self, address: usize, read: bool, write: bool, execute: bool) -> bool {
        for region in &self.memory_regions {
            if address >= region.start && address < region.end {
                let permitted = (!read || region.readable) && 
                                (!write || region.writable) && 
                                (!execute || region.executable);
                
                if !permitted {
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
        if read || write || execute {
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
        println!("Memory dump from 0x{:08X} to 0x{:08X}:", start, start + count * 4 - 1);
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
            println!("  Region {}: 0x{:08X}-0x{:08X} (r:{}, w:{}, x:{})",
                     i, region.start, region.end,
                     region.readable, region.writable, region.executable);
        }
    }
    
    // Check if an address range is writable
    pub fn is_range_writable(&self, start: usize, length: usize) -> bool {
        let end = start + length;
        for addr in start..end {
            if !self.check_permission(addr, false, true, false) {
                return false;
            }
        }
        true
    }
}