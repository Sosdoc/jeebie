//! The memory module.
//! This module declares a single struct, the Memory Management Unit (MMU).
//! The MMU acts as the system bus, allowing components to communicate with each other, reaches
//! RAM, ROM, I/O registers and more.
use std::fmt;
use std::cell::Cell;

use jeebie::video::gpu::GPU;
use jeebie::cart::Cartridge;
use jeebie::mbc::MemoryBankController;
use jeebie::mbc::nombc::RomOnly;
use jeebie::bootrom::DMG_BOOTROM;

/// The Memory Management Unit.
/// Provides access to all mapped memory in the system, including I/O and graphics.
pub struct MMU {
    // TODO: MMU should own RAM/High RAM (8k + 256 bytes), maybe some registers.
    data: Vec<u8>,
    loading_bios: Cell<bool>,
    mbc: Box<MemoryBankController>,
    pub gpu: GPU,
}

impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU 64k")
    }
}

impl MMU {

    /// Creates a new memory controller with no program loaded, except for the bootrom.
    pub fn new() -> Self {
        MMU {
            loading_bios: Cell::new(true),
            data: vec![0; 65536],
            mbc: Box::new(RomOnly::new()),
            gpu: GPU::new(),
        }
    }

    /// Creates a memory controller with the specified cartridge loaded.
    pub fn new_with_rom(cart: &Cartridge) -> Self {
        let mut mmu = MMU::new();
        mmu.load_rom(cart);
        mmu
    }

    fn load_rom(&mut self, cart: &Cartridge) {
        for i in 0..cart.size {
            self.data[i] = cart.data[i];
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
            0...0x00FF if self.loading_bios.get() => DMG_BOOTROM[(addr & 0xFF) as usize],
            // ROM0 area accessible only when bootrom is disabled (first 256 bytes).
            0x0000...0x00FF if !self.loading_bios.get() => self.data[addr as usize],
            // ROM0 area, this is banked memory, it will swap according to selected bank
            0x0100...0x3FFF => self.data[addr as usize],
            // ROM1 area, 16kB unbanked data
            0x4000...0x7FFF => self.data[addr as usize],
            // Graphics, 8kB VRAM
            0x8000...0x9FFF => self.gpu.read_vram((addr & 0x1FFF) as usize),
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
            0xFE00...0xFE9F => self.gpu.read_oam((addr - 0xFE00) as usize),
            // empty
            0xFEA0...0xFEFF | 0xFF4C...0xFF7F => 0,
            // I/O ports
            0xFF00...0xFF4B => {
                match addr & 0xFF {
                    0x40...0x47 => self.gpu.read_register(addr as usize),
                    _ => unimplemented!(),
                }
            },
            // High RAM (zero page), used with LDH instructions
            0xFF80...0xFFFE => self.data[addr as usize],
            // Interrupt Enable register
            0xFFFF => unimplemented!(),

            _ => panic!("tried to read at unknown address: {:04x}", addr),
        }
    }

    pub fn write_b(&mut self, addr: u16, data: u8) {
        match addr {
            // bios area, 256B long for regular gameboy.
            0...0x00FF if self.loading_bios.get() => panic!("Writing to bootrom ${:04x} <- {:02x}", addr, data),
            // ROM0 area, this is banked memory, it will swap according to selected bank
            0x0000...0x3FFF => panic!("Writing to ROM0 ${:04x} <- {:02x}", addr, data),
            // ROM1 area, 16kB unbanked data
            0x4000...0x7FFF => panic!("Writing to ROM1 ${:04x} <- {:02x}", addr, data),
            // Graphics, 8kB VRAM
            0x8000...0x9FFF => self.gpu.write_vram((addr & 0x1FFF) as usize, data),
            // Switchable RAM bank, 8kB
            0xA000...0xBFFF => {
                // TODO: handle RAM banks
                self.data[addr as usize] = data;
            }
            // Internal RAM, 8kB
            0xC000...0xDFFF => {
                self.data[addr as usize] = data;
            },
            // Echo of internal RAM, this is less than 8k, up to 0xFDFF,
            0xE000...0xFDFF => {
                self.data[(addr - 0x2000) as usize] = data;
            },
            // Sprite attribute memory, 160B
            0xFE00...0xFE9F => {
                self.gpu.write_oam((addr - 0xFE00) as usize, data);
            }
            // empty
            0xFEA0...0xFEFF | 0xFF4C...0xFF7F => {},
            // I/O ports
            0xFF00...0xFF4B => {
                match addr & 0xFF {
                    0x40...0x47 => self.gpu.write_register(addr as usize, data),
                    _ => {},
                }
            },
            // High RAM (zero page), used with LDH instructions
            0xFF80...0xFFFE => { self.data[addr as usize] = data; },
            // Interrupt Enable register
            0xFFFF => unimplemented!(),

            _ => panic!("tried to write at unknown address: {:4x}", addr),
        }
    }
}
