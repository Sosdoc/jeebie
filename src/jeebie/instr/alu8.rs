/// Module for 8 bit arithmetic (ALU instructions)

use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::*;

// 'ADD A,A' 87 4
pub fn ADD_a_a(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, A);
    4
}

// 'ADD A,B' 80 4
pub fn ADD_a_b(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, B);
    4
}

// 'ADD A,C' 81 4
pub fn ADD_a_c(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, C);
    4
}

// 'ADD A,D' 82 4
pub fn ADD_a_d(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, D);
    4
}

// 'ADD A,E' 83 4
pub fn ADD_a_e(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, E);
    4
}

// 'ADD A,H' 84 4
pub fn ADD_a_h(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, H);
    4
}

// 'ADD A,L' 85 4
pub fn ADD_a_l(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, L);
    4
}

// 'ADD A,(HL)' 86 8
pub fn ADD_a_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, RegisterAddress(HL));
    8
}

// 'ADD A,*' C6 8
pub fn ADD_a_n(cpu: &mut CPU) -> i32 {
    cpu.compute_add(A, N);
    8
}

// 'ADC A,A' 8F 4
pub fn ADC_a_a(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, A);
    4
}

// 'ADC A,B' 88 4
pub fn ADC_a_b(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, B);
    4
}

// 'ADC A,C' 89 4
pub fn ADC_a_c(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, C);
    4
}

// 'ADC A,D' 8A 4
pub fn ADC_a_d(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, D);
    4
}

// 'ADC A,E' 8B 4
pub fn ADC_a_e(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, E);
    4
}

// 'ADC A,H' 8C 4
pub fn ADC_a_h(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, H);
    4
}

// 'ADC A,L' 8D 4
pub fn ADC_a_l(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, L);
    4
}

// 'ADC A,(HL)' 8E 8
pub fn ADC_a_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, RegisterAddress(HL));
    8
}

// 'ADC A,*' CE 8
pub fn ADC_a_n(cpu: &mut CPU) -> i32 {
    cpu.compute_adc(A, N);
    8
}

// 'SUB A' 97 4
pub fn SUB_a_A(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(A);
    4
}

// 'SUB B' 90 4
pub fn SUB_a_B(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(B);
    4
}

// 'SUB C' 91 4
pub fn SUB_a_C(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(C);
    4
}

// 'SUB D' 92 4
pub fn SUB_a_D(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(D);
    4
}

// 'SUB E' 93 4
pub fn SUB_a_E(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(E);
    4
}

// 'SUB H' 94 4
pub fn SUB_a_H(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(H);
    4
}

// 'SUB L' 95 4
pub fn SUB_a_L(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(L);
    4
}

// 'SUB (HL)' 96 8
pub fn SUB_a_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(RegisterAddress(HL));
    8
}

// 'SUB *' D6 8
pub fn SUB_a_n(cpu: &mut CPU) -> i32 {
    cpu.compute_sub(N);
    8
}

// 'SBC A,A' 9F 4
pub fn SBC_a_a(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(A);
    4
}

// 'SBC A,B' 98 4
pub fn SBC_a_b(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(B);
    4
}

// 'SBC A,C' 99 4
pub fn SBC_a_c(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(C);
    4
}

// 'SBC A,D' 9A 4
pub fn SBC_a_d(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(D);
    4
}

// 'SBC A,E' 9B 4
pub fn SBC_a_e(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(E);
    4
}

// 'SBC A,H' 9C 4
pub fn SBC_a_h(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(H);
    4
}

// 'SBC A,L' 9D 4
pub fn SBC_a_l(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(L);
    4
}

// 'SBC A,(HL)' 9E 8
pub fn SBC_a_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(RegisterAddress(HL));
    8
}

// 'SBC A,*' DE 8
pub fn SBC_a_n(cpu: &mut CPU) -> i32 {
    cpu.compute_sbc(N);
    0
}

// 'AND A' A7 4
pub fn AND_a(cpu: &mut CPU) -> i32 {
    cpu.compute_and(A);
    4
}

// 'AND B' A0 4
pub fn AND_b(cpu: &mut CPU) -> i32 {
    cpu.compute_and(B);
    4
}

// 'AND C' A1 4
pub fn AND_c(cpu: &mut CPU) -> i32 {
    cpu.compute_and(C);
    4
}

// 'AND D' A2 4
pub fn AND_d(cpu: &mut CPU) -> i32 {
    cpu.compute_and(D);
    4
}

// 'AND E' A3 4
pub fn AND_e(cpu: &mut CPU) -> i32 {
    cpu.compute_and(E);
    4
}

// 'AND H' A4 4
pub fn AND_h(cpu: &mut CPU) -> i32 {
    cpu.compute_and(H);
    4
}

