use jeebie::memory::MMU;
use jeebie::core::registers::*;
use jeebie::core::registers::Flags::*;

use jeebie::instr::opcodes::{ CB_OPCODE_TABLE, OPCODE_TABLE };
use jeebie::utils::{ is_set, swap_bit, set_bit, reset_bit, combine_as_u16 };

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
        // disasm string
        // println!("{}", get_instruction_str(self));

        // fetch
        let opcode = self.mem.read_b(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);

        let instr_timing = match opcode {
            0xCB => {
                // 2-byte opcodes are prefixed with 0xCB
                let second_byte = self.mem.read_b(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);

                CB_OPCODE_TABLE[second_byte as usize](self)
            },
            _ => {
                OPCODE_TABLE[opcode as usize](self)
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
            Register16::AF => combine_as_u16(self.reg.a, self.reg.f),
            Register16::BC => combine_as_u16(self.reg.b, self.reg.c),
            Register16::DE => combine_as_u16(self.reg.d, self.reg.e),
            Register16::HL => combine_as_u16(self.reg.h, self.reg.l),
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
        let is_zero = !is_set(self.get8(reg), b);
        self.reg.set_or_clear(Zero, is_zero);
        self.reg.clear_flag(Sub);
        self.reg.set_flag(HalfCarry);
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
        self.reg.set_or_clear(Zero, result == 0);
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

    // Rotate the register left, old bit 7 goes to carry flag (RLC).
    pub fn rotate_left_carry(&mut self, reg: Register8) {
        self.reg.clear_all_flags();

        let value = self.get8(reg);
        let result = value.rotate_left(1);
        self.set8(reg, result);

        self.reg.set_or_clear(Carry, is_set(value, 7));
        self.reg.set_or_clear(Zero, result == 0);
    }

    // Rotate the register left through carry flag (RL).
    pub fn rotate_left(&mut self, reg: Register8) {
        let carry = if self.reg.is_set(Carry) { 1 } else { 0 };
        self.reg.clear_all_flags();

        let value = self.get8(reg);
        let result = (value << 1) | carry;
        self.set8(reg, result);

        self.reg.set_or_clear(Carry, is_set(value, 7));
        self.reg.set_or_clear(Zero, result == 0);
    }

    // Rotate the register right, old bit 0 goes to carry flag (RRC).
    pub fn rotate_right_carry(&mut self, reg: Register8) {
        self.reg.clear_all_flags();

        let value = self.get8(reg);
        let result = value.rotate_right(1);
        self.set8(reg, result);

        self.reg.set_or_clear(Carry, is_set(value, 0));
        self.reg.set_or_clear(Zero, result == 0);
    }

    // Rotate the register right through carry flag (RR).
    pub fn rotate_right(&mut self, reg: Register8) {
        let carry = if self.reg.is_set(Carry) { 1 } else { 0 };
        self.reg.clear_all_flags();

        let value = self.get8(reg);
        let result = (value >> 1) | carry;
        self.set8(reg, result);

        self.reg.set_or_clear(Carry, is_set(value, 0));
        self.reg.set_or_clear(Zero, result == 0);
    }

    // Computes the flags and result for a 16-bit ADD instruction.
    // The result is put in the specified `reg1`.
    // No flags are affected.
    pub fn compute_add16(&mut self, reg1: Register16, reg2: Register16) {
        let lhs = self.get16(reg1);
        let rhs = self.get16(reg2);
        let result = lhs.wrapping_add(rhs);

        self.reg.clear_flag(Sub);
        // checked add returns None if add overflows
        self.reg.set_or_clear(Carry, lhs.checked_add(rhs).is_none());
        // check if bit 4 was set (on the result of adding 4 low bits only)
        self.reg.set_or_clear(HalfCarry, (lhs as u8).checked_add(rhs as u8).is_none());

        self.set16(reg1, result);
    }

    // Computes the flags and result for an 8-bit ADD instruction.
    pub fn compute_add(&mut self, reg1: Register8, reg2: Register8) {
        let lhs = self.get8(reg1);
        let rhs = self.get8(reg2);
        let result = lhs.wrapping_add(rhs);
        let low_result = (rhs & 0x0F) + (lhs & 0x0F);

        self.reg.clear_all_flags();
        self.reg.set_or_clear(Zero, result == 0);
        // checked add returns None if add overflows
        self.reg.set_or_clear(Carry, lhs.checked_add(rhs).is_none());
        // check if bit 4 was set (on the result of adding 4 low bits only)
        self.reg.set_or_clear(HalfCarry, is_set(low_result, 4));

        self.set8(Register8::A, result);
    }

    pub fn compute_adc(&mut self, reg1: Register8, reg2: Register8) {
        let carry = if self.reg.is_set(Carry) { 1 } else { 0 };
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
        self.reg.set_flag(Sub);
        self.reg.set_or_clear(Zero, result == 0);
        // checked sub returns None if sub borrows (carry flag)
        self.reg.set_or_clear(Carry, lhs.checked_sub(rhs).is_none());
        // same as carry flag but values limited to their 4 low bits (half carry flag)
        self.reg.set_or_clear(HalfCarry, (lhs & 0xF).checked_sub(rhs & 0xF).is_none());

        self.set8(Register8::A, result);
    }

    pub fn compute_sbc(&mut self, reg: Register8) {
        let carry = if self.reg.is_set(Carry) { 1 } else { 0 };
        let reg = Register8::Value8(self.get8(reg).wrapping_sub(carry));
        self.compute_sub(reg);
    }

    // Computes the flags for a CP instruction.
    // this has the same effect on flags as a SUB, but the result is discarded.
    pub fn compute_cp(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);

        self.reg.clear_all_flags();
        self.reg.set_flag(Sub);

        // Same as A - n
        self.reg.set_or_clear(Zero, lhs == rhs);
        // if A < n
        self.reg.set_or_clear(Carry, lhs < rhs);
        // if 4 low bits of A < 4 low bits of n
        self.reg.set_or_clear(HalfCarry, (lhs & 0xF) < (rhs & 0xF));
    }

    // Computes the flags and result for an AND instruction.
    // lhs is always the register A
    pub fn compute_and(&mut self, reg: Register8) {
        let lhs = self.get8(Register8::A);
        let rhs = self.get8(reg);
        let result = lhs & rhs;

        // HC flag is always set, other flags always cleared except zero.
        self.reg.clear_all_flags();
        self.reg.set_flag(HalfCarry);
        self.reg.set_or_clear(Zero, result == 0);

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
        self.reg.set_or_clear(Zero, result == 0);

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
        self.reg.set_or_clear(Zero, result == 0);

        self.set8(Register8::A, result);
    }

    /// Computes a shift of 1 in the specified direction for the register.
    /// A 0 bit will be inserted on the other end and the bit shifted out will be stored in the
    /// Carry flag.
    pub fn compute_shift(&mut self, left: bool, reg: Register8) {
        // check bit 7 if shifting left, otherwise bit 0
        let bit_idx = if left {7} else {0};
        let data = self.get8(reg);

        self.reg.clear_all_flags();
        self.reg.set_or_clear(Carry, is_set(data, bit_idx));

        let result = if left {
            data << 1
        } else {
            data >> 1
        };

        self.reg.set_or_clear(Zero, result == 0);
        self.set8(reg, result);
    }

    /// Computes an INC instruction.
    /// Carry flag is left untouched, so they are not cleared.
    pub fn compute_inc(&mut self, reg: Register8) {
        let result = self.get8(reg).wrapping_add(1);
        self.reg.clear_flag(Sub);
        self.reg.set_or_clear(Zero, result == 0);
        // set HC if bit 0-3 were 1 before adding
        self.reg.set_or_clear(HalfCarry, (result.wrapping_sub(1) & 0xF) == 0xF);

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
        self.reg.set_flag(Sub);
        self.reg.set_or_clear(Zero, result == 0);
        // HC is set if bit 4 was borrowed
        // this happens only when bit 0-3 are all 0 before decrementing.
        self.reg.set_or_clear(HalfCarry, (result.wrapping_add(1) & 0x0F) == 0x00);

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

        self.set16(dest, combine_as_u16(high, low));
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
        let low = self.get_immediate8();
        let high = self.get_immediate8();
        combine_as_u16(high, low)
    }
}
