mod ray_tracing;
use ray_tracing::{color::*, ray::Ray};

use crate::ray_tracing::{point::Point, vec3::Vec3};

fn header(width: i32, height: i32) -> String {
    format!("P3\n{width} {height}\n255")
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_dir = ray.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point, radius: f32, ray: Ray) -> bool {
    // Equation of points on ray that are on sphere centered at C with radius r is:
    // 𝑡^2*𝐛⋅𝐛 + 2𝑡*𝐛⋅(𝐀−𝐂) + (𝐀−𝐂)⋅(𝐀−𝐂) − 𝑟^2 = 0
    // => discriminant = b^2 - 4*a*c
    // if discriminant > 0 => there are solutions to the equation (possibly negative)
    let shifted_center = Vec3::from(ray.origin - center);
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * ray.dir.dot(shifted_center);
    let c: f32 = shifted_center.dot(shifted_center) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if  discriminant < 0.0 {
        return false;
    }
    let solution_a = (-b + f32::sqrt(discriminant)) / (2.0 * a);
    let solution_b = (-b - f32::sqrt(discriminant)) / (2.0 * b);
    solution_a >= 0.0 || solution_b >= 0.0
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let (image_width, image_height) = (400, (400.0 / aspect_ratio).trunc() as i32);
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_len = 1.0;
    let origin = Point::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left = origin
        - (horizontal / 2.0).into()
        - (vertical / 2.0).into()
        - Vec3::new(0.0, 0.0, focal_len).into();

    let header = header(image_width, image_height);
    println!("{header}");

    for j in (0..image_height).rev() {
        eprintln!("Lines remaining {j}");
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let ray = Ray::new(
                origin,
                Vec3::from(lower_left) + u * horizontal + v * vertical - Vec3::from(origin),
            );
            let color = ray_color(ray);
            println!("{color}");
        }
    }
    eprintln!("Fin");
}
