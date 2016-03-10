use std::fmt;

/// An 8 bit register. This struct simply wraps an u8 value, it is also meant to offer simple
/// arithmetic operations with wrapping in case of overflows.
pub struct Register8 {
    value: u8,
}

impl fmt::Debug for Register8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02X}", self.value)
    }
}

impl Register8 {
    pub fn new(v: u8) -> Register8 {
        Register8 { value: v }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    /// adds the value with wrapping semantics
    pub fn add(&mut self, value: u8) {
        self.value.wrapping_add(value);
    }

    /// adds the value with wrapping semantics
    pub fn sub(&mut self, value: u8) {
        self.value.wrapping_sub(value);
    }
}

/// A 16 bit register. It is composed of 2 Register8 structs, making the high and low parts of the
/// 16 bits. The struct also offers the ability to get/set individual 8 bit registers or act as a
/// whole u16 number.
pub struct Register16 {
    pub low: Register8,
    pub high: Register8,
}

impl fmt::Debug for Register16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:04X}", self.get())
    }
}

impl Register16 {
    pub fn new(v: u16) -> Register16 {
        let low = (v & 0x00FF) as u8;
        let high = ((v & 0xFF00) >> 8) as u8;

        Register16 {
            low: Register8::new(low),
            high: Register8::new(high),
        }
    }

    /// adds the value with wrapping semantics
    pub fn add(&mut self, value: u16) {
        let new_value = self.get().wrapping_add(value);
        self.set(new_value);
    }

    /// adds the value with wrapping semantics
    pub fn sub(&mut self, value: u16) {
        let new_value = self.get().wrapping_sub(value);
        self.set(new_value);
    }

    pub fn get(&self) -> u16 {
        ((self.high.get() as u16) << 8) | self.low.get() as u16
    }

    pub fn set(&mut self, v: u16) {
        let low = (v & 0x00FF) as u8;
        let high = (v >> 8) as u8;

        self.low.set(low);
        self.high.set(high);
    }
}

/// The main registers in a gameboy CPU.
/// Registers are 16 bit wide and can be accessed as L(low) and H(high).
/// Register AF is used for (A) accumulator and (F) flags.
/// This struct also offers convenience methods to get and set individual flags.
#[derive(Debug)]
pub struct Registers {
    pub af: Register16,
    pub bc: Register16,
    pub de: Register16,
    pub hl: Register16,

    pub pc: Register16, // program counter
    pub sp: Register16, // stack pointer
}

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

impl Registers {
    /// Clears all flag values by resetting the F register to 0.
    pub fn clear_all_flags(&mut self) {
        self.af.low.set(0);
    }

    /// Sets the selected flag to 1.
    pub fn set_flag(&mut self, flag: Flags) {
        let new_flags = self.af.low.get() | (flag as u8);
        self.af.low.set(new_flags);
    }

    /// Clears the selected flag.
    pub fn clear_flag(&mut self, flag: Flags) {
        let new_flags = self.af.low.get() & !(flag as u8);
        self.af.low.set(new_flags);
    }

    /// Returns true if the selected flag is set.
    pub fn is_set(&self, flag: Flags) -> bool {
        let flag_value = flag as u8;
        (self.af.low.get() & flag_value) == flag_value
    }
}
