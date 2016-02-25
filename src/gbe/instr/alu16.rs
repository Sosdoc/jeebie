/// Module for 16 bit arithmetic (ALU instructions)

use gbe::cpu::CPU;

// ADD HL,BC 09 8
pub fn ADD_hl_bc(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.get();
    cpu.compute_add16(rhs);
}

// ADD HL,DE 19 8
pub fn ADD_hl_de(cpu: &mut CPU) {
    let rhs = cpu.reg.de.get();
    cpu.compute_add16(rhs);
}

// ADD HL,HL 29 8
pub fn ADD_hl_hl(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.get();
    cpu.compute_add16(rhs);
}

// ADD HL,SP 39 8
pub fn ADD_hl_sp(cpu: &mut CPU) {
    let rhs = cpu.reg.sp.get();
    cpu.compute_add16(rhs);
}
