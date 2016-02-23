use gbe::cpu::CPU;

pub mod load;
pub mod stack;
pub mod alu;

/// NO-OP, only updates clock
pub fn nop(cpu: &mut CPU) {}
