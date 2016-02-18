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

    pub fn increase(&mut self) {
        self.value.wrapping_add(1);
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

    pub fn increase(&mut self) {
        let new_value = self.get().wrapping_add(1);
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
/// Register AF is used for (A) accumulator and (F) flags
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

impl Registers {
    /// true if bit 7 of flags is 1
    pub fn zero_flag(&self) -> bool {
        (self.af.low.get() & 0x80) == 0x80
    }

    /// true if bit 6 of flags is 1
    pub fn add_sub_flag(&self) -> bool {
        (self.af.low.get() & 0x40) == 0x40
    }

    /// true if bit 5 of flags is 1
    pub fn half_carry_flag(&self) -> bool {
        (self.af.low.get() & 0x20) == 0x20
    }

    /// true if bit 4 of flags is 1
    pub fn carry_flag(&self) -> bool {
        (self.af.low.get() & 0x10) == 0x10
    }

    /// sets bit 7 of flags to the specified bool value
    pub fn set_zero_flag(&mut self, flag: bool) {
        let f = self.af.low.get();

        match flag {
            true => self.af.low.set(f | 0x80),
            false => self.af.low.set(f & 0x7F),
        };
    }

    /// sets bit 6 of flags to the specified bool value
    pub fn set_add_sub_flag(&mut self, flag: bool) {
        let f = {self.af.low.get()};

        match flag {
            true => self.af.low.set(f | 0x40),
            false => self.af.low.set(f & 0xBF),
        };
    }

    /// sets bit 5 of flags to the specified bool value
    pub fn set_half_carry_flag(&mut self, flag: bool) {
        let f = {self.af.low.get()};

        match flag {
            true => self.af.low.set(f | 0x20),
            false => self.af.low.set(f & 0xDF),
        };
    }

    /// sets bit 4 of flags to the specified bool value
    pub fn set_carry_flag(&mut self, flag: bool) {
        let f = {self.af.low.get()};

        match flag {
            true => self.af.low.set(f | 0x10),
            false => self.af.low.set(f & 0xEF),
        };
    }

}
