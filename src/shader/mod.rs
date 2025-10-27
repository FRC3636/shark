pub mod primitives;

use palette::{IntoColor, LinSrgb};
use primitives::{
    add, checkerboard, divide, extrude, mix, mod_position, mod_time, multiply, rotate_hue,
    scale_position, scale_time, subtract, translate_position, volume_blur, Add, Checkerboard,
    Divide, Extrude, Interpolate, ModPosition, ModTime, Multiply, RotateHue, ScalePosition,
    ScaleTime, Subtract, TranslatePosition, VolumeBlur,
};

pub trait Shader<F: Vertex>: Send + Sync {
    type Output: IntoColor<LinSrgb<f64>> + Send + Sync;

    fn shade(&self, frag: F) -> Self::Output;
}

pub trait IntoShader<F: Vertex, O: IntoColor<LinSrgb<f64>>> {
    type Shader: Shader<F, Output = O>;
    fn into_shader(self) -> Self::Shader;
}

#[derive(Clone, Copy, Debug)]
pub struct FnShader<I: Vertex, O: IntoColor<LinSrgb<f64>>, F: Fn(I) -> O + Send + Sync> {
    _marker: core::marker::PhantomData<(I, O)>,
    f: F,
}
impl<I: Vertex, O: IntoColor<LinSrgb<f64>> + Send + Sync, F: Fn(I) -> O + Send + Sync> Shader<I>
    for FnShader<I, O, F>
{
    type Output = O;

    fn shade(&self, frag: I) -> Self::Output {
        (self.f)(frag)
    }
}

impl<I: Vertex, O: IntoColor<LinSrgb<f64>> + Send + Sync, F: Fn(I) -> O + Send + Sync>
    IntoShader<I, O> for F
{
    type Shader = FnShader<I, O, F>;

    fn into_shader(self) -> Self::Shader {
        FnShader {
            _marker: core::marker::PhantomData,
            f: self,
        }
    }
}

impl<F: Vertex, O: IntoColor<LinSrgb<f64>> + Send + Sync> Shader<F>
    for dyn Fn(F) -> O + Send + Sync
{
    type Output = O;

    fn shade(&self, frag: F) -> Self::Output {
        (self)(frag)
    }
}

pub trait Vertex: Clone + Copy + core::fmt::Debug + Send + Sync {
    fn time(&self) -> f64;
    fn time_mut(&mut self) -> &mut f64;
    fn pos(&self) -> &[f64];
    fn pos_mut(&mut self) -> &mut [f64];
}
pub trait VertexDim<const D: usize>: Vertex {
    fn pos_sized(&self) -> &[f64; D];
    fn pos_sized_mut(&mut self) -> &mut [f64; D];
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct FragOne {
    pub pos: [f64; 1],
    pub time: f64,
}
impl Vertex for FragOne {
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
impl VertexDim<1> for FragOne {
    fn pos_sized(&self) -> &[f64; 1] {
        &self.pos
    }

    fn pos_sized_mut(&mut self) -> &mut [f64; 1] {
        &mut self.pos
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FragTwo {
    pub pos: [f64; 2],
    pub time: f64,
}
impl Vertex for FragTwo {
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
impl VertexDim<2> for FragTwo {
    fn pos_sized(&self) -> &[f64; 2] {
        &self.pos
    }

    fn pos_sized_mut(&mut self) -> &mut [f64; 2] {
        &mut self.pos
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FragThree {
    pub pos: [f64; 3],
    pub time: f64,
}
impl Vertex for FragThree {
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

pub trait ShaderExt<F: Vertex>: Shader<F> + Sized {
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

    fn volume_blur(self, radius: f64, num_samples: usize) -> VolumeBlur<F, Self> {
        volume_blur(self, radius, num_samples)
    }
}
impl<F: Vertex, T> ShaderExt<F> for T where T: Shader<F> {}

pub trait ShaderExtrudeExt<const D: usize, F: Vertex + VertexDim<D>>: Shader<F> + Sized {
    fn extrude(self) -> Extrude<D, F, Self> {
        extrude(self)
    }
}
impl<const D: usize, F: Vertex + VertexDim<D>, T> ShaderExtrudeExt<D, F> for T where T: Shader<F> {}

#[cfg(test)]
mod tests {
    use super::{FragOne, Shader};
    use crate::shader::IntoShader;
    use palette::Srgb;

    #[test]
    fn fn_shaders() {
        let shader = (|_: FragOne| Srgb::new(0.0, 1.0, 0.0)).into_shader();

        shader.shade(FragOne { pos: [0.0], time: 0.0 });
    }
}
