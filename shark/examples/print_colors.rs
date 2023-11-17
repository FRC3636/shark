extern crate shark;

use shark::palette::{IntoColor, Srgb};
use shark::shader::{FragThree, ShaderExport};
use shark::{
    primitives::*,
    shader::{FragOne, Fragment, Shader, ShaderExt},
};

fn main() {
    // A gradient that goes from off to purple in 5 leds that repeats after every 5 leds.
    let gradient_shader = position_gradient(off(), color(Srgb::new(1.0, 0.0, 1.0)), |pos| {
        pos as f32 / 5.0
    })
    .mod_position(5);

    let shader = color(Srgb::new(0.0, 1.0, 1.0)).checkerboard(gradient_shader, 10.0);
    for i in 0..500000 {
        println!("{:?}", shader.shade(FragOne { time: 0.0, pos: i as _ }));
    }
}
