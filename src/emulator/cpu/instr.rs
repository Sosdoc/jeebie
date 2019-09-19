use super::registers::Flags::*;
use super::registers::Register16::*;
use super::registers::Register8::*;
use super::CPU;
use super::MMU;

// 'ADD A,A' 87 4
pub fn ADD_a_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, A);
    4
}

// 'ADD A,B' 80 4
pub fn ADD_a_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, B);
    4
}

// 'ADD A,C' 81 4
pub fn ADD_a_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, C);
    4
}

// 'ADD A,D' 82 4
pub fn ADD_a_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, D);
    4
}

// 'ADD A,E' 83 4
pub fn ADD_a_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, E);
    4
}

// 'ADD A,H' 84 4
pub fn ADD_a_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, H);
    4
}

// 'ADD A,L' 85 4
pub fn ADD_a_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, L);
    4
}

// 'ADD A,(HL)' 86 8
pub fn ADD_a_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, RegisterAddress(HL));
    8
}

// 'ADD A,*' C6 8
pub fn ADD_a_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add(mem, A, N);
    8
}

// 'ADC A,A' 8F 4
pub fn ADC_a_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, A);
    4
}

// 'ADC A,B' 88 4
pub fn ADC_a_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, B);
    4
}

// 'ADC A,C' 89 4
pub fn ADC_a_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, C);
    4
}

// 'ADC A,D' 8A 4
pub fn ADC_a_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, D);
    4
}

// 'ADC A,E' 8B 4
pub fn ADC_a_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, E);
    4
}

// 'ADC A,H' 8C 4
pub fn ADC_a_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, H);
    4
}

// 'ADC A,L' 8D 4
pub fn ADC_a_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, L);
    4
}

// 'ADC A,(HL)' 8E 8
pub fn ADC_a_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, RegisterAddress(HL));
    8
}

// 'ADC A,*' CE 8
pub fn ADC_a_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_adc(mem, A, N);
    8
}

// 'SUB A' 97 4
pub fn SUB_a_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, A);
    4
}

// 'SUB B' 90 4
pub fn SUB_a_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, B);
    4
}

// 'SUB C' 91 4
pub fn SUB_a_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, C);
    4
}

// 'SUB D' 92 4
pub fn SUB_a_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, D);
    4
}

// 'SUB E' 93 4
pub fn SUB_a_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, E);
    4
}

// 'SUB H' 94 4
pub fn SUB_a_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, H);
    4
}

// 'SUB L' 95 4
pub fn SUB_a_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, L);
    4
}

// 'SUB (HL)' 96 8
pub fn SUB_a_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, RegisterAddress(HL));
    8
}

// 'SUB *' D6 8
pub fn SUB_a_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sub(mem, N);
    8
}

// 'SBC A,A' 9F 4
pub fn SBC_a_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, A);
    4
}

// 'SBC A,B' 98 4
pub fn SBC_a_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, B);
    4
}

// 'SBC A,C' 99 4
pub fn SBC_a_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, C);
    4
}

// 'SBC A,D' 9A 4
pub fn SBC_a_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, D);
    4
}

// 'SBC A,E' 9B 4
pub fn SBC_a_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, E);
    4
}

// 'SBC A,H' 9C 4
pub fn SBC_a_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, H);
    4
}

// 'SBC A,L' 9D 4
pub fn SBC_a_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, L);
    4
}

// 'SBC A,(HL)' 9E 8
pub fn SBC_a_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, RegisterAddress(HL));
    8
}

// 'SBC A,*' DE 8
pub fn SBC_a_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_sbc(mem, N);
    0
}

// 'AND A' A7 4
pub fn AND_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, A);
    4
}

// 'AND B' A0 4
pub fn AND_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, B);
    4
}

// 'AND C' A1 4
pub fn AND_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, C);
    4
}

// 'AND D' A2 4
pub fn AND_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, D);
    4
}

// 'AND E' A3 4
pub fn AND_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, E);
    4
}

// 'AND H' A4 4
pub fn AND_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, H);
    4
}

// 'AND L' A5 4
pub fn AND_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, L);
    4
}

// 'AND (HL)' A6 8
pub fn AND_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, RegisterAddress(HL));
    8
}

// 'AND *' E6 8
pub fn AND_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_and(mem, N);
    8
}

// 'OR A' B7 4
pub fn OR_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, A);
    4
}

// 'OR B' B0 4
pub fn OR_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, B);
    4
}

// 'OR C' B1 4
pub fn OR_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, C);
    4
}

// 'OR D' B2 4
pub fn OR_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, D);
    4
}

// 'OR E' B3 4
pub fn OR_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, E);
    4
}

// 'OR H' B4 4
pub fn OR_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, H);
    4
}

// 'OR L' B5 4
pub fn OR_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, L);
    4
}

// 'OR (HL)' B6 8
pub fn OR_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, RegisterAddress(HL));
    8
}

// 'OR *' F6 8
pub fn OR_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_or(mem, N);
    8
}

// 'XOR A' AF 4
pub fn XOR_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, A);
    4
}

// 'XOR B' A8 4
pub fn XOR_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, B);
    4
}

// 'XOR C' A9 4
pub fn XOR_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, C);
    4
}

// 'XOR D' AA 4
pub fn XOR_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, D);
    4
}

// 'XOR E' AB 4
pub fn XOR_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, E);
    4
}

// 'XOR H' AC 4
pub fn XOR_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, H);
    4
}

// 'XOR L' AD 4
pub fn XOR_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, L);
    4
}

// 'XOR (HL)' AE 8
pub fn XOR_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, RegisterAddress(HL));
    8
}

// 'XOR *' EE 8
pub fn XOR_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_xor(mem, N);
    8
}

// 'CP A' BF 4
pub fn CP_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, A);
    4
}

// 'CP B' B8 4
pub fn CP_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, B);
    4
}

// 'CP C' B9 4
pub fn CP_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, C);
    4
}

// 'CP D' BA 4
pub fn CP_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, D);
    4
}

// 'CP E' BB 4
pub fn CP_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, E);
    4
}

// 'CP H' BC 4
pub fn CP_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, H);
    4
}

