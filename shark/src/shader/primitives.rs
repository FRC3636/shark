use num::ToPrimitive;
use palette::{FromColor, Hsl, IntoColor, LinSrgb, Mix, RgbHue, ShiftHue, Srgb};

use crate::shader::{FragOne, FragThree, FragTwo, Fragment, Shader};

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
    interpolator: Box<dyn Fn(F) -> f64 + Send>,
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
    I: Fn(f64) -> f64 + Send + 'static,
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
    I: Fn(f64) -> f64 + Send + 'static,
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

pub struct Random<F: Fragment> {
    _marker: std::marker::PhantomData<F>,
}
impl<F: Fragment> Shader<F> for Random<F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        Srgb::new(
            fastrand::f64(),
            fastrand::f64(),
            fastrand::f64(),
        ).into_color()
    }
}

pub fn random<F: Fragment>() -> Random<F> {
    Random {
        _marker: std::marker::PhantomData,
    }
}

pub struct ModPosition<S: Shader<F>, M: ToPrimitive, F: Fragment> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    modulo: M,
}

impl<S: Shader<FragOne>, M: ToPrimitive> Shader<FragOne> for ModPosition<S, M, FragOne> {
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

impl<S: Shader<FragTwo>, M: ToPrimitive> Shader<FragTwo> for ModPosition<S, M, FragTwo> {
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

impl<S: Shader<FragThree>, M: ToPrimitive> Shader<FragThree> for ModPosition<S, M, FragThree> {
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

pub struct ModTime<F: Fragment, S: Shader<F>, M: ToPrimitive> {
    _marker: std::marker::PhantomData<F>,
    shader: S,
    modulo: M,
}

impl<F: Fragment, S: Shader<F>, M: ToPrimitive> Shader<F> for ModTime<F, S, M> {
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

pub struct Rainbow<F: Fragment, S: Fn(F) -> f64> {
    _marker: std::marker::PhantomData<F>,
    selector: S,
}
impl<F: Fragment, S: Fn(F) -> f64> Shader<F> for Rainbow<F, S> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let t = (self.selector)(frag);
        Hsl::new(t % 360.0, 1.0, 0.5).into_color()
    }
}

pub fn rainbow<F: Fragment, S: Fn(F) -> f64>(selector: S) -> Rainbow<F, S> {
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