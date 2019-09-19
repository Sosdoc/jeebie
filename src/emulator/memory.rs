//! The memory module.
//! This module declares a single struct, the Memory Management Unit (MMU).
//! The MMU acts as the system bus, allowing components to communicate with each other, reaches
//! RAM, ROM, I/O registers and more.

mod bootrom;
pub mod mbc;

use std::cell::Cell;
use std::fmt;

use bootrom::DMG_BOOTROM;

pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

/// The Memory Management Unit.
/// Provides access to all mapped memory in the system, including I/O and graphics.
pub struct MMU {
    data: Vec<u8>,
    loading_bios: Cell<bool>,
    mbc: Box<dyn Addressable>,
}

impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU 64k")
    }
}

impl MMU {
    /// Creates a new memory controller with no program loaded.
    pub fn new() -> Self {
        MMU {
            loading_bios: Cell::new(true),
            data: vec![0; 65536],
            mbc: Box::new(mbc::RomOnly::new()),
        }
    }

    /// reads a byte at the memory address specified
    pub fn read_b(&self, addr: u16) -> u8 {
        // when PC first reaches 0x100, the BIOS data is not addressable anymore.
        // Read is logically an immutable operation, but we need to make the
        // bios unaddressable after its first read.
        if self.loading_bios.get() && addr == 0x0100 {
            self.loading_bios.set(false);
        }

        let addr = addr as usize;

        match addr {
            // bios area, 256B long for regular gameboy, only accessible if loading_bios is true.
            0x0000..=0x00FF if self.loading_bios.get() => DMG_BOOTROM[(addr & 0xFF)],
            // ROM area, this is handled by the MBC
            0x0000..=0x7FFF => self.mbc.read(addr as u16),
            // Graphics, 8kB VRAM
            0x8000..=0x9FFF => self.data[(addr & 0x1FFF)],
            // Switchable RAM bank, 8kB, handled by MBC
            0xA000..=0xBFFF => self.mbc.read(addr as u16),
            // Internal RAM, 8kB
            0xC000..=0xDFFF => self.data[addr],
            // Echo of internal RAM, this is less than 8k, up to 0xFDFF
            0xE000..=0xFDFF => self.data[(addr - 0x2000)],
            // Sprite attribute memory, 160B
            0xFE00..=0xFE9F => self.data[(addr - 0xFE00)],
            // empty
            0xFEA0..=0xFEFF | 0xFF4C..=0xFF7F => 0,
            // I/O ports
            0xFF00..=0xFF4B => match addr & 0xFF {
                0x40..=0x47 => self.data[addr], // gpu registers
                _ => unimplemented!(),
            },
            // High RAM (zero page), used with LDH instructions
            0xFF80..=0xFFFF => self.data[addr],
            // Interrupt Enable register
            _ => unimplemented!(),
        }
    }

    pub fn write_b(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;

        match addr {
            // bios area, 256B long for regular gameboy.
            0x0000..=0x00FF if self.loading_bios.get() => {
                panic!("Writing to bootrom ${:04x} <- {:02x}", addr, data)
            }
            // ROM area, this is banked memory, it will swap according to selected bank
            0x0000..=0x7FFF => self.mbc.write(addr as u16, data),
            // Graphics, 8kB VRAM
            0x8000..=0x9FFF => self.data[(addr & 0x1FFF)] = data,
            // Switchable RAM bank, 8kB
            0xA000..=0xBFFF => self.mbc.write(addr as u16, data),
            // Internal RAM, 8kB
            0xC000..=0xDFFF => self.data[addr] = data,
            // Echo of internal RAM, this is less than 8k, up to 0xFDFF,
            0xE000..=0xFDFF => self.data[(addr - 0x2000) as usize] = data,
            // Sprite attribute memory, 160B
            0xFE00..=0xFE9F => self.data[(addr - 0xFE00)] = data,
            // empty
            0xFEA0..=0xFEFF | 0xFF4C..=0xFF7F => (),
            // I/O ports
            0xFF00..=0xFF4B => match addr & 0xFF {
                0x40..=0x47 => self.data[addr] = data, // GPU registers
                _ => {}
            },
            // High RAM (zero page), used with LDH instructions
            0xFF80..=0xFFFF => self.data[addr] = data,
            _ => unimplemented!(),
        }
    }
}
