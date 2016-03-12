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
