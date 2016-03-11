/// Module for 8 bit arithmetic (ALU instructions)

use jeebie::core::cpu::CPU;
use jeebie::registers::Register8::*;
use jeebie::registers::Register16::*;

// 'ADD A,A' 87 4
pub fn ADD_a_a(cpu: &mut CPU) {
    cpu.compute_add(A, A);
}

// 'ADD A,B' 80 4
pub fn ADD_a_b(cpu: &mut CPU) {
    cpu.compute_add(A, B);
}

// 'ADD A,C' 81 4
pub fn ADD_a_c(cpu: &mut CPU) {
    cpu.compute_add(A, C);
}

// 'ADD A,D' 82 4
pub fn ADD_a_d(cpu: &mut CPU) {
    cpu.compute_add(A, D);
}

// 'ADD A,E' 83 4
pub fn ADD_a_e(cpu: &mut CPU) {
    cpu.compute_add(A, E);
}

// 'ADD A,H' 84 4
pub fn ADD_a_h(cpu: &mut CPU) {
    cpu.compute_add(A, H);
}

// 'ADD A,L' 85 4
pub fn ADD_a_l(cpu: &mut CPU) {
    cpu.compute_add(A, L);
}

// 'ADD A,(HL)' 86 8
pub fn ADD_a_hlm(cpu: &mut CPU) {
    cpu.compute_add(A, RegisterAddress(HL));
}

// 'ADD A,#' C6 8
pub fn ADD_a_n(cpu: &mut CPU) {
    cpu.compute_add(A, Immediate);
}

// 'ADC A,A' 8F 4
pub fn ADC_a_a(cpu: &mut CPU) {
    cpu.compute_adc(A, A);
}

// 'ADC A,B' 88 4
pub fn ADC_a_b(cpu: &mut CPU) {
    cpu.compute_adc(A, B);
}

// 'ADC A,C' 89 4
pub fn ADC_a_c(cpu: &mut CPU) {
    cpu.compute_adc(A, C);
}

// 'ADC A,D' 8A 4
pub fn ADC_a_d(cpu: &mut CPU) {
    cpu.compute_adc(A, D);
}

// 'ADC A,E' 8B 4
pub fn ADC_a_e(cpu: &mut CPU) {
    cpu.compute_adc(A, E);
}

// 'ADC A,H' 8C 4
pub fn ADC_a_h(cpu: &mut CPU) {
    cpu.compute_adc(A, H);
}

// 'ADC A,L' 8D 4
pub fn ADC_a_l(cpu: &mut CPU) {
    cpu.compute_adc(A, L);
}

// 'ADC A,(HL)' 8E 8
pub fn ADC_a_hlm(cpu: &mut CPU) {
    cpu.compute_adc(A, RegisterAddress(HL));
}

// 'ADC A,#' CE 8
pub fn ADC_a_n(cpu: &mut CPU) {
    cpu.compute_adc(A, Immediate);
}

// 'SUB A' 97 4
pub fn SUB_a_A(cpu: &mut CPU) {
    cpu.compute_sub(A);
}

// 'SUB B' 90 4
pub fn SUB_a_B(cpu: &mut CPU) {
    cpu.compute_sub(B);
}

// 'SUB C' 91 4
pub fn SUB_a_C(cpu: &mut CPU) {
    cpu.compute_sub(C);
}

// 'SUB D' 92 4
pub fn SUB_a_D(cpu: &mut CPU) {
    cpu.compute_sub(D);
}

// 'SUB E' 93 4
pub fn SUB_a_E(cpu: &mut CPU) {
    cpu.compute_sub(E);
}

// 'SUB H' 94 4
pub fn SUB_a_H(cpu: &mut CPU) {
    cpu.compute_sub(H);
}

// 'SUB L' 95 4
pub fn SUB_a_L(cpu: &mut CPU) {
    cpu.compute_sub(L);
}

// 'SUB (HL)' 96 8
pub fn SUB_a_hlm(cpu: &mut CPU) {
    cpu.compute_sub(RegisterAddress(HL));
}

// 'SUB #' D6 8
pub fn SUB_a_n(cpu: &mut CPU) {
    cpu.compute_sub(Immediate);
}

