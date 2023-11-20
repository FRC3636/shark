use super::Point;

pub struct Line {
    pub a: Point,
    pub b: Point,
    pub num_points: usize,
    current_point: usize,
}
impl Line {
    pub fn new(a: Point, b: Point, num_points: usize) -> Self {
        Self {
            a,
            b,
            num_points,
            current_point: 0,
        }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_points == 0 {
            return None;
        }

        if self.current_point < self.num_points {
            let t = self.current_point as f64 / (self.num_points - 1) as f64;
            self.current_point += 1;
            Some((1.0 - t) * self.a + t * self.b)
        } else {
            None
        }
    }
}

pub fn line(a: Point, b: Point, num_points: usize) -> Line {
    Line::new(a, b, num_points)
}
