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


// 'RES 0,A' CB 87 8
pub fn RES_0_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, A);
    8
}

// 'RES 0,B' CB 80 8
pub fn RES_0_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, B);
    8
}

// 'RES 0,C' CB 81 8
pub fn RES_0_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, C);
    8
}

// 'RES 0,D' CB 82 8
pub fn RES_0_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, D);
    8
}

// 'RES 0,E' CB 83 8
pub fn RES_0_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, E);
    8
}

// 'RES 0,H' CB 84 8
pub fn RES_0_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, H);
    8
}

// 'RES 0,L' CB 85 8
pub fn RES_0_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, L);
    8
}

// 'RES 0,(HL)' CB 86 16
pub fn RES_0_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(0, RegisterAddress(HL));
    16
}

// 'RES 1,A' CB 8F 8
pub fn RES_1_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, A);
    8
}

// 'RES 1,B' CB 88 8
pub fn RES_1_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, B);
    8
}

// 'RES 1,C' CB 89 8
pub fn RES_1_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, C);
    8
}

// 'RES 1,D' CB 8A 8
pub fn RES_1_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, D);
    8
}

// 'RES 1,E' CB 8B 8
pub fn RES_1_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, E);
    8
}

// 'RES 1,H' CB 8C 8
pub fn RES_1_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, H);
    8
}

// 'RES 1,L' CB 8D 8
pub fn RES_1_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, L);
    8
}

// 'RES 1,(HL)' CB 8E 16
pub fn RES_1_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(1, RegisterAddress(HL));
    16
}

// 'RES 2,A' CB 97 8
pub fn RES_2_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, A);
    8
}

// 'RES 2,B' CB 90 8
pub fn RES_2_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, B);
    8
}

// 'RES 2,C' CB 91 8
pub fn RES_2_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, C);
    8
}

// 'RES 2,D' CB 92 8
pub fn RES_2_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, D);
    8
}

// 'RES 2,E' CB 93 8
pub fn RES_2_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, E);
    8
}

// 'RES 2,H' CB 94 8
pub fn RES_2_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, H);
    8
}

// 'RES 2,L' CB 95 8
pub fn RES_2_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, L);
    8
}

// 'RES 2,(HL)' CB 96 16
pub fn RES_2_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(2, RegisterAddress(HL));
    16
}

// 'RES 3,A' CB 9F 8
pub fn RES_3_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, A);
    8
}

// 'RES 3,B' CB 98 8
pub fn RES_3_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, B);
    8
}

// 'RES 3,C' CB 99 8
pub fn RES_3_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, C);
    8
}

// 'RES 3,D' CB 9A 8
pub fn RES_3_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, D);
    8
}

// 'RES 3,E' CB 9B 8
pub fn RES_3_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, E);
    8
}

// 'RES 3,H' CB 9C 8
pub fn RES_3_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, H);
    8
}

// 'RES 3,L' CB 9D 8
pub fn RES_3_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, L);
    8
}

// 'RES 3,(HL)' CB 9E 16
pub fn RES_3_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(3, RegisterAddress(HL));
    16
}

// 'RES 4,A' CB A7 8
pub fn RES_4_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, A);
    8
}

// 'RES 4,B' CB A0 8
pub fn RES_4_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, B);
    8
}

// 'RES 4,C' CB A1 8
pub fn RES_4_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, C);
    8
}

// 'RES 4,D' CB A2 8
pub fn RES_4_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, D);
    8
}

// 'RES 4,E' CB A3 8
pub fn RES_4_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, E);
    8
}

// 'RES 4,H' CB A4 8
pub fn RES_4_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, H);
    8
}

// 'RES 4,L' CB A5 8
pub fn RES_4_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, L);
    8
}

// 'RES 4,(HL)' CB A6 16
pub fn RES_4_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(4, RegisterAddress(HL));
    16
}

// 'RES 5,A' CB AF 8
pub fn RES_5_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, A);
    8
}

// 'RES 5,B' CB A8 8
pub fn RES_5_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, B);
    8
}

// 'RES 5,C' CB A9 8
pub fn RES_5_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, C);
    8
}

