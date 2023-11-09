pub mod primitives;

use palette::{IntoColor, Oklab};
use primitives::{
    mix, mod_position, mod_time, rotate, stride, Interpolate, ModPosition, ModTime, Rotate, Stride,
};

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

    fn mod_position<M: num::ToPrimitive>(self, modulo: M) -> ModPosition<Self, M> {
        mod_position(self, modulo)
    }

    fn mod_time<M: num::ToPrimitive>(self, modulo: M) -> ModTime<Self, M> {
        mod_time(self, modulo)
    }

    fn rotate(self, angle: f32) -> Rotate<Self> {
        rotate(self, angle)
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
