use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register16::*;

// 'PUSH AF' F5 16
pub fn PUSH_af(cpu: &mut CPU) -> i32 {
    cpu.push_stack(AF);
    16
}

// 'PUSH BC' C5 16
pub fn PUSH_bc(cpu: &mut CPU) -> i32 {
    cpu.push_stack(BC);
    16
}

// 'PUSH DE' D5 16
pub fn PUSH_de(cpu: &mut CPU) -> i32 {
    cpu.push_stack(DE);
    16
}

// 'PUSH HL' E5 16
pub fn PUSH_hl(cpu: &mut CPU) -> i32 {
    cpu.push_stack(HL);
    16
}

// 'POP AF' F1 12
pub fn POP_af(cpu: &mut CPU) -> i32 {
    cpu.pop_stack(AF);
    12
}

// 'POP BC' C1 12
pub fn POP_bc(cpu: &mut CPU) -> i32 {
    cpu.pop_stack(BC);
    12
}

// 'POP DE' D1 12
pub fn POP_de(cpu: &mut CPU) -> i32 {
    cpu.pop_stack(DE);
    12
}

// 'POP HL' E1 12
pub fn POP_hl(cpu: &mut CPU) -> i32 {
    cpu.pop_stack(HL);
    12
}