// 'CP L' BD 4
pub fn CP_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, L);
    4
}

// 'CP (HL)' BE 8
pub fn CP_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, RegisterAddress(HL));
    8
}

// 'CP *' FE 8
pub fn CP_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_cp(mem, N);
    8
}

// 'INC A' 3C 4
pub fn INC_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, A);
    4
}

// 'INC B' 04 4
pub fn INC_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, B);
    4
}

// 'INC C' 0C 4
pub fn INC_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, C);
    4
}

// 'INC D' 14 4
pub fn INC_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, D);
    4
}

// 'INC E' 1C 4
pub fn INC_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, E);
    4
}

// 'INC H' 24 4
pub fn INC_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, H);
    4
}

// 'INC L' 2C 4
pub fn INC_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, L);
    4
}

// 'INC (HL)' 34 12
pub fn INC_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc(mem, RegisterAddress(HL));
    12
}

// 'DEC A' 3D 4
pub fn DEC_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, A);
    4
}

// 'DEC B' 05 4
pub fn DEC_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, B);
    4
}

// 'DEC C' 0D 4
pub fn DEC_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, C);
    4
}

// 'DEC D' 15 4
pub fn DEC_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, D);
    4
}

// 'DEC E' 1D 4
pub fn DEC_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, E);
    4
}

// 'DEC H' 25 4
pub fn DEC_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, H);
    4
}

// 'DEC L' 2D 4
pub fn DEC_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, L);
    4
}

// 'DEC (HL)' 35 12
pub fn DEC_hlm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec(mem, RegisterAddress(HL));
    12
}

// 'ADD HL,BC' 09 8
pub fn ADD_hl_bc(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add16(mem, HL, BC);
    8
}

// 'ADD HL,DE' 19 8
pub fn ADD_hl_de(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add16(mem, HL, DE);
    8
}

// 'ADD HL,HL' 29 8
pub fn ADD_hl_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add16(mem, HL, HL);
    8
}

// 'ADD HL,SP' 39 8
pub fn ADD_hl_sp(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_add16(mem, HL, SP);
    8
}

// 'ADD SP,*' E8 16
pub fn ADD_sp_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let value = Value16(cpu.get_immediate8(mem) as u16);
    cpu.compute_add16(mem, SP, value);
    16
}

// 'INC BC' 03 8
pub fn INC_bc(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc16(mem, BC);
    8
}

// 'INC DE' 13 8
pub fn INC_de(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc16(mem, DE);
    8
}

// 'INC HL' 23 8
pub fn INC_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc16(mem, HL);
    8
}

// 'INC SP' 33 8
pub fn INC_sp(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_inc16(mem, SP);
    8
}

// 'DEC BC' 0B 8
pub fn DEC_bc(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec16(mem, BC);
    8
}

// 'DEC DE' 1B 8
pub fn DEC_de(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec16(mem, DE);
    8
}

// 'DEC HL' 2B 8
pub fn DEC_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec16(mem, HL);
    8
}

// 'DEC SP' 3B 8
pub fn DEC_sp(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_dec16(mem, SP);
    8
}

// 'BIT 0,A' CB 47 8
pub fn BIT_0_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, A);
    8
}

// 'BIT 0,B' CB 40 8
pub fn BIT_0_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, B);
    8
}

// 'BIT 0,C' CB 41 8
pub fn BIT_0_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, C);
    8
}

// 'BIT 0,D' CB 42 8
pub fn BIT_0_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, D);
    8
}

// 'BIT 0,E' CB 43 8
pub fn BIT_0_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, E);
    8
}

// 'BIT 0,H' CB 44 8
pub fn BIT_0_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, H);
    8
}

// 'BIT 0,L' CB 45 8
pub fn BIT_0_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, L);
    8
}

// 'BIT 0,(HL)' CB 46 16
pub fn BIT_0_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 0, RegisterAddress(HL));
    16
}

// 'BIT 1,A' CB 4F 8
pub fn BIT_1_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, A);
    8
}

// 'BIT 1,B' CB 48 8
pub fn BIT_1_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, B);
    8
}

// 'BIT 1,C' CB 49 8
pub fn BIT_1_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, C);
    8
}

// 'BIT 1,D' CB 4A 8
pub fn BIT_1_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, D);
    8
}

// 'BIT 1,E' CB 4B 8
pub fn BIT_1_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, E);
    8
}

// 'BIT 1,H' CB 4C 8
pub fn BIT_1_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, H);
    8
}

// 'BIT 1,L' CB 4D 8
pub fn BIT_1_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, L);
    8
}

// 'BIT 1,(HL)' CB 4E 16
pub fn BIT_1_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 1, RegisterAddress(HL));
    16
}

// 'BIT 2,A' CB 57 8
pub fn BIT_2_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, A);
    8
}

// 'BIT 2,B' CB 50 8
pub fn BIT_2_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, B);
    8
}

// 'BIT 2,C' CB 51 8
pub fn BIT_2_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, C);
    8
}

// 'BIT 2,D' CB 52 8
pub fn BIT_2_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, D);
    8
}

// 'BIT 2,E' CB 53 8
pub fn BIT_2_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, E);
    8
}

// 'BIT 2,H' CB 54 8
pub fn BIT_2_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, H);
    8
}

// 'BIT 2,L' CB 55 8
pub fn BIT_2_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, L);
    8
}

// 'BIT 2,(HL)' CB 56 16
pub fn BIT_2_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 2, RegisterAddress(HL));
    16
}

// 'BIT 3,A' CB 5F 8
pub fn BIT_3_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, A);
    8
}

// 'BIT 3,B' CB 58 8
pub fn BIT_3_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, B);
    8
}

// 'BIT 3,C' CB 59 8
pub fn BIT_3_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, C);
    8
}

// 'BIT 3,D' CB 5A 8
pub fn BIT_3_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, D);
    8
}

// 'BIT 3,E' CB 5B 8
pub fn BIT_3_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, E);
    8
}

// 'BIT 3,H' CB 5C 8
pub fn BIT_3_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, H);
    8
}

