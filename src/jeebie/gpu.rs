
pub struct GPU {
    mode: Mode,
    line: u32,
    cycles: u32,
}


impl GPU {
    pub fn new() -> GPU {
        GPU {
            mode: Mode::HBlank,
            line: 0,
            cycles: 0,
        }
    }

    /// Emulates the GPU. 
    /// This function should be called after an instruction is executed by the CPU,
    /// `delta` is the number of cycles passed from the last instruction.
    pub fn emulate(&mut self, delta: u32) {

        self.cycles += delta;

        match self.mode {            
            Mode::OAMRead => {
                if self.cycles >= 80 {
                    self.cycles = 0;
                    self.mode = Mode::VRAMRead;
                }
            }
            Mode::VRAMRead => {
                if self.cycles >= 172 {
                    self.cycles = 0;
                    self.mode = Mode::HBlank;

                    // TODO: scanline is done, write it to framebuffer
                }
            }
            Mode::HBlank => {
                if self.cycles >= 204 {
                    self.cycles = 0;
                    self.line += 1;

                    self.mode = if self.line == 143 {
                        Mode::VBlank
                    } else {
                        Mode::OAMRead
                    };
                }
            }
            Mode::VBlank => {
                if self.cycles >= 456 {
                    self.mode = Mode::HBlank;
                    self.line += 1;

                    if self.line > 153 {
                        self.mode = Mode::OAMRead;
                        self.line = 0;
                    }
                }
            }
        }
    }
}

enum Mode {
    OAMRead,
    VRAMRead,
    HBlank,
    VBlank,
}

/// The memory owned by the GPU
/// The main VRAM (`data`) is used for the following values:
///     8000-87FF	Tile set #1: tiles 0-127
///     8800-8FFF	Tile set #1: tiles 128-255 Tile set #0: tiles -1 to -128
///     9000-97FF	Tile set #0: tiles 0-127
///     9800-9BFF	Tile map #0
///     9C00-9FFF	Tile map #1
///
/// The rest of the memory (`oam`) is used for sprite data and addressed separately
/// from FE00 to FE9F.
struct VideoMemory {
    data: [u8; 8192],
    oam: [u8; 160],
}