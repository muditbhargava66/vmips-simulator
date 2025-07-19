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

// registers.rs
//
// This file contains the implementation of the MIPS register file.
// It defines the Registers struct, which manages the general-purpose and
// floating-point registers, as well as the special-purpose HI, LO, and PC
// registers.

#[derive(Debug, Clone)]
pub struct Registers {
    pub data: Vec<u32>,          // General-purpose registers
    pub fp_registers: Vec<f32>,  // Floating-point registers
    pub hi: u32,                 // HI register for mult/div results
    pub lo: u32,                 // LO register for mult/div results
    pub pc: u32,                 // Program counter
    pub fcsr: u32,               // Floating-point Control Status Register
    pub target_reg: Option<u32>, // Target register for certain instructions
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            data: vec![0; 32],           // 32 general purpose registers
            fp_registers: vec![0.0; 32], // 32 floating-point registers
            hi: 0,
            lo: 0,
            pc: 0,
            fcsr: 0,
            target_reg: None,
        }
    }

    pub fn read(&self, reg_num: u32) -> u32 {
        if reg_num == 0 {
            0 // $zero is always 0
        } else if reg_num < self.data.len() as u32 {
            self.data[reg_num as usize]
        } else {
            0 // Return 0 for out-of-bounds reads
        }
    }

    pub fn write(&mut self, reg_num: u32, value: u32) {
        if reg_num != 0 && reg_num < self.data.len() as u32 {
            self.data[reg_num as usize] = value;
        }
        // Ignore writes to $zero or out-of-bounds
    }

    pub fn read_float(&self, reg_num: u32) -> f32 {
        if reg_num < self.fp_registers.len() as u32 {
            self.fp_registers[reg_num as usize]
        } else {
            0.0 // Return 0.0 for out-of-bounds reads
        }
    }

    pub fn write_float(&mut self, reg_num: u32, value: f32) {
        if reg_num < self.fp_registers.len() as u32 {
            self.fp_registers[reg_num as usize] = value;
        }
        // Ignore out-of-bounds writes
    }

    pub fn get_hi(&self) -> u32 {
        self.hi
    }

    pub fn set_hi(&mut self, value: u32) {
        self.hi = value;
    }

    pub fn get_lo(&self) -> u32 {
        self.lo
    }

    pub fn set_lo(&mut self, value: u32) {
        self.lo = value;
    }

    pub fn dump_registers(&self) -> String {
        let mut result = String::new();

        // General purpose registers
        result.push_str("General Purpose Registers:\n");
        for i in 0..8 {
            for j in 0..4 {
                let reg_num = i + j * 8;
                let reg_name = match reg_num {
                    0 => "$zero",
                    1 => "$at",
                    2 => "$v0",
                    3 => "$v1",
                    4 => "$a0",
                    5 => "$a1",
                    6 => "$a2",
                    7 => "$a3",
                    8 => "$t0",
                    9 => "$t1",
                    10 => "$t2",
                    11 => "$t3",
                    12 => "$t4",
                    13 => "$t5",
                    14 => "$t6",
                    15 => "$t7",
                    16 => "$s0",
                    17 => "$s1",
                    18 => "$s2",
                    19 => "$s3",
                    20 => "$s4",
                    21 => "$s5",
                    22 => "$s6",
                    23 => "$s7",
                    24 => "$t8",
                    25 => "$t9",
                    26 => "$k0",
                    27 => "$k1",
                    28 => "$gp",
                    29 => "$sp",
                    30 => "$fp",
                    31 => "$ra",
                    _ => unreachable!(),
                };
                result.push_str(&format!("{:<5} = 0x{:08x} ", reg_name, self.read(reg_num)));
            }
            result.push('\n');
        }

        // Special registers
        result.push_str(&format!("HI    = 0x{:08x}\n", self.hi));
        result.push_str(&format!("LO    = 0x{:08x}\n", self.lo));
        result.push_str(&format!("PC    = 0x{:08x}\n", self.pc));
        result.push_str(&format!("FCSR  = 0x{:08x}\n", self.fcsr));

        // Floating-point registers (if enabled)
        result.push_str("\nFloating Point Registers:\n");
        for i in 0..8 {
            for j in 0..4 {
                let reg_num = i + j * 8;
                let value = self.read_float(reg_num);
                result.push_str(&format!("$f{:<2} = {:<15.6} ", reg_num, value));
            }
            result.push('\n');
        }

        result
    }
}
