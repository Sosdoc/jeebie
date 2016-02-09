#[derive(Debug)]
pub struct Register8 {
    value: u8,
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

#[derive(Debug)]
pub struct Register16 {
    pub low: Register8,
    pub high: Register8,
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
        // self.value.wrapping_add(1);
    }

    pub fn get_low(&self) -> u8 {
        self.low.get()
    }

    pub fn get_high(&self) -> u8 {
        self.high.get()
    }

    pub fn set_low(&mut self, v: u8) {
        self.low.set(v)
    }

    pub fn set_high(&mut self, v: u8) {
        self.high.set(v)
    }

    pub fn get(&self) -> u16 {
        ((self.high.get() as u16) << 8) | self.low.get() as u16
    }
}


#[derive(Debug)]
pub struct Registers {
    // registers are 16 bit wide and can be accessed as L(low) and H(high)
    // register AF is used for (A) accumulator and (F) flags
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
        (self.af.get_low() & 0x80) == 0x80
    }

    /// true if bit 6 of flags is 1
    pub fn add_sub_flag(&self) -> bool {
        (self.af.get_low() & 0x40) == 0x40
    }

    /// true if bit 5 of flags is 1
    pub fn half_carry_flag(&self) -> bool {
        (self.af.get_low() & 0x20) == 0x20
    }

    /// true if bit 4 of flags is 1
    pub fn carry_flag(&self) -> bool {
        (self.af.get_low() & 0x10) == 0x10
    }

    /// sets bit 7 of flags to 1
    pub fn set_zero_flag(&mut self) {
        let f = self.af.get_low();
        self.af.set_low(f | 0x80);
    }

    // /// sets bit 6 of flags to 1
    // pub fn set_add_sub_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() | 0x40);
    // }
    //
    // /// sets bit 5 of flags to 1
    // pub fn set_half_carry_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() | 0x20);
    // }
    //
    // /// sets bit 4 of flags to 1
    // pub fn set_carry_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() | 0x10);
    // }
    //
    // /// clears bit 7 of flags
    // pub fn clear_zero_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() & 0x7F);
    // }
    //
    // /// clears bit 6 of flags
    // pub fn clear_add_sub_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() & 0xBF);
    // }
    //
    // /// clears bit 5 of flags
    // pub fn clear_half_carry_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() & 0xDF);
    // }
    //
    // /// clears bit 4 of flags
    // pub fn clear_carry_flag(&mut self) {
    //     self.af.set_low(self.af.get_low() & 0xEF);
    // }
}
