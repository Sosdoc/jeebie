use gbe::cpu::*;

// - LD register to register

pub fn LDrr_bb(cpu: &mut CPU) {
    let v = cpu.r.bc.get_high();
    cpu.r.bc.set_high(v);
}

pub fn LDrr_bc(cpu: &mut CPU) {
    let v = cpu.r.bc.get_low();
    cpu.r.bc.set_high(v);
}

pub fn LDrr_bd(cpu: &mut CPU) {
    cpu.r.bc.set_high(cpu.r.de.get_high());
}

pub fn LDrr_be(cpu: &mut CPU) {
    cpu.r.bc.set_high(cpu.r.de.get_low());
}

pub fn LDrr_bh(cpu: &mut CPU) {
    cpu.r.bc.set_high(cpu.r.hl.get_high());
}

pub fn LDrr_bl(cpu: &mut CPU) {
    cpu.r.bc.set_high(cpu.r.hl.get_low());
}

pub fn LDrr_ba(cpu: &mut CPU) {
    cpu.r.bc.set_high(cpu.r.af.get_high());
}

pub fn LDrr_cb(cpu: &mut CPU) {
    let v = cpu.r.bc.get_high();
    cpu.r.bc.set_low(v);
}

pub fn LDrr_cc(cpu: &mut CPU) {
    let v = cpu.r.bc.get_low();
    cpu.r.bc.set_low(v);
}

pub fn LDrr_cd(cpu: &mut CPU) {
    cpu.r.bc.set_low(cpu.r.de.get_high());
}

pub fn LDrr_ce(cpu: &mut CPU) {
    cpu.r.bc.set_low(cpu.r.de.get_low());
}

pub fn LDrr_ch(cpu: &mut CPU) {
    cpu.r.bc.set_low(cpu.r.hl.get_high());
}

pub fn LDrr_cl(cpu: &mut CPU) {
    cpu.r.bc.set_low(cpu.r.hl.get_low());
}

pub fn LDrr_ca(cpu: &mut CPU) {
    cpu.r.bc.set_low(cpu.r.af.get_high());
}

pub fn LDrr_db(cpu: &mut CPU) {
    cpu.r.de.set_high(cpu.r.bc.get_high());
}

pub fn LDrr_dc(cpu: &mut CPU) {
    cpu.r.de.set_high(cpu.r.bc.get_low());
}

pub fn LDrr_de(cpu: &mut CPU) {
    let v = cpu.r.de.get_low();
    cpu.r.de.set_high(v);
}

pub fn LDrr_dd(cpu: &mut CPU) {
    let v = cpu.r.de.get_high();
    cpu.r.de.set_high(v);
}

pub fn LDrr_dh(cpu: &mut CPU) {
    cpu.r.de.set_high(cpu.r.hl.get_high());
}

pub fn LDrr_dl(cpu: &mut CPU) {
    cpu.r.de.set_high(cpu.r.hl.get_low());
}

pub fn LDrr_da(cpu: &mut CPU) {
    cpu.r.de.set_high(cpu.r.af.get_high());
}

pub fn LDrr_eb(cpu: &mut CPU) {
    cpu.r.de.set_low(cpu.r.bc.get_high());
}

pub fn LDrr_ec(cpu: &mut CPU) {
    cpu.r.de.set_low(cpu.r.bc.get_low());
}

pub fn LDrr_ed(cpu: &mut CPU) {
    let v = cpu.r.de.get_high();
    cpu.r.de.set_low(v);
}

pub fn LDrr_ee(cpu: &mut CPU) {
    let v = cpu.r.de.get_low();
    cpu.r.de.set_low(v);
}

pub fn LDrr_eh(cpu: &mut CPU) {
    cpu.r.de.set_low(cpu.r.hl.get_high());
}

pub fn LDrr_el(cpu: &mut CPU) {
    cpu.r.de.set_low(cpu.r.hl.get_low());
}

pub fn LDrr_ea(cpu: &mut CPU) {
    cpu.r.de.set_low(cpu.r.af.get_high());
}

pub fn LDrr_hb(cpu: &mut CPU) {
    cpu.r.hl.set_high(cpu.r.bc.get_high());
}

pub fn LDrr_hc(cpu: &mut CPU) {
    cpu.r.hl.set_high(cpu.r.bc.get_low());
}

pub fn LDrr_hd(cpu: &mut CPU) {
    cpu.r.hl.set_high(cpu.r.de.get_high());
}

pub fn LDrr_he(cpu: &mut CPU) {
    cpu.r.hl.set_high(cpu.r.de.get_low());
}

pub fn LDrr_hh(cpu: &mut CPU) {
    let v = cpu.r.hl.get_high();
    cpu.r.hl.set_high(v);
}

