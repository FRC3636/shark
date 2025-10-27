#![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_const_exprs)]

extern crate alloc;

pub mod point;
pub mod shader;

#[cfg(test)]
mod tests {
    use crate::shader::{
        primitives::{color, off},
        FragOne, Shader, ShaderExt,
    };

    #[test]
    fn shader_ext() {
        let shader = color(palette::Oklab::new(0.800, 0.159, -0.193)).checkerboard(off(), 2.0);

        shader.shade(FragOne {
            time: 0.0,
            pos: [0.0],
        });
    }
}