// 'RES 5,D' CB AA 8
pub fn RES_5_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, D);
    8
}

// 'RES 5,E' CB AB 8
pub fn RES_5_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, E);
    8
}

// 'RES 5,H' CB AC 8
pub fn RES_5_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, H);
    8
}

// 'RES 5,L' CB AD 8
pub fn RES_5_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, L);
    8
}

// 'RES 5,(HL)' CB AE 16
pub fn RES_5_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(5, RegisterAddress(HL));
    16
}

// 'RES 6,A' CB B7 8
pub fn RES_6_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, A);
    8
}

// 'RES 6,B' CB B0 8
pub fn RES_6_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, B);
    8
}

// 'RES 6,C' CB B1 8
pub fn RES_6_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, C);
    8
}

// 'RES 6,D' CB B2 8
pub fn RES_6_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, D);
    8
}

// 'RES 6,E' CB B3 8
pub fn RES_6_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, E);
    8
}

// 'RES 6,H' CB B4 8
pub fn RES_6_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, H);
    8
}

// 'RES 6,L' CB B5 8
pub fn RES_6_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, L);
    8
}

// 'RES 6,(HL)' CB B6 16
pub fn RES_6_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(6, RegisterAddress(HL));
    16
}

// 'RES 7,A' CB BF 8
pub fn RES_7_A(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, A);
    8
}

// 'RES 7,B' CB B8 8
pub fn RES_7_B(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, B);
    8
}

// 'RES 7,C' CB B9 8
pub fn RES_7_C(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, C);
    8
}

// 'RES 7,D' CB BA 8
pub fn RES_7_D(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, D);
    8
}

// 'RES 7,E' CB BB 8
pub fn RES_7_E(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, E);
    8
}

// 'RES 7,H' CB BC 8
pub fn RES_7_H(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, H);
    8
}

// 'RES 7,L' CB BD 8
pub fn RES_7_L(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, L);
    8
}

// 'RES 7,(HL)' CB BE 16
pub fn RES_7_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_reset(7, RegisterAddress(HL));
    16
}


// 'SET 0,A' CB C7 8
pub fn SET_0_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, A);
    8
}

// 'SET 0,B' CB C0 8
pub fn SET_0_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, B);
    8
}

// 'SET 0,C' CB C1 8
pub fn SET_0_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, C);
    8
}

// 'SET 0,D' CB C2 8
pub fn SET_0_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, D);
    8
}

// 'SET 0,E' CB C3 8
pub fn SET_0_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, E);
    8
}

// 'SET 0,H' CB C4 8
pub fn SET_0_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, H);
    8
}

// 'SET 0,L' CB C5 8
pub fn SET_0_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, L);
    8
}

// 'SET 0,(HL)' CB C6 16
pub fn SET_0_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(0, RegisterAddress(HL));
    16
}

// 'SET 1,A' CB CF 8
pub fn SET_1_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, A);
    8
}

// 'SET 1,B' CB C8 8
pub fn SET_1_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, B);
    8
}

// 'SET 1,C' CB C9 8
pub fn SET_1_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, C);
    8
}

// 'SET 1,D' CB CA 8
pub fn SET_1_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, D);
    8
}

// 'SET 1,E' CB CB 8
pub fn SET_1_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, E);
    8
}

// 'SET 1,H' CB CC 8
pub fn SET_1_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, H);
    8
}

// 'SET 1,L' CB CD 8
pub fn SET_1_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, L);
    8
}

// 'SET 1,(HL)' CB CE 16
pub fn SET_1_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(1, RegisterAddress(HL));
    16
}

// 'SET 2,A' CB D7 8
pub fn SET_2_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, A);
    8
}

// 'SET 2,B' CB D0 8
pub fn SET_2_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, B);
    8
}

// 'SET 2,C' CB D1 8
pub fn SET_2_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, C);
    8
}

// 'SET 2,D' CB D2 8
pub fn SET_2_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, D);
    8
}

// 'SET 2,E' CB D3 8
pub fn SET_2_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, E);
    8
}

// 'SET 2,H' CB D4 8
pub fn SET_2_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, H);
    8
}

