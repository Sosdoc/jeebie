#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate glium;

mod gbe;

use gbe::cpu::*;
use gbe::frontend::GpuFrontend;

fn main() {
    println!("Hello, cpu!");

    let mut cpu = CPU::new();
    println!("{:?}", cpu);

    gbe::instr::load::LD_HLm_a(&mut cpu);
    println!("{:?}", cpu);

    // TODO: remove after testing this thing.
    let mut front = gbe::frontend::glium::GliumFrontend::new();
    front.display_frame(vec![]);
}
