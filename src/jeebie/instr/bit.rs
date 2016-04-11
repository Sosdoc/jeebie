/// Module for bit operations (checking, set/reset, etc.
use jeebie::core::cpu::CPU;
use jeebie::registers::Register8::*;
use jeebie::registers::Register16::HL;

//'BIT 0,A' CB 47 8
pub fn BIT_0_A(cpu: &mut CPU) {
    cpu.bit_check(0, A);
}

//'BIT 0,B' CB 40 8
pub fn BIT_0_B(cpu: &mut CPU) {
    cpu.bit_check(0, B);
}

//'BIT 0,C' CB 41 8
pub fn BIT_0_C(cpu: &mut CPU) {
    cpu.bit_check(0, C);
}

//'BIT 0,D' CB 42 8
pub fn BIT_0_D(cpu: &mut CPU) {
    cpu.bit_check(0, D);
}

//'BIT 0,E' CB 43 8
pub fn BIT_0_E(cpu: &mut CPU) {
    cpu.bit_check(0, E);
}

//'BIT 0,H' CB 44 8
pub fn BIT_0_H(cpu: &mut CPU) {
    cpu.bit_check(0, H);
}

//'BIT 0,L' CB 45 8
pub fn BIT_0_L(cpu: &mut CPU) {
    cpu.bit_check(0, L);
}

//'BIT 0,(HL)' CB 46 16
pub fn BIT_0_HLm(cpu: &mut CPU) {
    cpu.bit_check(0, RegisterAddress(HL));
}


//'BIT 1,A' CB 4F 8
pub fn BIT_1_A(cpu: &mut CPU) {
    cpu.bit_check(1, A);
}

//'BIT 1,B' CB 48 8
pub fn BIT_1_B(cpu: &mut CPU) {
    cpu.bit_check(1, B);
}

//'BIT 1,C' CB 49 8
pub fn BIT_1_C(cpu: &mut CPU) {
    cpu.bit_check(1, C);
}

//'BIT 1,D' CB 4A 8
pub fn BIT_1_D(cpu: &mut CPU) {
    cpu.bit_check(1, D);
}

//'BIT 1,E' CB 4B 8
pub fn BIT_1_E(cpu: &mut CPU) {
    cpu.bit_check(1, E);
}

//'BIT 1,H' CB 4C 8
pub fn BIT_1_H(cpu: &mut CPU) {
    cpu.bit_check(1, H);
}

//'BIT 1,L' CB 4D 8
pub fn BIT_1_L(cpu: &mut CPU) {
    cpu.bit_check(1, L);
}

//'BIT 1,(HL)' CB 4E 16
pub fn BIT_1_HLm(cpu: &mut CPU) {
    cpu.bit_check(1, RegisterAddress(HL));
}


//'BIT 2,A' CB 57 8
pub fn BIT_2_A(cpu: &mut CPU) {
    cpu.bit_check(2, A);
}

//'BIT 2,B' CB 50 8
pub fn BIT_2_B(cpu: &mut CPU) {
    cpu.bit_check(2, B);
}

//'BIT 2,C' CB 51 8
pub fn BIT_2_C(cpu: &mut CPU) {
    cpu.bit_check(2, C);
}

//'BIT 2,D' CB 52 8
pub fn BIT_2_D(cpu: &mut CPU) {
    cpu.bit_check(2, D);
}

//'BIT 2,E' CB 53 8
pub fn BIT_2_E(cpu: &mut CPU) {
    cpu.bit_check(2, E);
}

//'BIT 2,H' CB 54 8
pub fn BIT_2_H(cpu: &mut CPU) {
    cpu.bit_check(2, H);
}

//'BIT 2,L' CB 55 8
pub fn BIT_2_L(cpu: &mut CPU) {
    cpu.bit_check(2, L);
}

//'BIT 2,(HL)' CB 56 16
pub fn BIT_2_HLm(cpu: &mut CPU) {
    cpu.bit_check(2, RegisterAddress(HL));
}


//'BIT 3,A' CB 5F 8
pub fn BIT_3_A(cpu: &mut CPU) {
    cpu.bit_check(3, A);
}

//'BIT 3,B' CB 58 8
pub fn BIT_3_B(cpu: &mut CPU) {
    cpu.bit_check(3, B);
}

//'BIT 3,C' CB 59 8
pub fn BIT_3_C(cpu: &mut CPU) {
    cpu.bit_check(3, C);
}

//'BIT 3,D' CB 5A 8
pub fn BIT_3_D(cpu: &mut CPU) {
    cpu.bit_check(3, D);
}

//'BIT 3,E' CB 5B 8
pub fn BIT_3_E(cpu: &mut CPU) {
    cpu.bit_check(3, E);
}

//'BIT 3,H' CB 5C 8
pub fn BIT_3_H(cpu: &mut CPU) {
    cpu.bit_check(3, H);
}

//'BIT 3,L' CB 5D 8
pub fn BIT_3_L(cpu: &mut CPU) {
    cpu.bit_check(3, L);
}

