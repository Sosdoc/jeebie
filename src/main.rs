#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate piston_window;

mod jeebie;

use jeebie::frontend::piston::PistonFrontend;
use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;

use std::time::Duration;

fn main() {
    let mut front = PistonFrontend::new_with_size((160, 144));

    // MMU outlives everything
    let mut mmu = MMU::new();
    let mut cpu = CPU::new(&mut mmu);

    while front.running {
        let _ = cpu.exec_one_frame();

        front.draw();
        front.update();

        // TODO: figure some decent wait time that doesn't blow things up.
        std::thread::sleep(Duration::from_millis(10));
    }
}
