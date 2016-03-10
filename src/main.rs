#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate glium;

mod gbe;

use gbe::core::cpu::*;
use gbe::frontend::GpuFrontend;

fn main() {
    println!("Hello, cpu!");

    let mut cpu = CPU::new();
    println!("{:?}", cpu);

    gbe::instr::load::LD_HLm_a(&mut cpu);
    println!("{:?}", cpu);

    // TODO: remove after testing this thing.
    let mut front = gbe::frontend::glium::GliumFrontend::new_with_size((160, 144));

    let tex_size = (160 * 144) as usize;
    let mut fb = Vec::with_capacity(tex_size);

    for _ in 0..tex_size {
        fb.push((255, 0, 0));
    }

    while front.should_run {
        for i in 0..tex_size - 1000 {
            fb[i] = (0, 255, 255);
        }

        front.display_frame(fb.as_slice());
        front.update();
    }
}
