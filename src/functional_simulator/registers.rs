// registers.rs
#[derive(Debug)]
pub struct Registers {

    pub data: [u32; 32],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            data: [0; 32],
        }
    }

    pub fn read(&self, reg_num: u32) -> u32 {
        self.data[reg_num as usize]
    }

    pub fn write(&mut self, reg_num: u32, value: u32) {
        if reg_num != 0 {
            self.data[reg_num as usize] = value;
        }
    }
}
