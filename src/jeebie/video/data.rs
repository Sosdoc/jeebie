use jeebie::utils::is_set;

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
    pub data: [u8; 8192],
    pub oam: [u8; 160],
}

///  Mode 0 (`HBlank`): The LCD controller is in the H-Blank period and
///          the CPU can access both the display RAM (`8000h`-`9FFFh`)
///          and OAM (`FE00h`-`FE9Fh`)
///
///  Mode 1 (`VBlank`): The LCD controller is in the V-Blank period (or the
///          display is disabled) and the CPU can access both the
///          display RAM (`8000h`-`9FFFh`) and OAM (`FE00h`-`FE9Fh`)
///
///  Mode 2 (`OAMRead`): The LCD controller is reading from OAM memory.
///          The CPU <cannot> access OAM memory (`FE00h`-`FE9Fh`)
///          during this period.
///
///  Mode 3 (`VRAMRead`): The LCD controller is reading from both OAM and VRAM,
///          The CPU <cannot> access OAM and VRAM during this period.
///          CGB Mode: Cannot access Palette Data (`FF69h`,`FF6Bh`) either.
pub enum Mode {
    HBlank,
    VBlank,
    OAMRead,
    VRAMRead,
}

/// An enum representing the possible color values for a pixel in the original GB.
/// While the GB is said to be monochromatic, it can actually display 4 different shades.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GBColor {
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

    pub fn to_u8u8u8(self) -> (u8, u8, u8) {
        match self {
            GBColor::Off => (255, 255, 255),
            GBColor::On33 => (192, 192, 192),
            GBColor::On66 => (96, 96, 96),
            GBColor::On => (0, 0, 0),
        }
    }
}

/// An enum used to discriminate tilesets (and maps)
#[derive(Copy, Clone)]
pub enum TileSelector {
    Set0, Set1
}

/// An enum for sprite size mode (8x8 or 8x16)
#[derive(Copy, Clone)]
pub enum SpriteSize {
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
pub struct LCDControl {
    pub lcd_enable: bool,
    pub window_tile_map: TileSelector,
    pub window_enable: bool,
    pub bgw_tile_data_select: TileSelector,
    pub bg_tile_map: TileSelector,
    pub sprite_size: SpriteSize,
    pub sprite_enable: bool,
    pub bg_enable: bool,
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
        self.lcd_enable = is_set(data, 7);
        self.window_tile_map = if is_set(data, 6) { TileSelector::Set1 } else { TileSelector::Set0 };
        self.window_enable = is_set(data, 5);
        self.bgw_tile_data_select = if is_set(data, 4) { TileSelector::Set1 } else { TileSelector::Set0 };
        self.bg_tile_map = if is_set(data, 3) { TileSelector::Set1 } else { TileSelector::Set0 };
        self.sprite_size = if is_set(data, 2) { SpriteSize::Size16 } else { SpriteSize::Size8 };
        self.sprite_enable = is_set(data, 1);
        self.bg_enable = is_set(data, 0);
    }

    /// Retrieves LCDC data as a byte.
    pub fn as_u8(&self) -> u8 {
        let mut result = 0;

        if self.lcd_enable { result |= 0x80 };
        if let TileSelector::Set1 = self.window_tile_map { result |= 0x40 };
        if self.window_enable { result |= 0x20 };
        if let TileSelector::Set1 = self.bgw_tile_data_select { result |= 0x10 };
        if let TileSelector::Set1 = self.bg_tile_map  { result |= 0x08 };
        if let SpriteSize::Size16 = self.sprite_size { result |= 0x04 };
        if self.sprite_enable { result |= 0x02 };
        if self.bg_enable { result |= 0x01 };

        result
    }
}

/// The LCD status register, holds information about the gpu.
/// - Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
/// - Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
/// - Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)
/// - Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)
/// - Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)
/// - Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)
///             0: During H-Blank
///             1: During V-Blank
///             2: During Searching OAM-RAM
///             3: During Transfering Data to LCD Driver
pub struct LCDStatus {
    pub coincidence_irq: bool,
    pub oam_irq: bool,
    pub vblank_irq: bool,
    pub hblank_irq: bool,
    pub coincidence_flag: bool,
    pub mode: Mode,
}

impl LCDStatus {
     pub fn new() -> LCDStatus {
        LCDStatus {
            coincidence_irq: false,
            oam_irq: false,
            vblank_irq: false,
            hblank_irq: false,
            coincidence_flag: false,
            mode: Mode::HBlank,
        }
    }

    pub fn set_from_u8(&mut self, data: u8) {
        self.coincidence_irq = is_set(data, 6);
        self.oam_irq = is_set(data, 5);
        self.vblank_irq = is_set(data, 4);
        self.hblank_irq = is_set(data, 3);
        self.coincidence_flag = is_set(data, 2);

        self.mode = match data & 0x3 {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OAMRead,
            3 => Mode::VRAMRead,
            _ => panic!("invalid mode flag {:?}", data & 0x3)
        };
    }

    pub fn to_u8(&self) -> u8 {
        let mut result = 0;

        if self.coincidence_irq { result |= 0x40 };
        if self.oam_irq { result |= 0x20 };
        if self.vblank_irq { result |= 0x10 };
        if self.hblank_irq { result |= 0x08 };
        if self.coincidence_flag  { result |= 0x04 };

        result |= match self.mode {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAMRead => 2,
            Mode::VRAMRead => 3,
        };

        result
    }
}

/// A tile is an 8x8 square of pixels, each one stored in one of the VRAM's tilesets.
pub struct Tile {
    pub pixels: [GBColor; 64],
}

impl Tile {
    pub fn new() -> Tile {
        Tile { pixels: [GBColor::Off; 64] }
    }
}

/// Holds position and scrolling data for the display
/// window_x is the actual position minus 7, i.e. window_x = 7 means x = 0
pub struct LCDPosition {
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub window_y: u8,
    pub window_x: u8,
}

impl LCDPosition {
    pub fn new() -> LCDPosition {
        LCDPosition { scroll_y: 0, scroll_x: 0, window_y: 0, window_x: 0 }
    }
}