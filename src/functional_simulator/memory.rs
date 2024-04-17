// memory.rs
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn read_word(&self, address: usize) -> u32 {
        let bytes = &self.data[address..address + 4];
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn write_word(&mut self, address: usize, value: u32) {
        let bytes = value.to_le_bytes();
        self.data[address..address + 4].copy_from_slice(&bytes);
    }
}