/// Module for 8 bit arithmetic (ALU instructions)

use gbe::cpu::CPU;
use gbe::registers::Flags;

// ADD A,A 87 4
pub fn ADD_a_a(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.af.high.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,B 80 4
pub fn ADD_a_b(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,C 81 4
pub fn ADD_a_c(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,D 82 4
pub fn ADD_a_d(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.high.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,E 83 4
pub fn ADD_a_e(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.low.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,H 84 4
pub fn ADD_a_h(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,L 85 4
pub fn ADD_a_l(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_add(lhs, rhs);
}

// ADD A,(HL) 86 8
pub fn ADD_a_hlm(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_add(lhs, rhs);
}

// ADD A,# C6 8
pub fn ADD_a_n(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.get_immediate8();
    cpu.compute_add(lhs, rhs);
}

// ADC A,A 8F 4
pub fn ADC_a_a(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.af.high.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,B 88 4
pub fn ADC_a_b(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,C 89 4
pub fn ADC_a_c(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,D 8A 4
pub fn ADC_a_d(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.high.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,E 8B 4
pub fn ADC_a_e(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.low.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,H 8C 4
pub fn ADC_a_h(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,L 8D 4
pub fn ADC_a_l(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,(HL) 8E 8
pub fn ADC_a_hlm(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// ADC A,# CE 8
pub fn ADC_a_n(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.get_immediate8();
    cpu.compute_add(lhs, rhs.wrapping_add(1));
}

// SUB A 97 4
pub fn SUB_a_A(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.af.high.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB B 90 4
pub fn SUB_a_B(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB C 91 4
pub fn SUB_a_C(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB D 92 4
pub fn SUB_a_D(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.high.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB E 93 4
pub fn SUB_a_E(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.low.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB H 94 4
pub fn SUB_a_H(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB L 95 4
pub fn SUB_a_L(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_sub(lhs, rhs);
}

// SUB (HL) 96 8
pub fn SUB_a_hlm(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_sub(lhs, rhs);
}

// SUB # D6 8
pub fn SUB_a_n(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.get_immediate8();
    cpu.compute_sub(lhs, rhs);
}

// SBC A,A 9F 4
pub fn SBC_a_a(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.af.high.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,B 98 4
pub fn SBC_a_b(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,C 99 4
pub fn SBC_a_c(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,D 9A 4
pub fn SBC_a_d(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.high.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,E 9B 4
pub fn SBC_a_e(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.low.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,H 9C 4
pub fn SBC_a_h(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,L 9D 4
pub fn SBC_a_l(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,(HL) 9E 8
pub fn SBC_a_hlm(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}

// SBC A,# ?? ? --- manual has no opcode for this... leave it
pub fn SBC_a_n(cpu: &mut CPU) {
    let carry = if cpu.reg.is_set(Flags::Carry) {1} else {0};
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.get_immediate8();
    cpu.compute_sub(lhs, rhs.wrapping_sub(1));
}


// AND A A7 4
pub fn AND_a(cpu: &mut CPU) {
    let rhs = cpu.reg.af.high.get();
    cpu.compute_and(rhs);
}

// AND B A0 4
pub fn AND_b(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_and(rhs);
}

// AND C A1 4
pub fn AND_c(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_and(rhs);
}

// AND D A2 4
pub fn AND_d(cpu: &mut CPU) {
    let rhs = cpu.reg.de.high.get();
    cpu.compute_and(rhs);
}

// AND E A3 4
pub fn AND_e(cpu: &mut CPU) {
    let rhs = cpu.reg.de.low.get();
    cpu.compute_and(rhs);
}

// AND H A4 4
pub fn AND_h(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_and(rhs);
}

// AND L A5 4
pub fn AND_l(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_and(rhs);
}

// AND (HL) A6 8
pub fn AND_hlm(cpu: &mut CPU) {
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_and(rhs);
}

// AND # E6 8
pub fn AND_n(cpu: &mut CPU) {
    let rhs = cpu.get_immediate8();
    cpu.compute_and(rhs);
}


// OR A B7 4
pub fn OR_a(cpu: &mut CPU) {
    let rhs = cpu.reg.af.high.get();
    cpu.compute_or(rhs);
}

// OR B B0 4
pub fn OR_b(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_or(rhs);
}

// OR C B1 4
pub fn OR_c(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_or(rhs);
}

// OR D B2 4
pub fn OR_d(cpu: &mut CPU) {
    let rhs = cpu.reg.de.high.get();
    cpu.compute_or(rhs);
}

// OR E B3 4
pub fn OR_e(cpu: &mut CPU) {
    let rhs = cpu.reg.de.low.get();
    cpu.compute_or(rhs);
}

// OR H B4 4
pub fn OR_h(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_or(rhs);
}

// OR L B5 4
pub fn OR_l(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_or(rhs);
}

// OR (HL) B6 8
pub fn OR_hlm(cpu: &mut CPU) {
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_or(rhs);
}

// OR # F6 8
pub fn OR_n(cpu: &mut CPU) {
    let rhs = cpu.get_immediate8();
    cpu.compute_or(rhs);
}

// XOR A AF 4
pub fn XOR_a(cpu: &mut CPU) {
    let rhs = cpu.reg.af.high.get();
    cpu.compute_xor(rhs);
}

// XOR B A8 4
pub fn XOR_b(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_xor(rhs);
}

// XOR C A9 4
pub fn XOR_c(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_xor(rhs);
}

// XOR D AA 4
pub fn XOR_d(cpu: &mut CPU) {
    let rhs = cpu.reg.de.high.get();
    cpu.compute_xor(rhs);
}

// XOR E AB 4
pub fn XOR_e(cpu: &mut CPU) {
    let rhs = cpu.reg.de.low.get();
    cpu.compute_xor(rhs);
}

// XOR H AC 4
pub fn XOR_h(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_xor(rhs);
}

// XOR L AD 4
pub fn XOR_l(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_xor(rhs);
}

// XOR (HL) AE 8
pub fn XOR_hlm(cpu: &mut CPU) {
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_xor(rhs);
}

// XOR * EE 8
pub fn XOR_n(cpu: &mut CPU) {
    let rhs = cpu.get_immediate8();
    cpu.compute_xor(rhs);
}

// CP A BF 4
pub fn CP_a(cpu: &mut CPU) {
    let rhs = cpu.reg.af.high.get();
    cpu.compute_cp(rhs);
}

// CP B B8 4
pub fn CP_b(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.high.get();
    cpu.compute_cp(rhs);
}

// CP C B9 4
pub fn CP_c(cpu: &mut CPU) {
    let rhs = cpu.reg.bc.low.get();
    cpu.compute_cp(rhs);
}

// CP D BA 4
pub fn CP_d(cpu: &mut CPU) {
    let rhs = cpu.reg.de.high.get();
    cpu.compute_cp(rhs);
}

// CP E BB 4
pub fn CP_e(cpu: &mut CPU) {
    let rhs = cpu.reg.de.low.get();
    cpu.compute_cp(rhs);
}

// CP H BC 4
pub fn CP_h(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.high.get();
    cpu.compute_cp(rhs);
}

// CP L BD 4
pub fn CP_l(cpu: &mut CPU) {
    let rhs = cpu.reg.hl.low.get();
    cpu.compute_cp(rhs);
}

// CP (HL) BE 8
pub fn CP_hlm(cpu: &mut CPU) {
    let rhs = cpu.mem.read_b(cpu.reg.hl.get());
    cpu.compute_cp(rhs);
}

// CP # FE 8
pub fn CP_n(cpu: &mut CPU) {
    let rhs = cpu.get_immediate8();
    cpu.compute_cp(rhs);
}

// INC A 3C 4
pub fn INC_a(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.af.high.get();
    cpu.compute_inc_flags(result);
}

// INC B 04 4
pub fn INC_b(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.bc.high.get();
    cpu.compute_inc_flags(result);
}

// INC C 0C 4
pub fn INC_c(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.bc.low.get();
    cpu.compute_inc_flags(result);
}

// INC D 14 4
pub fn INC_d(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.de.high.get();
    cpu.compute_inc_flags(result);
}

// INC E 1C 4
pub fn INC_e(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.de.low.get();
    cpu.compute_inc_flags(result);
}

// INC H 24 4
pub fn INC_h(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.hl.high.get();
    cpu.compute_inc_flags(result);
}

// INC L 2C 4
pub fn INC_l(cpu: &mut CPU) {
    cpu.reg.af.high.add(1);
    let result = cpu.reg.hl.low.get();
    cpu.compute_inc_flags(result);
}

// INC (HL) 34 12
pub fn INC_hlm(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let value = cpu.mem.read_b(addr);

    let result = value.wrapping_add(1);
    cpu.mem.write_b(addr, result);
    cpu.compute_inc_flags(result);
}

// DEC A 3D 4
pub fn DEC_a(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.af.high.get();
    cpu.compute_dec_flags(result);
}

// DEC B 05 4
pub fn DEC_b(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.bc.high.get();
    cpu.compute_dec_flags(result);
}

// DEC C 0D 4
pub fn DEC_c(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.bc.low.get();
    cpu.compute_dec_flags(result);
}

// DEC D 15 4
pub fn DEC_d(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.de.high.get();
    cpu.compute_dec_flags(result);
}

// DEC E 1D 4
pub fn DEC_e(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.de.low.get();
    cpu.compute_dec_flags(result);
}

// DEC H 25 4
pub fn DEC_h(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.hl.high.get();
    cpu.compute_dec_flags(result);
}

// DEC L 2D 4
pub fn DEC_l(cpu: &mut CPU) {
    cpu.reg.af.high.sub(1);
    let result = cpu.reg.hl.low.get();
    cpu.compute_dec_flags(result);
}

// DEC (HL) 35 12
pub fn DEC_hlm(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let value = cpu.mem.read_b(addr);

    let result = value.wrapping_sub(1);
    cpu.mem.write_b(addr, result);
    cpu.compute_dec_flags(result);
}
