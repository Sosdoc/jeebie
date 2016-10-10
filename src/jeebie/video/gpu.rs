use super::data::*;

/// Holds all information relative to the graphics subsystem.
/// Includes computed data like the framebuffer, in a format that can be drawn to screen.
pub struct GPU {
    mode: Mode,
    vblank_int: bool,
    line: u8,
    cycles: u32,
    vram: VideoMemory,
    lcdc: LCDControl,
    lcdp: LCDPosition,
    framebuffer: [(u8, u8, u8); 160 * 144]
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            mode: Mode::HBlank,
            vblank_int: false,
            line: 0,
            cycles: 0,
            vram: VideoMemory { data: [0; 8192], oam: [0; 160] },
            lcdc: LCDControl::new(),
            lcdp: LCDPosition::new(),
            framebuffer: [(0, 0, 0); 160 * 144],
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
                pixels[pixel_addr] = pixel_value;
            }

            addr += 2;
        }

        Tile { pixels: pixels }
    }

    /// Retrieves a single pixel from a given tile index.
    /// The pixel index is a number between 0 and 63.
    fn get_tile_pixel(&self, set: TileSelector, tile_index: usize, pixel_index: usize) -> GBColor {
        let offset = if let TileSelector::Set1 = set { 0x800 } else { 0 };
        let start_addr = (offset + tile_index) * 0x10;

        let addr = start_addr + ((pixel_index / 8) as usize) * 2;
        let (low, high) = (self.vram.data[addr], self.vram.data[addr + 1]);
        let i = pixel_index % 8;
        let low_bit = (low >> i) & 0x01;
        let high_bit = (high >> i) & 0x01;

        GBColor::from_u8(low_bit + high_bit * 2)
    }

    /// Retrieves a slice of the framebuffer.
    pub fn get_framebuffer(&mut self) -> &[(u8, u8, u8)]{
        if self.vblank_int {
            self.vblank_int = false;
        }

        &self.framebuffer
    }

    /// Renders a single scanline to the framebuffer, from the internal tile data.
    fn render_scanline(&mut self) {
        // map offset starts from 0x1C00 (for tileset #1) or 0x1800 (for tileset #0)
        let mut y_offset : u16 = if let TileSelector::Set1 = self.lcdc.bg_tile_map { 0x1C00 } else { 0x1800 };

        // add vertical offset (y_scroll plus the scanline we are at right now)
        // line and scroll_y are on a per pixel basis, shifting right by 3 gets the tile index offset
        y_offset += ((self.line as u8).wrapping_add(self.lcdp.scroll_y) >> 3) as u16;

        // same for x tile offset
        let mut x_offset = (self.lcdp.scroll_x >> 3) as u16;

        // x,y are the 3 least significant bits in the offsets
        // they represent the coordinates of the pixel inside a tile
        let y = (y_offset & 0x07) as usize;
        let mut x = (x_offset & 0x07) as usize;

        // starting scanline where we will write on the framebuffer
        let fb_offset = (self.line as usize) * 160;

        // read the tile index from the vram at the computed offset
        let mut tile_index = self.read_vram((y_offset + x_offset) as usize);

        // compute each of the 160 pixels in a scanline
        for i in 0..160 {
            // TODO: maybe load tile lines instead of single pixels, less function calls.
            let pixel = self.get_tile_pixel(self.lcdc.bg_tile_map, tile_index as usize, x + y);

            self.framebuffer[fb_offset + i] = pixel.to_u8u8u8();
            x += 1;

            if x == 8 {
                // reached end of current tile, go to the next one (right)
                x = 0;
                // tiles wrap horizontally (every 32)
                x_offset = (x_offset + 1) % 32;
                tile_index = self.read_vram((y_offset + x_offset) as usize);
            }
        }

    }

    pub fn write_vram(&mut self, addr: usize, value: u8) {
        // if let Mode::VRAMRead = self.mode { panic!("Attempted write to VRAM during VRAMRead mode") };
        self.vram.data[addr] = value;
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        // if let Mode::VRAMRead = self.mode { panic!("Attempted access to VRAM during VRAMRead mode") };
        self.vram.data[addr]
    }

    pub fn write_oam(&mut self, addr: usize, value: u8) {
        // if let Mode::OAMRead = self.mode { panic!("Attempted write to OAM during OAMRead mode") };
        self.vram.oam[addr] = value;
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        // if let Mode::OAMRead = self.mode { panic!("Attempted access to OAM during OAMRead mode") };
        self.vram.oam[addr]
    }

    pub fn read_register(&self, addr: usize) -> u8 {
        match addr {
            0xFF40 => self.lcdc.as_u8(), // LCDC
            0xFF42 => self.lcdp.scroll_x,
            0xFF43 => self.lcdp.scroll_y,
            0xFF44 => self.line, // current scanline
            0xFF47 => panic!("Palette is write only!"),
            _ => panic!("Attempted GPU register access with addr {:4x}", addr),
        }
    }

    pub fn write_register(&mut self, addr: usize, data: u8) {
        match addr {
            0xFF40 => self.lcdc.set_from_u8(data), // LCDC
            0xFF42 => { self.lcdp.scroll_x = data },
            0xFF43 => { self.lcdp.scroll_y = data },
            0xFF44 => { self.line = data }, // current scanline
            0xFF47 => {
                // TODO: figure palette writing (is it needed for CGB only?)
             },
            _ => panic!("Attempted GPU register write with addr {:4x}", addr),
        };
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

                    // scanline is done, write it to framebuffer
                    self.render_scanline();
                }
            }
            Mode::HBlank => {
                if self.cycles >= 204 {
                    self.cycles = 0;
                    self.line += 1;

                    self.mode = if self.line == 143 {
                        // TODO: push framebuffer data to frontend now (interrupt)
                        self.vblank_int = true;
                        Mode::VBlank

                    } else {
                        Mode::OAMRead
                    };
                }
            }
            Mode::VBlank => {
                if self.cycles >= 456 {
                    self.cycles = 0;
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

#[test]
fn get_pixel_test() {
    let mut gpu = GPU::new();
    // write first line of tile #0 in set #0
    // pixels should be: 0 1 2 3 3 2 1 0
    gpu.write_vram(0, 0b0101_1010u8);
    gpu.write_vram(1, 0b0011_1100u8);

    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 0), GBColor::Off);
    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 1), GBColor::On33);
    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 2), GBColor::On66);
    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 3), GBColor::On);

    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 4), GBColor::On);
    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 5), GBColor::On66);
    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 6), GBColor::On33);
    assert_eq!(gpu.get_tile_pixel(TileSelector::Set0, 0, 7), GBColor::Off);
}
