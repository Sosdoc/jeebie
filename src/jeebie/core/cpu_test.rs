use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::*;
use jeebie::core::registers::Flags::*;

#[test]
fn add_8_bit() {
    let mut cpu = CPU::new(MMU::new());
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
fn add_16_bit() {
    let mut cpu = CPU::new(MMU::new());

    // simple add, no flags set
    cpu.set16(BC, 256u16);
    cpu.set16(DE, 4u16);
    cpu.compute_add16(BC, DE);
    assert_eq!(260u16, cpu.get16(BC));

    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(Carry));
    assert!(!cpu.reg.is_set(HalfCarry));

    // overflow
    cpu.set16(BC, 0xFFFF);
    cpu.set16(DE, 1);
    cpu.compute_add16(BC, DE);
    assert_eq!(0, cpu.get16(BC));

    // zero is unaffected
    assert!(!cpu.reg.is_set(Sub));
    assert!(cpu.reg.is_set(Carry));
    assert!(cpu.reg.is_set(HalfCarry));

    // halfcarry only
    cpu.set16(BC, 0x00FF);
    cpu.set16(DE, 1);
    cpu.compute_add16(BC, DE);
    assert_eq!(0x0100, cpu.get16(BC));

    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(Carry));
    assert!(cpu.reg.is_set(HalfCarry));
}

#[test]
fn sub_8_bit() {
    let mut cpu = CPU::new(MMU::new());
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
    let mut cpu = CPU::new(MMU::new());

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
    let mut cpu = CPU::new(MMU::new());

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
fn dec_test() {
    let mut cpu = CPU::new(MMU::new());

    // NOTE: Carry flag is unmodified
    // simple dec
    cpu.set8(A, 0x9);
    cpu.compute_dec(A);

    assert_eq!(0x8, cpu.get8(A));
    assert!(cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(HalfCarry));

    // zero
    cpu.set8(A, 1);
    cpu.compute_dec(A);

    assert_eq!(0, cpu.get8(A));
    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(cpu.reg.is_set(Sub));

    // halfcarry
    cpu.set8(A, 0x10);
    cpu.compute_dec(A);

    assert_eq!(0x0F, cpu.get8(A));
    assert!(!cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(cpu.reg.is_set(Sub));

    // internal RAM, will be initialized to 0
    cpu.set16(HL, 0xC000);
    cpu.compute_dec(RegisterAddress(HL));

    assert_eq!(0xFF, cpu.get8(RegisterAddress(HL)));
    // HC should be set
    assert!(!cpu.reg.is_set(Zero));
    assert!(cpu.reg.is_set(Sub));
    assert!(cpu.reg.is_set(HalfCarry));
}

#[test]
fn bit_check_test() {
    let mut cpu = CPU::new(MMU::new());

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
    let mut cpu = CPU::new(MMU::new());

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

#[test]
fn and_test() {
    let mut cpu = CPU::new(MMU::new());

    cpu.set8(A, 0x0F);
    cpu.set8(B, 0xAB);
    cpu.compute_and(B);

    assert_eq!(0x0B, cpu.get8(A));
    // Z 0 1 0
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));

    // internal RAM, will be initialized to 0
    cpu.set16(HL, 0xC000);
    cpu.compute_and(RegisterAddress(HL));

    assert_eq!(0, cpu.get8(A));
    // Zero should be set
    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));
}

#[test]
fn or_test() {
    let mut cpu = CPU::new(MMU::new());

    cpu.set8(A, 0x0B);
    cpu.set8(B, 0xA0);
    cpu.compute_or(B);

    assert_eq!(0xAB, cpu.get8(A));
    // Z 0 0 0
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));

    // internal RAM, will be initialized to 0
    cpu.set8(A, 0);
    cpu.set16(HL, 0xC000);
    cpu.compute_or(RegisterAddress(HL));

    assert_eq!(0, cpu.get8(A));
    // Zero should be set
    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));
}