// 'BIT 3,L' CB 5D 8
pub fn BIT_3_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, L);
    8
}

// 'BIT 3,(HL)' CB 5E 16
pub fn BIT_3_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 3, RegisterAddress(HL));
    16
}

// 'BIT 4,A' CB 67 8
pub fn BIT_4_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, A);
    8
}

// 'BIT 4,B' CB 60 8
pub fn BIT_4_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, B);
    8
}

// 'BIT 4,C' CB 61 8
pub fn BIT_4_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, C);
    8
}

// 'BIT 4,D' CB 62 8
pub fn BIT_4_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, D);
    8
}

// 'BIT 4,E' CB 63 8
pub fn BIT_4_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, E);
    8
}

// 'BIT 4,H' CB 64 8
pub fn BIT_4_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, H);
    8
}

// 'BIT 4,L' CB 65 8
pub fn BIT_4_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, L);
    8
}

// 'BIT 4,(HL)' CB 66 16
pub fn BIT_4_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 4, RegisterAddress(HL));
    16
}

// 'BIT 5,A' CB 6F 8
pub fn BIT_5_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, A);
    8
}

// 'BIT 5,B' CB 68 8
pub fn BIT_5_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, B);
    8
}

// 'BIT 5,C' CB 69 8
pub fn BIT_5_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, C);
    8
}

// 'BIT 5,D' CB 6A 8
pub fn BIT_5_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, D);
    8
}

// 'BIT 5,E' CB 6B 8
pub fn BIT_5_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, E);
    8
}

// 'BIT 5,H' CB 6C 8
pub fn BIT_5_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, H);
    8
}

// 'BIT 5,L' CB 6D 8
pub fn BIT_5_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, L);
    8
}

// 'BIT 5,(HL)' CB 6E 16
pub fn BIT_5_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 5, RegisterAddress(HL));
    16
}

// 'BIT 6,A' CB 77 8
pub fn BIT_6_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, A);
    8
}

// 'BIT 6,B' CB 70 8
pub fn BIT_6_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, B);
    8
}

// 'BIT 6,C' CB 71 8
pub fn BIT_6_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, C);
    8
}

// 'BIT 6,D' CB 72 8
pub fn BIT_6_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, D);
    8
}

// 'BIT 6,E' CB 73 8
pub fn BIT_6_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, E);
    8
}

// 'BIT 6,H' CB 74 8
pub fn BIT_6_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, H);
    8
}

// 'BIT 6,L' CB 75 8
pub fn BIT_6_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, L);
    8
}

// 'BIT 6,(HL)' CB 76 16
pub fn BIT_6_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 6, RegisterAddress(HL));
    16
}

// 'BIT 7,A' CB 7F 8
pub fn BIT_7_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, A);
    8
}

// 'BIT 7,B' CB 78 8
pub fn BIT_7_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, B);
    8
}

// 'BIT 7,C' CB 79 8
pub fn BIT_7_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, C);
    8
}

// 'BIT 7,D' CB 7A 8
pub fn BIT_7_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, D);
    8
}

// 'BIT 7,E' CB 7B 8
pub fn BIT_7_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, E);
    8
}

// 'BIT 7,H' CB 7C 8
pub fn BIT_7_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, H);
    8
}

// 'BIT 7,L' CB 7D 8
pub fn BIT_7_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, L);
    8
}

// 'BIT 7,(HL)' CB 7E 16
pub fn BIT_7_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_check(mem, 7, RegisterAddress(HL));
    16
}

// 'RES 0,A' CB 87 8
pub fn RES_0_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, A);
    8
}

// 'RES 0,B' CB 80 8
pub fn RES_0_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, B);
    8
}

// 'RES 0,C' CB 81 8
pub fn RES_0_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, C);
    8
}

// 'RES 0,D' CB 82 8
pub fn RES_0_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, D);
    8
}

// 'RES 0,E' CB 83 8
pub fn RES_0_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, E);
    8
}

// 'RES 0,H' CB 84 8
pub fn RES_0_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, H);
    8
}

// 'RES 0,L' CB 85 8
pub fn RES_0_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, L);
    8
}

// 'RES 0,(HL)' CB 86 16
pub fn RES_0_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 0, RegisterAddress(HL));
    16
}

// 'RES 1,A' CB 8F 8
pub fn RES_1_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, A);
    8
}

// 'RES 1,B' CB 88 8
pub fn RES_1_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, B);
    8
}

// 'RES 1,C' CB 89 8
pub fn RES_1_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, C);
    8
}

// 'RES 1,D' CB 8A 8
pub fn RES_1_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, D);
    8
}

// 'RES 1,E' CB 8B 8
pub fn RES_1_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, E);
    8
}

// 'RES 1,H' CB 8C 8
pub fn RES_1_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, H);
    8
}

// 'RES 1,L' CB 8D 8
pub fn RES_1_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, L);
    8
}

// 'RES 1,(HL)' CB 8E 16
pub fn RES_1_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 1, RegisterAddress(HL));
    16
}

// 'RES 2,A' CB 97 8
pub fn RES_2_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, A);
    8
}

// 'RES 2,B' CB 90 8
pub fn RES_2_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, B);
    8
}

// 'RES 2,C' CB 91 8
pub fn RES_2_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, C);
    8
}

// 'RES 2,D' CB 92 8
pub fn RES_2_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, D);
    8
}

// 'RES 2,E' CB 93 8
pub fn RES_2_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, E);
    8
}

// 'RES 2,H' CB 94 8
pub fn RES_2_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, H);
    8
}

// 'RES 2,L' CB 95 8
pub fn RES_2_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, L);
    8
}

// 'RES 2,(HL)' CB 96 16
pub fn RES_2_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 2, RegisterAddress(HL));
    16
}

// 'RES 3,A' CB 9F 8
pub fn RES_3_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, A);
    8
}

// 'RES 3,B' CB 98 8
pub fn RES_3_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, B);
    8
}

// 'RES 3,C' CB 99 8
pub fn RES_3_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, C);
    8
}

// 'RES 3,D' CB 9A 8
pub fn RES_3_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, D);
    8
}

