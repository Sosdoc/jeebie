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
            cpu.reg.pc.increase();
            // TODO: compute clock timings
        }
    }

    pub fn LD_nn_n(reg1: &mut Register8, value: u8) {
        reg1.set(value);
    }
}
