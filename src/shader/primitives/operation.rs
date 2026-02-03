use num::ToPrimitive;
use palette::{FromColor, Hsl, IntoColor, LinSrgb, Mix, ShiftHue};

use crate::shader::{Shader, Vertex, VertexDim};
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, fmt::Debug};

pub struct Interpolate<S: Shader<F>, E: Shader<F>, F: Vertex> {
    start: S,
    end: E,
    interpolator: Box<dyn Fn(F) -> f64 + Send + Sync>,
}
impl<S: Shader<F>, E: Shader<F>, F: Vertex> Shader<F> for Interpolate<S, E, F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let factor = (self.interpolator)(frag);

        let start = self.start.shade(frag).into_color();
        let end = self.end.shade(frag).into_color();

        start.mix(end, factor)
    }
}

#[cfg(feature = "alloc")]
impl<S: Shader<F> + Debug, E: Shader<F> + Debug, F: Vertex + Debug> Debug for Interpolate<S, E, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Interpolate")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish_non_exhaustive()
    }
}

pub fn mix<F: Vertex, S: Shader<F>, E: Shader<F>>(
    start: S,
    end: E,
    factor: f64,
) -> Interpolate<S, E, F> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |_| factor),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RotateHue<F: Vertex, S: Shader<F>> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
    angle: f64,
}
impl<F: Vertex, S: Shader<F>> Shader<F> for RotateHue<F, S> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let col = self.shader.shade(frag).into_color();
        let col = Hsl::from_color(col);
        col.shift_hue(self.angle).into_color()
    }
}

pub fn rotate_hue<F: Vertex, S: Shader<F>>(shader: S, angle: f64) -> RotateHue<F, S> {
    RotateHue {
        _marker: core::marker::PhantomData,
        shader,
        angle,
    }
}

pub fn position_gradient<
    V: Vertex,
    S: Shader<V>,
    E: Shader<V>,
    I: Fn(f64) -> f64 + Send + Sync + 'static,
>(
    start: S,
    end: E,
    interpolator: I,
) -> Interpolate<S, E, V> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| (interpolator)(frag.pos().iter().sum())),
    }
}

pub fn time_gradient<
    F: Vertex,
    S: Shader<F>,
    E: Shader<F>,
    I: Fn(f64) -> f64 + Send + Sync + 'static,
>(
    start: S,
    end: E,
    interpolator: I,
) -> Interpolate<S, E, F> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| (interpolator)(frag.time())),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModPosition<S: Shader<F>, M: ToPrimitive, F: Vertex> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
    modulo: M,
}

impl<V: Vertex, S: Shader<V>, M: ToPrimitive + Send + Sync> Shader<V> for ModPosition<S, M, V> {
    type Output = S::Output;

    fn shade(&self, mut frag: V) -> Self::Output {
        frag.pos_mut().iter_mut().for_each(|p| {
            *p %= self
                .modulo
                .to_f64()
                .expect("Could not convert modulo type to f64.");
        });
        self.shader.shade(frag)
    }
}

