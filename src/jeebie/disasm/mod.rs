//! This module contains structs and data related to disassembly of gameboy code.

pub mod metadata;

use jeebie::core::cpu::CPU;
use jeebie::disasm::metadata::{ CB_DISASM_TABLE, DISASM_TABLE };

/// Returns an instruction string with the appropriate immediate
/// values filled in.
pub fn get_instruction_str(cpu: &mut CPU) -> String {
    let mut pc = cpu.reg.pc;
    let opcode = cpu.mem.read_b(pc);

    let mut instr = match opcode {
        0xCB => {
            pc = pc.wrapping_add(1);
            let second_byte = cpu.mem.read_b(pc);
            String::from(CB_DISASM_TABLE[second_byte as usize])
        },
        _ => String::from(DISASM_TABLE[opcode as usize])
    };

    let low = cpu.mem.read_b(pc.wrapping_add(1));
    let high = cpu.mem.read_b(pc.wrapping_add(2));

    let nl = format!("${:02x}", low);
    let nls = format!("{}", low as i8);
    let nn = format!("${:04x}", (high as u16) << 8 | low as u16 );

    instr = instr.replace("nn", &nn);
    instr = instr.replace("n", &nl);
    instr = instr.replace("*", &nls);
    instr = instr.replace("#", &nl);

    format!("{:<20}; ${:04x}", instr, pc)
}