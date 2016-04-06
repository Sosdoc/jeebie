/// Module for 16 bit arithmetic (ALU instructions)
use jeebie::core::cpu::CPU;
use jeebie::registers::{ Flags, Register16 };

// 'JP nn' C3 12
pub fn JP_nn(cpu: &mut CPU) {
    let addr = cpu.get_immediate16();
    cpu.jump(addr);
}

// 'JP NZ,nn' C2 12
pub fn JP_NZ_nn(cpu: &mut CPU) {
    cpu.jump_not_flag(Flags::Zero, Register16::Immediate16);
}

// 'JP Z,nn' CA 12
pub fn JP_Z_nn(cpu: &mut CPU) {
    cpu.jump_flag(Flags::Zero, Register16::Immediate16);
}

// 'JP NC,nn' D2 12
pub fn JP_NC_nn(cpu: &mut CPU) {
    cpu.jump_not_flag(Flags::Carry, Register16::Immediate16);
}

// 'JP C,nn' DA 12
pub fn JP_C_nn(cpu: &mut CPU) {
    cpu.jump_flag(Flags::Zero, Register16::Immediate16);
}

// 'JP (HL)' E9 4
pub fn JP_HL(cpu: &mut CPU) {
    let addr = cpu.get16(Register16::HL);
    cpu.jump(addr);
}

// 'JR n' 18 8 
pub fn JR_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1; // it isn't updated yet
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16); // n is loaded as a signed bit
    cpu.jump(addr);
}

// 'JR NZ,*' 20 8
pub fn JR_NZ_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_not_flag(Flags::Zero, Register16::Value16(addr));
}

// 'JR Z,*' 28 8
pub fn JR_Z_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_flag(Flags::Zero, Register16::Value16(addr));
}

// 'JR NC,*' 30 8
pub fn JR_NC_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_not_flag(Flags::Carry, Register16::Value16(addr));
}

// 'JR C,*' 38 8
pub fn JR_C_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_flag(Flags::Carry, Register16::Value16(addr));
}
