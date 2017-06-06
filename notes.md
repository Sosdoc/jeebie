# Development notes

This document holds any notes and interesting remarks about the development of
the emulator. It will cover details on the GB architecture, with (short)
explanations and references, along with implementation details, decisions,
mistakes and anything else related to the emulator.


## Gameboy architecture

*Information here refers to the original gameboy model if nothing else is noted.*

### Specs

- **CPU**: An 8-bit custom cpu, it shares similarities to the Z80 processor
- **Clock speed**: 4.194304 MHz
- **RAM**: 8KB RAM, it can also use RAM from cartridges
- **VRAM**: 8KB Video RAM, for holding sprites/framebuffer
- **Resolution**: 160x144
- **Sound**: 4 channels with stereo sound

**A note on timings**: some documents specify two different kinds of "cycles",
the [gameboy CPU manual][cpuman] mentions that 1 *"machine cycle"* is 1/4 of a
clock cycle, most instruction timings are reported in machine cycles, which are
always multiples of 4.

This is useful information in case there could be some ambiguity, if an instruction
is reported with a timing of 4, but uses memory or does multiple operations, it's
likely that 4 are the clock cycles and not the machine ones (16).

### Memory map

The memory map (taken from the [CPU manual][cpuman]):

```
Interrupt Enable Register
--------------------------- FFFF
Internal RAM
--------------------------- FF80
Empty but unusable for I/O
--------------------------- FF4C
I/O ports
--------------------------- FF00
Empty but unusable for I/O
--------------------------- FEA0
Sprite Attrib Memory (OAM)
--------------------------- FE00
Echo of 8kB Internal RAM
--------------------------- E000
8kB Internal RAM
--------------------------- C000
8kB switchable RAM bank
--------------------------- A000
8kB Video RAM
--------------------------- 8000 --
16kB switchable ROM bank         |
--------------------------- 4000 |= 32kB Cartridge
16kB ROM bank #0                 |
--------------------------- 0000 --
```

Some things to note from the mapping:

- The CPU uses 16-bit addressing (64kB addressable memory)
- The first 32kB are reserved to the cartridge ROM and the upper half is switchable, in case the rom has multiple banks (more than 32kB). Details on this are going to be described along with cartridges.
- Internal RAM is 8kB and is partially mirrored at $E000 - $FDFF, the upper addresses are not mirrored.
- Some cartridges offer extra switchable RAM to be used.

[cpuman]: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf


## Emulator implementation

This section contains details, thoughts and whatever else is relevant to the
actual implementation of the emulator.

First things first, the objective in this implementation is to stick at all
times to a Rust implementation, other languages are fine as long as *they only
support development and are not actually part of emulator code*.

**Why this choice?**

It's mostly a personal preference, but one perk of keeping it pure Rust means
that a simple `cargo build` command can get all dependencies after a checkout.
Other than that, I'd like to use Rust wherever possible to keep learning about
it.

### Tools, IDE or lack thereof

For development, the installation of rust I personally use is handled by
multirust. It's a great tool for trying out beta and nightly builds withouth
fussing too much with your environment.

Still, my objective is to keep the emulator buildable **at all times** on the
latest Rust stable release. Nightly is only used optionally if possible, for
things like clippy.

I use Atom as an editor. While it isn't exactly where I'd want it to be in terms
of performance, the support for Rust (via plugins) is currently one of the best
around.

Plugins I use (and recommend) for Rust:

- Linter and linter-rust: make sure to set it to `cargo check`, it will report compiler warnings and errors on save, which in Rust are extremely helpful;
- Racer: an autocompletion plugin, it might not work perfectly but is better than nothing! Install racer via `cargo install` and make sure to clone the rustc github repository;
- Atom beautify: requires rustfmt (`cargo install` saves the day), note that not all files are formatted via rustfmt, but it's great for fixing indentation!

Racer is a bit of a sore spot as it can't always give information (it can't do
much with a `RefCell` for example), fortunately, rustc will include full
support for autocompletion (and IDEs) later on, check on https://areweideyet.com/
to know about current progress.

### The CPU

The core of any system! The CPU is modeled as a struct (aptly named `CPU`) which mainly holds data on its registers,
a reference to a struct used to access memory (discussed later) and some more information needed
during execution, like the amount of elapsed clock cycles.

#### Registers

An important part of the CPU are its `Registers`, a struct that holds all the info on them. The GB CPU has a total
of 10 registers, some of them can be accessed both as 8 or 16 bit registers. Here's a list:

- AF: Accumulator and Flags
- BC: General purpose
- DE: General purpose
- HL: General purpose
- PC: Program counter
- SP: Stack pointer

