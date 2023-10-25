use num::ToPrimitive;
use palette::{IntoColor, Mix, Okhsl, Oklab, OklabHue, FromColor, ShiftHue};
use rand::Rng;

use crate::{Fragment, Shader};

pub struct Off;
impl Shader for Off {
    type Output = Oklab;

    fn shade(&self, _frag: Fragment) -> Self::Output {
        // Full black
        Oklab::new(0.0, 0.0, 0.0)
    }
}

pub fn off() -> Off {
    Off
}

pub struct Color {
    color: Oklab,
}

impl Shader for Color {
    type Output = Oklab;

    fn shade(&self, _frag: Fragment) -> Self::Output {
        self.color
    }
}

pub fn color(color: impl IntoColor<Oklab>) -> Color {
    Color {
        color: color.into_color(),
    }
}

pub struct Interpolate<S: Shader, E: Shader> {
    start: S,
    end: E,
    interpolator: Box<dyn Fn(Fragment) -> f32>,
}
impl<S: Shader, E: Shader> Shader for Interpolate<S, E> {
    type Output = Oklab;

    fn shade(&self, frag: Fragment) -> Self::Output {
        let factor = (self.interpolator)(frag);

        let start = self.start.shade(frag).into_color();
        let end = self.end.shade(frag).into_color();

        start.mix(end, factor)
    }
}

pub fn mix<S: Shader, E: Shader>(start: S, end: E, factor: f32) -> Interpolate<S, E> {
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

    fn shade(&self, frag: Fragment) -> Self::Output {
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

pub fn position_gradient<S: Shader, E: Shader, I: Fn(usize) -> f32 + 'static>(
    start: S,
    end: E,
    interpolator: I,
) -> Interpolate<S, E> {
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
) -> Interpolate<S, E> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| {
            let factor = (interpolator)(frag.time);
            factor
        }),
    }
}

pub struct Stride<S: Shader, T: Shader> {
    shaders: (S, T),
    stride: usize,
}

impl<S: Shader, T: Shader> Shader for Stride<S, T> {
    type Output = Oklab;

    fn shade(&self, frag: Fragment) -> Self::Output {
        let first_color = frag.pos / self.stride % 2 == 0;

        if first_color {
            self.shaders.0.shade(frag).into_color()
        } else {
            self.shaders.1.shade(frag).into_color()
        }
    }
}

pub fn stride<S: Shader, T: Shader>(first: S, second: T, stride: usize) -> Stride<S, T> {
    Stride {
        shaders: (first, second),
        stride,
    }
}

pub struct Random;
impl Shader for Random {
    type Output = Okhsl;

    fn shade(&self, _frag: Fragment) -> Self::Output {
        let mut rng = rand::thread_rng();
        // Okhsl because it's the easiest to generate random colors with
        Okhsl::new(
            OklabHue::new(rng.gen_range(0.0..360.0)),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }
}

pub struct ModPosition<S: Shader, M: ToPrimitive> {
    shader: S,
    modulo: M,
}

impl<S: Shader, M: ToPrimitive> Shader for ModPosition<S, M> {
    type Output = S::Output;

    fn shade(&self, frag: Fragment) -> Self::Output {
        let frag = Fragment {
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

pub fn mod_position<S: Shader, M: ToPrimitive>(shader: S, modulo: M) -> ModPosition<S, M> {
    ModPosition { shader, modulo }
}

pub struct ModTime<S: Shader, M: ToPrimitive> {
    shader: S,
    modulo: M,
}

impl<S: Shader, M: ToPrimitive> Shader for ModTime<S, M> {
    type Output = S::Output;

    fn shade(&self, frag: Fragment) -> Self::Output {
        let frag = Fragment {
            time: frag.time
                % self
                    .modulo
                    .to_f32()
                    .expect("Could not convert modulo type to float."),
            ..frag
        };
        self.shader.shade(frag)
    }
}

pub fn mod_time<S: Shader, M: ToPrimitive>(shader: S, modulo: M) -> ModTime<S, M> {
    ModTime { shader, modulo }
}
