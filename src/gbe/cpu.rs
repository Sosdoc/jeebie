use gbe::memory;

use gbe::registers::*;


#[derive(Debug)]
pub struct CPU {
    pub r: Registers,
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
        CPU { r: r }
    }

    /// executes the instruction
    pub fn dispatch(&mut self, opcode: u8) {

    }

    /// reads instructions and executes them
    pub fn fetch_and_exec() {
        let mut cpu = CPU::new();

        loop {
            // fetch
            let opcode = memory::read_b(cpu.r.pc.get());
            // execute
            cpu.dispatch(opcode);
            // increase PC
            cpu.r.pc.increase();
            // adds clock ticks corresponding to previous instruction
            // cpu.c.m = cpu.c.m.wrapping_add(cpu.r.m);
            // cpu.c.t = cpu.c.t.wrapping_add(cpu.c.t);

        }
    }

    pub fn ld_rr(reg1: &mut Register8, reg2: &Register8) {
        reg1.set(reg2.get());
    }
}
