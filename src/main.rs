mod ray_tracing;

use ray_tracing::{color::*, hittable::Hittable, ray::Ray};

use crate::ray_tracing::{camera::Camera, point::Point, sphere::Sphere};

fn header(width: i32, height: i32) -> String {
    format!("P3\n{width} {height}\n255")
}

fn ray_color<T: Hittable + ?Sized>(ray: Ray, world: &T) -> Color {
    match world.hit(&ray, 0.0..) {
        Some(hit_data) => 0.5 * (Color::from(hit_data.normal) + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_dir = ray.dir.unit();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let (image_width, image_height) = (400, (400.0 / aspect_ratio).trunc() as i32);

    // World

    let world = vec![
        Sphere {
            radius: 0.5,
            center: Point::new(0.0, 0.0, -1.0),
        },
        Sphere {
            radius: 100.0,
            center: Point::new(0.0, -100.5, -1.0),
        },
    ];

    // Camera

    let camera = Camera::new();

    // Render

    let header = header(image_width, image_height);
    println!("{header}");

    for j in (0..image_height).rev() {
        eprintln!("Lines remaining {j}");
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let ray = camera.ray_at(u, v);
            let color = ray_color(ray, world.as_slice());
            println!("{color}");
        }
    }
    eprintln!("Fin");
}
