#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate glium;

mod jeebie;

use jeebie::core::cpu::*;
use jeebie::frontend::GpuFrontend;

fn main() {
    // TODO: make a proper main
    println!("Hello, cpu!");

    let mut cpu = CPU::new();
    println!("{:?}", cpu);

    jeebie::instr::load::LD_HLm_a(&mut cpu);
    println!("{:?}", cpu);

    let mut front = jeebie::frontend::glium::GliumFrontend::new_with_size((160, 144));

    let tex_size = (160 * 144) as usize;
    let mut fb = Vec::with_capacity(tex_size);

    for _ in 0..tex_size {
        fb.push((255, 0, 0));
    }

    while front.should_run {
        for i in 0..tex_size - 1000 {
            fb[i] = (0, 255, 255);
        }

        front.display_frame(&fb);
        front.update();
    }
}
