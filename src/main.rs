#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate piston_window;
extern crate image as im;

mod jeebie;

use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;

use piston_window::*;

fn main() {
    // Size of the framebuffer
    let fb_size = (160, 144);

    let mut cpu = CPU::new(MMU::new());

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [fb_size.0, fb_size.1])
        .build()
        .unwrap();

    window.set_title("Jeebie".to_string());

    // Create an ImageBuffer and fill it
    let mut img = im::ImageBuffer::new(fb_size.0, fb_size.1);

    for x in 0..fb_size.0 {
        for y in 0..fb_size.1 {
            img.put_pixel(x, y, im::Rgba([0, 255, 0, 255]));
        }
    }

    // A texture based on framebuffer data
    let mut texture = Texture::from_image(&mut window.factory, &img, &TextureSettings::new())
        .unwrap();

    while let Some(e) = window.next() {
        let win_size = window.size();
        // Emulate until a frame is ready
        let frame_buffer = cpu.exec_one_frame();

        // Load frame_buffer into ImageBuffer
        for x in 0..fb_size.0 {
            for y in 0..fb_size.1 {
                let fb_pixel = frame_buffer[(fb_size.0 * y + x) as usize];
                img.put_pixel(
                    x, y,
                    im::Rgba([fb_pixel.0, fb_pixel.1, fb_pixel.2, 255]));
            }
        }

        // Update the texture from the new ImageBuffer data
        texture.update(&mut window.encoder, &img).unwrap();

        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            image(
                &texture,
                c.transform.scale(
                    win_size.width as f64 / fb_size.0 as f64,
                    win_size.height  as f64 / fb_size.1 as f64),
                g);
        });

        match e.press_args() {
            Some(Button::Keyboard(_)) => {
                window.set_should_close(true);
            }
            Some(Button::Mouse(_)) => {
                println!("mouse event");
            }
            Some(Button::Controller(_)) => {
                println!("controller event");
            }
            // TODO: handle other keypresses and pass to emulator core
            None => {}
        }
    }
}
