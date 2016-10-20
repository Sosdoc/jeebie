use super::MemoryBankController;

/// MBC1 is the first MBC chip for the Gameboy. It can address a maximum of
/// 2MB ROM (divided in 125 banks of size 16KB) and/or 32KB RAM.
///
/// The first 16KB in the rom (0 to 0x3FFF) are always pointing to the first
/// ROM bank. The following 16KB refer to the selected ROM bank.
/// Banks are numbered from 0 to 0x7F (128), but bank numbers 0x20, 0x40 and
/// 0x60 are not usable, thus the total bank number is 125.
struct MBC1 {
    data: Vec<u8>,
    ram: [u8; 32768], // 32KB RAM
    selected_rom_bank: u8,
    selected_ram_bank: u8,
}

impl MemoryBankController for MBC1 {

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0...0x3FFF => self.data[addr as usize], // bank 0
            0x4000...0x7FFF => {
                // the selected bank data (banks 1 to 0x7F)
                let base_address = self.selected_rom_bank as u16 * 0x4000;
                self.data[(base_address + addr) as usize]
            },
            0xA000...0xBFFF => {
                // the selected RAM bank data (banks 0 to 3)
                self.ram[(addr - 0xA000) as usize]
            }
            _ => panic!("RomOnly MBC attempted read at ${:04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        // TODO finish this
    }
}