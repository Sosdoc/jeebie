mod cpu;
mod bit;
mod memory;
mod register;

use memory::Memory;
use cpu::CPU;

pub type Pixel = (u8, u8, u8);

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;
pub const FB_SIZE: u32 = WIDTH * HEIGHT;

pub struct Emulator {
    framebuffer: Vec<Pixel>,
    cpu: CPU,
    memory: Memory,
}

impl Emulator {
    pub fn new() -> Emulator {
        let framebuffer: Vec<Pixel> = vec![(255,255,255); FB_SIZE as usize];

        Emulator{
            framebuffer,
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn run_until_frame(&mut self) -> &[Pixel] {
        &self.framebuffer
    }
}