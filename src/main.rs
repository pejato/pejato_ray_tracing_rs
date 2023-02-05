mod ray_tracing;
use ray_tracing::color::*;

fn header(width: i32, height: i32) -> String {
    format!("P3\n{width} {height}\n255")
}

fn main() {
    let (image_width, image_height) = (256, 256);
    let header = header(image_width, image_height);
    println!("{header}");

    for j in (0..image_height).rev() {
        eprintln!("Lines remaining {j}");
        for i in 0..image_width {
            let color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            println!("{color}");
        }
    }
    eprintln!("Fin");
}
