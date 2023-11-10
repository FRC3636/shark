#![cfg_attr(feature = "fn_trait_v2", feature(fn_trait_v2))]

pub mod primitives;
pub mod shader;

#[cfg(test)]
mod tests {
    use crate::{shader::{ShaderExt, Shader, FragOne}, primitives::{color, off}};

    #[test]
    fn shader_ext() {
        let shader = color(palette::Oklab::new(0.800, 0.159, -0.193)).checkerboard(off(), 2);

        shader.shade(FragOne { time: 0.0, pos: 0 });
    }
}