pub fn LDrr_hl(cpu: &mut CPU) {
    let v = cpu.r.hl.get_low();
    cpu.r.hl.set_high(v);
}

pub fn LDrr_ha(cpu: &mut CPU) {
    cpu.r.hl.set_high(cpu.r.af.get_high());
}

pub fn LDrr_lb(cpu: &mut CPU) {
    cpu.r.hl.set_low(cpu.r.bc.get_high());
}

pub fn LDrr_lc(cpu: &mut CPU) {
    cpu.r.hl.set_low(cpu.r.bc.get_low());
}

pub fn LDrr_ld(cpu: &mut CPU) {
    cpu.r.hl.set_low(cpu.r.de.get_high());
}

pub fn LDrr_le(cpu: &mut CPU) {
    cpu.r.hl.set_low(cpu.r.de.get_low());
}

pub fn LDrr_lh(cpu: &mut CPU) {
    let v = cpu.r.hl.get_high();
    cpu.r.hl.set_low(v);
}

pub fn LDrr_ll(cpu: &mut CPU) {
    let v = cpu.r.hl.get_low();
    cpu.r.hl.set_low(v);
}

pub fn LDrr_la(cpu: &mut CPU) {
    cpu.r.hl.set_low(cpu.r.af.get_high());
}

pub fn LDrr_ab(cpu: &mut CPU) {
    cpu.r.af.set_high(cpu.r.bc.get_high());
}

pub fn LDrr_ac(cpu: &mut CPU) {
    cpu.r.af.set_high(cpu.r.bc.get_low());
}

pub fn LDrr_ad(cpu: &mut CPU) {
    cpu.r.af.set_high(cpu.r.de.get_high());
}

pub fn LDrr_ae(cpu: &mut CPU) {
    cpu.r.af.set_high(cpu.r.de.get_low());
}

pub fn LDrr_ah(cpu: &mut CPU) {
    cpu.r.af.set_high(cpu.r.hl.get_high());
}

pub fn LDrr_al(cpu: &mut CPU) {
    cpu.r.af.set_high(cpu.r.hl.get_low());
}

pub fn LDrr_aa(cpu: &mut CPU) {
    let v = cpu.r.af.get_high();
    cpu.r.af.set_high(v);
}

/// Load in register from addr in HL

pub fn LDrHLm_b(cpu: &mut CPU) {
    let m = cpu.memory.borrow();
    cpu.r.bc.set_high(m.read_b(cpu.r.hl.get()));
}

pub fn LDrHLm_c(cpu: &mut CPU) {
    let m = cpu.memory.borrow();
    cpu.r.bc.set_low(m.read_b(cpu.r.hl.get()));
}

pub fn LDrHLm_d(cpu: &mut CPU) {
    let m = cpu.memory.borrow();
    cpu.r.de.set_high(m.read_b(cpu.r.hl.get()));
}

pub fn LDrHLm_e(cpu: &mut CPU) {
    let m = cpu.memory.borrow();
    cpu.r.de.set_low(m.read_b(cpu.r.hl.get()));
}

pub fn LDrHLm_h(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.hl.get())
    };
    cpu.r.hl.set_high(value);
}

pub fn LDrHLm_l(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.hl.get())
    };
    cpu.r.hl.set_low(value);
}

pub fn LDrHLm_a(cpu: &mut CPU) {
    let m = cpu.memory.borrow();
    cpu.r.af.set_high(m.read_b(cpu.r.hl.get()));
}

// write register in memory at addr in HL

pub fn LDHLmr_b(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.bc.get_high()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_c(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.bc.get_low()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_d(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.de.get_high()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_e(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.de.get_low()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_h(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.hl.get_high()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_l(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.hl.get_low()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

pub fn LDHLmr_a(cpu: &mut CPU) {
    let addr = {cpu.r.hl.get()};
    let data = {cpu.r.af.get_high()};
    let mut m = cpu.memory.borrow_mut();
    m.write_b(addr, data);
}

/// read from memory at addr in PC to register

pub fn LDrn_b(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.bc.set_high(value);
}

pub fn LDrn_c(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.bc.set_low(value);
}

pub fn LDrn_d(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.de.set_high(value);
}

pub fn LDrn_e(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.de.set_low(value);
}

pub fn LDrn_h(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.hl.set_high(value);
}

pub fn LDrn_l(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.hl.set_low(value);
}

pub fn LDrn_a(cpu: &mut CPU) {
    let value = {
        let m = cpu.memory.borrow();
        m.read_b(cpu.r.pc.get())
    };
    cpu.r.af.set_high(value);
}



/// NO-OP, only updates clock
pub fn nop(cpu: &mut CPU) {
    // cpu.r.m = 1;
    // cpu.r.t = 4;
}
