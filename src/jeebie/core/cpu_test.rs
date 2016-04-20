use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;
use jeebie::core::registers::*;
use jeebie::core::registers::Register8::*;

#[test]
fn compute_add_8_bit() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);
    // simple add, 2 + 2
    cpu.set8(A, 2u8);
    cpu.compute_add(A, A);
    assert_eq!(cpu.get8(A), 4u8);
    // no flags
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(!cpu.reg.is_set(Flags::HalfCarry));
    assert!(!cpu.reg.is_set(Flags::Sub));

    // overflow, 255 + 1
    cpu.set8(A, 255u8);
    cpu.set8(B, 1u8);
    cpu.compute_add(A, B);
    assert_eq!(cpu.get8(A), 0u8);
    // all except sub
    assert!(cpu.reg.is_set(Flags::Zero));
    assert!(cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(!cpu.reg.is_set(Flags::Sub));

    // halfcarry
    cpu.set8(A, 127u8);
    cpu.set8(B, 1u8);
    cpu.compute_add(A, B);
    assert_eq!(cpu.get8(A), 128u8);
    // HC only
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(!cpu.reg.is_set(Flags::Sub));
}

#[test]
fn compute_sub_8_bit() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);
    // simple sub
    cpu.set8(A, 8u8);
    cpu.set8(B, 2u8);
    cpu.compute_sub(B);
    assert_eq!(cpu.get8(A), 6u8);
    // sub only
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(!cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));

    // underflow
    cpu.set8(A, 0u8);
    cpu.set8(B, 1u8);
    cpu.compute_sub(B);
    assert_eq!(cpu.get8(A), 255u8);
    // all flags except zero
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));

    // halfcarry
    cpu.set8(A, 128u8);
    cpu.set8(B, 2u8);
    cpu.compute_sub(B);
    assert_eq!(cpu.get8(A), 126u8);
    // HC and sub
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));
}

#[test]
fn compute_cp_test() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    // simple sub
    cpu.set8(A, 10);
    cpu.set8(B, 2);
    cpu.compute_cp(B);

    assert!(cpu.reg.is_set(Flags::Sub));
    assert!(!cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(!cpu.reg.is_set(Flags::HalfCarry));

    // underflow
    cpu.set8(A, 100);
    cpu.set8(B, 100);
    cpu.compute_cp(B);

    assert!(cpu.reg.is_set(Flags::Zero));
    assert!(!cpu.reg.is_set(Flags::Carry));
    assert!(!cpu.reg.is_set(Flags::HalfCarry));
    assert!(cpu.reg.is_set(Flags::Sub));
}
