/// Module for 16 bit arithmetic (ALU instructions)

use jeebie::core::cpu::CPU;
use jeebie::registers::Register16::*;

// 'ADD HL,BC' 09 8
pub fn ADD_hl_bc(cpu: &mut CPU) {
    cpu.compute_add16(HL, BC);
}

// 'ADD HL,DE' 19 8
pub fn ADD_hl_de(cpu: &mut CPU) {
    cpu.compute_add16(HL, DE);
}

// 'ADD HL,HL' 29 8
pub fn ADD_hl_hl(cpu: &mut CPU) {
    cpu.compute_add16(HL, HL);
}

// 'ADD HL,SP' 39 8
pub fn ADD_hl_sp(cpu: &mut CPU) {
    cpu.compute_add16(HL, SP);
}

// 'ADD SP,#' E8 16
pub fn ADD_sp_n(cpu: &mut CPU) {
    let value = Value16(cpu.get_immediate8() as u16);
    cpu.compute_add16(SP, value);
}

// 'INC BC' 03 8
pub fn INC_bc(cpu: &mut CPU) {
    cpu.compute_inc16(BC);
}

// 'INC DE' 13 8
pub fn INC_de(cpu: &mut CPU) {
    cpu.compute_inc16(DE);
}

// 'INC HL' 23 8
pub fn INC_hl(cpu: &mut CPU) {
    cpu.compute_inc16(HL);
}

// 'INC SP' 33 8
pub fn INC_sp(cpu: &mut CPU) {
    cpu.compute_inc16(SP);
}

// 'DEC BC' 0B 8
pub fn DEC_bc(cpu: &mut CPU) {
    cpu.compute_dec16(BC);
}

// 'DEC DE' 1B 8
pub fn DEC_de(cpu: &mut CPU) {
    cpu.compute_dec16(DE);
}

// 'DEC HL' 2B 8
pub fn DEC_hl(cpu: &mut CPU) {
    cpu.compute_dec16(HL);
}

// 'DEC SP' 3B 8
pub fn DEC_sp(cpu: &mut CPU) {
    cpu.compute_dec16(SP);
}