// 'RES 3,E' CB 9B 8
pub fn RES_3_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, E);
    8
}

// 'RES 3,H' CB 9C 8
pub fn RES_3_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, H);
    8
}

// 'RES 3,L' CB 9D 8
pub fn RES_3_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, L);
    8
}

// 'RES 3,(HL)' CB 9E 16
pub fn RES_3_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 3, RegisterAddress(HL));
    16
}

// 'RES 4,A' CB A7 8
pub fn RES_4_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, A);
    8
}

// 'RES 4,B' CB A0 8
pub fn RES_4_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, B);
    8
}

// 'RES 4,C' CB A1 8
pub fn RES_4_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, C);
    8
}

// 'RES 4,D' CB A2 8
pub fn RES_4_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, D);
    8
}

// 'RES 4,E' CB A3 8
pub fn RES_4_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, E);
    8
}

// 'RES 4,H' CB A4 8
pub fn RES_4_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, H);
    8
}

// 'RES 4,L' CB A5 8
pub fn RES_4_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, L);
    8
}

// 'RES 4,(HL)' CB A6 16
pub fn RES_4_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 4, RegisterAddress(HL));
    16
}

// 'RES 5,A' CB AF 8
pub fn RES_5_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, A);
    8
}

// 'RES 5,B' CB A8 8
pub fn RES_5_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, B);
    8
}

// 'RES 5,C' CB A9 8
pub fn RES_5_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, C);
    8
}

// 'RES 5,D' CB AA 8
pub fn RES_5_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, D);
    8
}

// 'RES 5,E' CB AB 8
pub fn RES_5_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, E);
    8
}

// 'RES 5,H' CB AC 8
pub fn RES_5_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, H);
    8
}

// 'RES 5,L' CB AD 8
pub fn RES_5_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, L);
    8
}

// 'RES 5,(HL)' CB AE 16
pub fn RES_5_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 5, RegisterAddress(HL));
    16
}

// 'RES 6,A' CB B7 8
pub fn RES_6_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, A);
    8
}

// 'RES 6,B' CB B0 8
pub fn RES_6_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, B);
    8
}

// 'RES 6,C' CB B1 8
pub fn RES_6_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, C);
    8
}

// 'RES 6,D' CB B2 8
pub fn RES_6_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, D);
    8
}

// 'RES 6,E' CB B3 8
pub fn RES_6_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, E);
    8
}

// 'RES 6,H' CB B4 8
pub fn RES_6_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, H);
    8
}

// 'RES 6,L' CB B5 8
pub fn RES_6_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, L);
    8
}

// 'RES 6,(HL)' CB B6 16
pub fn RES_6_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 6, RegisterAddress(HL));
    16
}

// 'RES 7,A' CB BF 8
pub fn RES_7_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, A);
    8
}

// 'RES 7,B' CB B8 8
pub fn RES_7_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, B);
    8
}

// 'RES 7,C' CB B9 8
pub fn RES_7_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, C);
    8
}

// 'RES 7,D' CB BA 8
pub fn RES_7_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, D);
    8
}

// 'RES 7,E' CB BB 8
pub fn RES_7_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, E);
    8
}

// 'RES 7,H' CB BC 8
pub fn RES_7_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, H);
    8
}

// 'RES 7,L' CB BD 8
pub fn RES_7_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, L);
    8
}

// 'RES 7,(HL)' CB BE 16
pub fn RES_7_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_reset(mem, 7, RegisterAddress(HL));
    16
}

// 'SET 0,A' CB C7 8
pub fn SET_0_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, A);
    8
}

// 'SET 0,B' CB C0 8
pub fn SET_0_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, B);
    8
}

// 'SET 0,C' CB C1 8
pub fn SET_0_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, C);
    8
}

// 'SET 0,D' CB C2 8
pub fn SET_0_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, D);
    8
}

// 'SET 0,E' CB C3 8
pub fn SET_0_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, E);
    8
}

// 'SET 0,H' CB C4 8
pub fn SET_0_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, H);
    8
}

// 'SET 0,L' CB C5 8
pub fn SET_0_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, L);
    8
}

// 'SET 0,(HL)' CB C6 16
pub fn SET_0_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 0, RegisterAddress(HL));
    16
}

// 'SET 1,A' CB CF 8
pub fn SET_1_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, A);
    8
}

// 'SET 1,B' CB C8 8
pub fn SET_1_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, B);
    8
}

// 'SET 1,C' CB C9 8
pub fn SET_1_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, C);
    8
}

// 'SET 1,D' CB CA 8
pub fn SET_1_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, D);
    8
}

// 'SET 1,E' CB CB 8
pub fn SET_1_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, E);
    8
}

// 'SET 1,H' CB CC 8
pub fn SET_1_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, H);
    8
}

// 'SET 1,L' CB CD 8
pub fn SET_1_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, L);
    8
}

// 'SET 1,(HL)' CB CE 16
pub fn SET_1_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 1, RegisterAddress(HL));
    16
}

// 'SET 2,A' CB D7 8
pub fn SET_2_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, A);
    8
}

// 'SET 2,B' CB D0 8
pub fn SET_2_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, B);
    8
}

// 'SET 2,C' CB D1 8
pub fn SET_2_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, C);
    8
}

// 'SET 2,D' CB D2 8
pub fn SET_2_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, D);
    8
}

// 'SET 2,E' CB D3 8
pub fn SET_2_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, E);
    8
}

// 'SET 2,H' CB D4 8
pub fn SET_2_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, H);
    8
}

// 'SET 2,L' CB D5 8
pub fn SET_2_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, L);
    8
}

// 'SET 2,(HL)' CB D6 16
pub fn SET_2_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 2, RegisterAddress(HL));
    16
}

// 'SET 3,A' CB DF 8
pub fn SET_3_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, A);
    8
}

// 'SET 3,B' CB D8 8
pub fn SET_3_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, B);
    8
}

// 'SET 3,C' CB D9 8
pub fn SET_3_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, C);
    8
}

// 'SET 3,D' CB DA 8
pub fn SET_3_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, D);
    8
}