// 'SBC A,A' 9F 4
pub fn SBC_a_a(cpu: &mut CPU) {
    cpu.compute_sbc(A);
}

// 'SBC A,B' 98 4
pub fn SBC_a_b(cpu: &mut CPU) {
    cpu.compute_sbc(B);
}

// 'SBC A,C' 99 4
pub fn SBC_a_c(cpu: &mut CPU) {
    cpu.compute_sbc(C);
}

// 'SBC A,D' 9A 4
pub fn SBC_a_d(cpu: &mut CPU) {
    cpu.compute_sbc(D);
}

// 'SBC A,E' 9B 4
pub fn SBC_a_e(cpu: &mut CPU) {
    cpu.compute_sbc(E);
}

// 'SBC A,H' 9C 4
pub fn SBC_a_h(cpu: &mut CPU) {
    cpu.compute_sbc(H);
}

// 'SBC A,L' 9D 4
pub fn SBC_a_l(cpu: &mut CPU) {
    cpu.compute_sbc(L);
}

// 'SBC A,(HL)' 9E 8
pub fn SBC_a_hlm(cpu: &mut CPU) {
    cpu.compute_sbc(RegisterAddress(HL));
}

// 'SBC A,#' ?? ? --- manual has no opcode for this... leave it
pub fn SBC_a_n(cpu: &mut CPU) {
    cpu.compute_sbc(Immediate);
}

// 'AND A' A7 4
pub fn AND_a(cpu: &mut CPU) {
    cpu.compute_and(A);
}

// 'AND B' A0 4
pub fn AND_b(cpu: &mut CPU) {
    cpu.compute_and(B);
}

// 'AND C' A1 4
pub fn AND_c(cpu: &mut CPU) {
    cpu.compute_and(C);
}

// 'AND D' A2 4
pub fn AND_d(cpu: &mut CPU) {
    cpu.compute_and(D);
}

// 'AND E' A3 4
pub fn AND_e(cpu: &mut CPU) {
    cpu.compute_and(E);
}

// 'AND H' A4 4
pub fn AND_h(cpu: &mut CPU) {
    cpu.compute_and(H);
}

// 'AND L' A5 4
pub fn AND_l(cpu: &mut CPU) {
    cpu.compute_and(L);
}

// 'AND (HL)' A6 8
pub fn AND_hlm(cpu: &mut CPU) {
    cpu.compute_and(RegisterAddress(HL));
}

// 'AND #' E6 8
pub fn AND_n(cpu: &mut CPU) {
    cpu.compute_and(Immediate);
}

// 'OR A' B7 4
pub fn OR_a(cpu: &mut CPU) {
    cpu.compute_or(A);
}

// 'OR B' B0 4
pub fn OR_b(cpu: &mut CPU) {
    cpu.compute_or(B);
}

// 'OR C' B1 4
pub fn OR_c(cpu: &mut CPU) {
    cpu.compute_or(C);
}

// 'OR D' B2 4
pub fn OR_d(cpu: &mut CPU) {
    cpu.compute_or(D);
}

// 'OR E' B3 4
pub fn OR_e(cpu: &mut CPU) {
    cpu.compute_or(E);
}

// 'OR H' B4 4
pub fn OR_h(cpu: &mut CPU) {
    cpu.compute_or(H);
}

// 'OR L' B5 4
pub fn OR_l(cpu: &mut CPU) {
    cpu.compute_or(L);
}

// 'OR (HL)' B6 8
pub fn OR_hlm(cpu: &mut CPU) {
    cpu.compute_or(RegisterAddress(HL));
}

// 'OR #' F6 8
pub fn OR_n(cpu: &mut CPU) {
    cpu.compute_or(Immediate);
}

// 'XOR A' AF 4
pub fn XOR_a(cpu: &mut CPU) {
    cpu.compute_xor(A);
}

// 'XOR B' A8 4
pub fn XOR_b(cpu: &mut CPU) {
    cpu.compute_xor(B);
}

// 'XOR C' A9 4
pub fn XOR_c(cpu: &mut CPU) {
    cpu.compute_xor(C);
}

