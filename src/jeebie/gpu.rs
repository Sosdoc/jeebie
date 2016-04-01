pub struct GPU {
    mode: Mode,
    line: u32,
    cycles: u32,
    vram: VideoMemory
}

/// Video memory and related data/registers
/// The main VRAM (`data`) is used for the following values:
///     8000-87FF	Tile set #1: tiles 0-127
///     8800-8FFF	Tile set #1: tiles 128-255 Tile set #0: tiles -1 to -128
///     9000-97FF	Tile set #0: tiles 0-127
///     9800-9BFF	Tile map #0
///     9C00-9FFF	Tile map #1
///
/// There are 2 tile sets of 256 tiles, but 128 tiles are shared between them, so the
/// total amounts to 384 tiles. 
/// The tile maps hold indexes to a corresponding tile in the tilesets.
/// Pixel data is 2 bits and is split between two adjacent bytes, low bit first:
///
///     1 0 0 0 1 1 0 1 -- 0x8000
///     0 1 1 0 1 0 1 1 -- 0x8001
///
///     1 2 2 0 3 1 2 3 -- pixel value (0 to 3)
///     
/// The rest of the memory (`oam`) is used for sprite data and addressed separately
/// from FE00 to FE9F.
pub struct VideoMemory {
    data: [u8; 8192],
    oam: [u8; 160],
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

/// An enum representing the possible color values for a pixel in the original GB.
/// While the GB is said to be monochromatic, it can actually display 4 different shades. 
#[derive(Copy, Clone)]
enum GBColor {
    // white, rgb #FFFFFF
    Off = 0, 
    // light grey, rgb #C0C0C0
    On33 = 1,
    // dark grey, rgb #606060
    On66 = 2,
    // black, rgb #000000
    On = 3,
}

/// A tile is an 8x8 square of pixels, each one stored in one of the VRAM's tilesets.
struct Tile {
    pixels: [GBColor; 64],
}

impl Tile {
    pub fn new() -> Tile {
        Tile { pixels: [GBColor::Off; 64] }
    }    
}


impl GPU {
    pub fn new() -> GPU {
        GPU {
            mode: Mode::HBlank,
            line: 0,
            cycles: 0,
            vram: VideoMemory {
                data: [0; 8192],
                oam: [0; 160],
            },
        }
    }

    pub fn write_vram(&mut self, addr: usize, value: u8) {
        self.vram.data[addr] = value;
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        self.vram.data[addr]
    }

    pub fn write_oam(&mut self, addr: usize, value: u8) {
        self.vram.oam[addr] = value;
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        self.vram.oam[addr]
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
