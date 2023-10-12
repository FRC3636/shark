use palette::{Oklab, IntoColor, Mix};

use crate::{Fragment, Shader};

pub struct Off;
impl Shader for Off {
    type Output = Oklab;

    fn shade(&self, _frag: Fragment) -> Self::Output {
        // Full black
        Oklab::new(0.0, 0.0, 0.0)
    }
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

pub fn mix<S: Shader, E: Shader>(
    start: S,
    end: E,
    factor: f32,
) -> Interpolate<S, E> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |_| factor),
    }
}

pub fn position_gradient<S: Shader, E: Shader, I: Fn(usize) -> f32 + 'static>(start: S, end: E, interpolator: I) -> Interpolate<S, E> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| {
            let factor = (interpolator)(frag.pos);
            factor
        }),
    }
}

pub fn time_gradient<S: Shader, E: Shader, I: Fn(f32) -> f32 + 'static>(start: S, end: E, interpolator: I) -> Interpolate<S, E> {
    Interpolate {
        start,
        end,
        interpolator: Box::new(move |frag| {
            let factor = (interpolator)(frag.time);
            factor
        }),
    }
}