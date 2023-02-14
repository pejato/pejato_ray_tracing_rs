use super::{
    hittable::{HitData, Hittable},
    point::Point,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub radius: f32,
    pub center: Point,
}

impl Sphere {
    fn feasible_sol<Interval>(solution: f32, in_range: &Interval) -> Option<f32>
    where
        Interval: std::ops::RangeBounds<f32> + Clone,
    {
        match solution {
            a if in_range.contains(&a) => Some(solution),
            _ => None,
        }
    }

    fn hit_data(&self, t: f32, ray: &Ray) -> HitData {
        let hit_data = HitData {
            t,
            point: ray.at(t),
            normal: Vec3::zero(),
            is_front_face: false,
        };
        let outward_normal = Vec3::from((ray.at(t) - self.center) / self.radius);
        hit_data.with_normal(ray, outward_normal)
    }

    fn quadratic_solutions(
        &self,
        ray: &Ray,
        shifted_center: Vec3,
        half_b: f32,
    ) -> Option<(f32, f32)> {
        // Equation of points on ray that are on sphere centered at C with radius r is:
        // 𝑡^2*𝐛⋅𝐛 + 2𝑡*𝐛⋅(𝐀−𝐂) + (𝐀−𝐂)⋅(𝐀−𝐂) − 𝑟^2 = 0
        // => discriminant = b^2 - 4*a*c
        // if discriminant > 0 => there are solutions to the equation (possibly negative)

        let a = ray.dir.dot(ray.dir);
        let c: f32 = shifted_center.dot(shifted_center) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let solution_a = (-half_b - f32::sqrt(discriminant)) / a;
        let solution_b = (-half_b + f32::sqrt(discriminant)) / a;
        Some((solution_a, solution_b))
    }
}

// MARK: - Sphere + Hittable

impl Hittable for Sphere {
    fn hit<Interval>(&self, ray: &Ray, t_interval: Interval) -> Option<HitData>
    where
        Interval: std::ops::RangeBounds<f32> + Clone,
    {
        let shifted_center = Vec3::from(ray.origin - self.center);
        let half_b = ray.dir.dot(shifted_center);

        let (solution_a, solution_b) = self.quadratic_solutions(ray, shifted_center, half_b)?;
        // Solutions may not be in t_interval
        let solution_a = Self::feasible_sol(solution_a, &t_interval);
        let solution_b = Self::feasible_sol(solution_b, &t_interval);

        match (solution_a, solution_b) {
            (Some(a), _) => Some(self.hit_data(a, ray)),
            (_, Some(b)) => Some(self.hit_data(b, ray)),
            (None, None) => None,
        }
    }
}