#[test]
fn xor_test() {
    let mut cpu = CPU::new(MMU::new());

    cpu.set8(A, 0x21);
    cpu.set8(B, 0x41);
    cpu.compute_xor(B);

    assert_eq!(0x60, cpu.get8(A));
    // Z 0 0 0
    assert!(!cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));

    cpu.set8(A, 0xFF);
    cpu.set8(B, 0xFF);
    cpu.compute_xor(B);

    assert_eq!(0, cpu.get8(A));
    // Z 0 0 0
    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));

    // internal RAM, will be initialized to 0
    cpu.set8(A, 0);
    cpu.set16(HL, 0xC000);
    cpu.compute_xor(RegisterAddress(HL));

    assert_eq!(0, cpu.get8(A));
    // Zero should be set
    assert!(cpu.reg.is_set(Zero));
    assert!(!cpu.reg.is_set(Sub));
    assert!(!cpu.reg.is_set(HalfCarry));
    assert!(!cpu.reg.is_set(Carry));
}

#[test]
fn cpu_stack_test() {
    let mut cpu = CPU::new(MMU::new());

    cpu.set16(SP, 0xFFFE);
    cpu.push_stack(Value16(0xCAFE));
    cpu.push_stack(Value16(0xBABE));
    cpu.push_stack(Value16(0xFADE));

    assert_eq!(0xFFF8, cpu.get16(SP));

    cpu.pop_stack(HL);
    assert_eq!(0xFFFA, cpu.get16(SP));
    assert_eq!(0xFADE, cpu.get16(HL));

    cpu.pop_stack(DE);
    assert_eq!(0xFFFC, cpu.get16(SP));
    assert_eq!(0xBABE, cpu.get16(DE));

    cpu.pop_stack(BC);
    assert_eq!(0xFFFE, cpu.get16(SP));
    assert_eq!(0xCAFE, cpu.get16(BC));
}

#[test]
fn shift_test() {
    let mut cpu = CPU::new(MMU::new());

    // 0x80 -- 0b1000_0000
    // shift left, should be zero and carry 1
    cpu.set8(A, 0x80);
    cpu.compute_shift(true, A);
    assert_eq!(0, cpu.get8(A));
    assert_eq!(Zero as u8 | Carry as u8, cpu.reg.f);

    // shift right, should be 0x40 and no flags
    cpu.set8(A, 0x80);
    cpu.compute_shift(false, A);
    assert_eq!(0x40, cpu.get8(A));
    assert_eq!(0, cpu.reg.f);

    // 0xFF -- 0b1111_1111
    // left, should be 0xFE, carry 1
    cpu.set8(A, 0xFF);
    cpu.compute_shift(true, A);
    assert_eq!(0xFE, cpu.get8(A));
    assert_eq!(Carry as u8, cpu.reg.f);

    // right, should be 0x7F, carry 1
    cpu.set8(A, 0xFF);
    cpu.compute_shift(false, A);
    assert_eq!(0x7F, cpu.get8(A));
    assert_eq!(Carry as u8, cpu.reg.f);

    // shift 0x01 right, zero and carry 1
    cpu.set8(A, 0x01);
    cpu.compute_shift(false, A);
    assert_eq!(0, cpu.get8(A));
    assert_eq!(Zero as u8 | Carry as u8, cpu.reg.f);
}

#[test]
fn shift_r_test() {
    let mut cpu = CPU::new(MMU::new());

    // 0x80 -- 0b1000_0000
    // shift right and preserve MSB -> 0b1100_0000 / 0xC0
    cpu.set8(A, 0x80);
    cpu.compute_shift_r(A);
    assert_eq!(0xC0, cpu.get8(A));
    assert_eq!(0, cpu.reg.f);

    // 0xFF -- 0b1011_1111
    // shift right and preserve MSB -> 0b1101_1111 / 0xBF
    cpu.set8(A, 0xBF);
    cpu.compute_shift_r(A);
    assert_eq!(0xDF, cpu.get8(A));
    assert_eq!(Carry as u8, cpu.reg.f);

    // Shift right, no MSB -> 0
    cpu.set8(A, 0x01);
    cpu.compute_shift_r(A);
    assert_eq!(0, cpu.get8(A));
    assert_eq!(Zero as u8 | Carry as u8, cpu.reg.f);

    // shift 0 right, flag zero is set
    cpu.set8(A, 0);
    cpu.compute_shift_r(A);
    assert_eq!(0, cpu.get8(A));
    assert_eq!(Zero as u8, cpu.reg.f);
}