// 'SET 3,E' CB DB 8
pub fn SET_3_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, E);
    8
}

// 'SET 3,H' CB DC 8
pub fn SET_3_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, H);
    8
}

// 'SET 3,L' CB DD 8
pub fn SET_3_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, L);
    8
}

// 'SET 3,(HL)' CB DE 16
pub fn SET_3_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 3, RegisterAddress(HL));
    16
}

// 'SET 4,A' CB E7 8
pub fn SET_4_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, A);
    8
}

// 'SET 4,B' CB E0 8
pub fn SET_4_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, B);
    8
}

// 'SET 4,C' CB E1 8
pub fn SET_4_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, C);
    8
}

// 'SET 4,D' CB E2 8
pub fn SET_4_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, D);
    8
}

// 'SET 4,E' CB E3 8
pub fn SET_4_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, E);
    8
}

// 'SET 4,H' CB E4 8
pub fn SET_4_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, H);
    8
}

// 'SET 4,L' CB E5 8
pub fn SET_4_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, L);
    8
}

// 'SET 4,(HL)' CB E6 16
pub fn SET_4_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 4, RegisterAddress(HL));
    16
}

// 'SET 5,A' CB EF 8
pub fn SET_5_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, A);
    8
}

// 'SET 5,B' CB E8 8
pub fn SET_5_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, B);
    8
}

// 'SET 5,C' CB E9 8
pub fn SET_5_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, C);
    8
}

// 'SET 5,D' CB EA 8
pub fn SET_5_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, D);
    8
}

// 'SET 5,E' CB EB 8
pub fn SET_5_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, E);
    8
}

// 'SET 5,H' CB EC 8
pub fn SET_5_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, H);
    8
}

// 'SET 5,L' CB ED 8
pub fn SET_5_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, L);
    8
}

// 'SET 5,(HL)' CB EE 16
pub fn SET_5_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 5, RegisterAddress(HL));
    16
}

// 'SET 6,A' CB F7 8
pub fn SET_6_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, A);
    8
}

// 'SET 6,B' CB F0 8
pub fn SET_6_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, B);
    8
}

// 'SET 6,C' CB F1 8
pub fn SET_6_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, C);
    8
}

// 'SET 6,D' CB F2 8
pub fn SET_6_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, D);
    8
}

// 'SET 6,E' CB F3 8
pub fn SET_6_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, E);
    8
}

// 'SET 6,H' CB F4 8
pub fn SET_6_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, H);
    8
}

// 'SET 6,L' CB F5 8
pub fn SET_6_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, L);
    8
}

// 'SET 6,(HL)' CB F6 16
pub fn SET_6_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 6, RegisterAddress(HL));
    16
}

// 'SET 7,A' CB FF 8
pub fn SET_7_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, A);
    8
}

// 'SET 7,B' CB F8 8
pub fn SET_7_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, B);
    8
}

// 'SET 7,C' CB F9 8
pub fn SET_7_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, C);
    8
}

// 'SET 7,D' CB FA 8
pub fn SET_7_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, D);
    8
}

// 'SET 7,E' CB FB 8
pub fn SET_7_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, E);
    8
}

// 'SET 7,H' CB FC 8
pub fn SET_7_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, H);
    8
}

// 'SET 7,L' CB FD 8
pub fn SET_7_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, L);
    8
}

// 'SET 7,(HL)' CB FE 16
pub fn SET_7_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.bit_set(mem, 7, RegisterAddress(HL));
    16
}

// TODO: make appropriate timings for jumps when a branch is taken or not.

// 'JP nn' C3 12
pub fn JP_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr = cpu.get_immediate16(mem);
    cpu.jump(addr);
    12
}

// 'JP NZ,nn' C2 12
pub fn JP_NZ_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.jump_not_flag(mem, Zero, NN);
    12
}

// 'JP Z,nn' CA 12
pub fn JP_Z_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.jump_flag(mem, Zero, NN);
    12
}

// 'JP NC,nn' D2 12
pub fn JP_NC_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.jump_not_flag(mem, Carry, NN);
    12
}

// 'JP C,nn' DA 12
pub fn JP_C_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.jump_flag(mem, Zero, NN);
    12
}

// 'JP (HL)' E9 4
pub fn JP_HL(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr = cpu.get16(mem, HL);
    cpu.jump(addr);
    4
}

// 'JR n' 18 8
pub fn JR_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let pc = cpu.reg.pc + 1; // it isn't updated yet
    let n = mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16); // n is loaded as a signed bit
    cpu.jump(addr);
    8
}

// 'JR NZ,*' 20 8
pub fn JR_NZ_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_not_flag(mem, Zero, Value16(addr));
    8
}

// 'JR Z,*' 28 8
pub fn JR_Z_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_flag(mem, Zero, Value16(addr));
    8
}

// 'JR NC,*' 30 8
pub fn JR_NC_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_not_flag(mem, Carry, Value16(addr));
    8
}

// 'JR C,*' 38 8
pub fn JR_C_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let pc = cpu.reg.pc + 1;
    let n = mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    cpu.jump_flag(mem, Carry, Value16(addr));
    8
}

// 'CALL nn' CD 12
pub fn CALL_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let call_addr = cpu.get16(mem, NN);
    let next_instr = cpu.reg.pc;
    cpu.push_stack(mem, Value16(next_instr));
    cpu.jump(call_addr);
    12
}

// 'CALL NZ,nn' C4 12
pub fn CALL_NZ_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    if !cpu.reg.is_set(Zero) {
        CALL_nn(cpu, mem);
    }
    12
}

// 'CALL Z,nn' CC 12
pub fn CALL_Z_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    if cpu.reg.is_set(Zero) {
        CALL_nn(cpu, mem);
    }
    12
}

// 'CALL NC,nn' D4 12
pub fn CALL_NC_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    if !cpu.reg.is_set(Carry) {
        CALL_nn(cpu, mem);
    }
    12
}

// 'CALL C,nn' DC 12
pub fn CALL_C_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    if cpu.reg.is_set(Carry) {
        CALL_nn(cpu, mem);
    }
    12
}

// 'RST 00H' C7 32
pub fn RST_00h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x00);
    32
}

