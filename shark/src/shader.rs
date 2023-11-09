use core::slice;

use crate::primitives::{
    checkerboard, extrude, mix, mod_position, mod_time, rotate, Checkerboard, Extrude, Interpolate,
    ModPosition, ModTime, Rotate,
};
use palette::{IntoColor, Oklab};

pub trait Shader {
    type Output: IntoColor<Oklab>;
    type Fragment: Fragment;

    fn shade(&self, frag: Self::Fragment) -> Self::Output;
}

pub trait Fragment: Clone + Copy + std::fmt::Debug {
    fn time(&self) -> f32;
    fn time_mut(&mut self) -> &mut f32;
    fn pos(&self) -> &[usize];
    fn pos_mut(&mut self) -> &mut [usize];
}

#[derive(Clone, Copy, Debug)]
pub struct FragOne {
    pub pos: usize,
    pub time: f32,
}
impl Fragment for FragOne {
    fn time(&self) -> f32 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }

    fn pos(&self) -> &[usize] {
        slice::from_ref(&self.pos)
    }

    fn pos_mut(&mut self) -> &mut [usize] {
        slice::from_mut(&mut self.pos)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FragTwo {
    pub pos: [usize; 2],
    pub time: f32,
}
impl Fragment for FragTwo {
    fn time(&self) -> f32 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }

    fn pos(&self) -> &[usize] {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut [usize] {
        &mut self.pos
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FragThree {
    pub pos: [usize; 3],
    pub time: f32,
}
impl Fragment for FragThree {
    fn time(&self) -> f32 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }

    fn pos(&self) -> &[usize] {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut [usize] {
        &mut self.pos
    }
}

pub trait ShaderExt: Shader + Sized {
    fn mix<S: Shader>(self, other: S, factor: f32) -> Interpolate<Self, S, Self::Fragment> {
        mix(self, other, factor)
    }

    fn checkerboard<F: Fragment, S: Shader<Fragment = F>>(
        self,
        other: S,
        stride: usize,
    ) -> Checkerboard<F, Self, S>
    where
        Self: Shader<Fragment = F>,
    {
        checkerboard(self, other, stride)
    }

    fn mod_position<M: num::ToPrimitive>(self, modulo: M) -> ModPosition<Self, M, Self::Fragment> {
        mod_position(self, modulo)
    }

    fn mod_time<M: num::ToPrimitive>(self, modulo: M) -> ModTime<Self, M> {
        mod_time(self, modulo)
    }

    fn rotate(self, angle: f32) -> Rotate<Self> {
        rotate(self, angle)
    }

    fn extrude(self) -> Extrude<Self::Fragment, Self> {
        extrude(self)
    }
}
impl<T> ShaderExt for T where T: Shader {}
