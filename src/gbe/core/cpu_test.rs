use gbe::core::cpu::CPU;
use gbe::registers::*;

#[test]
fn compute_add_8_bit() {
    let mut cpu = CPU::new();
    // simple add
    cpu.compute_add(2u8, 2u8);
    assert_eq!(cpu.reg.af.high.get(), 4u8);
    // no flags
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(!cpu.reg.is_set(Flags::HalfCarry));
    assert!(!cpu.reg.is_set(Flags::Sub));

    // overflow
    cpu.compute_add(255u8, 1u8);
    assert_eq!(cpu.reg.af.high.get(), 0u8);
    // all except sub
    assert!(cpu.reg.is_set(Flags::Zero));
    assert!(cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(!cpu.reg.is_set(Flags::Sub));

    // halfcarry
    cpu.compute_add(127u8, 1u8);
    assert_eq!(cpu.reg.af.high.get(), 128u8);
    // HC only
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(!cpu.reg.is_set(Flags::Sub));
}

#[test]
fn compute_sub_8_bit() {
    let mut cpu = CPU::new();
    // simple sub
    cpu.compute_sub(8u8, 2u8);
    assert_eq!(cpu.reg.af.high.get(), 6u8);
    // sub only
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(!cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));

    // underflow
    cpu.compute_sub(0u8, 1u8);
    assert_eq!(cpu.reg.af.high.get(), 255u8);
    // all flags except zero
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));

    // halfcarry
    cpu.compute_sub(128u8, 1u8);
    assert_eq!(cpu.reg.af.high.get(), 127u8);
    // HC and sub
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));
}