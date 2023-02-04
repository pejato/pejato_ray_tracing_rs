use super::vec3::Vec3;

pub struct Point(Vec3);

impl Point {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

// TODO: Define a (proc?) macro to automatically derive impl's defined on Vec3