pub mod cpu;
pub mod memory;
mod util;

pub struct Emulator {
    cpu: cpu::CPU,
    mem: memory::MMU,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CPU::new(),
            mem: memory::MMU::new(),
        }
    }
}
