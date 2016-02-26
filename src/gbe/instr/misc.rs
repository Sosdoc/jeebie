/// Module for miscellaneous instructions

use gbe::cpu::CPU;

// SWAP A CB 37 8
pub fn SWAP_a(cpu: &mut CPU) {
    let value = cpu.reg.af.high.get();
    let res = cpu.compute_swap(value);
    cpu.reg.af.high.set(res);
}

// SWAP B CB 30 8
pub fn SWAP_b(cpu: &mut CPU) {
    let value = cpu.reg.bc.high.get();
    let res = cpu.compute_swap(value);
    cpu.reg.bc.high.set(res);
}

// SWAP C CB 31 8
pub fn SWAP_c(cpu: &mut CPU) {
    let value = cpu.reg.bc.low.get();
    let res = cpu.compute_swap(value);
    cpu.reg.bc.low.set(res);
}

// SWAP D CB 32 8
pub fn SWAP_d(cpu: &mut CPU) {
    let value = cpu.reg.de.high.get();
    let res = cpu.compute_swap(value);
    cpu.reg.de.high.set(res);
}

// SWAP E CB 33 8
pub fn SWAP_e(cpu: &mut CPU) {
    let value = cpu.reg.de.low.get();
    let res = cpu.compute_swap(value);
    cpu.reg.de.low.set(res);
}

// SWAP H CB 34 8
pub fn SWAP_h(cpu: &mut CPU) {
    let value = cpu.reg.hl.high.get();
    let res = cpu.compute_swap(value);
    cpu.reg.hl.high.set(res);
}

// SWAP L CB 35 8
pub fn SWAP_l(cpu: &mut CPU) {
    let value = cpu.reg.hl.low.get();
    let res = cpu.compute_swap(value);
    cpu.reg.hl.low.set(res);
}

// SWAP (HL) CB 36 16
pub fn SWAP_hl(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let value = cpu.mem.read_b(addr);
    let res = cpu.compute_swap(value);
    cpu.mem.write_b(addr, res);
}
