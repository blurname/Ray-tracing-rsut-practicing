use Ray_tracing::{Color, Point3, Vec3};
fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..=(IMAGE_WIDTH - 1)).rev() {
        for i in 0..IMAGE_HEIGHT {
            // println!("{} {}", j, i);
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            let pixel_color = Color::new(r, g, b);
            write_color(pixel_color)
        }
    }
}

fn write_color(pixel_color: Color) {
    let cr = (255.999 * pixel_color.x()) as i32;
    let cg = (255.999 * pixel_color.y()) as i32;
    let cb = (255.999 * pixel_color.z()) as i32;
    print!("{} {} {}\n", cr, cg, cb);
}
