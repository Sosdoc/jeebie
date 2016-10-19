pub mod nombc;
pub mod mbc1;

/// A MemoryBankController (MBC) is the interface used to read/write
/// data on a gameboy cartridge.
/// MBCs allow the gameboy to address more than 32kB of ROM data and, in some cases,
/// also provide extra RAM.
pub trait MemoryBankController {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}
