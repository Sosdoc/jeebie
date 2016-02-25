use gbe::cpu::CPU;

pub mod load;
pub mod stack;
pub mod alu8;
pub mod alu16;
pub mod misc;

/// NO-OP, only updates clock
pub fn nop(cpu: &mut CPU) {}
