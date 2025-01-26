pub mod primitives;

use core::slice;

use palette::{IntoColor, LinSrgb};
use primitives::{
    add, checkerboard, divide, extrude, mix, mod_position, mod_time, multiply, rotate_hue,
    scale_position, scale_time, subtract, translate_position, volume_blur, Add, Checkerboard,
    Divide, Extrude, Interpolate, Memoize, ModPosition, ModTime, Multiply, RotateHue,
    ScalePosition, ScaleTime, Subtract, TranslatePosition, VolumeBlur,
};

pub trait Shader<F: Fragment>: Send + Sync {
    type Output: IntoColor<LinSrgb<f64>> + Send + Sync;

    fn shade(&self, frag: F) -> Self::Output;
}

pub trait IntoShader<F: Fragment, O: IntoColor<LinSrgb<f64>>> {
    type Shader: Shader<F, Output = O>;
    fn into_shader(self) -> Self::Shader;
}

#[derive(Clone, Copy, Debug)]
pub struct FnShader<I: Fragment, O: IntoColor<LinSrgb<f64>>, F: Fn(I) -> O + Send + Sync> {
    _marker: std::marker::PhantomData<(I, O)>,
    f: F,
}
impl<I: Fragment, O: IntoColor<LinSrgb<f64>> + Send + Sync, F: Fn(I) -> O + Send + Sync> Shader<I>
    for FnShader<I, O, F>
{
    type Output = O;

    fn shade(&self, frag: I) -> Self::Output {
        (self.f)(frag)
    }
}

impl<I: Fragment, O: IntoColor<LinSrgb<f64>> + Send + Sync, F: Fn(I) -> O + Send + Sync>
    IntoShader<I, O> for F
{
    type Shader = FnShader<I, O, F>;

    fn into_shader(self) -> Self::Shader {
        FnShader {
            _marker: std::marker::PhantomData,
            f: self,
        }
    }
}

impl<F: Fragment, O: IntoColor<LinSrgb<f64>> + Send + Sync> Shader<F>
    for dyn Fn(F) -> O + Send + Sync
{
    type Output = O;

    fn shade(&self, frag: F) -> Self::Output {
        (self)(frag)
    }
}

pub trait Fragment: Clone + Copy + std::fmt::Debug + Send + Sync {
    fn time(&self) -> f64;
    fn time_mut(&mut self) -> &mut f64;
    fn pos(&self) -> &[f64];
    fn pos_mut(&mut self) -> &mut [f64];
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct FragOne {
    pub pos: f64,
    pub time: f64,
}
impl Fragment for FragOne {
    fn time(&self) -> f64 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f64 {
        &mut self.time
    }

    fn pos(&self) -> &[f64] {
        slice::from_ref(&self.pos)
    }

    fn pos_mut(&mut self) -> &mut [f64] {
        slice::from_mut(&mut self.pos)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FragTwo {
    pub pos: [f64; 2],
    pub time: f64,
}
impl Fragment for FragTwo {
    fn time(&self) -> f64 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f64 {
        &mut self.time
    }

    fn pos(&self) -> &[f64] {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut [f64] {
        &mut self.pos
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FragThree {
    pub pos: [f64; 3],
    pub time: f64,
}
impl Fragment for FragThree {
    fn time(&self) -> f64 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f64 {
        &mut self.time
    }

    fn pos(&self) -> &[f64] {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut [f64] {
        &mut self.pos
    }
}

pub trait ShaderExt<F: Fragment>: Shader<F> + Sized {
    fn mix<S: Shader<F>>(self, other: S, factor: f64) -> Interpolate<Self, S, F> {
        mix(self, other, factor)
    }

    fn checkerboard<S: Shader<F>>(self, other: S, stride: f64) -> Checkerboard<F, Self, S>
    where
        Self: Shader<F>,
    {
        checkerboard(self, other, stride)
    }

    fn mod_position<M: num::ToPrimitive>(self, modulo: M) -> ModPosition<Self, M, F> {
        mod_position(self, modulo)
    }

    fn mod_time<M: num::ToPrimitive>(self, modulo: M) -> ModTime<F, Self, M> {
        mod_time(self, modulo)
    }

    fn rotate_hue(self, angle: f64) -> RotateHue<F, Self> {
        rotate_hue(self, angle)
    }

    fn extrude(self) -> Extrude<F, Self> {
        extrude(self)
    }

    fn scale_time(self, factor: f64) -> ScaleTime<F, Self> {
        scale_time(self, factor)
    }

    fn scale_position(self, scale: f64) -> ScalePosition<F, Self> {
        scale_position(self, scale)
    }

    fn translate_position<O>(self, offset: O) -> TranslatePosition<F, Self, O> {
        translate_position(self, offset)
    }

    fn add<O: Shader<F>>(self, other: O) -> Add<Self, O, F> {
        add(self, other)
    }
    fn subtract<O: Shader<F>>(self, other: O) -> Subtract<Self, O, F> {
        subtract(self, other)
    }

    fn multiply<O: Shader<F>>(self, other: O) -> Multiply<Self, O, F> {
        multiply(self, other)
    }
    fn divide<O: Shader<F>>(self, other: O) -> Divide<Self, O, F> {
        divide(self, other)
    }

    fn volume_blur(self, radius: f64, density: f64) -> VolumeBlur<F, Self> {
        volume_blur(self, radius, density)
    }
}
impl<F: Fragment, T> ShaderExt<F> for T where T: Shader<F> {}

#[cfg(test)]
mod tests {
    use super::{FragOne, Shader};
    use crate::shader::IntoShader;
    use palette::Srgb;

    #[test]
    fn fn_shaders() {
        let shader = (|_: FragOne| Srgb::new(0.0, 1.0, 0.0)).into_shader();

        shader.shade(FragOne { pos: 0, time: 0.0 });
    }
}
