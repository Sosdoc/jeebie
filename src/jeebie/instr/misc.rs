/// Module for miscellaneous instructions
use jeebie::core::cpu::CPU;
use jeebie::registers::Register8::*;
use jeebie::registers::Register16::*;

// 'NOP' 00 4
pub fn nop(cpu: &mut CPU) {}

// 'SWAP A' CB 37 8
pub fn SWAP_a(cpu: &mut CPU) {
    cpu.compute_swap(A);
}

// 'SWAP B' CB 30 8
pub fn SWAP_b(cpu: &mut CPU) {
    cpu.compute_swap(B);
}

// 'SWAP C' CB 31 8
pub fn SWAP_c(cpu: &mut CPU) {
    cpu.compute_swap(C);
}

// 'SWAP D' CB 32 8
pub fn SWAP_d(cpu: &mut CPU) {
    cpu.compute_swap(D);
}

// 'SWAP E' CB 33 8
pub fn SWAP_e(cpu: &mut CPU) {
    cpu.compute_swap(E);
}

// 'SWAP H' CB 34 8
pub fn SWAP_h(cpu: &mut CPU) {
    cpu.compute_swap(H);
}

// 'SWAP L' CB 35 8
pub fn SWAP_l(cpu: &mut CPU) {
    cpu.compute_swap(L);
}

// 'SWAP (HL)' CB 36 16
pub fn SWAP_hl(cpu: &mut CPU) {
    cpu.compute_swap(RegisterAddress(HL));
}

// 'EI' FB 4
pub fn EI(cpu: &mut CPU) {
    cpu.interrupts_enabled = true;
}

// 'DI' F3 4
pub fn FI(cpu: &mut CPU) {
    cpu.interrupts_enabled = false;
}