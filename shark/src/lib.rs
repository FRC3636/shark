pub mod primitives;

use palette::{IntoColor, Oklab};
use primitives::{mix, stride, Interpolate, Stride};

pub trait Shader {
    type Output: IntoColor<Oklab>;

    fn shade(&self, frag: Fragment) -> Self::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    pub pos: usize,
    pub time: f32,
}

pub trait ShaderExt: Shader + Sized {
    fn mix<S: Shader>(self, other: S, factor: f32) -> Interpolate<Self, S> {
        mix(self, other, factor)
    }

    fn stride<S: Shader>(self, other: S, stride: usize) -> Stride<Self, S> {
        crate::stride(self, other, stride)
    }
}
impl<T> ShaderExt for T where T: Shader {}

#[cfg(test)]
mod tests {
    pub use crate::primitives::*;
    use crate::ShaderExt;

    #[test]
    fn shader_ext() {
        let shader = color(palette::Oklab::new(0.800, 0.159, -0.193)).stride(off(), 2);
    }
}
