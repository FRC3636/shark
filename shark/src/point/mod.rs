pub mod primitives;

#[derive(Clone, Copy, Debug, Default)]
// So that it can be exported to the visualizer.
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
pub struct Points<'a> {
    _marker: std::marker::PhantomData<&'a Point>,
    points: *const Point,
}

impl<'a> From<&'a[Point]> for Points<'a> {
    fn from(points: &'a [Point]) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            points: points.as_ptr(),
        }
    }
}

// This probably shouldn't be used outside of exporting to the visualizer.
impl FromIterator<Point> for Points<'static> {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let points = Box::leak(Box::new(iter.into_iter().collect::<Vec<_>>())).as_ptr();
        Self {
            _marker: std::marker::PhantomData,
            points,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn from_iterator() {
        let points = crate::point::primitives::Line::new(
            crate::point::Point::default(),
            crate::point::Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            2,
        );

        let points: Points<'static> = points.collect();
    }
}