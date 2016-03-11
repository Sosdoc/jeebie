use jeebie::core::cpu::CPU;

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
    Address(u16), Immediate, Value8(u8)
}

#[derive(Clone, Copy)]
pub enum Register16 {
    AF, BC, DE, HL, SP, PC,
    Immediate16, Value16(u16)
}

/// The main registers in a gameboy CPU.
/// Registers are 16 bit wide and can be accessed as L(low) and H(high).
/// Register AF is used for (A) accumulator and (F) flags.
/// This struct also offers convenience methods to get and set individual flags.
#[derive(Debug)]
pub struct Registers {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,

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

    /// Returns true if the selected flag is set.
    pub fn is_set(&self, flag: Flags) -> bool {
        let flag_value = flag as u8;
        (self.f & flag_value) == flag_value
    }

    fn combine_as_u16(high: u8, low: u8) -> u16 {
        ((high as u16) << 8) & (low as u16)
    }
}

impl CPU {
    pub fn get8(&mut self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.reg.a,
            Register8::B => self.reg.b,
            Register8::C => self.reg.c,
            Register8::D => self.reg.d,
            Register8::E => self.reg.e,
            Register8::H => self.reg.h,
            Register8::L => self.reg.l,
            Register8::RegisterAddress(r) => {
                let addr = self.get16(r);
                self.mem.read_b(addr)
            },
            Register8::Address(addr) => self.mem.read_b(addr),
            Register8::Immediate => self.get_immediate8(),
            Register8::Value8(n) => n,
        }
    }

    pub fn set8(&mut self, reg: Register8, value: u8) {
        match reg {
            Register8::A => self.reg.a = value,
            Register8::B => self.reg.b = value,
            Register8::C => self.reg.c = value,
            Register8::D => self.reg.d = value,
            Register8::E => self.reg.e = value,
            Register8::H => self.reg.h = value,
            Register8::L => self.reg.l = value,
            Register8::RegisterAddress(r) => {
                let addr = self.get16(r);
                self.mem.write_b(addr, value);
            },
            Register8::Address(addr) => self.mem.write_b(addr, value),
            _ => {},
        };
    }

    pub fn get16(&mut self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => Registers::combine_as_u16(self.reg.a, self.reg.f),
            Register16::BC => Registers::combine_as_u16(self.reg.b, self.reg.c),
            Register16::DE => Registers::combine_as_u16(self.reg.d, self.reg.e),
            Register16::HL => Registers::combine_as_u16(self.reg.h, self.reg.l),
            Register16::SP => self.reg.sp,
            Register16::PC => self.reg.pc,
            Register16::Immediate16 => self.get_immediate16(),
            Register16::Value16(n) => n,
        }
    }

    pub fn set16(&mut self, reg: Register16, value: u16) {
        match reg {
            Register16::AF => { self.reg.a = (value >> 8) as u8 ; self.reg.f = value as u8; },
            Register16::BC => { self.reg.b = (value >> 8) as u8 ; self.reg.c = value as u8; },
            Register16::DE => { self.reg.d = (value >> 8) as u8 ; self.reg.e = value as u8; },
            Register16::HL => { self.reg.h = (value >> 8) as u8 ; self.reg.l = value as u8; },
            Register16::SP => { self.reg.sp = value },
            Register16::PC => { self.reg.pc = value },
            _ => {},
        };
    }
}