// 'SET 2,L' CB D5 8
pub fn SET_2_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, L);
    8
}

// 'SET 2,(HL)' CB D6 16
pub fn SET_2_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(2, RegisterAddress(HL));
    16
}

// 'SET 3,A' CB DF 8
pub fn SET_3_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, A);
    8
}

// 'SET 3,B' CB D8 8
pub fn SET_3_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, B);
    8
}

// 'SET 3,C' CB D9 8
pub fn SET_3_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, C);
    8
}

// 'SET 3,D' CB DA 8
pub fn SET_3_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, D);
    8
}

// 'SET 3,E' CB DB 8
pub fn SET_3_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, E);
    8
}

// 'SET 3,H' CB DC 8
pub fn SET_3_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, H);
    8
}

// 'SET 3,L' CB DD 8
pub fn SET_3_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, L);
    8
}

// 'SET 3,(HL)' CB DE 16
pub fn SET_3_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(3, RegisterAddress(HL));
    16
}

// 'SET 4,A' CB E7 8
pub fn SET_4_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, A);
    8
}

// 'SET 4,B' CB E0 8
pub fn SET_4_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, B);
    8
}

// 'SET 4,C' CB E1 8
pub fn SET_4_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, C);
    8
}

// 'SET 4,D' CB E2 8
pub fn SET_4_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, D);
    8
}

// 'SET 4,E' CB E3 8
pub fn SET_4_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, E);
    8
}

// 'SET 4,H' CB E4 8
pub fn SET_4_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, H);
    8
}

// 'SET 4,L' CB E5 8
pub fn SET_4_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, L);
    8
}

// 'SET 4,(HL)' CB E6 16
pub fn SET_4_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(4, RegisterAddress(HL));
    16
}

// 'SET 5,A' CB EF 8
pub fn SET_5_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, A);
    8
}

// 'SET 5,B' CB E8 8
pub fn SET_5_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, B);
    8
}

// 'SET 5,C' CB E9 8
pub fn SET_5_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, C);
    8
}

// 'SET 5,D' CB EA 8
pub fn SET_5_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, D);
    8
}

// 'SET 5,E' CB EB 8
pub fn SET_5_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, E);
    8
}

// 'SET 5,H' CB EC 8
pub fn SET_5_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, H);
    8
}

// 'SET 5,L' CB ED 8
pub fn SET_5_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, L);
    8
}

// 'SET 5,(HL)' CB EE 16
pub fn SET_5_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(5, RegisterAddress(HL));
    16
}

// 'SET 6,A' CB F7 8
pub fn SET_6_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, A);
    8
}

// 'SET 6,B' CB F0 8
pub fn SET_6_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, B);
    8
}

// 'SET 6,C' CB F1 8
pub fn SET_6_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, C);
    8
}

// 'SET 6,D' CB F2 8
pub fn SET_6_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, D);
    8
}

// 'SET 6,E' CB F3 8
pub fn SET_6_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, E);
    8
}

// 'SET 6,H' CB F4 8
pub fn SET_6_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, H);
    8
}

// 'SET 6,L' CB F5 8
pub fn SET_6_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, L);
    8
}

// 'SET 6,(HL)' CB F6 16
pub fn SET_6_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(6, RegisterAddress(HL));
    16
}

// 'SET 7,A' CB FF 8
pub fn SET_7_A(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, A);
    8
}

// 'SET 7,B' CB F8 8
pub fn SET_7_B(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, B);
    8
}

// 'SET 7,C' CB F9 8
pub fn SET_7_C(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, C);
    8
}

// 'SET 7,D' CB FA 8
pub fn SET_7_D(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, D);
    8
}

// 'SET 7,E' CB FB 8
pub fn SET_7_E(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, E);
    8
}

// 'SET 7,H' CB FC 8
pub fn SET_7_H(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, H);
    8
}

// 'SET 7,L' CB FD 8
pub fn SET_7_L(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, L);
    8
}

// 'SET 7,(HL)' CB FE 16
pub fn SET_7_HLm(cpu: &mut CPU) -> i32 {
    cpu.bit_set(7, RegisterAddress(HL));
    16
}
