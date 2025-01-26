use std::{collections::BTreeMap, fmt::Debug, sync::RwLock};

use num::{integer::Roots, ToPrimitive};
use palette::{FromColor, Hsl, IntoColor, LinSrgb, Mix, Okhsl, ShiftHue, Srgb};

use crate::shader::{FragOne, FragThree, FragTwo, Fragment, Shader};

#[derive(Debug, Clone, Copy)]
pub struct Off<F: Fragment> {
    _marker: std::marker::PhantomData<F>,
}
impl<F: Fragment> Shader<F> for Off<F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        // Full black
        LinSrgb::new(0.0, 0.0, 0.0)
    }
}

pub fn off<F: Fragment>() -> Off<F> {
    Off {
        _marker: std::marker::PhantomData,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color<F: Fragment> {
    color: LinSrgb<f64>,
    _marker: std::marker::PhantomData<F>,
}

impl<F: Fragment> Shader<F> for Color<F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        self.color
    }
}

pub fn color<F: Fragment>(color: impl IntoColor<LinSrgb<f64>>) -> Color<F> {
    Color {
        color: color.into_color(),
        _marker: std::marker::PhantomData,
    }
}

pub struct Interpolate<S: Shader<F>, E: Shader<F>, F: Fragment> {
    start: S,
    end: E,
    interpolator: Box<dyn Fn(F) -> f64 + Send + Sync>,
}
impl<S: Shader<F>, E: Shader<F>, F: Fragment> Shader<F> for Interpolate<S, E, F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let factor = (self.interpolator)(frag);

        let start = self.start.shade(frag).into_color();
        let end = self.end.shade(frag).into_color();

        start.mix(end, factor)
    }
}
impl<S: Shader<F> + Debug, E: Shader<F> + Debug, F: Fragment + Debug> Debug
    for Interpolate<S, E, F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interpolate")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish_non_exhaustive()
    }
}

pub fn mix<F: Fragment, S: Shader<F>, E: Shader<F>>(
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
pub struct RotateHue<F: Fragment, S: Shader<F>> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    angle: f64,
}
impl<F: Fragment, S: Shader<F>> Shader<F> for RotateHue<F, S> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let col = self.shader.shade(frag).into_color();
        let col = Hsl::from_color(col);
        col.shift_hue(self.angle).into_color()
    }
}

pub fn rotate_hue<F: Fragment, S: Shader<F>>(shader: S, angle: f64) -> RotateHue<F, S> {
    RotateHue {
        _marker: std::marker::PhantomData,
        shader,
        angle: angle.to_radians(),
    }
}

// A one dimensional gradient
pub fn position_gradient<
    S: Shader<FragOne>,
    E: Shader<FragOne>,
    I: Fn(f64) -> f64 + Send + Sync + 'static,
>(
    start: S,
    end: E,
    interpolator: I,
) -> Interpolate<S, E, FragOne> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| (interpolator)(frag.pos)),
    }
}

pub fn time_gradient<
    F: Fragment,
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
pub struct Checkerboard<F: Fragment, S: Shader<F>, T: Shader<F>> {
    _marker: std::marker::PhantomData<F>,
    shaders: (S, T),
    stride: f64,
}

impl<F: Fragment, S: Shader<F>, T: Shader<F>> Shader<F> for Checkerboard<F, S, T> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let first_color = frag
            .pos()
            .iter()
            .map(|pos| (pos / self.stride).abs() as usize)
            .sum::<usize>()
            % 2
            == 0;

        if first_color {
            self.shaders.0.shade(frag).into_color()
        } else {
            self.shaders.1.shade(frag).into_color()
        }
    }
}

pub fn checkerboard<F: Fragment, S: Shader<F>, T: Shader<F>>(
    first: S,
    second: T,
    stride: f64,
) -> Checkerboard<F, S, T> {
    Checkerboard {
        _marker: std::marker::PhantomData,
        shaders: (first, second),
        stride,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Random<F: Fragment> {
    _marker: std::marker::PhantomData<F>,
}
impl<F: Fragment> Shader<F> for Random<F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        Srgb::new(fastrand::f64(), fastrand::f64(), fastrand::f64()).into_color()
    }
}

