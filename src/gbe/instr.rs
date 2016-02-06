use gbe::cpu::*;

//- LD register to register

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

/// NO-OP, only updates clock
pub fn nop(cpu: &mut CPU) {
    // cpu.r.m = 1;
    // cpu.r.t = 4;
}
