/// Module for jump and related instructions (call, return, reset)
use jeebie::core::cpu::CPU;
use jeebie::core::registers::{Flags, Register16};

// TODO: make appropriate timings for jumps when a branch is taken or not.

// 'JP nn' C3 12
pub fn JP_nn(cpu: &mut CPU) -> i32 {
    let addr = cpu.get_immediate16();
    cpu.jump(addr);
    12
}

// 'JP NZ,nn' C2 12
pub fn JP_NZ_nn(cpu: &mut CPU) -> i32 {
    cpu.jump_not_flag(Flags::Zero, Register16::NN);
    12
}

// 'JP Z,nn' CA 12
pub fn JP_Z_nn(cpu: &mut CPU) -> i32 {
    cpu.jump_flag(Flags::Zero, Register16::NN);
    12
}

// 'JP NC,nn' D2 12
pub fn JP_NC_nn(cpu: &mut CPU) -> i32 {
    cpu.jump_not_flag(Flags::Carry, Register16::NN);
    12
}

// 'JP C,nn' DA 12
pub fn JP_C_nn(cpu: &mut CPU) -> i32 {
    cpu.jump_flag(Flags::Zero, Register16::NN);
    12
}

// 'JP (HL)' E9 4
pub fn JP_HL(cpu: &mut CPU) -> i32 {
    let addr = cpu.get16(Register16::HL);
    cpu.jump(addr);
    4
}

// 'JR n' 18 8
pub fn JR_n(cpu: &mut CPU) -> i32 {
    let pc = cpu.reg.pc + 1; // it isn't updated yet
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16); // n is loaded as a signed bit
    cpu.jump(addr);
    8
}

// 'JR NZ,*' 20 8
pub fn JR_NZ_n(cpu: &mut CPU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_not_flag(Flags::Zero, Register16::Value16(addr));
    8
}

// 'JR Z,*' 28 8
pub fn JR_Z_n(cpu: &mut CPU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_flag(Flags::Zero, Register16::Value16(addr));
    8
}

// 'JR NC,*' 30 8
pub fn JR_NC_n(cpu: &mut CPU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_not_flag(Flags::Carry, Register16::Value16(addr));
    8
}

// 'JR C,*' 38 8
pub fn JR_C_n(cpu: &mut CPU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_flag(Flags::Carry, Register16::Value16(addr));
    8
}

// 'CALL nn' CD 12
pub fn CALL_nn(cpu: &mut CPU) -> i32 {
    let call_addr = cpu.get16(Register16::NN);
    let next_instr = cpu.reg.pc;
    cpu.push_stack(Register16::Value16(next_instr));
    cpu.jump(call_addr);
    12
}

// 'CALL NZ,nn' C4 12
pub fn CALL_NZ_nn(cpu: &mut CPU) -> i32 {
    if !cpu.reg.is_set(Flags::Zero) {
        CALL_nn(cpu);
    }
    12
}

// 'CALL Z,nn' CC 12
pub fn CALL_Z_nn(cpu: &mut CPU) -> i32 {
    if cpu.reg.is_set(Flags::Zero) {
        CALL_nn(cpu);
    }
    12
}

// 'CALL NC,nn' D4 12
pub fn CALL_NC_nn(cpu: &mut CPU) -> i32 {
    if !cpu.reg.is_set(Flags::Carry) {
        CALL_nn(cpu);
    }
    12
}

// 'CALL C,nn' DC 12
pub fn CALL_C_nn(cpu: &mut CPU) -> i32 {
    if cpu.reg.is_set(Flags::Carry) {
        CALL_nn(cpu);
    }
    12
}

// 'RST 00H' C7 32
pub fn RST_00h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x00);
    32
}

// 'RST 08H' CF 32
pub fn RST_08h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x08);
    32
}

// 'RST 10H' D7 32
pub fn RST_10h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x10);
    32
}

// 'RST 18H' DF 32
pub fn RST_18h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x18);
    32
}

// 'RST 20H' E7 32
pub fn RST_20h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x20);
    32
}

// 'RST 28H' EF 32
pub fn RST_28h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x28);
    32
}

// 'RST 30H' F7 32
pub fn RST_30h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x30);
    32
}

// 'RST 38H' FF 32
pub fn RST_38h(cpu: &mut CPU) -> i32 {
    cpu.restart(0x38);
    32
}

// 'RET' C9 8
pub fn RET(cpu: &mut CPU) -> i32 {
    cpu.pop_stack(Register16::PC);
    8
}

// 'RET NZ' C0 8
pub fn RET_NZ(cpu: &mut CPU) -> i32 {
    cpu.return_flag(Flags::Zero);
    8
}

// 'RET Z' C8 8
pub fn RET_Z(cpu: &mut CPU) -> i32 {
    cpu.return_not_flag(Flags::Zero);
    8
}

// 'RET NC' D0 8
pub fn RET_NC(cpu: &mut CPU) -> i32 {
    cpu.return_flag(Flags::Carry);
    8
}

// 'RET C' D8 8
pub fn RET_C(cpu: &mut CPU) -> i32 {
    cpu.return_not_flag(Flags::Carry);
    8
}

// 'RETI' D9 8
pub fn RETI(cpu: &mut CPU) -> i32 {
    cpu.pop_stack(Register16::PC);
    cpu.interrupts_enabled = true;
    8
}
