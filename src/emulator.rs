pub type Pixel = (u8, u8, u8);

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;

pub struct Emulator {
    framebuffer: Vec<Pixel>
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator{
            framebuffer: vec![]
        }
    }

    pub fn run_until_frame(&mut self) -> &[Pixel] {
        &self.framebuffer
    }
}