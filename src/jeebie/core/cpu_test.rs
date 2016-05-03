use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::*;
use jeebie::core::registers::Flags::*;

#[test]
fn add_8_bit() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);
    // simple add, 2 + 2
    cpu.set8(A, 2u8);
    cpu.compute_add(A, A);
    assert_eq!(cpu.get8(A), 4u8);
    // no flags
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Carry));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));

    // overflow, 255 + 1
    cpu.set8(A, 255u8);
    cpu.set8(B, 1u8);
    cpu.compute_add(A, B);
    assert_eq!(cpu.get8(A), 0u8);
    // all except sub
    assert!(cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(Carry));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));

    // halfcarry
    cpu.set8(A, 127u8);
    cpu.set8(B, 1u8);
    cpu.compute_add(A, B);
    assert_eq!(cpu.get8(A), 128u8);
    // HC only
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Carry));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));
}

#[test]
fn sub_8_bit() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);
    // simple sub
    cpu.set8(A, 8u8);
    cpu.set8(B, 2u8);
    cpu.compute_sub(B);
    assert_eq!(cpu.get8(A), 6u8);
    // sub only
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Carry));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(cpu.reg.is_set(Sub));

    // underflow
    cpu.set8(A, 0u8);
    cpu.set8(B, 1u8);
    cpu.compute_sub(B);
    assert_eq!(cpu.get8(A), 255u8);
    // all flags except zero
    assert!(!cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(Carry));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(cpu.reg.is_set(Sub));

    // halfcarry
    cpu.set8(A, 128u8);
    cpu.set8(B, 2u8);
    cpu.compute_sub(B);
    assert_eq!(cpu.get8(A), 126u8);
    // HC and sub
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Carry));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(cpu.reg.is_set(Sub));
}

#[test]
fn cp_test() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    // simple sub
    cpu.set8(A, 10);
    cpu.set8(B, 2);
    cpu.compute_cp(B);

    assert!(cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Carry));
    assert!(!cpu.reg.is_set(HalfCarry));

    // underflow
    cpu.set8(A, 100);
    cpu.set8(B, 100);
    cpu.compute_cp(B);

    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Carry));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(cpu.reg.is_set(Sub));
}

#[test]
fn inc_test() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    // NOTE: Carry flag is unmodified
    // simple inc
    cpu.set8(A, 0x9);
    cpu.compute_inc(A);

    assert_eq!(0xA, cpu.get8(A));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(HalfCarry));

    // zero
    cpu.set8(A, 0xFF);
    cpu.compute_inc(A);

    assert_eq!(0, cpu.get8(A));
    assert!(cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));

    // halfcarry
    cpu.set8(A, 0x0F);
    cpu.compute_inc(A);

    assert_eq!(0x10, cpu.get8(A));
    assert!(!cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));
}

#[test]
fn bit_check_test() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    cpu.set8(B, 0b1000_0001u8);
    cpu.bit_check(0, B);
    // Sub is always 0, HC always 1
    assert!(!cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));

    cpu.bit_check(1, B);
    // Sub is always 0, HC always 1
    assert!(cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Sub));
}

#[test]
fn swap_test() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    cpu.set8(B, 0xAB);
    cpu.compute_swap(B);

    assert_eq!(0xBA, cpu.get8(B));
    // all reset except zero
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));

    // internal RAM, will be initialized to 0
    cpu.set16(HL, 0xC000);
    cpu.compute_swap(RegisterAddress(HL));

    assert_eq!(0, cpu.get8(RegisterAddress(HL)));
    // Zero should be set
    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));
}