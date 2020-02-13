mod emulator;

use sdl2::render::{Canvas, Texture};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::error::Error;
use emulator::Emulator;

fn main() {
    main_loop().expect("An error occurred when running the emulator");
}

fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let (width, height) = (emulator::WIDTH, emulator::HEIGHT);

    let window = video_subsystem.window("Jeebie", width * 3, height * 3)
        .position_centered()
        .resizable()
        .opengl()
        .build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = window.into_canvas()
                    .present_vsync()
                    .accelerated()
                    .build()?;

    let tc = canvas.texture_creator();
    let mut texture = tc.create_texture_streaming(PixelFormatEnum::RGB24, width, height)?;

    let mut emulator = Emulator::new();

    'running: loop {
        // Handle inputs
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {},
            };
        }

        // TODO: send input to emulator

        // Run one frame
        let fb = emulator.run_until_frame();

        // Draw
        draw_step(&mut canvas, &mut texture, &fb)?;
    }

    Ok(())
}


fn draw_step(canvas: &mut Canvas<sdl2::video::Window>, texture: &mut Texture, framebuffer: &[(u8, u8, u8)]) -> Result<(), Box<dyn Error>> {
    canvas.clear();

    let (width, height) = (emulator::WIDTH as usize, emulator::HEIGHT as usize);

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

        for y in 0..height {
            for x in 0..width {
                let offset = x * 3 + pitch * y;
                let fb_index = x + (y * height);
                let (r, g, b) = framebuffer[fb_index];

                buffer[offset] = r;
                buffer[offset + 1] = g;
                buffer[offset + 2] = b;
            }
        }
    })?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    Ok(())
}