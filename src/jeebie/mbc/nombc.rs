use super::MemoryBankController;

/// RomOnly is the simplest MBC, as in, there's actually no controller at all.
/// This maps the ROM directly to addresses 0x0000 to 0x7FFF
/// Some cartridges that specify no MBC might also pack up to 8KB of RAM at the
/// addresses 0xA000 to 0xBFFF.
struct RomOnly {
    data: [u8; 32768],
    ram: [u8; 8192]
}

impl MemoryBankController for RomOnly {

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0...0x7FFF => self.data[addr as usize],
            0xA000...0xBFFF => self.ram[(addr - 0xA000) as usize],
            _ => panic!("RomOnly MBC attempted read at ${:04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0...0x7FFF => {
                // nothing happens when writing to ROM
            },
            0xA000...0xBFFF => self.ram[(addr - 0xA000) as usize] = data,
            _ => panic!("RomOnly MBC attempted write at ${:04x}", addr),
        };
    }
}