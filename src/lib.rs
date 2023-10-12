pub mod primitives;

use palette::{IntoColor, Oklab};
use primitives::{Interpolate, mix, Stride, stride};

pub trait Shader {
    type Output: IntoColor<Oklab>;

    fn shade(&self, frag: Fragment) -> Self::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    pub pos: usize,
    pub time: f32,
}

pub trait ShaderExt: Shader + Sized {
    fn mix<S: Shader>(self, other: S, factor: f32) -> Interpolate<Self, S> {
        mix(self, other, factor)
    }

    fn stride<S: Shader>(self, other: S, stride: usize) -> Stride<Self, S> {
        crate::stride(self, other, stride)
    }
}
impl<T> ShaderExt for T where T: Shader {}

#[cfg(test)]
mod tests {
    use crate::ShaderExt;
    pub use crate::primitives::*;

    #[test]
    fn shader_ext() {
        let shader = color(palette::Oklab::new(0.800, 0.159, -0.193)).stride(off(), 2);
    }
}

// TODO
// pub struct Channel {
//     pub channel: rs_ws281x::bindings::ws2811_channel_t,
//     pub strip_type: rs_ws281x::StripType,
//     pub index: usize,
// }

// pub struct LedStrip {
//     shader: Box<dyn Shader>,
//     strip_type: rs_ws281x::StripType,
//     controller: rs_ws281x::Controller,
// }

// impl LedStrip {
//     pub fn new(shader: impl Shader, channel: Channel) -> Self {
//         let shader = Box::new(shader);
//         let controller = rs_ws281x::ControllerBuilder::new()
//             .channel(channel.index, channel.channel)
//             .build();

//         Self {
//             shader,
//             strip_type: channel.strip_type,
//             controller,
//         }
//     }

//     pub fn render(&self) {
//         let pixels = self.controller.leds_mut(channel);
//         for (i, pixel) in pixels.iter_mut().enumerate() {
//             let color = self.shader.shade(i as u32);
//             todo!();
//         }

//         self.controller.render();
//     }
// }