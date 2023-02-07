use super::{point::Point, ray::Ray, vec3::Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_interval: std::ops::RangeInclusive<f32>) -> Option<HitData>;
}

pub struct HitData {
    point: Point,
    normal: Vec3,
    t: f32,
}
