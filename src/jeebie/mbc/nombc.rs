use super::MemoryBankController;

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
        RomOnly { data: vec![0; 0x8000], ram: vec![0; 0x2000] }
    }

    /// Creates a new RomOnly MBC with the provided data.
    pub fn with_data(rom_data: Vec<u8>) -> Self {
        RomOnly {
            data: rom_data,
            ram: vec![0; 8192],
        }
    }
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

#[test]
fn nombc_loads_vec() {
    let mbc = RomOnly::with_data(vec![0xFF; 0x8000]);

    // ROM
    assert_eq!(0xFF, mbc.read(0x0));
    assert_eq!(0xFF, mbc.read(0x7FFF));
    // RAM
    assert_eq!(0x0, mbc.read(0xA000));
    assert_eq!(0x0, mbc.read(0xBFFF));
}

#[test]
fn nombc_write() {
    let mut mbc = RomOnly::with_data(vec![0xFF; 0x8000]);

    // ROM write should do nothing
    mbc.write(0x0, 0xAF);
    mbc.write(0x7FFF, 0xAF);
    assert_eq!(0xFF, mbc.read(0x0));
    assert_eq!(0xFF, mbc.read(0x7FFF));
    // RAM write
    mbc.write(0xA000, 0xAF);
    mbc.write(0xBFFF, 0xAF);
    assert_eq!(0xAF, mbc.read(0xA000));
    assert_eq!(0xAF, mbc.read(0xBFFF));
}