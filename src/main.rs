
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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
struct Vec3 {
    e: Vec<f64>,
}
impl Vec3 {
    fn new(&this){

    }
}
