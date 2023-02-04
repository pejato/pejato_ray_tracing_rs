mod ray_tracing;
use ray_tracing::vec3::*;

fn header(width: i32, height: i32) -> String {
    format!("P3\n{width} {height}\n255").into()
}

fn main() {
    let (image_width, image_height) = (256, 256);
    let header = header(image_width, image_height);
    println!("{header}");

    let pixel = |flt: f32| (flt * 255.999).round() as i32;
    for j in (0..image_height).rev() {
        eprintln!("Lines remaining {j}");
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let (ir, ig, ib) = (pixel(r), pixel(g), pixel(b));
            println!("{ir} {ig} {ib}");
        }
    }
    eprintln!("Fin");
}
