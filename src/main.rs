#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod gbe;

use gbe::cpu::*;

fn main() {
    println!("Hello, cpu!");

    let mut cpu = CPU::new();
    println!("{:?}", cpu);

    gbe::instr::load::LD_HLm_a(&mut cpu);
    println!("{:?}", cpu);
}
