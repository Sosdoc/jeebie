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
16kB switchable ROM bank |
--------------------------- 4000 |= 32kB Cartrigbe
16kB ROM bank #0 |
--------------------------- 0000 --
```

Some things to note from the mapping:

- The CPU uses 16-bit addressing (64kB addressable memory)
- The first 32kB are reserved to the cartridge ROM and the upper half is switchable, in case the rom has multiple banks (more than 32kB). Details on this are going to be described along with cartridges.
- Internal RAM is 8kB and is partially mirrored at $E000 - $FDFF, the upper addresses are not mirrored.
- Some cartridges offer extra switchable RAM to be used.

[cpuman]: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