// 'XOR D' AA 4
pub fn XOR_d(cpu: &mut CPU) {
    cpu.compute_xor(D);
}

// 'XOR E' AB 4
pub fn XOR_e(cpu: &mut CPU) {
    cpu.compute_xor(E);
}

// 'XOR H' AC 4
pub fn XOR_h(cpu: &mut CPU) {
    cpu.compute_xor(H);
}

// 'XOR L' AD 4
pub fn XOR_l(cpu: &mut CPU) {
    cpu.compute_xor(L);
}

// 'XOR (HL)' AE 8
pub fn XOR_hlm(cpu: &mut CPU) {
    cpu.compute_xor(RegisterAddress(HL));
}

// 'XOR *' EE 8
pub fn XOR_n(cpu: &mut CPU) {
    cpu.compute_xor(Immediate);
}

// 'CP A' BF 4
pub fn CP_a(cpu: &mut CPU) {
    cpu.compute_cp(A);
}

// 'CP B' B8 4
pub fn CP_b(cpu: &mut CPU) {
    cpu.compute_cp(B);
}

// 'CP C' B9 4
pub fn CP_c(cpu: &mut CPU) {
    cpu.compute_cp(C);
}

// 'CP D' BA 4
pub fn CP_d(cpu: &mut CPU) {
    cpu.compute_cp(D);
}

// 'CP E' BB 4
pub fn CP_e(cpu: &mut CPU) {
    cpu.compute_cp(E);
}

// 'CP H' BC 4
pub fn CP_h(cpu: &mut CPU) {
    cpu.compute_cp(H);
}

// 'CP L' BD 4
pub fn CP_l(cpu: &mut CPU) {
    cpu.compute_cp(L);
}

// 'CP (HL)' BE 8
pub fn CP_hlm(cpu: &mut CPU) {
    cpu.compute_cp(RegisterAddress(HL));
}

// 'CP #' FE 8
pub fn CP_n(cpu: &mut CPU) {
    cpu.compute_cp(Immediate);
}

// 'INC A' 3C 4
pub fn INC_a(cpu: &mut CPU) {
    cpu.compute_inc(A);
}

// 'INC B' 04 4
pub fn INC_b(cpu: &mut CPU) {
    cpu.compute_inc(B);
}

// 'INC C' 0C 4
pub fn INC_c(cpu: &mut CPU) {
    cpu.compute_inc(C);
}

// 'INC D' 14 4
pub fn INC_d(cpu: &mut CPU) {
    cpu.compute_inc(D);
}

// 'INC E' 1C 4
pub fn INC_e(cpu: &mut CPU) {
    cpu.compute_inc(E);
}

// 'INC H' 24 4
pub fn INC_h(cpu: &mut CPU) {
    cpu.compute_inc(H);
}

// 'INC L' 2C 4
pub fn INC_l(cpu: &mut CPU) {
    cpu.compute_inc(L);
}

// 'INC (HL)' 34 12
pub fn INC_hlm(cpu: &mut CPU) {
    cpu.compute_inc(RegisterAddress(HL));
}

// 'DEC A' 3D 4
pub fn DEC_a(cpu: &mut CPU) {
    cpu.compute_dec(A);
}

// 'DEC B' 05 4
pub fn DEC_b(cpu: &mut CPU) {
    cpu.compute_dec(B);
}

// 'DEC C' 0D 4
pub fn DEC_c(cpu: &mut CPU) {
    cpu.compute_dec(C);
}

// 'DEC D' 15 4
pub fn DEC_d(cpu: &mut CPU) {
    cpu.compute_dec(D);
}

// 'DEC E' 1D 4
pub fn DEC_e(cpu: &mut CPU) {
    cpu.compute_dec(E);
}

// 'DEC H' 25 4
pub fn DEC_h(cpu: &mut CPU) {
    cpu.compute_dec(H);
}

// 'DEC L' 2D 4
pub fn DEC_l(cpu: &mut CPU) {
    cpu.compute_dec(L);
}

// 'DEC (HL)' 35 12
pub fn DEC_hlm(cpu: &mut CPU) {
    cpu.compute_dec(RegisterAddress(HL));
}
