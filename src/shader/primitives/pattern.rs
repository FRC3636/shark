use palette::{IntoColor, LinSrgb, Okhsl, Srgb};

use crate::shader::{Shader, Vertex};

#[derive(Debug, Clone, Copy)]
pub struct Checkerboard<F: Vertex, S: Shader<F>, T: Shader<F>> {
    _marker: core::marker::PhantomData<fn(F)>,
    shaders: (S, T),
    stride: f64,
}

impl<F: Vertex, S: Shader<F>, T: Shader<F>> Shader<F> for Checkerboard<F, S, T> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let first_color = frag
            .pos()
            .iter()
            .map(|pos| (pos / self.stride).abs() as usize)
            .sum::<usize>()
            .is_multiple_of(2);

        if first_color {
            self.shaders.0.shade(frag).into_color()
        } else {
            self.shaders.1.shade(frag).into_color()
        }
    }
}

pub fn checkerboard<F: Vertex, S: Shader<F>, T: Shader<F>>(
    first: S,
    second: T,
    stride: f64,
) -> Checkerboard<F, S, T> {
    Checkerboard {
        _marker: core::marker::PhantomData,
        shaders: (first, second),
        stride,
    }
}

#[derive(Debug)]
pub struct Random {
    seed: portable_atomic::AtomicU64,
}
impl<F: Vertex> Shader<F> for Random {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        let mut rng = fastrand::Rng::with_seed(self.seed.load(portable_atomic::Ordering::Relaxed));
        let color = Srgb::new(rng.f64(), rng.f64(), rng.f64()).into_color();
        let new_seed = rng.get_seed();
        self.seed
            .store(new_seed, portable_atomic::Ordering::Relaxed);
        color
    }
}
impl Clone for Random {
    fn clone(&self) -> Self {
        Random {
            seed: portable_atomic::AtomicU64::new(
                self.seed.load(portable_atomic::Ordering::Relaxed),
            ),
        }
    }
}

pub fn random() -> Random {
    Random {
        seed: portable_atomic::AtomicU64::new(0xdeadbeef),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rainbow<F: Vertex, S: Fn(F) -> f64 + Send + Sync> {
    _marker: core::marker::PhantomData<fn(F)>,
    selector: S,
}
impl<F: Vertex, S: Fn(F) -> f64 + Send + Sync> Shader<F> for Rainbow<F, S> {
    type Output = Okhsl<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let t = (self.selector)(frag);
        Okhsl::new(t % 360.0, 1.0, 0.5)
    }
}

pub fn rainbow<F: Vertex, S: Fn(F) -> f64 + Send + Sync>(selector: S) -> Rainbow<F, S> {
    Rainbow {
        _marker: core::marker::PhantomData,
        selector,
    }
}

pub fn time_rainbow<F: Vertex>() -> Rainbow<F, impl Fn(F) -> f64> {
    Rainbow {
        _marker: core::marker::PhantomData,
        selector: |frag| frag.time(),
    }
}

pub fn position_rainbow<F: Vertex>() -> Rainbow<F, impl Fn(F) -> f64> {
    Rainbow {
        _marker: core::marker::PhantomData,
        selector: |frag| frag.pos().iter().sum(),
    }
}
