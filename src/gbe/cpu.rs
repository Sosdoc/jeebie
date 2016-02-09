use std::rc::Rc;
use std::cell::RefCell;

use gbe::memory::MMU;
use gbe::registers::*;

pub struct CPU {
    pub r: Registers,
    pub memory: Rc<RefCell<MMU>>,
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

        CPU {
            r: r,
            memory: mmu,
        }
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
                let mem = cpu.memory.borrow();
                mem.read_b(cpu.r.pc.get())
            };

            // execute
            cpu.dispatch(opcode);
            // increase PC
            cpu.r.pc.increase();
            // TODO: compute clock timings
        }
    }

    pub fn ld_rr(reg1: &mut Register8, reg2: &Register8) {
        reg1.set(reg2.get());
    }
}
