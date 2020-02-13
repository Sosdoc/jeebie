const MEMORY_SIZE: usize = 64 * 1024;

pub struct Memory {
    data: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: vec![0; MEMORY_SIZE] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.data[addr as usize] = data;
    }
}

