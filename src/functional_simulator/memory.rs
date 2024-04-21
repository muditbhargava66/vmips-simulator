// memory.rs

#[derive(Clone)]
pub struct Memory {
    pub data: Vec<u8>,
    pub size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            size,
        }
    }

    pub fn read_byte(&self, address: usize) -> Option<u8> {
        if address < self.size {
            Some(self.data[address])
        } else {
            None
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) -> bool {
        if address < self.size {
            self.data[address] = value;
            true
        } else {
            false
        }
    }

    pub fn read_word(&self, address: usize) -> Option<u32> {
        if address + 3 < self.size {
            let bytes = &self.data[address..address + 4];
            Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            None
        }
    }

    pub fn write_word(&mut self, address: usize, value: u32) -> bool {
        if address + 3 < self.size {
            let bytes = value.to_le_bytes();
            self.data[address..address + 4].copy_from_slice(&bytes);
            true
        } else {
            false
        }
    }
}