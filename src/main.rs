#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate glium;

mod jeebie;

use jeebie::core::cpu::CPU;
use jeebie::memory::MMU;
use jeebie::cart::Cartridge;

use jeebie::frontend::GpuFrontend;
use std::time::Duration;

fn main() {
    // TODO: make a proper main
    let mut front = jeebie::frontend::glium::GliumFrontend::new_with_size((160, 144));
    let cart = Cartridge::new_with_path("path_to_test_rom");
    
    // MMU outlives everything    
    let mut mmu = MMU::new();    
    mmu.load_rom(&cart);
    let mut cpu = CPU::new(&mut mmu);
                
    while front.should_run {
        let frame = cpu.exec_one_frame();
        front.display_frame(frame);
        front.update();
        // TODO: figure some decent wait time that doesn't blow things up.
        std::thread::sleep(Duration::from_millis(10));
    }
}
