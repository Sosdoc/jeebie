/// Module for miscellaneous instructions
use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::*;

// 'NOP' 00 4
pub fn nop(cpu: &mut CPU) -> i32 { 4 }

// 'SWAP A' CB 37 8
pub fn SWAP_a(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(A);
    8
}

// 'SWAP B' CB 30 8
pub fn SWAP_b(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(B);
    8
}

// 'SWAP C' CB 31 8
pub fn SWAP_c(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(C);
    8
}

// 'SWAP D' CB 32 8
pub fn SWAP_d(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(D);
    8
}

// 'SWAP E' CB 33 8
pub fn SWAP_e(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(E);
    8
}

// 'SWAP H' CB 34 8
pub fn SWAP_h(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(H);
    8
}

// 'SWAP L' CB 35 8
pub fn SWAP_l(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(L);
    8
}

// 'SWAP (HL)' CB 36 16
pub fn SWAP_hl(cpu: &mut CPU) -> i32 {
    cpu.compute_swap(RegisterAddress(HL));
    16
}

// 'EI' FB 4
pub fn EI(cpu: &mut CPU) -> i32 {
    cpu.interrupts_enabled = true;
    4
}

// 'DI' F3 4
pub fn DI(cpu: &mut CPU) -> i32 {
    cpu.interrupts_enabled = false;
    4
}