
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
struct VideoMemory {
    data: [u8; 8192],
    oam: [u8; 160],
}