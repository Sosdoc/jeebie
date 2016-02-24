use std::rc::Rc;
use std::cell::RefCell;

use gbe::memory::MMU;
use gbe::registers::*;

#[derive(Debug)]
pub struct CPU {
    pub reg: Registers,
    pub mem: Rc<RefCell<MMU>>,
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

        // TODO: this should be created only once and cloned for every struct
        let mmu = Rc::new(RefCell::new(MMU::new()));

        CPU { reg: r, mem: mmu }
    }

    /// executes the instruction
    pub fn dispatch(&mut self, opcode: u8) {}

    /// reads instructions and executes them
    pub fn fetch_and_exec() {
        let mut cpu = CPU::new();

        loop {
            // fetch
            // the block is needed so that the borrow of cpu.memory ends
            // before dispatch, which will mutably borrow cpu
            let opcode = {
                let mem = cpu.mem.borrow();
                mem.read_b(cpu.reg.pc.get())
            };

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

    // Computes the flags and result for an ADD instruction.
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
    pub fn compute_sub(&mut self, lhs: u8, rhs: u8)  {
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
    pub fn compute_cp(&mut self, rhs: u8)  {
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
    pub fn compute_inc_flags(&mut self, increased_value: u8) {
        // TODO complete this
    }

    /// Pushes on the stack a 16-bit register value,
    /// it decrements SP and pushes the LSB before the MSB.
    pub fn push_stack(&mut self, reg: u16) {
        self.reg.sp.sub(1);
        let addr = self.reg.sp.get();
        let low = (reg & 0x00FF) as u8;
        self.mem.borrow_mut().write_b(addr, low);

        self.reg.sp.sub(1);
        let addr = self.reg.sp.get();
        let high = ((reg >> 8) & 0x00FF) as u8;
        self.mem.borrow_mut().write_b(addr, high);
    }

    /// Pops a 16-bit value from the stack, MSB first, returning the u16 value.
    pub fn pop_stack(&mut self) -> u16 {
        let high = self.mem.borrow().read_b(self.reg.sp.get());
        self.reg.sp.add(1);

        let low = self.mem.borrow().read_b(self.reg.sp.get());
        self.reg.sp.add(1);

        ((high as u16) << 8) & (low as u16)
    }

    /// Retrieves an immediate 8-bit value.
    /// Immediates are retrieved by reading at the address in the PC register.
    pub fn get_immediate8(&mut self) -> u8 {
        let value = self.mem.borrow().read_b(self.reg.pc.get());
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
