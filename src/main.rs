mod ray_tracing;

use ray_tracing::color::Averageable;
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

use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let (image_width, image_height) = (400, (400.0 / aspect_ratio).trunc() as i32);
    // Anti-aliasing
    let samples_per_pixel = 100;
    // Use a fixed seed for reproducibility. This could be handled with proper DI but I don't care tbh.
    let mut rng = StdRng::seed_from_u64(143);

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
            let color = get_sampled_color(
                samples_per_pixel,
                &mut rng,
                i,
                j,
                image_width,
                image_height,
                &camera,
                &world,
            );
            println!("{color}");
        }
    }
    eprintln!("Fin");
}

fn get_sampled_color(
    samples_per_pixel: i32,
    rng: &mut StdRng,
    i: i32,
    j: i32,
    image_width: i32,
    image_height: i32,
    camera: &Camera,
    world: &Vec<Sphere>,
) -> Color {
    let to_color = |i: f32, j: f32| {
        let u = i / (image_width - 1) as f32;
        let v = j / (image_height - 1) as f32;
        let ray = camera.ray_at(u, v);
        ray_color(ray, world.as_slice())
    };
    if samples_per_pixel <= 1 {
        to_color(i as f32, j as f32)
    } else {
        (0..samples_per_pixel)
            .map(|_| {
                let (di, dj) = (rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
                to_color(i as f32 + di, j as f32 + dj)
            })
            .into_iter()
            .averaged()
    }
}