//'BIT 3,(HL)' CB 5E 16
pub fn BIT_3_HLm(cpu: &mut CPU) {
    cpu.bit_check(3, RegisterAddress(HL));
}


//'BIT 4,A' CB 67 8
pub fn BIT_4_A(cpu: &mut CPU) {
    cpu.bit_check(4, A);
}

//'BIT 4,B' CB 60 8
pub fn BIT_4_B(cpu: &mut CPU) {
    cpu.bit_check(4, B);
}

//'BIT 4,C' CB 61 8
pub fn BIT_4_C(cpu: &mut CPU) {
    cpu.bit_check(4, C);
}

//'BIT 4,D' CB 62 8
pub fn BIT_4_D(cpu: &mut CPU) {
    cpu.bit_check(4, D);
}

//'BIT 4,E' CB 63 8
pub fn BIT_4_E(cpu: &mut CPU) {
    cpu.bit_check(4, E);
}

//'BIT 4,H' CB 64 8
pub fn BIT_4_H(cpu: &mut CPU) {
    cpu.bit_check(4, H);
}

//'BIT 4,L' CB 65 8
pub fn BIT_4_L(cpu: &mut CPU) {
    cpu.bit_check(4, L);
}

//'BIT 4,(HL)' CB 66 16
pub fn BIT_4_HLm(cpu: &mut CPU) {
    cpu.bit_check(4, RegisterAddress(HL));
}


//'BIT 5,A' CB 6F 8
pub fn BIT_5_A(cpu: &mut CPU) {
    cpu.bit_check(5, A);
}

//'BIT 5,B' CB 68 8
pub fn BIT_5_B(cpu: &mut CPU) {
    cpu.bit_check(5, B);
}

//'BIT 5,C' CB 69 8
pub fn BIT_5_C(cpu: &mut CPU) {
    cpu.bit_check(5, C);
}

//'BIT 5,D' CB 6A 8
pub fn BIT_5_D(cpu: &mut CPU) {
    cpu.bit_check(5, D);
}

//'BIT 5,E' CB 6B 8
pub fn BIT_5_E(cpu: &mut CPU) {
    cpu.bit_check(5, E);
}

//'BIT 5,H' CB 6C 8
pub fn BIT_5_H(cpu: &mut CPU) {
    cpu.bit_check(5, H);
}

//'BIT 5,L' CB 6D 8
pub fn BIT_5_L(cpu: &mut CPU) {
    cpu.bit_check(5, L);
}

//'BIT 5,(HL)' CB 6E 16
pub fn BIT_5_HLm(cpu: &mut CPU) {
    cpu.bit_check(5, RegisterAddress(HL));
}


//'BIT 6,A' CB 77 8
pub fn BIT_6_A(cpu: &mut CPU) {
    cpu.bit_check(6, A);
}

//'BIT 6,B' CB 70 8
pub fn BIT_6_B(cpu: &mut CPU) {
    cpu.bit_check(6, B);
}

//'BIT 6,C' CB 71 8
pub fn BIT_6_C(cpu: &mut CPU) {
    cpu.bit_check(6, C);
}

//'BIT 6,D' CB 72 8
pub fn BIT_6_D(cpu: &mut CPU) {
    cpu.bit_check(6, D);
}

//'BIT 6,E' CB 73 8
pub fn BIT_6_E(cpu: &mut CPU) {
    cpu.bit_check(6, E);
}

//'BIT 6,H' CB 74 8
pub fn BIT_6_H(cpu: &mut CPU) {
    cpu.bit_check(6, H);
}

//'BIT 6,L' CB 75 8
pub fn BIT_6_L(cpu: &mut CPU) {
    cpu.bit_check(6, L);
}

//'BIT 6,(HL)' CB 76 16
pub fn BIT_6_HLm(cpu: &mut CPU) {
    cpu.bit_check(6, RegisterAddress(HL));
}


//'BIT 7,A' CB 7F 8
pub fn BIT_7_A(cpu: &mut CPU) {
    cpu.bit_check(7, A);
}

//'BIT 7,B' CB 78 8
pub fn BIT_7_B(cpu: &mut CPU) {
    cpu.bit_check(7, B);
}

//'BIT 7,C' CB 79 8
pub fn BIT_7_C(cpu: &mut CPU) {
    cpu.bit_check(7, C);
}

//'BIT 7,D' CB 7A 8
pub fn BIT_7_D(cpu: &mut CPU) {
    cpu.bit_check(7, D);
}

//'BIT 7,E' CB 7B 8
pub fn BIT_7_E(cpu: &mut CPU) {
    cpu.bit_check(7, E);
}

//'BIT 7,H' CB 7C 8
pub fn BIT_7_H(cpu: &mut CPU) {
    cpu.bit_check(7, H);
}

//'BIT 7,L' CB 7D 8
pub fn BIT_7_L(cpu: &mut CPU) {
    cpu.bit_check(7, L);
}

//'BIT 7,(HL)' CB 7E 16
pub fn BIT_7_HLm(cpu: &mut CPU) {
    cpu.bit_check(7, RegisterAddress(HL));
}