pub fn random<F: Fragment>() -> Random<F> {
    Random {
        _marker: std::marker::PhantomData,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModPosition<S: Shader<F>, M: ToPrimitive, F: Fragment> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    modulo: M,
}

impl<S: Shader<FragOne>, M: ToPrimitive + Send + Sync> Shader<FragOne>
    for ModPosition<S, M, FragOne>
{
    type Output = S::Output;

    fn shade(&self, frag: FragOne) -> Self::Output {
        let frag = FragOne {
            pos: frag.pos
                % self
                    .modulo
                    .to_f64()
                    .expect("Could not convert modulo type to usize."),
            ..frag
        };
        self.shader.shade(frag)
    }
}

impl<S: Shader<FragTwo>, M: ToPrimitive + Send + Sync> Shader<FragTwo>
    for ModPosition<S, M, FragTwo>
{
    type Output = S::Output;

    fn shade(&self, frag: FragTwo) -> Self::Output {
        let modulo = self
            .modulo
            .to_f64()
            .expect("Could not convert modulo type to usize.");

        let frag = FragTwo {
            pos: [frag.pos[0] % modulo, frag.pos[1] % modulo],
            ..frag
        };
        self.shader.shade(frag)
    }
}

impl<S: Shader<FragThree>, M: ToPrimitive + Send + Sync> Shader<FragThree>
    for ModPosition<S, M, FragThree>
{
    type Output = S::Output;

    fn shade(&self, frag: FragThree) -> Self::Output {
        let modulo = self
            .modulo
            .to_f64()
            .expect("Could not convert modulo type to usize.");

        let frag = FragThree {
            pos: [
                frag.pos[0] % modulo,
                frag.pos[1] % modulo,
                frag.pos[2] % modulo,
            ],
            ..frag
        };
        self.shader.shade(frag)
    }
}

pub fn mod_position<F: Fragment, S: Shader<F>, M: ToPrimitive>(
    shader: S,
    modulo: M,
) -> ModPosition<S, M, F> {
    ModPosition {
        _marker: std::marker::PhantomData,
        shader,
        modulo,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModTime<F: Fragment, S: Shader<F>, M: ToPrimitive> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    modulo: M,
}

impl<F: Fragment, S: Shader<F>, M: ToPrimitive + Send + Sync> Shader<F> for ModTime<F, S, M> {
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

pub fn mod_time<F: Fragment, S: Shader<F>, M: ToPrimitive>(
    shader: S,
    modulo: M,
) -> ModTime<F, S, M> {
    ModTime {
        _marker: std::marker::PhantomData,
        shader,
        modulo,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Extrude<F: Fragment, S: Shader<F>> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
}

impl<S: Shader<FragOne>> Shader<FragTwo> for Extrude<FragOne, S> {
    type Output = S::Output;

    fn shade(&self, frag: FragTwo) -> Self::Output {
        let frag = FragOne {
            pos: frag.pos[0],
            time: frag.time,
        };
        self.shader.shade(frag)
    }
}

impl<S: Shader<FragTwo>> Shader<FragThree> for Extrude<FragTwo, S> {
    type Output = S::Output;

    fn shade(&self, frag: FragThree) -> Self::Output {
        let frag = FragTwo {
            pos: [frag.pos[0], frag.pos[1]],
            time: frag.time,
        };
        self.shader.shade(frag)
    }
}

pub fn extrude<F: Fragment, S: Shader<F>>(shader: S) -> Extrude<F, S> {
    Extrude {
        _marker: std::marker::PhantomData,
        shader,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rainbow<F: Fragment, S: Fn(F) -> f64 + Send + Sync> {
    _marker: std::marker::PhantomData<F>,
    selector: S,
}
impl<F: Fragment, S: Fn(F) -> f64 + Send + Sync> Shader<F> for Rainbow<F, S> {
    type Output = Okhsl<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let t = (self.selector)(frag);
        Okhsl::new(t % 360.0, 1.0, 0.5)
    }
}

pub fn rainbow<F: Fragment, S: Fn(F) -> f64 + Send + Sync>(selector: S) -> Rainbow<F, S> {
    Rainbow {
        _marker: std::marker::PhantomData,
        selector,
    }
}

pub fn time_rainbow<F: Fragment>() -> Rainbow<F, impl Fn(F) -> f64> {
    Rainbow {
        _marker: std::marker::PhantomData,
        selector: |frag| frag.time(),
    }
}

pub fn one_dimensional_position_rainbow() -> Rainbow<FragOne, impl Fn(FragOne) -> f64> {
    Rainbow {
        _marker: std::marker::PhantomData,
        selector: |frag| frag.pos,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScaleTime<F: Fragment, S: Shader<F>> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    scale: f64,
}

impl<F: Fragment, S: Shader<F>> Shader<F> for ScaleTime<F, S> {
    type Output = S::Output;

    fn shade(&self, mut frag: F) -> Self::Output {
        *frag.time_mut() *= self.scale;
        self.shader.shade(frag)
    }
}

pub fn scale_time<F: Fragment, S: Shader<F>>(shader: S, scale: f64) -> ScaleTime<F, S> {
    ScaleTime {
        _marker: std::marker::PhantomData,
        shader,
        scale,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScalePosition<F: Fragment, S: Shader<F>> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    scale: f64,
}

impl<F: Fragment, S: Shader<F>> Shader<F> for ScalePosition<F, S> {
    type Output = S::Output;

    fn shade(&self, mut frag: F) -> Self::Output {
        let position = frag.pos_mut();
        for part in position.iter_mut() {
            *part *= self.scale;
        }
        self.shader.shade(frag)
    }
}

pub fn scale_position<F: Fragment, S: Shader<F>>(shader: S, scale: f64) -> ScalePosition<F, S> {
    ScalePosition {
        _marker: std::marker::PhantomData,
        shader,
        scale,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TranslatePosition<F: Fragment, S: Shader<F>, O> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    offset: O,
}

impl<S: Shader<FragOne>> Shader<FragOne> for TranslatePosition<FragOne, S, f64> {
    type Output = S::Output;

    fn shade(&self, mut frag: FragOne) -> Self::Output {
        frag.pos += self.offset;
        self.shader.shade(frag)
    }
}

impl<S: Shader<FragTwo>> Shader<FragTwo> for TranslatePosition<FragTwo, S, [f64; 2]> {
    type Output = S::Output;

    fn shade(&self, mut frag: FragTwo) -> Self::Output {
        frag.pos[0] += self.offset[0];
        frag.pos[1] += self.offset[1];
        self.shader.shade(frag)
    }
}

impl<S: Shader<FragThree>> Shader<FragThree> for TranslatePosition<FragThree, S, [f64; 3]> {
    type Output = S::Output;

    fn shade(&self, mut frag: FragThree) -> Self::Output {
        frag.pos[0] += self.offset[0];
        frag.pos[1] += self.offset[1];
        frag.pos[2] += self.offset[2];
        self.shader.shade(frag)
    }
}

pub fn translate_position<F: Fragment, O, S: Shader<F>>(
    shader: S,
    offset: O,
) -> TranslatePosition<F, S, O> {
    TranslatePosition {
        _marker: std::marker::PhantomData,
        shader,
        offset,
    }
}

macro_rules! simple_op_combinator {
    ($name:ident, $func_name:ident = $op:tt) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<L: Shader<F>, R: Shader<F>, F: Fragment> {
            _marker: std::marker::PhantomData<F>,
            left: L,
            right: R,
        }
        impl<L: Shader<F>, R: Shader<F>, F: Fragment> Shader<F> for $name<L, R, F> {
            type Output = LinSrgb<f64>;

            fn shade(&self, frag: F) -> Self::Output {
                let lhs = self.left.shade(frag).into_color();
                let rhs = self.right.shade(frag).into_color();
                lhs $op rhs
            }
        }

        pub fn $func_name<L: Shader<F>, R: Shader<F>, F: Fragment>(left: L, right: R) -> $name<L, R, F> {
            $name {
                _marker: std::marker::PhantomData,
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

type OrderedFloat = ordered_float::OrderedFloat<f64>;

#[derive(Debug)]
pub struct Memoize<F: Fragment, S: Shader<F>, K> {
    shader: S,
    cache: RwLock<BTreeMap<K, S::Output>>,
}
impl<S: Shader<FragOne>> Shader<FragOne> for Memoize<FragOne, S, OrderedFloat>
where
    S::Output: Clone,
{
    type Output = S::Output;

    fn shade(&self, frag: FragOne) -> Self::Output {
        if let Some(color) = self.cache.read().unwrap().get(&frag.pos.into()) {
            return color.clone();
        }

        let color = self.shader.shade(frag);
        self.cache
            .write()
            .unwrap()
            .insert(frag.pos.into(), color.clone());
        color
    }
}
impl<S: Shader<FragTwo>> Shader<FragTwo> for Memoize<FragTwo, S, (OrderedFloat, OrderedFloat)>
where
    S::Output: Clone,
{
    type Output = S::Output;

    fn shade(&self, frag: FragTwo) -> Self::Output {
        let key = (frag.pos[0].into(), frag.pos[1].into());
        if let Some(color) = self.cache.read().unwrap().get(&key) {
            return color.clone();
        }

        let color = self.shader.shade(frag);
        self.cache.write().unwrap().insert(key, color.clone());
        color
    }
}
impl<S: Shader<FragThree>> Shader<FragThree>
    for Memoize<FragThree, S, (OrderedFloat, OrderedFloat, OrderedFloat)>
where
    S::Output: Clone,
{
    type Output = S::Output;

    fn shade(&self, frag: FragThree) -> Self::Output {
        let key = (frag.pos[0].into(), frag.pos[1].into(), frag.pos[2].into());
        if let Some(color) = self.cache.read().unwrap().get(&key) {
            dbg!();
            return color.clone();
        }

        let color = self.shader.shade(frag);
        self.cache.write().unwrap().insert(key, color.clone());
        color
    }
}

pub fn memoize<F: Fragment, S: Shader<F>, K>(shader: S) -> Memoize<F, S, K> {
    Memoize {
        shader,
        cache: RwLock::new(BTreeMap::new()),
    }
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

#[derive(Debug, Clone, Copy)]
pub struct VolumeBlur<F: Fragment, S: Shader<F>> {
    shader: S,
    radius: f64,
    samples: usize,
    _marker: std::marker::PhantomData<F>,
}
impl<S: Shader<FragOne>> Shader<FragOne> for VolumeBlur<FragOne, S>
where
    S::Output: Clone,
{
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: FragOne) -> Self::Output {
        let mut colors = Vec::with_capacity(self.samples as _);

        // Sample the shader at different positions
        for i in 0..self.samples {
            let offset = lerp(-self.radius, self.radius, i as f64 / self.samples as f64);
            let frag = FragOne {
                pos: frag.pos + offset,
                ..frag
            };
            colors.push(self.shader.shade(frag).into_color());
        }

        colors
            .iter()
            .fold(LinSrgb::new(0.0, 0.0, 0.0), |acc, c| acc + *c)
            / (colors.len() as f64)
    }
}
impl<S: Shader<FragTwo>> Shader<FragTwo> for VolumeBlur<FragTwo, S>
where
    S::Output: Clone,
{
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: FragTwo) -> Self::Output {
        let mut colors = Vec::with_capacity(self.samples * self.samples);

        let box_size = self.samples.sqrt();

        // Sample the shader at different positions
        for x in 0..box_size {
            let x_offset = lerp(-self.radius, self.radius, x as f64 / box_size as f64);
            for y in 0..box_size {
                let y_offset = lerp(-self.radius, self.radius, y as f64 / box_size as f64);
                let frag = FragTwo {
                    pos: [frag.pos[0] + x_offset, frag.pos[1] + y_offset],
                    ..frag
                };
                colors.push(self.shader.shade(frag).into_color());
            }
        }

        colors
            .iter()
            .fold(LinSrgb::new(0.0, 0.0, 0.0), |acc, c| acc + *c)
            / (colors.len() as f64)
    }
}
impl<S: Shader<FragThree>> Shader<FragThree> for VolumeBlur<FragThree, S>
where
    S::Output: Clone,
{
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: FragThree) -> Self::Output {
        let cube_size = self.samples.cbrt();
        let mut colors = Vec::with_capacity(cube_size * cube_size * cube_size);

        // Sample the shader at different positions
        for x in 0..cube_size {
            let x_offset = lerp(-self.radius, self.radius, x as f64 / cube_size as f64);
            for y in 0..cube_size {
                let y_offset = lerp(-self.radius, self.radius, y as f64 / cube_size as f64);
                for z in 0..cube_size {
                    let z_offset = lerp(-self.radius, self.radius, z as f64 / cube_size as f64);
                    let frag = FragThree {
                        pos: [
                            frag.pos[0] + x_offset,
                            frag.pos[1] + y_offset,
                            frag.pos[2] + z_offset,
                        ],
                        ..frag
                    };
                    colors.push(self.shader.shade(frag).into_color());
                }
            }
        }

        colors
            .iter()
            .fold(LinSrgb::new(0.0, 0.0, 0.0), |acc, c| acc + *c)
            / (colors.len() as f64)
    }
}

pub fn volume_blur<F: Fragment, S: Shader<F>>(shader: S, radius: f64, num_samples: usize) -> VolumeBlur<F, S> {
    VolumeBlur {
        shader,
        radius,
        samples: num_samples,
        _marker: std::marker::PhantomData,
    }
}
