//! A simple disassembler, useful for showing the actual assembly code and eventually debugging.

use jeebie::disasm::metadata::{CB_DISASM_TABLE, DISASM_TABLE};
use std::cell::Cell;

/// The disassembler struct, it has internal mutable state required to disassemble a binary file
/// from start to finish, but exposes an immutable interface.
pub struct Disassembler {
    pc: Cell<usize>,
}

impl Disassembler {
    pub fn new() -> Self {
        Disassembler { pc: Cell::new(0) }
    }

    pub fn rom_to_string(data: &[u8]) -> Result<String, String> {
        let d = Disassembler::new();
        let mut out = String::new();

        while let Ok(instr) = d.get_instruction_str(data) {
            out = out + &instr + "\n";
        }

        Ok(out)
    }

    fn get_opcode_name(&self, data: &[u8]) -> Result<String, String> {
        match data.get(self.pc.get()) {
            Some(&0xCB) => {
                self.pc.set(self.pc.get().wrapping_add(1));

                if let Some(second_byte) = data.get(self.pc.get()) {
                    Ok(String::from(CB_DISASM_TABLE[*second_byte as usize]))
                } else {
                    Err(String::from("2 Byte opcode incomplete (CB prefix)"))
                }
            }
            Some(byte) => Ok(String::from(DISASM_TABLE[*byte as usize])),
            None => Err(String::from("End of data reached")),
        }
    }

    /// Returns an instruction string with the appropriate immediate
    /// values filled in.
    pub fn get_instruction_str(&self, data: &[u8]) -> Result<String, String> {
        let pc = self.pc.get();
        let mut instr = match self.get_opcode_name(data) {
            Ok(s) => s,
            Err(s) => return Err(s),
        };

        let low = data.get(self.pc.get().wrapping_add(1)).map(|x| *x as u16);
        let high = data.get(self.pc.get().wrapping_add(2)).map(|x| (*x as u16) << 8);
        self.pc.set(self.pc.get().wrapping_add(1));

        if instr.contains("nn") {
            let immediate16 = match (low, high) {
                (Some(l), Some(h)) => h | l,
                _ => return Err(String::from("3 Byte opcode incomplete, missing immediate value")),
            };

            let nn = format!("${:04x}", immediate16);
            instr = instr.replace("nn", &nn);
            self.pc.set(self.pc.get().wrapping_add(2));
        }

        if instr.contains('n') || instr.contains('*') {
            let immediate8 = match low {
                Some(l) => l,
                None => {
                    return Err(String::from("2 Byte opcode incomplete, missing immediate value"))
                }
            };

            let n = format!("${:02x}", immediate8);
            let n_signed = format!("{}", immediate8 as i8);

            instr = instr.replace("n", &n);
            instr = instr.replace("*", &n_signed);
            self.pc.set(self.pc.get().wrapping_add(1));
        }

        Ok(format!("{:<20}; ${:04x}", instr, pc))
    }
}

#[test]
fn test_single_instr() {
    let d = Disassembler::new();
    let data: [u8; 2] = [0x00, 0x07];

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("NOP"));
    assert_eq!(1, d.pc.get());

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("RLCA"));
    assert_eq!(2, d.pc.get());
}

#[test]
fn test_immediate8_instr() {
    let d = Disassembler::new();
    let data: [u8; 4] = [0x26, 0x07, 0x20, 0xFF];

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("LD H,$07"));
    assert_eq!(2, d.pc.get());

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("JR NZ,-1"));
    assert_eq!(4, d.pc.get());
}

#[test]
fn test_immediate16_instr() {
    let d = Disassembler::new();
    let data: [u8; 5] = [0x31, 0x20, 0x04, 0x00, 0x04];

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("LD SP,$0420"));
    assert_eq!(3, d.pc.get());

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("NOP"));
    assert_eq!(4, d.pc.get());

    let instr = d.get_instruction_str(&data).unwrap();
    assert!(instr.starts_with("INC B"));
    assert_eq!(5, d.pc.get());
}