// 'RST 08H' CF 32
pub fn RST_08h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x08);
    32
}

// 'RST 10H' D7 32
pub fn RST_10h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x10);
    32
}

// 'RST 18H' DF 32
pub fn RST_18h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x18);
    32
}

// 'RST 20H' E7 32
pub fn RST_20h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x20);
    32
}

// 'RST 28H' EF 32
pub fn RST_28h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x28);
    32
}

// 'RST 30H' F7 32
pub fn RST_30h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x30);
    32
}

// 'RST 38H' FF 32
pub fn RST_38h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.restart(mem, 0x38);
    32
}

// 'RET' C9 8
pub fn RET(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.pop_stack(mem, PC);
    8
}

// 'RET NZ' C0 8
pub fn RET_NZ(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.return_flag(mem, Zero);
    8
}

// 'RET Z' C8 8
pub fn RET_Z(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.return_not_flag(mem, Zero);
    8
}

// 'RET NC' D0 8
pub fn RET_NC(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.return_flag(mem, Carry);
    8
}

// 'RET C' D8 8
pub fn RET_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.return_not_flag(mem, Carry);
    8
}

// 'RETI' D9 8
pub fn RETI(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.pop_stack(mem, PC);
    cpu.interrupts_enabled = true;
    8
}

// 'LD B,n' 06 8
pub fn LD_B_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, N);
    8
}

// 'LD C,n' 0E 8
pub fn LD_C_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, N);
    8
}

// 'LD D,n' 16 8
pub fn LD_D_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, N);
    8
}

// 'LD E,n' 1E 8
pub fn LD_E_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, N);
    8
}

// 'LD H,n' 26 8
pub fn LD_H_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, N);
    8
}

// 'LD L,n' 2E 8
pub fn LD_L_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, N);
    8
}

// 'LD A,A' 7F 4
pub fn LD_a_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, A);
    4
}

// 'LD A,B' 78 4
pub fn LD_a_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, B);
    4
}

// 'LD A,C' 79 4
pub fn LD_a_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, C);
    4
}

// 'LD A,D' 7A 4
pub fn LD_a_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, D);
    4
}

// 'LD A,E' 7B 4
pub fn LD_a_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, E);
    4
}

// 'LD A,H' 7C 4
pub fn LD_a_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, H);
    4
}

// 'LD A,L' 7D 4
pub fn LD_a_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, L);
    4
}

// 'LD B,B' 40 4
pub fn LD_b_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, B);
    4
}
// 'LD B,C' 41 4
pub fn LD_b_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, C);
    4
}

// 'LD B,D' 42 4
pub fn LD_b_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, D);
    4
}

// 'LD B,E' 43 4
pub fn LD_b_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, E);
    4
}

// 'LD B,H' 44 4
pub fn LD_b_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, H);
    4
}

// 'LD B,L' 45 4
pub fn LD_b_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, L);
    4
}

// 'LD B,A' 47 4
pub fn LD_b_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, A);
    4
}

// 'LD B,(HL)' 46 8
pub fn LD_b_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, B, RegisterAddress(HL));
    8
}

// 'LD C,B' 48 4
pub fn LD_c_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, B);
    4
}

// 'LD C,C' 49 4
pub fn LD_c_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, C);
    4
}

// 'LD C,D' 4A 4
pub fn LD_c_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, D);
    4
}

// 'LD C,E' 4B 4
pub fn LD_c_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, E);
    4
}

// 'LD C,H' 4C 4
pub fn LD_c_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, H);
    4
}

// 'LD C,L' 4D 4
pub fn LD_c_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, L);
    4
}

// 'LD C,A' 4F 4
pub fn LD_c_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, A);
    4
}

// 'LD C,(HL)' 4E 8
pub fn LD_c_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, C, RegisterAddress(HL));
    8
}

// 'LD D,B' 50 4
pub fn LD_d_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, B);
    4
}

// 'LD D,C' 51 4
pub fn LD_d_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, C);
    4
}

// 'LD D,D' 52 4
pub fn LD_d_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, D);
    4
}

// 'LD D,E' 53 4
pub fn LD_d_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, E);
    4
}

// 'LD D,H' 54 4
pub fn LD_d_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, H);
    4
}

// 'LD D,L' 55 4
pub fn LD_d_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, L);
    4
}

// 'LD D,A' 57 4
pub fn LD_d_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, A);
    4
}

// 'LD D,(HL)' 56 8
pub fn LD_d_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, D, RegisterAddress(HL));
    8
}

// 'LD E,B' 58 4
pub fn LD_e_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, B);
    4
}

// 'LD E,C' 59 4
pub fn LD_e_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, C);
    4
}

// 'LD E,D' 5A 4
pub fn LD_e_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, D);
    4
}

// 'LD E,E' 5B 4
pub fn LD_e_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, E);
    4
}

// 'LD E,H' 5C 4
pub fn LD_e_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, H);
    4
}

// 'LD E,L' 5D 4
pub fn LD_e_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, L);
    4
}

// 'LD E,(HL)' 5E 8
pub fn LD_e_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, RegisterAddress(HL));
    8
}

// 'LD E,A' 5F 4
pub fn LD_e_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, E, A);
    4
}

// 'LD H,B' 60 4
pub fn LD_h_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, B);
    4
}

// 'LD H,C' 61 4
pub fn LD_h_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, C);
    4
}

// 'LD H,D' 62 4
pub fn LD_h_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, D);
    4
}

// 'LD H,E' 63 4
pub fn LD_h_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, E);
    4
}

// 'LD H,H' 64 4
pub fn LD_h_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, H);
    4
}

// 'LD H,L' 65 4
pub fn LD_h_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, L);
    4
}

// 'LD H,(HL)' 66 8
pub fn LD_h_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, RegisterAddress(HL));
    8
}

// 'LD H,A' 67 4
pub fn LD_h_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, H, A);
    4
}

// 'LD L,B' 68 4
pub fn LD_l_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, B);
    4
}

// 'LD L,C' 69 4
pub fn LD_l_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, C);
    4
}

