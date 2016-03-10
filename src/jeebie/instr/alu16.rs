/// Module for 16 bit arithmetic (ALU instructions)

use jeebie::core::cpu::CPU;
use jeebie::registers::Flags;

// 'ADD HL,BC' 09 8
pub fn ADD_hl_bc(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.get();
    cpu.compute_add16(rhs);
}

// 'ADD HL,DE' 19 8
pub fn ADD_hl_de(cpu: &mut CPU) {
    let rhs = cpu.reg.de.get();
    cpu.compute_add16(rhs);
}

// 'ADD HL,HL' 29 8
pub fn ADD_hl_hl(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.get();
    cpu.compute_add16(rhs);
}

// 'ADD HL,SP' 39 8
pub fn ADD_hl_sp(cpu: &mut CPU) {
    let rhs = cpu.reg.sp.get();
    cpu.compute_add16(rhs);
}

// 'ADD SP,#' E8 16
pub fn ADD_sp_n(cpu: &mut CPU) {
    let value = cpu.get_immediate8() as u16;
    let sp = cpu.reg.sp.get();

    let result = sp.wrapping_add(value);

    // flags
    cpu.reg.clear_all_flags();

    if let None = sp.checked_add(value) {
        cpu.reg.set_flag(Flags::Carry);
    }

    if let None = (sp as u8).checked_add(value as u8) {
        cpu.reg.set_flag(Flags::HalfCarry);
    }

    cpu.reg.sp.set(sp);
}

// 'INC BC' 03 8
pub fn INC_bc(cpu: &mut CPU) {
    cpu.reg.bc.add(1);
}

// 'INC DE' 13 8
pub fn INC_de(cpu: &mut CPU) {
    cpu.reg.de.add(1);
}

// 'INC HL' 23 8
pub fn INC_hl(cpu: &mut CPU) {
    cpu.reg.hl.add(1);
}

// 'INC SP' 33 8
pub fn INC_sp(cpu: &mut CPU) {
    cpu.reg.sp.add(1);
}

// 'DEC BC' 0B 8
pub fn DEC_bc(cpu: &mut CPU) {
    cpu.reg.bc.sub(1);
}

// 'DEC DE' 1B 8
pub fn DEC_de(cpu: &mut CPU) {
    cpu.reg.de.sub(1);
}

// 'DEC HL' 2B 8
pub fn DEC_hl(cpu: &mut CPU) {
    cpu.reg.hl.sub(1);
}

// 'DEC SP' 3B 8
pub fn DEC_sp(cpu: &mut CPU) {
    cpu.reg.sp.sub(1);
}
