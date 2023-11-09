use num::ToPrimitive;
use palette::{FromColor, IntoColor, Mix, Okhsl, Oklab, OklabHue, ShiftHue};
use rand::Rng;

use crate::shader::{FragOne, FragThree, FragTwo, Fragment, Shader};

pub struct Off<F: Fragment> {
    _marker: std::marker::PhantomData<F>,
}
impl<F: Fragment> Shader for Off<F> {
    type Output = Oklab;
    type Fragment = F;

    fn shade(&self, _frag: Self::Fragment) -> Self::Output {
        // Full black
        Oklab::new(0.0, 0.0, 0.0)
    }
}

pub fn off<F: Fragment>() -> Off<F> {
    Off {
        _marker: std::marker::PhantomData,
    }
}

pub struct Color<F: Fragment> {
    color: Oklab,
    _marker: std::marker::PhantomData<F>,
}

impl<F: Fragment> Shader for Color<F> {
    type Output = Oklab;
    type Fragment = F;

    fn shade(&self, _frag: Self::Fragment) -> Self::Output {
        self.color
    }
}

pub fn color<F: Fragment>(color: impl IntoColor<Oklab>) -> Color<F> {
    Color {
        color: color.into_color(),
        _marker: std::marker::PhantomData,
    }
}

pub struct Interpolate<S: Shader, E: Shader, F: Fragment> {
    start: S,
    end: E,
    interpolator: Box<dyn Fn(F) -> f32>,
}
impl<S: Shader<Fragment = F>, E: Shader<Fragment = F>, F: Fragment> Shader
    for Interpolate<S, E, F>
{
    type Output = Oklab;
    type Fragment = F;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let factor = (self.interpolator)(frag);

        let start = self.start.shade(frag).into_color();
        let end = self.end.shade(frag).into_color();

        start.mix(end, factor)
    }
}

pub fn mix<S: Shader, E: Shader>(start: S, end: E, factor: f32) -> Interpolate<S, E, S::Fragment> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |_| factor),
    }
}

pub struct Rotate<S: Shader> {
    shader: S,
    angle: f32,
}
impl<S: Shader> Shader for Rotate<S> {
    type Output = Oklab;
    type Fragment = S::Fragment;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let col = self.shader.shade(frag);
        let col: Okhsl = Okhsl::from_color(col.into_color());
        col.shift_hue(self.angle).into_color()
    }
}

pub fn rotate<S: Shader>(shader: S, angle: f32) -> Rotate<S> {
    Rotate {
        shader,
        angle: angle.to_radians(),
    }
}

// A one dimensional gradient
pub fn position_gradient<S: Shader, E: Shader, I: Fn(usize) -> f32 + 'static>(
    start: S,
    end: E,
    interpolator: I,
) -> Interpolate<S, E, FragOne> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| {
            let factor = (interpolator)(frag.pos);
            factor
        }),
    }
}

pub fn time_gradient<S: Shader, E: Shader, I: Fn(f32) -> f32 + 'static>(
    start: S,
    end: E,
    interpolator: I,
) -> Interpolate<S, E, S::Fragment> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| {
            let factor = (interpolator)(frag.time());
            factor
        }),
    }
}

pub struct Checkerboard<F: Fragment, S: Shader<Fragment = F>, T: Shader<Fragment = F>> {
    shaders: (S, T),
    stride: usize,
}

impl<F: Fragment, S: Shader<Fragment = F>, T: Shader<Fragment = F>> Shader
    for Checkerboard<F, S, T>
{
    type Output = Oklab;
    type Fragment = F;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let first_color = frag
            .pos()
            .iter()
            .map(|pos| pos / self.stride)
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

pub fn checkerboard<F: Fragment, S: Shader<Fragment = F>, T: Shader<Fragment = F>>(
    first: S,
    second: T,
    stride: usize,
) -> Checkerboard<F, S, T> {
    Checkerboard {
        shaders: (first, second),
        stride,
    }
}

pub struct Random<F: Fragment> {
    _marker: std::marker::PhantomData<F>,
}
impl<F: Fragment> Shader for Random<F> {
    type Output = Okhsl;
    type Fragment = F;

    fn shade(&self, _frag: Self::Fragment) -> Self::Output {
        let mut rng = rand::thread_rng();
        // Okhsl because it's the easiest to generate random colors with
        Okhsl::new(
            OklabHue::new(rng.gen_range(0.0..360.0)),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }
}

pub fn random<F: Fragment>() -> Random<F> {
    Random {
        _marker: std::marker::PhantomData,
    }
}

pub struct ModPosition<S: Shader<Fragment = F>, M: ToPrimitive, F: Fragment> {
    shader: S,
    modulo: M,
}

impl<S: Shader<Fragment = FragOne>, M: ToPrimitive> Shader for ModPosition<S, M, FragOne> {
    type Output = S::Output;
    type Fragment = S::Fragment;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let frag = FragOne {
            pos: frag.pos
                % self
                    .modulo
                    .to_usize()
                    .expect("Could not convert modulo type to usize."),
            ..frag
        };
        self.shader.shade(frag)
    }
}

impl<S: Shader<Fragment = FragTwo>, M: ToPrimitive> Shader for ModPosition<S, M, FragTwo> {
    type Output = S::Output;
    type Fragment = S::Fragment;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let modulo = self
            .modulo
            .to_usize()
            .expect("Could not convert modulo type to usize.");

        let frag = FragTwo {
            pos: [frag.pos[0] % modulo, frag.pos[1] % modulo],
            ..frag
        };
        self.shader.shade(frag)
    }
}

impl<S: Shader<Fragment = FragThree>, M: ToPrimitive> Shader for ModPosition<S, M, FragThree> {
    type Output = S::Output;
    type Fragment = S::Fragment;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let modulo = self
            .modulo
            .to_usize()
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

pub fn mod_position<S: Shader, M: ToPrimitive>(
    shader: S,
    modulo: M,
) -> ModPosition<S, M, S::Fragment> {
    ModPosition { shader, modulo }
}

pub struct ModTime<S: Shader, M: ToPrimitive> {
    shader: S,
    modulo: M,
}

impl<S: Shader, M: ToPrimitive> Shader for ModTime<S, M> {
    type Output = S::Output;
    type Fragment = S::Fragment;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let mut frag = frag.clone();

        *frag.time_mut() = frag.time()
            % self
                .modulo
                .to_f32()
                .expect("Could not convert modulo type to f32.");

        self.shader.shade(frag)
    }
}

pub fn mod_time<S: Shader, M: ToPrimitive>(shader: S, modulo: M) -> ModTime<S, M> {
    ModTime { shader, modulo }
}

pub struct Extrude<F: Fragment, S: Shader<Fragment = F>> {
    shader: S,
}

impl<S: Shader<Fragment = FragOne>> Shader for Extrude<FragOne, S> {
    type Output = S::Output;
    type Fragment = FragTwo;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let frag = FragOne {
            pos: frag.pos[0],
            time: frag.time,
        };
        self.shader.shade(frag)
    }
}

impl<S: Shader<Fragment = FragTwo>> Shader for Extrude<FragTwo, S> {
    type Output = S::Output;
    type Fragment = FragThree;

    fn shade(&self, frag: Self::Fragment) -> Self::Output {
        let frag = FragTwo {
            pos: [frag.pos[0], frag.pos[1]],
            time: frag.time,
        };
        self.shader.shade(frag)
    }
}

pub fn extrude<F: Fragment, S: Shader<Fragment = F>>(shader: S) -> Extrude<F, S> {
    Extrude { shader }
}
