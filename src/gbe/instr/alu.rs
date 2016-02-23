use gbe::cpu::CPU;
use gbe::registers::Flags;

// ADD A,A 87 4
pub fn ADD_a_a(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.af.high.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,B 80 4
pub fn ADD_a_b(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.high.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,C 81 4
pub fn ADD_a_c(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.low.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,D 82 4
pub fn ADD_a_d(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.high.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,E 83 4
pub fn ADD_a_e(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.low.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,H 84 4
pub fn ADD_a_h(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.high.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,L 85 4
pub fn ADD_a_l(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.low.get();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,(HL) 86 8
pub fn ADD_a_hlm(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADD A,# C6 8
pub fn ADD_a_n(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.get_immediate8();
    let result = cpu.compute_flags_add8(lhs, rhs);
    cpu.reg.af.high.set(result);
}

// ADC A,A 8F 4
pub fn ADC_a_a(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.af.high.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,B 88 4
pub fn ADC_a_b(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.high.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,C 89 4
pub fn ADC_a_c(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.bc.low.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,D 8A 4
pub fn ADC_a_d(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.high.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,E 8B 4
pub fn ADC_a_e(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.de.low.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,H 8C 4
pub fn ADC_a_h(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.high.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,L 8D 4
pub fn ADC_a_l(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.reg.hl.low.get();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,(HL) 8E 8
pub fn ADC_a_hlm(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.mem.borrow().read_b(cpu.reg.hl.get());
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}

// ADC A,# CE 8
pub fn ADC_a_n(cpu: &mut CPU) {
    let lhs = cpu.reg.af.high.get();
    let rhs = cpu.get_immediate8();
    let mut result = cpu.compute_flags_add8(lhs, rhs);
    if cpu.reg.is_set(Flags::Carry) {
        result = result.wrapping_add(1);
    }
    cpu.reg.af.high.set(result);
}
