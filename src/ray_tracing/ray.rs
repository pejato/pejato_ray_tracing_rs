use super::point::Point;
use super::vec3::Vec3;

mod tests;

// MARK: - Data

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

// MARK: - Methods

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + (self.dir * t).into()
    }
}