// 'LD L,D' 6A 4
pub fn LD_l_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, D);
    4
}

// 'LD L,E' 6B 4
pub fn LD_l_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, E);
    4
}

// 'LD L,H' 6C 4
pub fn LD_l_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, H);
    4
}

// 'LD L,L' 6D 4
pub fn LD_l_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, L);
    4
}

// 'LD L,A' 6F 4
pub fn LD_l_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, A);
    4
}

// 'LD L,(HL)' 6E 8
pub fn LD_l_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, L, RegisterAddress(HL));
    8
}

// 'LD (HL),B' 70 8
pub fn LD_HLm_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), B);
    8
}

// 'LD (HL),C' 71 8
pub fn LD_HLm_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), C);
    8
}

// 'LD (HL),D' 72 8
pub fn LD_HLm_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), D);
    8
}

// 'LD (HL),E' 73 8
pub fn LD_HLm_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), E);
    8
}

// 'LD (HL),H' 74 8
pub fn LD_HLm_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), H);
    8
}

// 'LD (HL),L' 75 8
pub fn LD_HLm_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), L);
    8
}

// 'LD (HL),n' 36 12
pub fn LD_HLm_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), N);
    2
}

// 'LD (HL),A' 77 8
pub fn LD_HLm_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), A);
    8
}

// 'LD A,(BC)' 0A 8
pub fn LD_a_BCm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, RegisterAddress(BC));
    8
}

// 'LD A,(DE)' 1A 8
pub fn LD_a_DEm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, RegisterAddress(DE));
    8
}

// 'LD A,(HL)' 7E 8
pub fn LD_a_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, RegisterAddress(HL));
    8
}

// 'LD A,(nn)' FA 16
pub fn LD_a_nnm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, RegisterAddress(NN));
    6
}

// 'LD A,n' 3E 8
pub fn LD_a_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, N);
    8
}

// 'LD (BC),A' 02 8
pub fn LD_BCm_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(BC), A);
    8
}

// 'LD (DE),A' 12 8
pub fn LD_DEm_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(DE), A);
    8
}

// 'LD (nn),A' EA 16
pub fn LD_nnm_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(NN), A);
    6
}

// 'LD A,($FF00 + C)' F2 8
pub fn LD_a_c_mem(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(mem, C) as u16);
    cpu.load_rr(mem, A, Address(addr));
    8
}

// 'LD ($FF00+C),A' E2 8
pub fn LD_c_mem_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(mem, C) as u16);
    cpu.load_rr(mem, Address(addr), A);
    8
}

// 'LDD A,(HL)' 3A 8
pub fn LDD_a_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, RegisterAddress(HL));
    cpu.compute_dec16(mem, HL);
    8
}

// 'LDD (HL),A' 32 8
pub fn LDD_HLm_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), A);
    cpu.compute_dec16(mem, HL);
    8
}

// 'LDI A,(HL)' 2A 8
pub fn LDI_a_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, A, RegisterAddress(HL));
    cpu.compute_inc16(mem, HL);
    8
}

// 'LDI (HL),A' 22 8
pub fn LDI_HLm_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr(mem, RegisterAddress(HL), A);
    cpu.compute_inc16(mem, HL);
    8
}

// 'LDH ($FF00+n),A' E0 12
pub fn LDH_nm_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(mem, N) as u16);
    cpu.load_rr(mem, Address(addr), A);
    12
}

// 'LDH A,($FF00+n)' F0 12
pub fn LDH_a_nm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr = 0xFF00 | (cpu.get8(mem, N) as u16);
    cpu.load_rr(mem, A, Address(addr));
    12
}

// *** 16-bit loads ***

// 'LD BC,nn' 01 12
pub fn LD_bc_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr16(mem, BC, NN);
    12
}

// 'LD DE,nn' 11 12
pub fn LD_de_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr16(mem, DE, NN);
    12
}

// 'LD HL,nn' 21 12
pub fn LD_hl_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr16(mem, HL, NN);
    12
}

// 'LD SP,nn' 31 12
pub fn LD_sp_nn(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr16(mem, SP, NN);
    12
}

// 'LD SP,HL' F9 8
pub fn LD_sp_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.load_rr16(mem, SP, HL);
    8
}

// 'LD HL,SP+n' F8 12
pub fn LDHL_sp_n(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let spn = cpu.get16(mem, SP).wrapping_add(cpu.get8(mem, N) as u16);
    cpu.compute_add16(mem, HL, Value16(spn));
    12
}

// 'LD (nn),SP' 08 20
pub fn LD_nnm_sp(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    let addr_low = cpu.get_immediate16(mem);
    let addr_high = addr_low.wrapping_add(1);

    let low = (cpu.get16(mem, SP) & 0x00FF) as u8;
    let high = (cpu.get16(mem, SP) >> 8) as u8;

    cpu.load_rr(mem, Address(addr_low), Value8(low));
    cpu.load_rr(mem, Address(addr_high), Value8(high));
    20
}

// 'NOP' 00 4
pub fn nop(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    4
}

// 'SWAP A' CB 37 8
pub fn SWAP_a(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, A);
    8
}

// 'SWAP B' CB 30 8
pub fn SWAP_b(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, B);
    8
}

// 'SWAP C' CB 31 8
pub fn SWAP_c(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, C);
    8
}

// 'SWAP D' CB 32 8
pub fn SWAP_d(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, D);
    8
}

// 'SWAP E' CB 33 8
pub fn SWAP_e(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, E);
    8
}

// 'SWAP H' CB 34 8
pub fn SWAP_h(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, H);
    8
}

// 'SWAP L' CB 35 8
pub fn SWAP_l(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, L);
    8
}

// 'SWAP (HL)' CB 36 16
pub fn SWAP_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_swap(mem, RegisterAddress(HL));
    16
}

// 'EI' FB 4
pub fn EI(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.interrupts_enabled = true;
    4
}

// 'DI' F3 4
pub fn DI(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.interrupts_enabled = false;
    4
}

// 'RLCA' 07 4
pub fn RLCA(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, A);
    4
}

// 'RLA' 17 4
pub fn RLA(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, A);
    4
}

