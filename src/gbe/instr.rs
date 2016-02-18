use gbe::cpu::*;

// 8 bit loads, load immediate value into register
pub fn LD_B_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.bc.high, cpu.reg.pc.low.get());
}

pub fn LD_C_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.bc.low, cpu.reg.pc.low.get());
}

pub fn LD_D_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.de.high, cpu.reg.pc.low.get());
}

pub fn LD_E_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.de.low, cpu.reg.pc.low.get());
}

pub fn LD_H_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.hl.high, cpu.reg.pc.low.get());
}

pub fn LD_L_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.hl.low, cpu.reg.pc.low.get());
}

// - LD register to register
pub fn LDrr_bb(cpu: &mut CPU) {
    let v = cpu.reg.bc.high.get();
    cpu.reg.bc.high.set(v);
}

pub fn LDrr_bc(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.bc.low.get());
}

pub fn LDrr_bd(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.de.high.get());
}

pub fn LDrr_be(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.de.low.get());
}

pub fn LDrr_bh(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.hl.high.get());
}

pub fn LDrr_bl(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.hl.low.get());
}

pub fn LDrr_ba(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.af.high.get());
}

pub fn LDrr_cb(cpu: &mut CPU) {
    let v = cpu.reg.bc.high.get();
    cpu.reg.bc.low.set(v);
}

pub fn LDrr_cc(cpu: &mut CPU) {
    let v = cpu.reg.bc.low.get();
    cpu.reg.bc.low.set(v);
}

pub fn LDrr_cd(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.de.high.get());
}

pub fn LDrr_ce(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.de.low.get());
}

pub fn LDrr_ch(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.hl.high.get());
}

pub fn LDrr_cl(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.hl.low.get());
}

pub fn LDrr_ca(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.af.high.get());
}

pub fn LDrr_db(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.bc.high.get());
}

pub fn LDrr_dc(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.bc.low.get());
}

pub fn LDrr_de(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.de.low.get());
}

pub fn LDrr_dd(cpu: &mut CPU) {
    let v = cpu.reg.de.high.get();
    cpu.reg.de.high.set(v);
}

pub fn LDrr_dh(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.hl.high.get());
}

pub fn LDrr_dl(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.hl.low.get());
}

pub fn LDrr_da(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.af.high.get());
}

pub fn LDrr_eb(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.bc.high.get());
}

pub fn LDrr_ec(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.bc.low.get());
}

pub fn LDrr_ed(cpu: &mut CPU) {
    let v = cpu.reg.de.high.get();
    cpu.reg.de.low.set(v);
}

pub fn LDrr_ee(cpu: &mut CPU) {
    let v = cpu.reg.de.low.get();
    cpu.reg.de.low.set(v);
}

pub fn LDrr_eh(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.hl.high.get());
}

pub fn LDrr_el(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.hl.low.get());
}

pub fn LDrr_ea(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.af.high.get());
}

pub fn LDrr_hb(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.bc.high.get());
}

pub fn LDrr_hc(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.bc.low.get());
}

pub fn LDrr_hd(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.de.high.get());
}

pub fn LDrr_he(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.de.low.get());
}

pub fn LDrr_hh(cpu: &mut CPU) {
    let v = cpu.reg.hl.high.get();
    cpu.reg.hl.high.set(v);
}

pub fn LDrr_hl(cpu: &mut CPU) {
    let v = cpu.reg.hl.low.get();
    cpu.reg.hl.high.set(v);
}

pub fn LDrr_ha(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.af.high.get());
}

pub fn LDrr_lb(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.bc.high.get());
}

pub fn LDrr_lc(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.bc.low.get());
}

pub fn LDrr_ld(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.de.high.get());
}

pub fn LDrr_le(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.de.low.get());
}

pub fn LDrr_lh(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.hl.high.get());
}

pub fn LDrr_ll(cpu: &mut CPU) {
    let v = cpu.reg.hl.low.get();
    cpu.reg.hl.low.set(v);
}

pub fn LDrr_la(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.af.high.get());
}

pub fn LDrr_ab(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.bc.high.get());
}

pub fn LDrr_ac(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.bc.low.get());
}

pub fn LDrr_ad(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.de.high.get());
}

pub fn LDrr_ae(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.de.low.get());
}

pub fn LDrr_ah(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.hl.high.get());
}

pub fn LDrr_al(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.hl.low.get());
}

pub fn LDrr_aa(cpu: &mut CPU) {
    let v = cpu.reg.af.high.get();
    cpu.reg.af.high.set(v);
}

/// Load in register from addr in HL

pub fn LDrHLm_b(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.bc.high.set(m.read_b(cpu.reg.hl.get()));
}

pub fn LDrHLm_c(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.bc.low.set(m.read_b(cpu.reg.hl.get()));
}

pub fn LDrHLm_d(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.de.high.set(m.read_b(cpu.reg.hl.get()));
}

pub fn LDrHLm_e(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.de.low.set(m.read_b(cpu.reg.hl.get()));
}

pub fn LDrHLm_h(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    let value = m.read_b(cpu.reg.hl.get());
    cpu.reg.hl.high.set(value);
}

pub fn LDrHLm_l(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    let value = m.read_b(cpu.reg.hl.get());
    cpu.reg.hl.low.set(value);
}

pub fn LDrHLm_a(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.af.high.set(m.read_b(cpu.reg.hl.get()));
}

// write register in memory at addr in HL

pub fn LDHLmr_b(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.bc.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_c(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.bc.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_d(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.de.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_e(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.de.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_h(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.hl.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_l(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.hl.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_a(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.af.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

/// read from memory at addr in PC to register

pub fn LDrn_b(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.bc.high.set(value);
}

pub fn LDrn_c(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.bc.low.set(value);
}

pub fn LDrn_d(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.de.high.set(value);
}

pub fn LDrn_e(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.de.low.set(value);
}

pub fn LDrn_h(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.hl.high.set(value);
}

pub fn LDrn_l(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.hl.low.set(value);
}

pub fn LDrn_a(cpu: &mut CPU) {
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    cpu.reg.af.high.set(value);
}

// Write PC at address that is read from memory (addr = HL)
pub fn LDHLmn(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let value = cpu.mem.borrow().read_b(cpu.reg.pc.get());

    cpu.mem.borrow_mut().write_b(addr, value);
    cpu.reg.pc.increase();
}

// write A at address BC
pub fn LDBCmA(cpu: &mut CPU) {
    let value = cpu.reg.af.high.get();
    let addr = cpu.reg.bc.get();

    cpu.mem.borrow_mut().write_b(addr, value);
}

// write A at address DE
pub fn LDDEmA(cpu: &mut CPU) {
    let value = cpu.reg.af.high.get();
    let addr = cpu.reg.de.get();

    cpu.mem.borrow_mut().write_b(addr, value);
}

pub fn LDmmA(cpu: &mut CPU) {
// function () { MMU.wb(MMU.rw(Z80._r.pc), Z80._r.a); Z80._r.pc += 2; Z80._r.m = 4; Z80._r.t = 16; },
    let value = cpu.reg.af.high.get();
    let addr = cpu.mem.borrow().read_b(cpu.reg.pc.get());
    // TODO: check that address is correct
    cpu.mem.borrow_mut().write_b(addr as u16, value);
    cpu.reg.pc.increase();
    cpu.reg.pc.increase();
}


/// NO-OP, only updates clock
pub fn nop(cpu: &mut CPU) {
    // cpu.reg.m = 1;
    // cpu.reg.t = 4;
}
