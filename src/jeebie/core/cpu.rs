use jeebie::memory::MMU;
use jeebie::registers::*;

use jeebie::opcodes::{ CB_OPCODE_TABLE, OPCODE_TABLE };
use jeebie::timings::{ CB_TIMING_TABLE, TIMING_TABLE };
// use jeebie::disasm::metadata::{ CB_DISASM_TABLE, DISASM_TABLE };
use jeebie::utils::{ is_set, swap_bit, set_bit, reset_bit };

#[derive(Debug)]
pub struct CPU<'a> {
    pub reg: Registers,
    pub mem: &'a mut MMU,
    pub interrupts_enabled: bool,

    // amount of machine cycles (as reported in timing tables) elapsed.
    cycles: u64,
}

impl<'a> CPU<'a> {
    pub fn new(mmu: &'a mut MMU) -> CPU<'a> {
        let r = Registers::new();
        CPU { reg: r, mem: mmu, cycles: 0, interrupts_enabled: false}
    }

    /// Executes one instruction, updating cycles and PC register accordingly.
    /// Returns the number of elapsed machine cycles.
    pub fn exec(&mut self) -> u32 {
        // fetch
        let opcode = self.mem.read_b(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);

        let instr_timing = match opcode {
            0xCB => {
                // 2-byte opcodes are prefixed with 0xCB
                let second_byte = self.mem.read_b(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);

                // println!("{} - {} ", self.reg.pc - 1, CB_DISASM_TABLE[second_byte as usize]);
                CB_OPCODE_TABLE[second_byte as usize](self);
                CB_TIMING_TABLE[second_byte as usize]
            },
            _ => {
                // println!("{} - {}", self.reg.pc - 1, DISASM_TABLE[opcode as usize]);
                OPCODE_TABLE[opcode as usize](self);
                TIMING_TABLE[opcode as usize]
            }
        };

        self.cycles = self.cycles.wrapping_add(instr_timing as u64);
        instr_timing as u32
    }

    /// Executes instructions until a single frame is produced.
    /// A frame is 144 scanlines, plus 10 vertical blanks, and scanlines are rendered every 456 machine cycles.
    /// This means one frame is ready every 154 * 456 = 70224 machine cycles.
    pub fn exec_one_frame(&mut self) -> &[(u8, u8, u8)]{
        // TODO: handle overflows
        let target = self.cycles + 70224;

        while self.cycles < target {
            let cycles = self.exec();
            self.mem.gpu.emulate(cycles);
        }

        // frame is ready
        self.mem.gpu.get_framebuffer()
    }

    fn combine_as_u16(high: u8, low: u8) -> u16 {
        // TODO: write tests for these things, when they fail they make you feel stupid.
        ((high as u16) << 8) | (low as u16)
    }

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
            Register16::AF => CPU::combine_as_u16(self.reg.a, self.reg.f),
            Register16::BC => CPU::combine_as_u16(self.reg.b, self.reg.c),
            Register16::DE => CPU::combine_as_u16(self.reg.d, self.reg.e),
            Register16::HL => CPU::combine_as_u16(self.reg.h, self.reg.l),
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
            Register16::SP => { self.reg.sp = value; },
            Register16::PC => { self.reg.pc = value; },
            _ => {},
        };
    }

    // Checks the b bit of a register.
    // Zero flag is set if the bit is 0
    // Sub reset, HC set
    pub fn bit_check(&mut self, b: usize, reg: Register8) {
        let set = is_set(self.get8(reg), b);
        if !set { self.reg.set_flag(Flags::Zero) } else { self.reg.clear_flag(Flags::Zero) };
        self.reg.clear_flag(Flags::Sub);
        self.reg.set_flag(Flags::HalfCarry);
    }

    // Swaps the bit b in the specified register.
    // No flags are affected.
    pub fn bit_swap(&mut self, b: usize, reg: Register8) {
        let data = swap_bit(self.get8(reg), b);
        self.set8(reg, data);
    }

    // Set the bit b in the specified register.
    // No flags are affected.
    pub fn bit_set(&mut self, b: usize, reg: Register8) {
        let data = set_bit(self.get8(reg), b);
        self.set8(reg, data);
    }

    // Reset the bit b in the specified register.
    // No flags are affected.
    pub fn bit_reset(&mut self, b: usize, reg: Register8) {
        let data = reset_bit(self.get8(reg), b);
        self.set8(reg, data);
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

    pub fn jump(&mut self, addr: u16) {
        self.reg.pc = addr;
    }

    pub fn jump_flag(&mut self, flag: Flags, reg: Register16) {
        if self.reg.is_set(flag) {
            let addr = self.get16(reg);
            self.jump(addr);
        }
    }

    pub fn jump_not_flag(&mut self, flag: Flags, reg: Register16) {
        if !self.reg.is_set(flag) {
            let addr = self.get16(reg);
            self.jump(addr);
        }
    }

    /// Performs a return (RET) instruction if the specified flag is set.
    pub fn return_flag(&mut self, flag: Flags) {
        if self.reg.is_set(flag) {
            self.pop_stack(Register16::PC);
        }
    }

    /// Performs a return (RET) instruction if the specified flag is not set.
    pub fn return_not_flag(&mut self, flag: Flags) {
        if !self.reg.is_set(flag) {
            self.pop_stack(Register16::PC);
        }
    }

    // A restart (RST) will push the current address on the stack and jump to the provided address
    // addresses are encoded in the opcode, with a total of 8 possible ones.
    pub fn restart(&mut self, addr: u16) {
        self.push_stack(Register16::PC);
        self.jump(addr);
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

        self.reg.sp = self.reg.sp.wrapping_sub(1);
        let addr = self.reg.sp;
        let low = (value & 0x00FF) as u8;
        self.mem.write_b(addr, low);

        self.reg.sp = self.reg.sp.wrapping_sub(1);
        let addr = self.reg.sp;
        let high = ((value >> 8) & 0x00FF) as u8;
        self.mem.write_b(addr, high);
    }

    /// Pops a 16-bit value from the stack, MSB first, setting the value to the specified register.
    pub fn pop_stack(&mut self, dest: Register16) {
        let high = self.mem.read_b(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);

        let low = self.mem.read_b(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);

        let result = ((high as u16) << 8) & (low as u16);
        self.set16(dest, result);
    }

    /// Retrieves an immediate 8-bit value.
    /// Immediates are retrieved by reading at the address in the PC register.
    pub fn get_immediate8(&mut self) -> u8 {
        let value = self.mem.read_b(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        value
    }

    /// Retrieves an immediate 16-bit value.
    /// 16-bit immediates are read as two 8-bit immediates, the first being the LSB.
    pub fn get_immediate16(&mut self) -> u16 {
        let low = self.get_immediate8() as u16;
        let high = self.get_immediate8() as u16;
        (high << 8) | low
    }
}
