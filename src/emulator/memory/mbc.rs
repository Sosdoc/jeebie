//! The Memory Bank Controller (MBC) module.
//! Gameboy cartridges come with some memory dedicated to their own assets/code (ROM), and
//! sometimes extra RAM.
//!
//! The gameboy MMU can only address 8k of ROM and 8k of RAM at once, thus most cartridges
//! offer a memory controller to handle switching all the addressable memory in those two
//! allotted spaces.

use super::Addressable;

/// RomOnly is the simplest MBC, as in, there's actually no controller at all.
/// This maps the ROM directly to addresses 0x0000 to 0x7FFF
/// Some cartridges that specify no MBC might also pack up to 8KB of RAM at the
/// addresses 0xA000 to 0xBFFF.
pub struct RomOnly {
    data: Vec<u8>,
    ram: Vec<u8>,
}

impl RomOnly {
    /// Creates a new RomOnly MBC with no data (all initialized to 0).
    /// This is only useful for testing purposes, only the bootloader will run and in most cases the emulator
    /// might crash after trying to execute all memory.
    pub fn new() -> Self {
        Self {
            data: vec![0; 0x8000],
            ram: vec![0; 0x2000],
        }
    }

    /// Creates a new RomOnly MBC with the provided data.
    pub fn with_data(rom_data: Vec<u8>) -> Self {
        Self {
            data: rom_data,
            ram: vec![0; 8192],
        }
    }
}

impl Addressable for RomOnly {
    fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0..=0x7FFF => self.data[addr],
            0xA000..=0xBFFF => self.ram[(addr - 0xA000)],
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        match addr {
            0..=0x7FFF => {
                // nothing happens when writing to ROM
            }
            0xA000..=0xBFFF => self.ram[(addr - 0xA000)] = data,
            _ => {}
        };
    }
}
