use gbe::cpu::CPU;

// 'PUSH AF' F5 16
pub fn PUSH_af(cpu: &mut CPU) {
    let reg = cpu.reg.af.get();
    cpu.push_stack(reg);
}

// 'PUSH BC' C5 16
pub fn PUSH_bc(cpu: &mut CPU) {
    let reg = cpu.reg.bc.get();
    cpu.push_stack(reg);
}

// 'PUSH DE' D5 16
pub fn PUSH_de(cpu: &mut CPU) {
    let reg = cpu.reg.de.get();
    cpu.push_stack(reg);
}

// 'PUSH HL' E5 16
pub fn PUSH_hl(cpu: &mut CPU) {
    let reg = cpu.reg.hl.get();
    cpu.push_stack(reg);
}

// 'POP AF' F1 12
pub fn POP_af(cpu: &mut CPU) {
    let value = cpu.pop_stack();
    cpu.reg.af.set(value);
}

// 'POP BC' C1 12
pub fn POP_bc(cpu: &mut CPU) {
    let value = cpu.pop_stack();
    cpu.reg.bc.set(value);
}

// 'POP DE' D1 12
pub fn POP_de(cpu: &mut CPU) {
    let value = cpu.pop_stack();
    cpu.reg.de.set(value);
}

// 'POP HL' E1 12
pub fn POP_hl(cpu: &mut CPU) {
    let value = cpu.pop_stack();
    cpu.reg.hl.set(value);
}
