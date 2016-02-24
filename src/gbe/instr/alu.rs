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
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
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
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
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
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
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
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
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
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
    cpu.compute_and(rhs);
}

// AND # E6 8
pub fn AND_n(cpu: &mut CPU) {
    let rhs = cpu.get_immediate8();
    cpu.compute_and(rhs);
}
