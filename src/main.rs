#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate piston_window;

mod jeebie;

use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;

use piston_window::*;

fn main() {

    // MMU outlives everything
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .build()
        .unwrap();

    window.set_title("Jeebie".to_string());

    while let Some(e) = window.next() {

        // Fit actual emulation in
        let _ = cpu.exec();

        window.draw_2d(&e, |_, g| {
            clear([0.66; 4], g);
            // TODO: draw images and handle buffers
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
