pub mod point;
pub mod shader;

#[cfg(test)]
mod tests {
    use crate::{
        primitives::{color, off},
        shader::{FragOne, Shader, ShaderExt},
    };

    #[test]
    fn shader_ext() {
        let shader = color(palette::Oklab::new(0.800, 0.159, -0.193)).checkerboard(off(), 2);

        shader.shade(FragOne { time: 0.0, pos: 0 });
    }
}
