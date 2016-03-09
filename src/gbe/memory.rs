use std::fmt;
use std::cell::Cell;

/// The Memory Management Unit.
/// Provides access to all mapped memory in the system, including I/O and graphics.
pub struct MMU {
    pub loading_bios: Cell<bool>,
    data: [u8; 65536],
}

impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU 64k")
    }
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            loading_bios: Cell::new(true),
            data: [1; 65536],
        }
    }

    /// reads a byte at the memory address specified
    pub fn read_b(&self, addr: u16) -> u8 {
        // when PC first reaches 0x100, the BIOS data is not addressable anymore.
        // using Cell sucks, but this is one of the few cases mentioned by official docs where
        // it is an option. Read is logically an immutable operation, but we need to make the
        // bios unaddressable after its first read.
        if self.loading_bios.get() && addr == 0x0100 {
            self.loading_bios.set(false);
        }

        match addr {
            // bios area, 256B long for regular gameboy.
            0...0x00FF if self.loading_bios.get() => 0,
            // ROM0 area, this is banked memory, it will swap according to selected bank
            0x0000...0x3FFF if !self.loading_bios.get() => 0,
            // ROM1 area, 16kB unbanked data
            0x4000...0x7FFF => 0,
            // Graphics, 8kB VRAM
            0x8000...0x9FFF => 0,
            // Switchable RAM bank, 8kB
            0xA000...0xBFFF => {
                // TODO: handle RAM banks
                self.data[addr as usize]
            }
            // Internal RAM, 8kB
            0xC000...0xDFFF => self.data[addr as usize],
            // Echo of internal RAM, this is less than 8k, up to 0xFDFF,
            0xE000...0xFDFF => self.data[(addr - 0x2000) as usize],
            // Sprite attribute memory, 160B
            0xFE00...0xFE9F => 0,
            // empty
            0xFEA0...0xFEFF | 0xFF4C...0xFF7F => 0,
            // I/O ports
            0xFF00...0xFF4B => 0,
            // More internal RAM
            0xFF80...0xFFFE => 0,
            // Interrupt Enable register
            0xFFFF => 0,

            _ => panic!("tried to write at unkown address: {:4X}", addr)
        }
    }

    pub fn write_b(&mut self, addr: u16, data: u8) {
        self.data[addr as usize] = data;
    }
}
