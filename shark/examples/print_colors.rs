use palette::{IntoColor, Srgb};
use shark::{primitives::*, Fragment, Shader, ShaderExt};

fn main() {
    // A gradient that goes from off to purple in 5 leds that repeats after every 5 leds.
    let gradient_shader =
        position_gradient(Off, color(Srgb::new(1.0, 0.0, 1.0)), |pos| pos as f32 / 5.0)
            .mod_position(5);

    let shader = color(Srgb::new(0.0, 1.0, 1.0)).stride(gradient_shader, 10);
    for i in 0.. {
        println!("{:?}", shader.shade(Fragment { time: 0.0, pos: i }));
    }
}
