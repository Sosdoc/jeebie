use jeebie::memory::MMU;
use jeebie::registers::*;

#[derive(Debug)]
pub struct CPU {
    pub reg: Registers,
    pub mem: MMU,
}

impl CPU {
    pub fn new() -> CPU {

        let r = Registers {
            af: Register16::new(0),
            bc: Register16::new(0),
            de: Register16::new(0),
            hl: Register16::new(0),
            pc: Register16::new(0),
            sp: Register16::new(0),
        };

        let mmu = MMU::new();

        CPU { reg: r, mem: mmu }
    }

    /// executes the instruction
    pub fn dispatch(&mut self, opcode: u8) {}

    /// reads instructions and executes them
    pub fn fetch_and_exec() {
        let mut cpu = CPU::new();

        loop {
            // fetch
            let opcode = cpu.mem.read_b(cpu.reg.pc.get());
            // execute
            cpu.dispatch(opcode);
            // increase PC
            cpu.reg.pc.add(1);
            // TODO: compute clock timings
        }
    }

    pub fn LD_nn_n(reg1: &mut Register8, value: u8) {
        reg1.set(value);
    }

    // Swaps low and high nibble of an 8 bit value and sets flags.
    // Returns the result of the swap operation.
    pub fn compute_swap(&mut self, value: u8) -> u8 {
        let result = (value << 4) | (value >> 4);

        self.reg.clear_all_flags();
        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        result
    }

    // Computes the flags and result for a 16-bit ADD instruction.
    pub fn compute_add16(&mut self, rhs: u16) {
        let lhs = self.reg.hl.get();
        let result = lhs.wrapping_add(rhs);

        // flag zero is not affected, sub is reset
        self.reg.clear_flag(Flags::Sub);

        // checked add returns None if add overflows (carry from bit 15)
        if let None = lhs.checked_add(rhs) {
            self.reg.set_flag(Flags::Carry);
        }

        // halfcarry is set if bit 11 has carry
        // sum the 12 LSb and check bit 12
        let low_result = (rhs & 0xFFF) + (lhs & 0xFFF);
        if (low_result >> 12) == 1 {
            self.reg.set_flag(Flags::HalfCarry);
        }

        self.reg.hl.set(result);
    }

    // Computes the flags and result for an 8-bit ADD instruction.
    pub fn compute_add(&mut self, lhs: u8, rhs: u8) {
        let result = lhs.wrapping_add(rhs);

        self.reg.clear_all_flags();

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        // checked add returns None if add overflows
        if let None = lhs.checked_add(rhs) {
            self.reg.set_flag(Flags::Carry);
        }

        let low_result = (rhs & 0x0F) + (lhs & 0x0F);
        // check if bit 4 is set
        if (low_result >> 4) == 1 {
            self.reg.set_flag(Flags::HalfCarry);
        }

        self.reg.af.high.set(result);
    }

    // Computes the flags and result for a SUB instruction.
    pub fn compute_sub(&mut self, lhs: u8, rhs: u8) {
        let result = lhs.wrapping_sub(rhs);

        self.reg.clear_all_flags();

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.reg.set_flag(Flags::Sub);

        // checked sub returns None if sub borrows (carry flag)
        if let None = lhs.checked_sub(rhs) {
            self.reg.set_flag(Flags::Carry);
        }

        // same as carry flag but values limited to their 4 low bits (half carry flag)
        if let None = (lhs & 0xF).checked_sub(rhs & 0xF) {
            self.reg.set_flag(Flags::HalfCarry);
        }

        self.reg.af.high.set(result);
    }

