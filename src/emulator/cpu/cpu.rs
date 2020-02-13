use std::num::Wrapping;

use crate::emulator::cpu::OPCODES;
use crate::emulator::bit::{split_word, combine_bytes};
use crate::emulator::register::{Register16, Register8};

pub struct CPU {
	a: Register8,
	f: Register8,
	b: Register8,
	c: Register8,
	d: Register8,
	e: Register8,
	h: Register8,
	l: Register8,
	sp: Register16,
	pc: Register16,
}

impl CPU {
	pub fn new() -> CPU {
		CPU{
			a: Wrapping(0),
			f: Wrapping(0),
			b: Wrapping(0),
			c: Wrapping(0),
			d: Wrapping(0),
			e: Wrapping(0),
			h: Wrapping(0),
			l: Wrapping(0),
			sp: Wrapping(0),
			pc: Wrapping(0)
		}
	}

	pub fn get_bc(&self) -> Register16 {
		let Wrapping(high) = self.b;
		let Wrapping(low) = self.c;
		Wrapping(combine_bytes(high, low))
	}

	pub fn get_de(&self) -> Register16 {
		let Wrapping(high) = self.d;
		let Wrapping(low) = self.e;
		Wrapping(combine_bytes(high, low))
	}

	pub fn get_hl(&self) -> Register16 {
		let Wrapping(high) = self.h;
		let Wrapping(low) = self.l;
		Wrapping(combine_bytes(high, low))
	}

	pub fn set_bc(&mut self, value: Register16) {
		let Wrapping(word) = value;
		let (high, low) = split_word(word);
		self.b = Wrapping(high);
		self.c = Wrapping(low);
	}

	pub fn set_de(&mut self, value: Register16) {
		let Wrapping(word) = value;
		let (high, low) = split_word(word);
		self.d = Wrapping(high);
		self.e = Wrapping(low);
	}

	pub fn set_hl(&mut self, value: Register16) {
		let Wrapping(word) = value;
		let (high, low) = split_word(word);
		self.h = Wrapping(high);
		self.l = Wrapping(low);
	}

    pub fn exec(&mut self, opcode: u8) {
        let instruction = OPCODES[opcode as usize];
        instruction()
    }
}
