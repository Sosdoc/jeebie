mod gbe;

use gbe::cpu::*;
use gbe::instr::*;

fn main() {
    println!("Hello, cpu!");

    let mut cpu = CPU::new();
    println!("{:?}", cpu);

    gbe::instr::LDrn_b(&mut cpu);
    println!("{:?}", cpu);
}