pub fn mod_position<F: Vertex, S: Shader<F>, M: ToPrimitive>(
    shader: S,
    modulo: M,
) -> ModPosition<S, M, F> {
    ModPosition {
        _marker: core::marker::PhantomData,
        shader,
        modulo,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModTime<F: Vertex, S: Shader<F>, M: ToPrimitive> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
    modulo: M,
}

impl<F: Vertex, S: Shader<F>, M: ToPrimitive + Send + Sync> Shader<F> for ModTime<F, S, M> {
    type Output = S::Output;

    fn shade(&self, mut frag: F) -> Self::Output {
        *frag.time_mut() = frag.time()
            % self
                .modulo
                .to_f64()
                .expect("Could not convert modulo type to f64.");

        self.shader.shade(frag)
    }
}

pub fn mod_time<F: Vertex, S: Shader<F>, M: ToPrimitive>(shader: S, modulo: M) -> ModTime<F, S, M> {
    ModTime {
        _marker: core::marker::PhantomData,
        shader,
        modulo,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Extrude<const D: usize, F: Vertex, S: Shader<F>> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
}

impl<
        const D_START: usize,
        VStart: VertexDim<D_START>,
        VEnd: VertexDim<{ D_START + 1 }> + Into<VStart>,
        S: Shader<VStart>,
    > Shader<VEnd> for Extrude<D_START, VStart, S>
{
    type Output = S::Output;

    fn shade(&self, frag: VEnd) -> Self::Output {
        let new_frag: VStart = frag.into();
        self.shader.shade(new_frag)
    }
}

pub fn extrude<const D: usize, F: Vertex, S: Shader<F>>(shader: S) -> Extrude<D, F, S> {
    Extrude {
        _marker: core::marker::PhantomData,
        shader,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScaleTime<F: Vertex, S: Shader<F>> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
    scale: f64,
}

impl<F: Vertex, S: Shader<F>> Shader<F> for ScaleTime<F, S> {
    type Output = S::Output;

    fn shade(&self, mut frag: F) -> Self::Output {
        *frag.time_mut() *= self.scale;
        self.shader.shade(frag)
    }
}

pub fn scale_time<F: Vertex, S: Shader<F>>(shader: S, scale: f64) -> ScaleTime<F, S> {
    ScaleTime {
        _marker: core::marker::PhantomData,
        shader,
        scale,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScalePosition<F: Vertex, S: Shader<F>> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
    scale: f64,
}

impl<F: Vertex, S: Shader<F>> Shader<F> for ScalePosition<F, S> {
    type Output = S::Output;

    fn shade(&self, mut frag: F) -> Self::Output {
        let position = frag.pos_mut();
        for part in position.iter_mut() {
            *part *= self.scale;
        }
        self.shader.shade(frag)
    }
}

pub fn scale_position<F: Vertex, S: Shader<F>>(shader: S, scale: f64) -> ScalePosition<F, S> {
    ScalePosition {
        _marker: core::marker::PhantomData,
        shader,
        scale,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TranslatePosition<F: Vertex, S: Shader<F>, O> {
    _marker: core::marker::PhantomData<fn(F)>,
    shader: S,
    offset: O,
}

impl<V: Vertex, S: Shader<V>> Shader<V> for TranslatePosition<V, S, f64> {
    type Output = S::Output;

    fn shade(&self, mut frag: V) -> Self::Output {
        frag.pos_mut()
            .iter_mut()
            .for_each(|component| *component += self.offset);
        self.shader.shade(frag)
    }
}

pub fn translate_position<F: Vertex, O, S: Shader<F>>(
    shader: S,
    offset: O,
) -> TranslatePosition<F, S, O> {
    TranslatePosition {
        _marker: core::marker::PhantomData,
        shader,
        offset,
    }
}

macro_rules! simple_op_combinator {
    ($name:ident, $func_name:ident = $op:tt) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<L: Shader<F>, R: Shader<F>, F: Vertex> {
            _marker: core::marker::PhantomData<fn(F)>,
            left: L,
            right: R,
        }
        impl<L: Shader<F>, R: Shader<F>, F: Vertex> Shader<F> for $name<L, R, F> {
            type Output = LinSrgb<f64>;

            fn shade(&self, frag: F) -> Self::Output {
                let lhs = self.left.shade(frag).into_color();
                let rhs = self.right.shade(frag).into_color();
                lhs $op rhs
            }
        }

        pub fn $func_name<L: Shader<F>, R: Shader<F>, F: Vertex>(left: L, right: R) -> $name<L, R, F> {
            $name {
                _marker: core::marker::PhantomData,
                left,
                right,
            }
        }
    };
}

simple_op_combinator!(Add, add = +);
simple_op_combinator!(Subtract, subtract = -);
simple_op_combinator!(Multiply, multiply = *);
simple_op_combinator!(Divide, divide = /);

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

#[derive(Debug, Clone, Copy)]
pub struct VolumeBlur<const P: usize, F: Vertex, S: Shader<F>> {
    shader: S,
    radius: f64,
    _marker: core::marker::PhantomData<fn(F)>,
}
impl<const P: usize, V: Vertex, S: Shader<V>> Shader<V> for VolumeBlur<P, V, S>
where
    S::Output: Clone,
{
    type Output = LinSrgb<f64>;

    fn shade(&self, mut frag: V) -> Self::Output {
        let mut colors = [LinSrgb::<f64>::new(0.0, 0.0, 0.0); P];

        // Sample the shader at different positions
        for (i, color) in colors.iter_mut().enumerate() {
            let offset = lerp(-self.radius, self.radius, i as f64 / P as f64);
            frag.pos_mut().iter_mut().for_each(|c| *c += offset);
            *color = self.shader.shade(frag).into_color();
        }

        colors
            .iter()
            .fold(LinSrgb::new(0.0, 0.0, 0.0), |acc, c| acc + *c)
            / (colors.len() as f64)
    }
}

pub fn volume_blur<const P: usize, F: Vertex, S: Shader<F>>(
    shader: S,
    radius: f64,
) -> VolumeBlur<P, F, S> {
    VolumeBlur {
        shader,
        radius,
        _marker: core::marker::PhantomData,
    }
}
