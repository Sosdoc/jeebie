#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate glium;
extern crate rand;

mod jeebie;

use jeebie::frontend::GpuFrontend;
use std::time::Duration;
use rand::distributions::{IndependentSample, Range};

fn main() {
    // TODO: make a proper main
    let mut front = jeebie::frontend::glium::GliumFrontend::new_with_size((160, 144));

    let tex_size = (160 * 144) as usize;
    let mut fb = Vec::with_capacity(tex_size);

    for _ in 0..tex_size {
        fb.push((0, 0, 0));
    }
       
    let range = Range::new(0, 255);
    let mut rng = rand::thread_rng();
    
    while front.should_run {
        for i in 0..tex_size {
            let a = range.ind_sample(&mut rng);
            fb[i] = (a, a, a);
        }

        front.display_frame(&fb);
        front.update();
        
        std::thread::sleep(Duration::from_millis(10));
    }
}
