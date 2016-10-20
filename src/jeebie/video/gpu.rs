use super::data::*;

use jeebie::utils::is_set;


const SCREEN_WIDTH: i32 = 160;
const SCREEN_HEIGHT: i32 = 144;

/// Holds all information relative to the graphics subsystem.
/// Includes computed data like the framebuffer, in a format that can be drawn to screen.
pub struct GPU {
    line: u8,
    lyc: u8, // LYC (line Y compare) register
    cycles: u32,
    vram: VideoMemory,
    lcdc: LCDControl,
    lcdp: LCDPosition,
    lcds: LCDStatus,
    framebuffer: [(u8, u8, u8); (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            line: 0,
            lyc: 0,
            cycles: 0,
            vram: VideoMemory::new(),
            lcdc: LCDControl::new(),
            lcdp: LCDPosition::new(),
            lcds: LCDStatus::new(),
            framebuffer: [(0, 0, 0); (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }

    /// Emulates the GPU.
    /// This function should be called after an instruction is executed by the CPU,
    /// `delta` is the number of cycles passed from the last instruction.
    pub fn emulate(&mut self, delta: u32) {

        // If screen is disabled
        if !self.lcdc.lcd_enable {
            return;
        }

        self.cycles += delta;

        match self.lcds.mode {
            Mode::OAMRead => {
                if self.cycles >= 80 {
                    self.cycles = 0;
                    self.lcds.mode = Mode::VRAMRead;
                }
            }
            Mode::VRAMRead => {
                if self.cycles >= 172 {
                    self.cycles = 0;
                    self.lcds.mode = Mode::HBlank;

                    // scanline is done, write it to framebuffer
                    self.render_scanline();
                }
            }
            Mode::HBlank => {
                if self.cycles >= 204 {
                    self.cycles = 0;
                    self.line += 1;

                    self.lcds.mode = if self.line == 143 {
                        // TODO: push framebuffer data to frontend now (interrupt)
                        self.lcds.vblank_irq = true;
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
                        self.lcds.mode = Mode::OAMRead;
                        self.line = 0;
                    }
                }
            }
        }
    }

    /// Renders a single scanline to the framebuffer, from the internal tile data.
    fn render_scanline(&mut self) {
        self.render_background();
        self.render_window();
        self.render_sprites();
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
        if self.lcds.vblank_irq {
            self.lcds.vblank_irq = false;
        }

        &self.framebuffer
    }

    /// Renders the current window.
    /// The window is an alternate background area that can be rendered above the normal background.
    ///
    /// The window becomes visible (if enabled) when positions are set in range WX=0..166, WY=0..143.
    /// A position of WX=7, WY=0 locates the window at upper left, it is then completely covering normal background.
    fn render_window(&mut self) {
        // get window X and Y, X is offset by 7 so we subtract 7
        let wy = self.lcdp.window_y;
        let wx = self.lcdp.window_x as i32 - 7;

        // continue rendering only if wx/wy are in range (0 to 160 and 0 to 144)
        // also, the current line being rendered must be inside the window (line < wy)
        match (wx, wy) {
            (0...159, 0...143) if self.line < wy => {}
            (_, _) => { return; }
        };

        // Tile data start offset (0=8800-97FF, 1=8000-8FFF)
        let tiles_start = match self.lcdc.bgw_tile_data_select {
            TileSelector::Set0 => 0 as u16,
            TileSelector::Set1 => 0x800,
        };

        // Window tile map start (0=9800-9BFF, 1=9C00-9FFF)
        let tilemap_start = match self.lcdc.window_tile_map {
            TileSelector::Set0 => 0x1800,
            TileSelector::Set1 => 0x1C00,
        };

        // offset for the tile map given by the current line value
        let tilemap_y_offset = ((self.line / 8) * 32) as usize;
        // y value inside a tile identified by the current line
        let y_offset = ((self.line % 8) * 2) as u16;

        for x in 0..32 {

            // Read the tile index from the map in memory
            let tile_index = match self.lcdc.window_tile_map {
                TileSelector::Set0 => self.vram.data[tilemap_start + tilemap_y_offset + x],
                TileSelector::Set1 => {
                    // set 1 is indexed from -128 to 127, indexes are signed bytes
                    let tile_signed = self.vram.data[tilemap_start + tilemap_y_offset + x] as i16;
                    (tile_signed + 128) as u8
                },
            };

            // offset in the map for x value, it's x (current x in tile) times 8 (tile width)
            let map_offset = x * 8;

            // index of a tile in a byte array (tiles are 16 bytes long)
            let tile_byte_idx = (tile_index * 16) as u16;

            // actual address for tile data
            let tile_address = tiles_start + tile_byte_idx + y_offset;

            let high_byte = self.vram.data[tile_address as usize];
            let low_byte = self.vram.data[tile_address as usize + 1];

            // compute each of the 8 pixels in the 2 bytes, starting from bit 7
            let mut pixel_idx = 7;
            while pixel_idx >= 0 {
                // the offset given by the windowX
                let buffer_x_offset = map_offset as i32 + pixel_idx + wx;

                let pixel = if is_set(high_byte, pixel_idx as usize) { 1 } else { 0 } |
                            if is_set(low_byte, pixel_idx as usize) { 2 } else { 0 };

                // final position of this pixel offset by line * line_width
                let position = SCREEN_WIDTH as usize * self.line as usize + buffer_x_offset as usize;

                let color = GBColor::from_u8(pixel);
                self.framebuffer[position] = color.to_u8u8u8();

                pixel_idx -= 1;
            }

            // TODO update line value?
        }
    }


    fn render_sprites(&mut self) {

        if !self.lcdc.sprite_enable {
            return;
        }

        let sprite_height = self.lcdc.sprite_size as u8;
        let line_width = self.line as i32 * SCREEN_WIDTH;

        // 40 sprites can be displayed
        let mut sprite = 39;
        while sprite >= 0 {
            let sprite_offset = sprite * 4;

            // Sprite data is 4 bytes with the following information
            //   Byte0  Y position on the screen
            //   Byte1  X position on the screen
            //   Byte2  Tile map index, like with tiles an index from 0 to 255
            //   Byte3  Flags:
            //     Bit7  Priority
            //       Sprite is displayed in front of the window if this bit is set to 1.
            //       Otherwise, sprite is shown behind the window but in front of the background.
            //     Bit6  Y flip
            //       Sprite pattern is flipped vertically if this bit is set to 1.
            //     Bit5  X flip
            //       Sprite pattern is flipped horizontally if this bit is set to 1.
            //     Bit4  Palette number
            //       Use colors from OBJ1PAL if this bit is set and from OBJ0PAL otherwise.


            let sprite_y = self.vram.oam[sprite_offset as usize] as i32 - 16;
            let sprite_x = self.vram.oam[(sprite_offset + 1) as usize] as i32 - 8;

            // skip sprite if outside boundaries
            if (sprite_y > self.line as i32) || (sprite_y + sprite_height as i32) <= self.line as i32 {
                continue;
            }

            if (sprite_x < -7) || (sprite_x >= SCREEN_WIDTH) {
                continue;
            }

            // TODO what if sprite height is 16? any changes?
            let sprite_tile = self.vram.oam[(sprite_offset + 2) as usize];
            let sprite_flags = self.vram.oam[(sprite_offset + 3) as usize];

            // TODO refactor this into a struct?
            let sprite_palette = if is_set(sprite_flags, 4) { 1 } else { 0 };
            let x_flip = is_set(sprite_flags, 5);
            let y_flip = is_set(sprite_flags, 6);
            let over_window = is_set(sprite_flags, 7);

            let pixel_y = if y_flip {
                // sprite is flipped vertically
                (sprite_height as i32 - 1) - (self.line as i32 - sprite_y)
            } else {
                self.line as i32- sprite_y
            };

            // offset in tile map due to y position
            let pixel_y_offset = if sprite_height == 16 && pixel_y >= 8 {
                ((pixel_y - 8) * 2) + 16
            } else {
                pixel_y * 2
            };

            let tile_address = sprite_tile as i32 + pixel_y_offset as i32;
            // retrieve the pixel data
            let high_byte = self.vram.data[tile_address as usize];
            let low_byte = self.vram.data[(tile_address + 1) as usize];

            // compute each pixel and render
            for x in 0..8 {
                let pixel_idx = if x_flip { x } else { 7 - x };

                let pixel = if is_set(high_byte, pixel_idx as usize) { 1 } else { 0 } |
                            if is_set(low_byte, pixel_idx as usize) { 2 } else { 0 };

                // skip transparent pixels (value == 0)
                if pixel == 0 {
                    continue;
                }

                // skip pixels outside boundaries
                let x_pos = x + sprite_x;

                if x_pos < 0 || x_pos >= 144 {
                    continue;
                }

                // index in the framebuffer
                let position = line_width as i32 + x_pos;

                // TODO skip pixel if it is above the BG (need to keep BG data)
                // if !over_window {
                //
                // }

                // TODO: read palette from memory (0xFF49 for OBJ1, 0xFF48 for OBJ0)
                let color = GBColor::from_u8(pixel);
                self.framebuffer[position as usize] = color.to_u8u8u8();
            }

            sprite -= 1;
        }
    }

    /// Renders the background
    fn render_background(&mut self) {
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
        let fb_offset = (self.line as usize) * SCREEN_WIDTH as usize;

        // read the tile index from the vram at the computed offset
        let mut tile_index = self.read_vram((y_offset + x_offset) as usize);

        // compute each of the 160 pixels in a scanline
        for i in 0..SCREEN_WIDTH {
            // TODO: maybe load tile lines instead of single pixels, less function calls.
            let pixel = self.get_tile_pixel(self.lcdc.bg_tile_map, tile_index as usize, x + y);

            self.framebuffer[fb_offset + i as usize] = pixel.to_u8u8u8();
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
        // if let Mode::VRAMRead = self.lcds.mode { panic!("Attempted write to VRAM during VRAMRead mode") };
        self.vram.data[addr] = value;
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        // if let Mode::VRAMRead = self.lcds.mode { panic!("Attempted access to VRAM during VRAMRead mode") };
        self.vram.data[addr]
    }

    pub fn write_oam(&mut self, addr: usize, value: u8) {
        // if let Mode::OAMRead = self.lcds.mode { panic!("Attempted write to OAM during OAMRead mode") };
        self.vram.oam[addr] = value;
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        // if let Mode::OAMRead = self.lcds.mode { panic!("Attempted access to OAM during OAMRead mode") };
        self.vram.oam[addr]
    }

    pub fn read_register(&self, addr: usize) -> u8 {
        match addr {
            0xFF40 => self.lcdc.as_u8(), // LCDC
            0xFF41 => self.lcds.to_u8(), // LCDStat
            0xFF42 => self.lcdp.scroll_x,
            0xFF43 => self.lcdp.scroll_y,
            0xFF44 => self.line, // current scanline
            0xFF45 => self.lyc,
            0xFF47 => panic!("Palette is write only!"),
            _ => panic!("Attempted GPU register access with addr {:4x}", addr),
        }
    }

    pub fn write_register(&mut self, addr: usize, data: u8) {
        match addr {
            0xFF40 => self.lcdc.set_from_u8(data), // LCDC
            0xFF41 => self.lcds.set_from_u8(data), // LCDStat
            0xFF42 => { self.lcdp.scroll_x = data },
            0xFF43 => { self.lcdp.scroll_y = data },
            0xFF44 => { self.line = data }, // current scanline
            0xFF45 => { self.lyc = data },
            0xFF47 => {
                // TODO: figure palette writing (is it needed for CGB only?)
             },
            _ => panic!("Attempted GPU register write with addr {:4x}", addr),
        };
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