For example, the registers B and C can be accessed separately, or as a single 16 bit register where B is the 8 high bits
and C is the low 8 bits.
Register F cannot be accessed directly, as it holds the flags, set by instructions. PC and SP are generally accessible only
as 16 bit registers.

Here's a neat little trick employed in the emulator for accessing registers.

We have the basic `Registers` struct that holds all the register data:

```
pub struct Registers {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,

    pub pc: u16, // program counter
    pub sp: u16, // stack pointer
}
```

Then, two types of enums, that represent 8 and 16 bit registers:

```
pub enum Register8 {
    A, B, C, D, E, H, L,
    RegisterAddress(Register16),
    Address(u16),
    N,
    Value8(u8)
}

pub enum Register16 {
    AF, BC, DE, HL, SP, PC,
    NN,
    Value16(u16)
}
```

With this, any register value can be retrieved (using a function) and specifying only the name of
a register. This makes writing instructions easier by abstracting the read/write logic for registers.

But wait! There's also a bunch of odd declarations... What is a `Address(u16)` and why is it an 8 bit register?
Well, an address is 16 bit wide but it points to an 8 bit value in memory, so, with this enum variant we can treat
any value in memory just like a register, and use it in functions that accept a register!
Also, this allows to keep read/write logic separated from functions that implement instructions on registers... Just take a
look at the `alu8.rs` file and you'll see how handy it is!

These are the **algebraic enums** (or discriminated unions, variants, whatever you want to call them, etc.) of Rust in
action, which allow us to embed extra data in each enum variant.

You can also see that we can specify a 16 bit register and use it as a pointer to memory (with `RegisterAddress`)
or even retrieve and immediate value (which will also automatically increase the PC).

the read/write logic is implemented in the `CPU` struct with some simple methods such as this:

```
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
        Register8::N => self.get_immediate8(),
        Register8::Value8(n) => n,
    }
}
```

Here, pattern matching plays well with it and we can cover all cases with appropriate logic.

#### Instructions

A very tedious part to write for any cpu emulator is certain to be the implementation of the instruction set (IS).

The instructions are of variable length, ranging from 1 to 3 bytes, and the cpu has about 500 of them.
The operation codes (opcodes) are only one byte long, but the prefix `CB` can be used to address another 256 instructions, thus bringing the total length to 2 bytes and a maximum of 511 addressable instructions (512 minus the `CB` opcode).

Some instructions include what is called an "immediate" value, generally identified in the mnemonics by the characters:

- `n` : 1 byte unsigned integer;
- `nn` : 2 byte unsigned integer;
- `*` : 1 byte signed integer.

Immediate values are only present in the unprefixed opcode instructions, so the total length can be a maximum of 3 bytes.

In jeebie, there are two "layers" of functions that implement instruction logic. The first one is found in `cpu.rs`, and is a collection of functions that implement generic functionality, like adding two registers, shifting one, jumping to a certain address, etc.

An example function would be:

```
/// Check the value of bit b in reg and set flags accordingly
pub fn bit_check(&mut self, b: usize, reg: Register8) {
    let is_zero = !is_set(self.get8(reg), b);
    self.reg.set_or_clear(Zero, is_zero);
    self.reg.clear_flag(Sub);
    self.reg.set_flag(HalfCarry);
}
```

The second "layer" is found in the `jeebie::instr` module, and this contains multiple source files split by broad instruction "categories".
These files declare a function for each opcode and implement the functionality by calling the more general cpu function.

For example, using the `bit_check` function above:

```
// 'BIT 0,D' CB 42 8
pub fn BIT_0_D(cpu: &mut CPU) -> i32 {
    cpu.bit_check(0, D);
    8
}
```

Each function has the same signature and returns the amount of cycles that the instruction takes to execute. This is useful for functions that can take a variable amount of cycles to complete, like conditional jumps.

Every single one of these functions has a comment line above it, detailing the mnemonic (name), opcode (can be prefixed by `CB` or just a single byte hex value) and the timing.
This data is used by a python script (found in the `scripts` folder) that generates mappings for these functions.

All mappings can be found in the file `jeebie\instr\opcodes.rs`, it contains two static arrays of function references, one represents all the instructions with opcodes from `0x00` to `0xFF`, the other all instructions with prefix `0xCB`.

This mapping is simple (although tedious to write and update, hence why the script) and introduces very little overhead when executing, it's a common way of handling instructions in emulators.
One downside of this would be that repeating instruction code so many times can lead to executable bloating, especially when functions get inlined.