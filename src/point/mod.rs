pub mod primitives;

#[derive(Clone, Copy, Debug, Default)]
// So that it can be exported to the visualizer.
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl core::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl core::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl core::ops::Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl core::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl core::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
pub mod tests {
    use super::Point;
    use alloc::vec::Vec;

    #[test]
    fn from_iterator() {
        let points = super::primitives::Line::new(
            Point::default(),
            Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            2,
        );

        let points: Vec<Point> = points.collect();
    }
}
