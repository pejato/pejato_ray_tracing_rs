use super::{point::Point, ray::Ray, vec3::Vec3};

// MARK:  Hittable

pub trait Hittable {
    fn hit<Interval>(&self, ray: &Ray, t_interval: Interval) -> Option<HitData>
    where
        Interval: std::ops::RangeBounds<f32> + Clone;
}

// MARK: HitData

#[derive(Clone, Copy)]
pub struct HitData {
    pub(crate) t: f32,
    pub(crate) point: Point,
    pub(crate) normal: Vec3,
    pub(crate) is_front_face: bool,
}

// MARK: - HitData + Methods

impl HitData {
    pub fn with_normal(self, ray: &Ray, outward_normal: Vec3) -> Self {
        let is_front_face = ray.dir.dot(outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point: self.point,
            normal,
            t: self.t,
            is_front_face,
        }
    }
}

// MARK: - &[T: Hittable] + Hittable

impl<T> Hittable for &[T]
where
    T: Hittable,
{
    fn hit<Interval>(&self, ray: &Ray, t_interval: Interval) -> Option<HitData>
    where
        Interval: std::ops::RangeBounds<f32> + Clone,
    {
        self.iter().find_map(|h| h.hit(ray, t_interval.clone()))
    }
}

impl<T> Hittable for [T]
where
    T: Hittable,
{
    fn hit<Interval>(&self, ray: &Ray, t_interval: Interval) -> Option<HitData>
    where
        Interval: std::ops::RangeBounds<f32> + Clone,
    {
        self.iter().find_map(|h| h.hit(ray, t_interval.clone()))
    }
}

// MARK: - &[Ref<T>] + Hittable

impl<T> Hittable for &[std::cell::Ref<'_, T>]
where
    T: Hittable,
{
    fn hit<Interval>(&self, ray: &Ray, t_interval: Interval) -> Option<HitData>
    where
        Interval: std::ops::RangeBounds<f32> + Clone,
    {
        self.iter().find_map(|h| h.hit(ray, t_interval.clone()))
    }
}
