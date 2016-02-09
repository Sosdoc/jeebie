//TODO: should handle memory mapped io (e.g. gpu buffers, cartridge). Also this needs to handle
// mirrored memory and bank switching where appropriate
use std::fmt;

/// Basic struct for memory management
pub struct MMU {
    data: [u8; 65536]
}

impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU 64k")
    }
}

impl MMU {

    pub fn new() -> MMU {
        MMU {
            data: [1; 65536]
        }
    }

    /// reads a byte at the memory address specified
    pub fn read_b(&self, addr : u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_b(&mut self, addr : u16, data : u8) {
        self.data[addr as usize] = data;
    }

}
