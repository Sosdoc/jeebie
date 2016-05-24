/// Module for bit operations (checking, set/reset, etc.)

use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::HL;


// 'BIT 0,A' CB 47 8
pub fn BIT_0_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, A);
    8
}

// 'BIT 0,B' CB 40 8
pub fn BIT_0_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, B);
    8
}

// 'BIT 0,C' CB 41 8
pub fn BIT_0_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, C);
    8
}

// 'BIT 0,D' CB 42 8
pub fn BIT_0_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, D);
    8
}

// 'BIT 0,E' CB 43 8
pub fn BIT_0_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, E);
    8
}

// 'BIT 0,H' CB 44 8
pub fn BIT_0_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, H);
    8
}

// 'BIT 0,L' CB 45 8
pub fn BIT_0_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, L);
    8
}

// 'BIT 0,(HL)' CB 46 16
pub fn BIT_0_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, RegisterAddress(HL));
    16
}


// 'BIT 1,A' CB 4F 8
pub fn BIT_1_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, A);
    8
}

// 'BIT 1,B' CB 48 8
pub fn BIT_1_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, B);
    8
}

// 'BIT 1,C' CB 49 8
pub fn BIT_1_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, C);
    8
}

// 'BIT 1,D' CB 4A 8
pub fn BIT_1_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, D);
    8
}

// 'BIT 1,E' CB 4B 8
pub fn BIT_1_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, E);
    8
}

// 'BIT 1,H' CB 4C 8
pub fn BIT_1_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, H);
    8
}

// 'BIT 1,L' CB 4D 8
pub fn BIT_1_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, L);
    8
}

// 'BIT 1,(HL)' CB 4E 16
pub fn BIT_1_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(1, RegisterAddress(HL));
    16
}


// 'BIT 2,A' CB 57 8
pub fn BIT_2_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, A);
    8
}

// 'BIT 2,B' CB 50 8
pub fn BIT_2_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, B);
    8
}

// 'BIT 2,C' CB 51 8
pub fn BIT_2_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, C);
    8
}

// 'BIT 2,D' CB 52 8
pub fn BIT_2_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, D);
    8
}

// 'BIT 2,E' CB 53 8
pub fn BIT_2_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, E);
    8
}

// 'BIT 2,H' CB 54 8
pub fn BIT_2_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, H);
    8
}

// 'BIT 2,L' CB 55 8
pub fn BIT_2_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, L);
    8
}

// 'BIT 2,(HL)' CB 56 16
pub fn BIT_2_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(2, RegisterAddress(HL));
    16
}

// 'BIT 3,A' CB 5F 8
pub fn BIT_3_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, A);
    8
}

// 'BIT 3,B' CB 58 8
pub fn BIT_3_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, B);
    8
}

// 'BIT 3,C' CB 59 8
pub fn BIT_3_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, C);
    8
}

// 'BIT 3,D' CB 5A 8
pub fn BIT_3_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, D);
    8
}

// 'BIT 3,E' CB 5B 8
pub fn BIT_3_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, E);
    8
}

// 'BIT 3,H' CB 5C 8
pub fn BIT_3_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, H);
    8
}

// 'BIT 3,L' CB 5D 8
pub fn BIT_3_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, L);
    8
}

// 'BIT 3,(HL)' CB 5E 16
pub fn BIT_3_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(3, RegisterAddress(HL));
    16
}

// 'BIT 4,A' CB 67 8
pub fn BIT_4_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, A);
    8
}

// 'BIT 4,B' CB 60 8
pub fn BIT_4_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, B);
    8
}

// 'BIT 4,C' CB 61 8
pub fn BIT_4_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, C);
    8
}

// 'BIT 4,D' CB 62 8
pub fn BIT_4_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, D);
    8
}

// 'BIT 4,E' CB 63 8
pub fn BIT_4_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, E);
    8
}

// 'BIT 4,H' CB 64 8
pub fn BIT_4_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, H);
    8
}

// 'BIT 4,L' CB 65 8
pub fn BIT_4_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, L);
    8
}

// 'BIT 4,(HL)' CB 66 16
pub fn BIT_4_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(4, RegisterAddress(HL));
    16
}

// 'BIT 5,A' CB 6F 8
pub fn BIT_5_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, A);
    8
}

// 'BIT 5,B' CB 68 8
pub fn BIT_5_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, B);
    8
}

// 'BIT 5,C' CB 69 8
pub fn BIT_5_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, C);
    8
}

// 'BIT 5,D' CB 6A 8
pub fn BIT_5_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, D);
    8
}

// 'BIT 5,E' CB 6B 8
pub fn BIT_5_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, E);
    8
}

// 'BIT 5,H' CB 6C 8
pub fn BIT_5_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, H);
    8
}

// 'BIT 5,L' CB 6D 8
pub fn BIT_5_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, L);
    8
}

// 'BIT 5,(HL)' CB 6E 16
pub fn BIT_5_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(5, RegisterAddress(HL));
    16
}


// 'BIT 6,A' CB 77 8
pub fn BIT_6_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, A);
    8
}

// 'BIT 6,B' CB 70 8
pub fn BIT_6_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, B);
    8
}

// 'BIT 6,C' CB 71 8
pub fn BIT_6_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, C);
    8
}

// 'BIT 6,D' CB 72 8
pub fn BIT_6_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, D);
    8
}

// 'BIT 6,E' CB 73 8
pub fn BIT_6_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, E);
    8
}

// 'BIT 6,H' CB 74 8
pub fn BIT_6_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, H);
    8
}

// 'BIT 6,L' CB 75 8
pub fn BIT_6_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, L);
    8
}

// 'BIT 6,(HL)' CB 76 16
pub fn BIT_6_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(6, RegisterAddress(HL));
    16
}

// 'BIT 7,A' CB 7F 8
pub fn BIT_7_A(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, A);
    8
}

// 'BIT 7,B' CB 78 8
pub fn BIT_7_B(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, B);
    8
}

// 'BIT 7,C' CB 79 8
pub fn BIT_7_C(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, C);
    8
}

// 'BIT 7,D' CB 7A 8
pub fn BIT_7_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, D);
    8
}

// 'BIT 7,E' CB 7B 8
pub fn BIT_7_E(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, E);
    8
}

// 'BIT 7,H' CB 7C 8
pub fn BIT_7_H(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, H);
    8
}

// 'BIT 7,L' CB 7D 8
pub fn BIT_7_L(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, L);
    8
}

// 'BIT 7,(HL)' CB 7E 16
pub fn BIT_7_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_check(7, RegisterAddress(HL));
    16
}
