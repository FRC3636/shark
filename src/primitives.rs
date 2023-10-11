use palette::Oklab;

use crate::{Fragment, Shader};

pub struct Off;
impl Shader for Off {
    type Output = Oklab;

    fn shade(&self, _frag: Fragment) -> Self::Output {
        // Full black
        Oklab::new(0, 0, 0)
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

pub struct Mix<S: Shader, E: Shader> {
    start: S,
    end: E,
    factor: f32,
}

impl<S: Shader, E: Shader> Shader for Mix<S, E> {
    type Output = S::Output;

    fn shade(&self, frag: Fragment) -> Self::Output {
        let t = self.interpolator(frag.pos);
        let start = self.start.shade(frag);
        let end = self.end.shade(frag);

        start.mix(&end, self.factor)
    }
}

pub struct Fade<S: Shader, E: Shader> {
    start: S,
    end: E,

    interpolator: Fn(f32) -> f32,
}
impl<S: Shader, E: Shader> Shader for Fade<S, E> {
    type Output = S::Output;

    fn shade(&self, frag: Fragment) -> Self::Output {
        let factor = (self.interpolator)(frag.pos);
        let start = self.start.shade(frag);
        let end = self.end.shade(frag);

        start.mix(&end, factor)
    }
}