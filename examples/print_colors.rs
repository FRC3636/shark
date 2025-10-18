extern crate shark;

use crate::shark::shader::ShaderExt;
use palette::Srgb;
use shark::shader::{primitives::*, FragOne, FragThree, Shader};

fn main() {
    // A gradient that goes from off to purple in 5 leds that repeats after every 5 leds.
    let gradient_shader =
        position_gradient(Off, color(Srgb::new(1.0, 0.0, 1.0)), |pos| pos / 5.0).mod_position(5);

    let shader = color(Srgb::new(0.0, 1.0, 1.0)).checkerboard(gradient_shader, 10.0);
    for i in 0..500000 {
        println!(
            "{:?}",
            shader.shade(FragThree {
                time: 0.0,
                pos: [i as _, 0.0, 0.0]
            })
        );
    }
}