    // Computes the flags for a CP instruction.
    // this has the same effect on flags as a SUB, but the result is discarded.
    pub fn compute_cp(&mut self, rhs: u8) {
        let lhs = self.reg.af.high.get();
        self.reg.clear_all_flags();
        self.reg.set_flag(Flags::Sub);

        // Same as A - n
        if lhs == rhs {
            self.reg.set_flag(Flags::Zero);
        }

        // if A < n
        if lhs < rhs {
            self.reg.set_flag(Flags::Carry);
        }

        // if 4 low bits of A < 4 low bits of n
        if (lhs & 0xF) < (rhs & 0xF) {
            self.reg.set_flag(Flags::HalfCarry);
        }
    }

    // Computes the flags and result for an AND instruction.
    // lhs is always the register A
    pub fn compute_and(&mut self, rhs: u8) {
        let result = self.reg.af.high.get() & rhs;

        // HC flag is always set, other flags always cleared except zero.
        self.reg.clear_all_flags();
        self.reg.set_flag(Flags::HalfCarry);

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.reg.af.high.set(result);
    }

    // Computes the flags and result for an OR instruction.
    // lhs is always the register A
    pub fn compute_or(&mut self, rhs: u8) {
        let result = self.reg.af.high.get() | rhs;

        // all flags are cleared except zero.
        self.reg.clear_all_flags();

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.reg.af.high.set(result);
    }

    // Computes the flags and result for an XOR instruction.
    // lhs is always the register A
    pub fn compute_xor(&mut self, rhs: u8) {
        let result = self.reg.af.high.get() ^ rhs;

        // all flags are cleared except zero.
        self.reg.clear_all_flags();

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.reg.af.high.set(result);
    }

    /// Computes flags after an INC instruction based on the final value.
    /// Carry flag is left untouched, so no clearing all of them.
    pub fn compute_inc_flags(&mut self, increased_value: u8) {
        self.reg.clear_flag(Flags::Sub);

        if increased_value == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        // TODO: this condition sucks
        // set HC if bit 0-3 were 1 before adding
        if (increased_value.wrapping_sub(1) & 0x0F) == 0x0F {
            self.reg.set_flag(Flags::HalfCarry);
        }
    }

    /// Computes flags after a DEC instruction based on the final value.
    /// Carry flag is left untouched, so no clearing all of them.
    pub fn compute_dec_flags(&mut self, decreased_value: u8) {
        self.reg.set_flag(Flags::Sub);

        if decreased_value == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        // HC is set if bit 4 was borrowed
        // this happens only when bit 0-3 are all 0 before decrementing.
        if (decreased_value.wrapping_add(1) & 0x0F) == 0x00 {
            self.reg.set_flag(Flags::HalfCarry);
        }
    }

    /// Pushes on the stack a 16-bit register value,
    /// it decrements SP and pushes the LSB before the MSB.
    pub fn push_stack(&mut self, reg: u16) {
        self.reg.sp.sub(1);
        let addr = self.reg.sp.get();
        let low = (reg & 0x00FF) as u8;
        self.mem.write_b(addr, low);

        self.reg.sp.sub(1);
        let addr = self.reg.sp.get();
        let high = ((reg >> 8) & 0x00FF) as u8;
        self.mem.write_b(addr, high);
    }

    /// Pops a 16-bit value from the stack, MSB first, returning the u16 value.
    pub fn pop_stack(&mut self) -> u16 {
        let high = self.mem.read_b(self.reg.sp.get());
        self.reg.sp.add(1);

        let low = self.mem.read_b(self.reg.sp.get());
        self.reg.sp.add(1);

        ((high as u16) << 8) & (low as u16)
    }

    /// Retrieves an immediate 8-bit value.
    /// Immediates are retrieved by reading at the address in the PC register.
    pub fn get_immediate8(&mut self) -> u8 {
        let value = self.mem.read_b(self.reg.pc.get());
        self.reg.pc.add(1);

        value
    }

    /// Retrieves an immediate 16-bit value.
    /// 16-bit immediates are read as two 8-bit immediates, the first being the LSB.
    pub fn get_immediate16(&mut self) -> u16 {
        let low = self.get_immediate8();
        let high = self.get_immediate8();

        (high as u16) & (low as u16)
    }
}
