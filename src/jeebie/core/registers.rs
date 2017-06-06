/// The four flags and their respective bit values. Bits 0-3 are unused.
pub enum Flags {
    /// This bit is set when the result of a math operation
    /// is zero or two values match when using the CP
    /// instruction.
    Zero = 0b10000000,

    /// This bit is set if a subtraction was performed in the
    /// last math instruction.
    Sub = 0b01000000,

    /// This bit is set if a carry occurred from the lower
    /// nibble in the last math operation.
    Carry = 0b00100000,

    /// This bit is set if a carry occurred from the last
    /// math operation or if register A is the smaller value
    /// when executing the CP instruction.
    HalfCarry = 0b00010000,
}

#[derive(Clone, Copy)]
pub enum Register8 {
    A, B, C, D, E, H, L,
    RegisterAddress(Register16),
    Address(u16), N, Value8(u8)
}

#[derive(Clone, Copy)]
pub enum Register16 {
    AF, BC, DE, HL, SP, PC,
    NN, Value16(u16)
}

/// The main registers in a gameboy CPU.
/// Registers are 16 bit wide and can be accessed as L(low) and H(high).
/// Register AF is used for (A) accumulator and (F) flags.
/// This struct also offers convenience methods to get and set individual flags.
#[derive(Debug)]
pub struct Registers {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,

    pub pc: u16, // program counter
    pub sp: u16, // stack pointer
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            pc: 0, sp: 0
        }
    }

    /// Clears all flag values by resetting the F register to 0.
    pub fn clear_all_flags(&mut self) {
       self.f = 0;
    }

    /// Sets the selected flag to 1.
    pub fn set_flag(&mut self, flag: Flags) {
        self.f |= flag as u8;
    }

    /// Clears the selected flag.
    pub fn clear_flag(&mut self, flag: Flags) {
        self.f &= !(flag as u8);
    }

    pub fn set_or_clear(&mut self, flag: Flags, condition: bool) {
        if condition {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    /// Returns true if the selected flag is set.
    pub fn is_set(&self, flag: Flags) -> bool {
        let flag_value = flag as u8;
        (self.f & flag_value) == flag_value
    }
}

