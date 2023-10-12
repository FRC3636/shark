use palette::IntoColor;
use shark::{primitives::*, Fragment, Shader, ShaderExt};

fn main() {
    let shader = color(palette::Srgb::new(1.0, 0.0, 1.0)).stride(Off, 5);
    for i in 0.. {
        println!("{:?}", shader.shade(Fragment { time: 0.0, pos: i }));
    }
}
