use jeebie::memory::MMU;
use jeebie::registers::*;

#[derive(Debug)]
pub struct CPU {
    pub reg: Registers,
    pub mem: MMU,
}

impl CPU {
    pub fn new() -> CPU {
        let r = Registers::new();
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
            let opcode = cpu.mem.read_b(cpu.reg.pc);
            // execute
            cpu.dispatch(opcode);
            // increase PC
            cpu.reg.pc.wrapping_add(1);
            // TODO: compute clock timings
        }
    }

    // Swaps low and high nibble of an 8 bit value and sets flags.
    // Returns the result of the swap operation.
    pub fn compute_swap(&mut self, reg: Register8) {
        let value = self.get8(reg);
        let result = (value << 4) | (value >> 4);

        self.reg.clear_all_flags();
        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.set8(reg, result);
    }

    pub fn load_rr(&mut self, reg1: Register8, reg2: Register8) {
        let value = self.get8(reg2);
        self.set8(reg1, value);
    }

    pub fn load_rr16(&mut self, reg1: Register16, reg2: Register16) {
        let value = self.get16(reg2);
        self.set16(reg1, value);
    }

    // Computes the flags and result for a 16-bit ADD instruction.
    // The result is put in the specified `reg1`.
    pub fn compute_add16(&mut self, reg1: Register16, reg2: Register16) {
        let lhs = self.get16(reg1);
        let rhs = self.get16(reg2);
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

        self.set16(reg1, result);
    }

    // Computes the flags and result for an 8-bit ADD instruction.
    pub fn compute_add(&mut self, reg1: Register8, reg2: Register8) {
        let lhs = self.get8(reg1);
        let rhs = self.get8(reg2);
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

        self.set8(Register8::A, result);
    }

    pub fn compute_adc(&mut self, reg1: Register8, reg2: Register8) {
        let carry = if self.reg.is_set(Flags::Carry) { 1 } else { 0 };
        let reg2 = Register8::Value8(self.get8(reg2).wrapping_add(carry));
        self.compute_add(reg1, reg2);
    }

    // Computes the flags and result for a SUB instruction.
    // Left hand operator is always register A
    pub fn compute_sub(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);
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

        self.set8(Register8::A, result);
    }

    pub fn compute_sbc(&mut self, reg: Register8) {
        let carry = if self.reg.is_set(Flags::Carry) { 1 } else { 0 };
        let reg = Register8::Value8(self.get8(reg).wrapping_sub(carry));
        self.compute_sub(reg);
    }

    // Computes the flags for a CP instruction.
    // this has the same effect on flags as a SUB, but the result is discarded.
    pub fn compute_cp(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);

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
    pub fn compute_and(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);
        let result = lhs & rhs;

        // HC flag is always set, other flags always cleared except zero.
        self.reg.clear_all_flags();
        self.reg.set_flag(Flags::HalfCarry);

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.set8(Register8::A, result);
    }

    // Computes the flags and result for an OR instruction.
    // lhs is always the register A
    pub fn compute_or(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);
        let result = lhs | rhs;

        // all flags are cleared except zero.
        self.reg.clear_all_flags();

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.set8(Register8::A, result);
    }

    // Computes the flags and result for an XOR instruction.
    // lhs is always the register A
    pub fn compute_xor(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);
        let result = lhs ^ rhs;

        // all flags are cleared except zero.
        self.reg.clear_all_flags();

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        self.set8(Register8::A, result);
    }

    /// Computes an INC instruction.
    /// Carry flag is left untouched, so they are not cleared.
    pub fn compute_inc(&mut self, reg: Register8) {
        let result = self.get8(reg).wrapping_add(1);
        self.reg.clear_flag(Flags::Sub);

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        // set HC if bit 0-3 were 1 before adding
        if (result.wrapping_sub(1) & 0x0F) == 0x0F {
            self.reg.set_flag(Flags::HalfCarry);
        }

        self.set8(reg, result);
    }

    /// Computes an INC instruction on a 16 bit register.
    /// Flags are not affected.
    pub fn compute_inc16(&mut self, reg: Register16) {
        let value = self.get16(reg).wrapping_add(1);
        self.set16(reg, value);
    }

    /// Computes an INC instruction on a 16 bit register.
    /// Flags are not affected.
    pub fn compute_dec16(&mut self, reg: Register16) {
        let value = self.get16(reg).wrapping_sub(1);
        self.set16(reg, value);
    }

    /// Computes flags after a DEC instruction based on the final value.
    /// Carry flag is left untouched, so no clearing all of them.
    pub fn compute_dec(&mut self, reg: Register8) {
        let result = self.get8(reg).wrapping_sub(1);
        self.reg.set_flag(Flags::Sub);

        if result == 0 {
            self.reg.set_flag(Flags::Zero);
        }

        // HC is set if bit 4 was borrowed
        // this happens only when bit 0-3 are all 0 before decrementing.
        if (result.wrapping_add(1) & 0x0F) == 0x00 {
            self.reg.set_flag(Flags::HalfCarry);
        }

        self.set8(reg, result);
    }

    /// Pushes on the stack a 16-bit register value,
    /// it decrements SP and pushes the LSB before the MSB.
    pub fn push_stack(&mut self, reg: Register16) {
        let value = self.get16(reg);

        self.reg.sp.wrapping_sub(1);
        let addr = self.reg.sp;
        let low = (value & 0x00FF) as u8;
        self.mem.write_b(addr, low);

        self.reg.sp.wrapping_sub(1);
        let addr = self.reg.sp;
        let high = ((value >> 8) & 0x00FF) as u8;
        self.mem.write_b(addr, high);
    }

    /// Pops a 16-bit value from the stack, MSB first, returning the u16 value.
    pub fn pop_stack(&mut self, dest: Register16) {
        let high = self.mem.read_b(self.reg.sp);
        self.reg.sp.wrapping_add(1);

        let low = self.mem.read_b(self.reg.sp);
        self.reg.sp.wrapping_add(1);

        let result = ((high as u16) << 8) & (low as u16);
        self.set16(dest, result);
    }

    /// Retrieves an immediate 8-bit value.
    /// Immediates are retrieved by reading at the address in the PC register.
    pub fn get_immediate8(&mut self) -> u8 {
        let value = self.mem.read_b(self.reg.pc);
        self.reg.pc.wrapping_add(1);

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
