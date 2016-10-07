# Jeebie [![Build Status](https://travis-ci.org/valep27/jeebie.svg?branch=master)](https://travis-ci.org/valep27/jeebie)
A gameboy emulator written in Rust.

## Status
The emulator is in its early stages and can handle only part of the instruction set.
At this point any part might go under substantial changes while I learn more about the hardware, language, libraries, etc.

For more details on the architecture and development process check out the [development notes](./notes.md).

## Building

Make sure you have the latest rust **stable** toolchain installed and simply run:

```
cargo build
```

Jeebie is regulary built on both Windows (using `msvc` rust toolchain) and Mac OSX, and should generally build without
any issues on those systems.

Jeebie is also built with rust `nightly` in order to use `clippy` for extra code linting. In general, no features from
nightly should be required to build it.

## License
See the [license](./LICENSE.md) file for license rights and limitations (MIT).
