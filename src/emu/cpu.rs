#[derive(Debug)]
struct Clock {
    m : u16,
    t : u16
}

#[derive(Debug)]
struct Registers {
    // these registers are split into 8-bits for H/L
    a : u8, f : u8, // (a) accumulator and (f) flag registers
    b : u8, c : u8,
    d : u8, e : u8,
    h : u8, l : u8,

    // 16-bit
    pc : u16, // program counter
    sp : u16, // stack pointer

    // last instruction's timings (m/t)
    m : u16, t : u16
}

#[derive(Debug)]
pub struct CPU {
    c : Clock,
    r : Registers
}

impl CPU {
    pub fn new() -> CPU {
        let c = Clock{m:0, t:0};
        let r = Registers{a:0, b:0, c:0, d:0, e:0, h:0, l:0, f:0, pc:0, sp:0, m:0, t:0};
        CPU{c:c, r:r}
    }

    /// Returns bit 4 of the flag register (carry)
    pub fn carry_flag(&self) -> u8 {
        (self.r.f & 0x10) >> 4
    }

    /// add register specified to a
    pub fn add_r(&mut self, r : u8) {
        let overflow = self.r.a as u16 + r as u16 > 0xFF;
        self.r.a = self.r.a.wrapping_add(r);
        self.r.f = 0;

        // check overflow
        self.r.f |= if overflow {0x10} else {0};
        // check zero
        self.r.f |= if self.r.a == 0 {0x80} else {0};

        // ticks
        self.r.m = 1;
        self.r.t = 4;
    }

    /// compare register r to a
    pub fn cmp_r(&mut self, r : u8) {
        // copy of A to check underflow
        let mut tmp = self.r.a as i16;
        tmp -= r as i16;

        self.r.f |= 0x40;   //subtraction flag
        self.r.f |= if self.r.a == 0 {0x80} else {0}; // zero flag
        self.r.f |= if tmp < 0 {0x10} else {0}; // carry flag

        // ticks
        self.r.m = 1;
        self.r.t = 4;
    }

    /// NO-OP, only updates clock
    pub fn nop(&mut self) {
        // ticks
        self.r.m = 1;
        self.r.t = 4;
    }

}




#[test]
fn cpu_new_reset() {
    let cpu = CPU::new();

    assert_eq!(cpu.c.m, 0);
    assert_eq!(cpu.r.pc, 0);
}

#[test]
fn carry_flag() {
    // extract bit 4
    let mut cpu = CPU::new();

    cpu.r.f = 0x10;
    let mut carry = cpu.carry_flag();
    assert_eq!(carry, 1);

    cpu.r.f = 0xF0;
    carry = cpu.carry_flag();
    assert_eq!(carry, 1);

    cpu.r.f = 0x80;
    carry = cpu.carry_flag();
    assert_eq!(carry, 0);
}

#[test]
fn add_to_acc() {
    let mut cpu = CPU::new();

    cpu.r.a = 200;
    cpu.r.e = 75;

    let e = cpu.r.e;

    cpu.add_r(e);
    assert_eq!(cpu.r.a, 19);
    assert_eq!(cpu.r.f, 0x10);
}


#[test]
fn compare_to_acc() {
    let mut cpu = CPU::new();

    cpu.r.a = 10;
    cpu.r.e = 10;

    let e = cpu.r.e;

    cpu.cmp_r(e);
    assert_eq!(cpu.r.a, 10);
    assert_eq!(cpu.r.f, 0x40);
}
