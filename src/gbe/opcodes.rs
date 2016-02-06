use gbe::instr::*;
use gbe::cpu::CPU;

pub type Opcode = fn(&mut CPU) -> ();

pub static OPCODE_TABLE : [Opcode; 256] = [
    // 00
    nop,    LDrr_bb,    LDrr_bc,    LDrr_bd,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 10
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 20
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 30
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 40
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 50
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 60
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 70
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 80
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // 90
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // A0
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // B0
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // C0
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // D0
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // E0
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    // F0
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
    nop,    nop,    nop,    nop,
];
