// registers.rs
#[derive(Debug)]
pub struct Registers {
    pub data: Vec<u32>, // Change from fixed array to Vec for LO/HI registers
}

impl Registers {
    pub fn new() -> Self {
        // Create 33 registers (0-31 standard MIPS registers, 32 for LO)
        Self {
            data: vec![0; 33],
        }
    }

    pub fn read(&self, reg_num: u32) -> u32 {
        if reg_num >= self.data.len() as u32 {
            return 0;
        }
        self.data[reg_num as usize]
    }

    pub fn write(&mut self, reg_num: u32, value: u32) {
        if reg_num != 0 && reg_num < self.data.len() as u32 {
            self.data[reg_num as usize] = value;
        }
    }
}