use palette::IntoColor;
use shark::{primitives::*, Fragment, Shader, ShaderExt};

fn main() {
    let shader = Random.stride(Off, 5);
    for i in 0.. {
        println!("{:?}", shader.shade(Fragment { time: 0.0, pos: i }));
    }
}
