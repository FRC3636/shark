use palette::{IntoColor, LinSrgb};

use crate::shader::{Shader, Vertex};

#[derive(Debug, Clone, Copy)]
pub struct Off;
impl<F: Vertex> Shader<F> for Off {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        // Full black
        LinSrgb::new(0.0, 0.0, 0.0)
    }
}

pub fn off() -> Off {
    Off
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    color: LinSrgb<f64>,
}

impl<F: Vertex> Shader<F> for Color {
    type Output = LinSrgb<f64>;

    fn shade(&self, _frag: F) -> Self::Output {
        self.color
    }
}

pub fn color(color: impl IntoColor<LinSrgb<f64>>) -> Color {
    Color {
        color: color.into_color(),
    }
}
