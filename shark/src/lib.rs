use point::Points;
use shader::{ShaderExport, Fragment};

pub mod shader;
pub mod point;

#[repr(C)]
pub struct VisualizationExports<F: Fragment + 'static> {
    pub shader: ShaderExport<'static, F>,
    pub points: Points<'static>,
}

impl<F: Fragment> VisualizationExports<F> {
    pub fn new(shader: ShaderExport<'static, F>, points: Points<'static>) -> Self {
        Self { shader, points }
    }
}

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
