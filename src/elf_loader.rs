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

//! ELF Binary Loader
//!
//! This module provides functionality to load ELF (Executable and Linkable Format)
//! binaries for MIPS architecture into the simulator's memory.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// ELF file header structure (simplified for MIPS32)
#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader {
    pub e_ident: [u8; 16], // ELF identification
    pub e_type: u16,       // Object file type
    pub e_machine: u16,    // Architecture
    pub e_version: u32,    // Object file version
    pub e_entry: u32,      // Entry point virtual address
    pub e_phoff: u32,      // Program header table file offset
    pub e_shoff: u32,      // Section header table file offset
    pub e_flags: u32,      // Processor-specific flags
    pub e_ehsize: u16,     // ELF header size in bytes
    pub e_phentsize: u16,  // Program header table entry size
    pub e_phnum: u16,      // Program header table entry count
    pub e_shentsize: u16,  // Section header table entry size
    pub e_shnum: u16,      // Section header table entry count
    pub e_shstrndx: u16,   // Section header string table index
}

/// Program header structure
#[repr(C)]
#[derive(Debug)]
pub struct ProgramHeader {
    pub p_type: u32,   // Segment type
    pub p_offset: u32, // Segment file offset
    pub p_vaddr: u32,  // Segment virtual address
    pub p_paddr: u32,  // Segment physical address
    pub p_filesz: u32, // Segment size in file
    pub p_memsz: u32,  // Segment size in memory
    pub p_flags: u32,  // Segment flags
    pub p_align: u32,  // Segment alignment
}

/// ELF constants
pub const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
pub const EM_MIPS: u16 = 8; // MIPS architecture
pub const PT_LOAD: u32 = 1; // Loadable segment

/// Errors that can occur during ELF loading
#[derive(Debug)]
pub enum ElfError {
    IoError(io::Error),
    InvalidMagic,
    UnsupportedArchitecture,
    InvalidFormat,
    MemoryError,
}

impl std::fmt::Display for ElfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElfError::IoError(e) => write!(f, "IO error: {}", e),
            ElfError::InvalidMagic => write!(f, "Invalid ELF magic number"),
            ElfError::UnsupportedArchitecture => write!(f, "Unsupported architecture (not MIPS)"),
            ElfError::InvalidFormat => write!(f, "Invalid ELF format"),
            ElfError::MemoryError => write!(f, "Memory error during loading"),
        }
    }
}

impl std::error::Error for ElfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ElfError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ElfError {
    fn from(error: io::Error) -> Self {
        ElfError::IoError(error)
    }
}

/// ELF binary loader
pub struct ElfLoader {
    data: Vec<u8>,
    header: ElfHeader,
    program_headers: Vec<ProgramHeader>,
}

impl ElfLoader {
    /// Load an ELF file from disk
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, ElfError> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Self::load_from_bytes(data)
    }

    /// Load an ELF file from byte array
    pub fn load_from_bytes(data: Vec<u8>) -> Result<Self, ElfError> {
        if data.len() < std::mem::size_of::<ElfHeader>() {
            return Err(ElfError::InvalidFormat);
        }

        // Parse ELF header
        let header = unsafe { std::ptr::read(data.as_ptr() as *const ElfHeader) };

        // Validate ELF magic
        if header.e_ident[0..4] != ELF_MAGIC {
            return Err(ElfError::InvalidMagic);
        }

        // Check architecture
        if header.e_machine != EM_MIPS {
            return Err(ElfError::UnsupportedArchitecture);
        }

        // Parse program headers
        let mut program_headers = Vec::new();
        let ph_offset = header.e_phoff as usize;
        let ph_size = header.e_phentsize as usize;
        let ph_count = header.e_phnum as usize;

        for i in 0..ph_count {
            let offset = ph_offset + i * ph_size;
            if offset + std::mem::size_of::<ProgramHeader>() > data.len() {
                return Err(ElfError::InvalidFormat);
            }

            let ph = unsafe {
                std::ptr::read((data.as_ptr() as usize + offset) as *const ProgramHeader)
            };
            program_headers.push(ph);
        }

        Ok(ElfLoader {
            data,
            header,
            program_headers,
        })
    }

    /// Get the entry point address
    pub fn entry_point(&self) -> u32 {
        self.header.e_entry
    }

    /// Load the ELF binary into memory
    pub fn load_into_memory(
        &self,
        memory: &mut crate::functional_simulator::memory::Memory,
    ) -> Result<(), ElfError> {
        for ph in &self.program_headers {
            if ph.p_type == PT_LOAD && ph.p_filesz > 0 {
                let file_offset = ph.p_offset as usize;
                let vaddr = ph.p_vaddr as usize;
                let size = ph.p_filesz as usize;

                // Validate bounds
                if file_offset + size > self.data.len() {
                    return Err(ElfError::InvalidFormat);
                }

                // Copy segment data to memory
                let segment_data = &self.data[file_offset..file_offset + size];

                // Load data word by word
                for (i, chunk) in segment_data.chunks(4).enumerate() {
                    let addr = vaddr + i * 4;
                    let mut word = 0u32;

                    // Handle partial words at the end
                    for (j, &byte) in chunk.iter().enumerate() {
                        word |= (byte as u32) << (j * 8);
                    }

                    if !memory.write_word(addr, word) {
                        return Err(ElfError::MemoryError);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get information about loaded segments
    pub fn get_segments(&self) -> Vec<(u32, u32, u32)> {
        self.program_headers
            .iter()
            .filter(|ph| ph.p_type == PT_LOAD)
            .map(|ph| (ph.p_vaddr, ph.p_memsz, ph.p_flags))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functional_simulator::memory::Memory;

    #[test]
    fn test_elf_magic_validation() {
        let mut invalid_data = vec![0u8; 64];
        invalid_data[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'X']); // Invalid magic

        let result = ElfLoader::load_from_bytes(invalid_data);
        assert!(matches!(result, Err(ElfError::InvalidMagic)));
    }

    #[test]
    fn test_insufficient_data() {
        let small_data = vec![0u8; 10]; // Too small for ELF header
        let result = ElfLoader::load_from_bytes(small_data);
        assert!(matches!(result, Err(ElfError::InvalidFormat)));
    }
}