// 'AND L' A5 4
pub fn AND_l(cpu: &mut CPU) -> i32 {
    cpu.compute_and(L);
    4
}

// 'AND (HL)' A6 8
pub fn AND_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_and(RegisterAddress(HL));
    8
}

// 'AND *' E6 8
pub fn AND_n(cpu: &mut CPU) -> i32 {
    cpu.compute_and(N);
    8
}

// 'OR A' B7 4
pub fn OR_a(cpu: &mut CPU) -> i32 {
    cpu.compute_or(A);
    4
}

// 'OR B' B0 4
pub fn OR_b(cpu: &mut CPU) -> i32 {
    cpu.compute_or(B);
    4
}

// 'OR C' B1 4
pub fn OR_c(cpu: &mut CPU) -> i32 {
    cpu.compute_or(C);
    4
}

// 'OR D' B2 4
pub fn OR_d(cpu: &mut CPU) -> i32 {
    cpu.compute_or(D);
    4
}

// 'OR E' B3 4
pub fn OR_e(cpu: &mut CPU) -> i32 {
    cpu.compute_or(E);
    4
}

// 'OR H' B4 4
pub fn OR_h(cpu: &mut CPU) -> i32 {
    cpu.compute_or(H);
    4
}

// 'OR L' B5 4
pub fn OR_l(cpu: &mut CPU) -> i32 {
    cpu.compute_or(L);
    4
}

// 'OR (HL)' B6 8
pub fn OR_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_or(RegisterAddress(HL));
    8
}

// 'OR *' F6 8
pub fn OR_n(cpu: &mut CPU) -> i32 {
    cpu.compute_or(N);
    8
}

// 'XOR A' AF 4
pub fn XOR_a(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(A);
    4
}

// 'XOR B' A8 4
pub fn XOR_b(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(B);
    4
}

// 'XOR C' A9 4
pub fn XOR_c(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(C);
    4
}

// 'XOR D' AA 4
pub fn XOR_d(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(D);
    4
}

// 'XOR E' AB 4
pub fn XOR_e(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(E);
    4
}

// 'XOR H' AC 4
pub fn XOR_h(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(H);
    4
}

// 'XOR L' AD 4
pub fn XOR_l(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(L);
    4
}

// 'XOR (HL)' AE 8
pub fn XOR_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(RegisterAddress(HL));
    8
}

// 'XOR *' EE 8
pub fn XOR_n(cpu: &mut CPU) -> i32 {
    cpu.compute_xor(N);
    8
}

// 'CP A' BF 4
pub fn CP_a(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(A);
    4
}

// 'CP B' B8 4
pub fn CP_b(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(B);
    4
}

// 'CP C' B9 4
pub fn CP_c(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(C);
    4
}

// 'CP D' BA 4
pub fn CP_d(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(D);
    4
}

// 'CP E' BB 4
pub fn CP_e(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(E);
    4
}

// 'CP H' BC 4
pub fn CP_h(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(H);
    4
}

// 'CP L' BD 4
pub fn CP_l(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(L);
    4
}

// 'CP (HL)' BE 8
pub fn CP_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(RegisterAddress(HL));
    8
}

// 'CP *' FE 8
pub fn CP_n(cpu: &mut CPU) -> i32 {
    cpu.compute_cp(N);
    8
}

// 'INC A' 3C 4
pub fn INC_a(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(A);
    4
}

// 'INC B' 04 4
pub fn INC_b(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(B);
    4
}

// 'INC C' 0C 4
pub fn INC_c(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(C);
    4
}

// 'INC D' 14 4
pub fn INC_d(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(D);
    4
}

// 'INC E' 1C 4
pub fn INC_e(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(E);
    4
}

// 'INC H' 24 4
pub fn INC_h(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(H);
    4
}

// 'INC L' 2C 4
pub fn INC_l(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(L);
    4
}

// 'INC (HL)' 34 12
pub fn INC_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_inc(RegisterAddress(HL));
    12
}

// 'DEC A' 3D 4
pub fn DEC_a(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(A);
    4
}

// 'DEC B' 05 4
pub fn DEC_b(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(B);
    4
}

// 'DEC C' 0D 4
pub fn DEC_c(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(C);
    4
}

// 'DEC D' 15 4
pub fn DEC_d(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(D);
    4
}

// 'DEC E' 1D 4
pub fn DEC_e(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(E);
    4
}

// 'DEC H' 25 4
pub fn DEC_h(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(H);
    4
}

// 'DEC L' 2D 4
pub fn DEC_l(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(L);
    4
}

// 'DEC (HL)' 35 12
pub fn DEC_hlm(cpu: &mut CPU) -> i32 {
    cpu.compute_dec(RegisterAddress(HL));
    12
}
