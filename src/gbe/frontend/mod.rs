
pub mod glium;

/// A trait for graphical frontends.
/// Normally, the only thing that the cpu does is send the framebuffer data.
pub trait GpuFrontend {
    /// Displays one frame of data from the raw framebuffer.
    fn display_frame(&mut self, framebuffer: Vec<(u8, u8, u8)>);
}
