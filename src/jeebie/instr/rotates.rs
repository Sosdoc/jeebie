//! Rotates & Shifts
use jeebie::core::cpu::CPU;
use jeebie::registers::Register8::*;
use jeebie::registers::Register16::*;

// 'RLCA' 07 4
pub fn RLCA(cpu: &mut CPU) {
    cpu.rotate_left_carry(A);
}

// 'RLA' 17 4
pub fn RLA(cpu: &mut CPU) {
    cpu.rotate_left(A);
}

// 'RRCA' 0F 4
pub fn RRCA(cpu: &mut CPU) {
    cpu.rotate_right_carry(A);
}

// 'RRA' 1F 4
pub fn RRA(cpu: &mut CPU) {
    cpu.rotate_right(A);
}

// 'RLC A' CB 07 8
pub fn RLC_A(cpu: &mut CPU) {
    cpu.rotate_left_carry(A);
}

// 'RLC B' CB 00 8
pub fn RLC_B(cpu: &mut CPU) {
    cpu.rotate_left_carry(B);
}

// 'RLC C' CB 01 8
pub fn RLC_C(cpu: &mut CPU) {
    cpu.rotate_left_carry(C);
}

// 'RLC D' CB 02 8
pub fn RLC_D(cpu: &mut CPU) {
    cpu.rotate_left_carry(D);
}

// 'RLC E' CB 03 8
pub fn RLC_E(cpu: &mut CPU) {
    cpu.rotate_left_carry(E);
}

// 'RLC H' CB 04 8
pub fn RLC_H(cpu: &mut CPU) {
    cpu.rotate_left_carry(H);
}

// 'RLC L' CB 05 8
pub fn RLC_L(cpu: &mut CPU) {
    cpu.rotate_left_carry(L);
}

// 'RLC (HL)' CB 06 16
pub fn RLC_HLm(cpu: &mut CPU) {
    cpu.rotate_left_carry(RegisterAddress(HL));
}

// 'RL A' CB 17 8
pub fn RL_A(cpu: &mut CPU) {
    cpu.rotate_left(A);
}

// 'RL B' CB 10 8
pub fn RL_B(cpu: &mut CPU) {
    cpu.rotate_left(B);
}

// 'RL C' CB 11 8
pub fn RL_C(cpu: &mut CPU) {
    cpu.rotate_left(C);
}

// 'RL D' CB 12 8
pub fn RL_D(cpu: &mut CPU) {
    cpu.rotate_left(D);
}

// 'RL E' CB 13 8
pub fn RL_E(cpu: &mut CPU) {
    cpu.rotate_left(E);
}

// 'RL H' CB 14 8
pub fn RL_H(cpu: &mut CPU) {
    cpu.rotate_left(H);
}

// 'RL L' CB 15 8
pub fn RL_L(cpu: &mut CPU) {
    cpu.rotate_left(L);
}

// 'RL (HL)' CB 16 16
pub fn RL_HLm(cpu: &mut CPU) {
    cpu.rotate_left(RegisterAddress(HL));
}

// 'RRC A' CB 0F 8
pub fn RRC_A(cpu: &mut CPU) {
    cpu.rotate_right_carry(A);
}

// 'RRC B' CB 08 8
pub fn RRC_B(cpu: &mut CPU) {
    cpu.rotate_right_carry(B);
}

// 'RRC C' CB 09 8
pub fn RRC_C(cpu: &mut CPU) {
    cpu.rotate_right_carry(C);
}

// 'RRC D' CB 0A 8
pub fn RRC_D(cpu: &mut CPU) {
    cpu.rotate_right_carry(D);
}

// 'RRC E' CB 0B 8
pub fn RRC_E(cpu: &mut CPU) {
    cpu.rotate_right_carry(E);
}

// 'RRC H' CB 0C 8
pub fn RRC_H(cpu: &mut CPU) {
    cpu.rotate_right_carry(H);
}

// 'RRC L' CB 0D 8
pub fn RRC_L(cpu: &mut CPU) {
    cpu.rotate_right_carry(L);
}

// 'RRC (HL)' CB 0E 16
pub fn RRC_HLm(cpu: &mut CPU) {
    cpu.rotate_right_carry(RegisterAddress(HL));
}

// 'RR A' CB 1F 8
pub fn RR_A(cpu: &mut CPU) {
    cpu.rotate_right(A);
}

// 'RR B' CB 18 8
pub fn RR_B(cpu: &mut CPU) {
    cpu.rotate_right(B);
}

// 'RR C' CB 19 8
pub fn RR_C(cpu: &mut CPU) {
    cpu.rotate_right(C);
}

// 'RR D' CB 1A 8
pub fn RR_D(cpu: &mut CPU) {
    cpu.rotate_right(D);
}

// 'RR E' CB 1B 8
pub fn RR_E(cpu: &mut CPU) {
    cpu.rotate_right(E);
}

// 'RR H' CB 1C 8
pub fn RR_H(cpu: &mut CPU) {
    cpu.rotate_right(H);
}

// 'RR L' CB 1D 8
pub fn RR_L(cpu: &mut CPU) {
    cpu.rotate_right(L);
}

// 'RR (HL)' CB 1E 16
pub fn RR_HLm(cpu: &mut CPU) {
    cpu.rotate_right(RegisterAddress(HL));
}