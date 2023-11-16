use core::slice;

use crate::primitives::{
    checkerboard, extrude, mix, mod_position, mod_time, rotate_hue, Checkerboard, Extrude,
    Interpolate, ModPosition, ModTime, RotateHue,
};
use palette::{IntoColor, LinSrgb};

pub trait Shader<F: Fragment> {
    type Output: IntoColor<LinSrgb<f64>>;

    fn shade(&self, frag: F) -> Self::Output;
}

pub trait IntoShader<F: Fragment, O: IntoColor<LinSrgb<f64>>> {
    type Shader: Shader<F, Output = O>;
    fn into_shader(self) -> Self::Shader;
}

pub struct FnShader<I: Fragment, O: IntoColor<LinSrgb<f64>>, F: Fn(I) -> O> {
    _marker: std::marker::PhantomData<(I, O)>,
    f: F,
}
impl<I: Fragment, O: IntoColor<LinSrgb<f64>>, F: Fn(I) -> O> Shader<I> for FnShader<I, O, F> {
    type Output = O;

    fn shade(&self, frag: I) -> Self::Output {
        (self.f)(frag)
    }
}

impl<I: Fragment, O: IntoColor<LinSrgb<f64>>, F: Fn(I) -> O> IntoShader<I, O> for F {
    type Shader = FnShader<I, O, F>;

    fn into_shader(self) -> Self::Shader {
        FnShader {
            _marker: std::marker::PhantomData,
            f: self,
        }
    }
}

impl<F: Fragment, O: IntoColor<LinSrgb<f64>>> Shader<F> for dyn Fn(F) -> O {
    type Output = O;

    fn shade(&self, frag: F) -> Self::Output {
        (self)(frag)
    }
}

#[repr(C)]
pub struct ShaderExport<'a, F: Fragment> {
    shader: *const (),
    f: &'a extern "C" fn(*const (), F) -> LinSrgb<f64>,
}
// f will always be Send even when F isn't, and the only way to create shader is when it points to a static shader implementing Send
unsafe impl<'a, F: Fragment + Send> Send for ShaderExport<'a, F> {}

pub fn create_shader_export<S: Shader<F> + 'static + Send, F: Fragment>(
    shader: S,
) -> ShaderExport<'static, F> {
    let shader_ptr = (Box::leak(Box::new(shader)) as *const S).cast();

    extern "C" fn shader_export_fn<S: Shader<F>, F: Fragment>(
        shader: *const (),
        frag: F,
    ) -> LinSrgb<f64> {
        let shader = unsafe { &*(shader.cast::<S>()) };
        shader.shade(frag).into_color()
    }

    ShaderExport {
        shader: shader_ptr,
        f: &(shader_export_fn::<S, F> as extern "C" fn(*const (), F) -> LinSrgb<f64>),
    }
}

impl<F: Fragment> Shader<F> for ShaderExport<'static, F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        (self.f)(self.shader, frag)
    }
}

// #[cfg(feature = "fn_trait_v2")]
// impl<I: Fragment, O: IntoColor<LinSrgb<f64>>, F: Fn<(I,)>> Shader<I> for F<Output = O> {

// }

pub trait Fragment: Clone + Copy + std::fmt::Debug {
    fn time(&self) -> f64;
    fn time_mut(&mut self) -> &mut f64;
    fn pos(&self) -> &[usize];
    fn pos_mut(&mut self) -> &mut [usize];
}

#[derive(Clone, Copy, Debug)]
pub struct FragOne {
    pub pos: usize,
    pub time: f64,
}
impl Fragment for FragOne {
    fn time(&self) -> f64 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f64 {
        &mut self.time
    }

    fn pos(&self) -> &[usize] {
        slice::from_ref(&self.pos)
    }

    fn pos_mut(&mut self) -> &mut [usize] {
        slice::from_mut(&mut self.pos)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FragTwo {
    pub pos: [usize; 2],
    pub time: f64,
}
impl Fragment for FragTwo {
    fn time(&self) -> f64 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f64 {
        &mut self.time
    }

    fn pos(&self) -> &[usize] {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut [usize] {
        &mut self.pos
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FragThree {
    pub pos: [usize; 3],
    pub time: f64,
}
impl Fragment for FragThree {
    fn time(&self) -> f64 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f64 {
        &mut self.time
    }

    fn pos(&self) -> &[usize] {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut [usize] {
        &mut self.pos
    }
}

pub trait ShaderExt<F: Fragment>: Shader<F> + Sized {
    fn mix<S: Shader<F>>(self, other: S, factor: f64) -> Interpolate<Self, S, F> {
        mix(self, other, factor)
    }

    fn checkerboard<S: Shader<F>>(self, other: S, stride: usize) -> Checkerboard<F, Self, S>
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
