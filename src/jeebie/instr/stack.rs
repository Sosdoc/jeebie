use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register16::*;

// 'PUSH AF' F5 16
pub fn PUSH_af(cpu: &mut CPU) {
    cpu.push_stack(AF);
}

// 'PUSH BC' C5 16
pub fn PUSH_bc(cpu: &mut CPU) {
    cpu.push_stack(BC);
}

// 'PUSH DE' D5 16
pub fn PUSH_de(cpu: &mut CPU) {
    cpu.push_stack(DE);
}

// 'PUSH HL' E5 16
pub fn PUSH_hl(cpu: &mut CPU) {
    cpu.push_stack(HL);
}

// 'POP AF' F1 12
pub fn POP_af(cpu: &mut CPU) {
    cpu.pop_stack(AF);
}

// 'POP BC' C1 12
pub fn POP_bc(cpu: &mut CPU) {
    cpu.pop_stack(BC);
}

// 'POP DE' D1 12
pub fn POP_de(cpu: &mut CPU) {
    cpu.pop_stack(DE);
}

// 'POP HL' E1 12
pub fn POP_hl(cpu: &mut CPU) {
    cpu.pop_stack(HL);
}
