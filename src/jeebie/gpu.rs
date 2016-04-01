use jeebie::utils::is_set;

pub struct GPU {
    mode: Mode,
    line: u32,
    cycles: u32,
    vram: VideoMemory,
    lcdc: LCDControl,
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
#[derive(Copy, Clone, PartialEq, Debug)]
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

impl GBColor {    
    pub fn from_u8(number: u8) -> GBColor {
        match number {
            0 => GBColor::Off,
            1 => GBColor::On33,
            2 => GBColor::On66,
            3 => GBColor::On,
            _ => panic!("Invalid color value {}", number),
        }
    }
}

/// An enum used to discriminate tilesets (and maps)
enum TileSelector {
    Set0, Set1
}

/// An enum for sprite size mode (8x8 or 8x16)
enum SpriteSize {
    Size8, Size16
}

/// LCD Control register data 
///     Bit 7 - LCD Display Enable             (0=Off, 1=On)
///     Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
///     Bit 5 - Window Display Enable          (0=Off, 1=On)
///     Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
///     Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
///     Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
///     Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
///     Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
struct LCDControl {
    lcd_enable: bool,
    window_tile_map: TileSelector,    
    window_enable: bool,    
    bgw_tile_data_select: TileSelector,
    bg_tile_map: TileSelector,   
    sprite_size: SpriteSize,
    sprite_enable: bool,   
    bg_enable: bool,
}

impl LCDControl {
    
    pub fn new() -> LCDControl {
        LCDControl {
            lcd_enable: false,
            window_tile_map: TileSelector::Set0,
            window_enable: false,
            bgw_tile_data_select: TileSelector::Set0,
            bg_tile_map: TileSelector::Set0,
            sprite_size: SpriteSize::Size8,
            sprite_enable: false,
            bg_enable: false,
        }
    }
    
    /// Sets LCDC data from a byte value     
    pub fn set_from_u8(&mut self, data: u8) {        
        self.lcd_enable = if is_set(data, 7) { true } else { false };
        self.window_tile_map = if is_set(data, 6) { TileSelector::Set1 } else { TileSelector::Set0 };
        self.window_enable = if is_set(data, 5) { true } else { false };
        self.bgw_tile_data_select = if is_set(data, 4) { TileSelector::Set1 } else { TileSelector::Set0 };
        self.bg_tile_map = if is_set(data, 3) { TileSelector::Set1 } else { TileSelector::Set0 };
        self.sprite_size = if is_set(data, 2) { SpriteSize::Size16 } else { SpriteSize::Size8 };
        self.sprite_enable = if is_set(data, 1) { true } else { false };
        self.bg_enable = if is_set(data, 0) { true } else { false };
    }
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
            lcdc: LCDControl::new(),
        }
    }
    
    /// Retrieves Tile information from the VRAM.
    /// A tile is held in 16 bytes in the VRAM, enough information for 64 pixels (8x8 matrix, 2 bits per pixel).
    /// When selecting tiles from Set #1, the index 0 represents tile -128 (equal to tile 128 from Set #0) 
    fn get_tile(&self, set: TileSelector, tile_index: usize) -> Tile {
        // Set1 starts after 128 tiles, or after 16 * 128 = 2048 (0x800) bytes
        let offset = if let TileSelector::Set1 = set { 0x800 } else { 0 };
        let start_addr = (offset + tile_index) * 0x10;         
        let end_addr = start_addr + 0x10;
        
        let mut addr = start_addr;
        let mut pixels = [GBColor::Off; 64];             
        
        // this is basically a for with step 2. Iterates 8 times (two bytes read at a time). 
        while addr < end_addr {
            let (low, high) = (self.vram.data[addr], self.vram.data[addr + 1]);
            
            for i in 0..8 {
                let low_bit = (low >> i) & 0x01;
                let high_bit = (high >> i) & 0x01;
                
                let pixel_value = GBColor::from_u8(low_bit + high_bit * 2);                
                // px 0..7, then 8..15
                let pixel_addr = (addr / 2) * 8 + i;                
                pixels[pixel_addr as usize] = pixel_value;
            }
            
            addr += 2;
        }
        
        Tile { pixels: pixels }
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

#[test]
fn get_tile_test() {
    let mut gpu = GPU::new();
    // write first line of tile #0 in set #0
    // pixels should be: 0 1 2 3 3 2 1 0
    gpu.write_vram(0, 0b0101_1010u8);
    gpu.write_vram(1, 0b0011_1100u8);
    
    let tile = gpu.get_tile(TileSelector::Set0, 0);
    
    assert_eq!(tile.pixels[0], GBColor::Off);
    assert_eq!(tile.pixels[1], GBColor::On33);
    assert_eq!(tile.pixels[2], GBColor::On66);
    assert_eq!(tile.pixels[3], GBColor::On);
    
    assert_eq!(tile.pixels[4], GBColor::On);
    assert_eq!(tile.pixels[5], GBColor::On66);
    assert_eq!(tile.pixels[6], GBColor::On33);
    assert_eq!(tile.pixels[7], GBColor::Off);
    
    // rest of the pixels are off
    for i in 8..64 {
        assert_eq!(tile.pixels[i], GBColor::Off);
    }
}