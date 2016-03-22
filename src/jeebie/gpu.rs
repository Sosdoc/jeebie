use jeebie::memory::MMU;

pub struct GPU<'a> {
    mode: Mode,
    line: u32,
    cycles: u32,
    mem: &'a mut MMU,
}


impl<'a> GPU<'a> {
    pub fn new(mmu: &mut MMU) -> GPU {
        GPU {
            mode: Mode::HBlank,
            line: 0,
            cycles: 0,
            mem: mmu,
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

///  Mode 0 (HBlank): The LCD controller is in the H-Blank period and
///          the CPU can access both the display RAM (8000h-9FFFh)
///          and OAM (FE00h-FE9Fh)
///
///  Mode 1 (VBlank): The LCD controller is in the V-Blank period (or the
///          display is disabled) and the CPU can access both the
///          display RAM (8000h-9FFFh) and OAM (FE00h-FE9Fh)
///
///  Mode 2 (OAMRead): The LCD controller is reading from OAM memory.
///          The CPU <cannot> access OAM memory (FE00h-FE9Fh)
///          during this period.
///
///  Mode 3 (VRAMRead): The LCD controller is reading from both OAM and VRAM,
///          The CPU <cannot> access OAM and VRAM during this period.
///          CGB Mode: Cannot access Palette Data (FF69,FF6B) either.
enum Mode {
    HBlank,
    VBlank,
    OAMRead,
    VRAMRead,
}