// 'RRCA' 0F 4
pub fn RRCA(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, A);
    4
}

// 'RRA' 1F 4
pub fn RRA(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, A);
    4
}

// 'RLC A' CB 07 8
pub fn RLC_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, A);
    8
}

// 'RLC B' CB 00 8
pub fn RLC_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, B);
    8
}

// 'RLC C' CB 01 8
pub fn RLC_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, C);
    8
}

// 'RLC D' CB 02 8
pub fn RLC_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, D);
    8
}

// 'RLC E' CB 03 8
pub fn RLC_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, E);
    8
}

// 'RLC H' CB 04 8
pub fn RLC_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, H);
    8
}

// 'RLC L' CB 05 8
pub fn RLC_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, L);
    8
}

// 'RLC (HL)' CB 06 16
pub fn RLC_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left_carry(mem, RegisterAddress(HL));
    16
}

// 'RL A' CB 17 8
pub fn RL_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, A);
    8
}

// 'RL B' CB 10 8
pub fn RL_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, B);
    8
}

// 'RL C' CB 11 8
pub fn RL_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, C);
    8
}

// 'RL D' CB 12 8
pub fn RL_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, D);
    8
}

// 'RL E' CB 13 8
pub fn RL_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, E);
    8
}

// 'RL H' CB 14 8
pub fn RL_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, H);
    8
}

// 'RL L' CB 15 8
pub fn RL_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, L);
    8
}

// 'RL (HL)' CB 16 16
pub fn RL_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_left(mem, RegisterAddress(HL));
    16
}

// 'RRC A' CB 0F 8
pub fn RRC_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, A);
    8
}

// 'RRC B' CB 08 8
pub fn RRC_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, B);
    8
}

// 'RRC C' CB 09 8
pub fn RRC_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, C);
    8
}

// 'RRC D' CB 0A 8
pub fn RRC_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, D);
    8
}

// 'RRC E' CB 0B 8
pub fn RRC_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, E);
    8
}

// 'RRC H' CB 0C 8
pub fn RRC_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, H);
    8
}

// 'RRC L' CB 0D 8
pub fn RRC_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, L);
    8
}

// 'RRC (HL)' CB 0E 16
pub fn RRC_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right_carry(mem, RegisterAddress(HL));
    16
}

// 'RR A' CB 1F 8
pub fn RR_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, A);
    8
}

// 'RR B' CB 18 8
pub fn RR_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, B);
    8
}

// 'RR C' CB 19 8
pub fn RR_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, C);
    8
}

// 'RR D' CB 1A 8
pub fn RR_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, D);
    8
}

// 'RR E' CB 1B 8
pub fn RR_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, E);
    8
}

// 'RR H' CB 1C 8
pub fn RR_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, H);
    8
}

// 'RR L' CB 1D 8
pub fn RR_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, L);
    8
}

// 'RR (HL)' CB 1E 16
pub fn RR_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.rotate_right(mem, RegisterAddress(HL));
    16
}

// 'SLA A' CB 27 8
pub fn SLA_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, A);
    8
}

// 'SLA B' CB 20 8
pub fn SLA_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, B);
    8
}

// 'SLA C' CB 21 8
pub fn SLA_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, C);
    8
}

// 'SLA D' CB 22 8
pub fn SLA_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, D);
    8
}

// 'SLA E' CB 23 8
pub fn SLA_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, E);
    8
}

// 'SLA H' CB 24 8
pub fn SLA_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, H);
    8
}

// 'SLA L' CB 25 8
pub fn SLA_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, L);
    8
}

// 'SLA (HL)' CB 26 16
pub fn SLA_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, true, RegisterAddress(HL));
    16
}

// 'SRA A' CB 2F 8
pub fn SRA_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, A);
    8
}

// 'SRA B' CB 28 8
pub fn SRA_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, B);
    8
}

// 'SRA C' CB 29 8
pub fn SRA_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, C);
    8
}

// 'SRA D' CB 2A 8
pub fn SRA_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, D);
    8
}

// 'SRA E' CB 2B 8
pub fn SRA_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, E);
    8
}

// 'SRA H' CB 2C 8
pub fn SRA_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, H);
    8
}

// 'SRA L' CB 2D 8
pub fn SRA_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, L);
    8
}

// 'SRA (HL)' CB 2E 16
pub fn SRA_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift_r(mem, RegisterAddress(HL));
    16
}

// 'SRL A' CB 3F 8
pub fn SRL_A(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, A);
    8
}

// 'SRL B' CB 38 8
pub fn SRL_B(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, B);
    8
}

// 'SRL C' CB 39 8
pub fn SRL_C(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, C);
    8
}

// 'SRL D' CB 3A 8
pub fn SRL_D(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, D);
    8
}

// 'SRL E' CB 3B 8
pub fn SRL_E(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, E);
    8
}

// 'SRL H' CB 3C 8
pub fn SRL_H(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, H);
    8
}

// 'SRL L' CB 3D 8
pub fn SRL_L(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, L);
    8
}

// 'SRL (HL)' CB 3E 16
pub fn SRL_HLm(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.compute_shift(mem, false, RegisterAddress(HL));
    16
}

// 'PUSH AF' F5 16
pub fn PUSH_af(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.push_stack(mem, AF);
    16
}

// 'PUSH BC' C5 16
pub fn PUSH_bc(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.push_stack(mem, BC);
    16
}

// 'PUSH DE' D5 16
pub fn PUSH_de(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.push_stack(mem, DE);
    16
}

// 'PUSH HL' E5 16
pub fn PUSH_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.push_stack(mem, HL);
    16
}

// 'POP AF' F1 12
pub fn POP_af(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.pop_stack(mem, AF);
    12
}

// 'POP BC' C1 12
pub fn POP_bc(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.pop_stack(mem, BC);
    12
}

// 'POP DE' D1 12
pub fn POP_de(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.pop_stack(mem, DE);
    12
}

// 'POP HL' E1 12
pub fn POP_hl(cpu: &mut CPU, mem: &mut MMU) -> i32 {
    cpu.pop_stack(mem, HL);
    12
}
