//! Rotates & Shifts
use jeebie::core::cpu::CPU;
use jeebie::core::registers::Register8::*;
use jeebie::core::registers::Register16::*;

// 'RLCA' 07 4
pub fn RLCA(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(A);
    4
}

// 'RLA' 17 4
pub fn RLA(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(A);
    4
}

// 'RRCA' 0F 4
pub fn RRCA(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(A);
    4
}

// 'RRA' 1F 4
pub fn RRA(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(A);
    4
}

// 'RLC A' CB 07 8
pub fn RLC_A(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(A);
    8
}

// 'RLC B' CB 00 8
pub fn RLC_B(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(B);
    8
}

// 'RLC C' CB 01 8
pub fn RLC_C(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(C);
    8
}

// 'RLC D' CB 02 8
pub fn RLC_D(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(D);
    8
}

// 'RLC E' CB 03 8
pub fn RLC_E(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(E);
    8
}

// 'RLC H' CB 04 8
pub fn RLC_H(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(H);
    8
}

// 'RLC L' CB 05 8
pub fn RLC_L(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(L);
    8
}

// 'RLC (HL)' CB 06 16
pub fn RLC_HLm(cpu: &mut CPU) -> i32 {
    cpu.rotate_left_carry(RegisterAddress(HL));
    16
}

// 'RL A' CB 17 8
pub fn RL_A(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(A);
    8
}

// 'RL B' CB 10 8
pub fn RL_B(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(B);
    8
}

// 'RL C' CB 11 8
pub fn RL_C(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(C);
    8
}

// 'RL D' CB 12 8
pub fn RL_D(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(D);
    8
}

// 'RL E' CB 13 8
pub fn RL_E(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(E);
    8
}

// 'RL H' CB 14 8
pub fn RL_H(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(H);
    8
}

// 'RL L' CB 15 8
pub fn RL_L(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(L);
    8
}

// 'RL (HL)' CB 16 16
pub fn RL_HLm(cpu: &mut CPU) -> i32 {
    cpu.rotate_left(RegisterAddress(HL));
    16
}

// 'RRC A' CB 0F 8
pub fn RRC_A(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(A);
    8
}

// 'RRC B' CB 08 8
pub fn RRC_B(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(B);
    8
}

// 'RRC C' CB 09 8
pub fn RRC_C(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(C);
    8
}

// 'RRC D' CB 0A 8
pub fn RRC_D(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(D);
    8
}

// 'RRC E' CB 0B 8
pub fn RRC_E(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(E);
    8
}

// 'RRC H' CB 0C 8
pub fn RRC_H(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(H);
    8
}

// 'RRC L' CB 0D 8
pub fn RRC_L(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(L);
    8
}

// 'RRC (HL)' CB 0E 16
pub fn RRC_HLm(cpu: &mut CPU) -> i32 {
    cpu.rotate_right_carry(RegisterAddress(HL));
    16
}

// 'RR A' CB 1F 8
pub fn RR_A(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(A);
    8
}

// 'RR B' CB 18 8
pub fn RR_B(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(B);
    8
}

// 'RR C' CB 19 8
pub fn RR_C(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(C);
    8
}

// 'RR D' CB 1A 8
pub fn RR_D(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(D);
    8
}

// 'RR E' CB 1B 8
pub fn RR_E(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(E);
    8
}

// 'RR H' CB 1C 8
pub fn RR_H(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(H);
    8
}

// 'RR L' CB 1D 8
pub fn RR_L(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(L);
    8
}

// 'RR (HL)' CB 1E 16
pub fn RR_HLm(cpu: &mut CPU) -> i32 {
    cpu.rotate_right(RegisterAddress(HL));
    16
}


 // 'SLA A' CB 27 8
 pub fn SLA_A(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, A);
     8
 }

 // 'SLA B' CB 20 8
 pub fn SLA_B(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, B);
     8
 }

 // 'SLA C' CB 21 8
 pub fn SLA_C(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, C);
     8
 }

 // 'SLA D' CB 22 8
 pub fn SLA_D(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, D);
     8
 }

 // 'SLA E' CB 23 8
 pub fn SLA_E(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, E);
     8
 }

 // 'SLA H' CB 24 8
 pub fn SLA_H(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, H);
     8
 }

 // 'SLA L' CB 25 8
 pub fn SLA_L(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, L);
     8
 }

 // 'SLA (HL)' CB 26 16
 pub fn SLA_HLm(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(true, RegisterAddress(HL));
     16
 }

 // 'SRA A' CB 2F 8
 pub fn SRA_A(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(A);
     8
 }

 // 'SRA B' CB 28 8
 pub fn SRA_B(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(B);
     8
 }

 // 'SRA C' CB 29 8
 pub fn SRA_C(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(C);
     8
 }

 // 'SRA D' CB 2A 8
 pub fn SRA_D(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(D);
     8
 }

 // 'SRA E' CB 2B 8
 pub fn SRA_E(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(E);
     8
 }

 // 'SRA H' CB 2C 8
 pub fn SRA_H(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(H);
     8
 }

 // 'SRA L' CB 2D 8
 pub fn SRA_L(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(L);
     8
 }

 // 'SRA (HL)' CB 2E 16
 pub fn SRA_HLm(cpu: &mut CPU) -> i32 {
     cpu.compute_shift_r(RegisterAddress(HL));
     16
 }

 // 'SRL A' CB 3F 8
 pub fn SRL_A(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, A);
     8
 }

 // 'SRL B' CB 38 8
 pub fn SRL_B(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, B);
     8
 }

 // 'SRL C' CB 39 8
 pub fn SRL_C(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, C);
     8
 }

 // 'SRL D' CB 3A 8
 pub fn SRL_D(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, D);
     8
 }

 // 'SRL E' CB 3B 8
 pub fn SRL_E(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, E);
     8
 }

 // 'SRL H' CB 3C 8
 pub fn SRL_H(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, H);
     8
 }

 // 'SRL L' CB 3D 8
 pub fn SRL_L(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, L);
     8
 }

 // 'SRL (HL)' CB 3E 16
 pub fn SRL_HLm(cpu: &mut CPU) -> i32 {
     cpu.compute_shift(false, RegisterAddress(HL));
     16
 }