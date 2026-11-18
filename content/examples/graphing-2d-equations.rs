/**
 * Graphing 2D Equations
 * by Daniel Shiffman.
 *
 * Graphics the following equation:
 * sin(n*cos(r) + 5*theta)
 * where n is a function of horizontal mouse location.
 */

let mut pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
fn setup() {
    size(640, 360);
}

fn draw() {
    pixels = load_pixels();
    let n: f32 = (mouse_x() * 10.0) / width();
    let w: f32 = 16.0;  // 2D space width
    let h: f32 = 16.0;  // 2D space height
    let dx: f32 = w / width();  // Increment x this amount per pixel
    let dy: f32 = h / height();  // Increment y this amount per pixel
    let mut x: f32 = -w / 2.0;  // Start x at -1 * width / 2
    for i in 0..width() as i32 {
        let mut y: f32 = -h / 2.0;  // Start y at -1 * height / 2
        for j in 0..height() as i32 {
            let r: f32 = sqrt((x * x) + (y * y));  // Convert cartesian to polar
            let theta: f32 = y.atan2(x);  // Convert cartesian to polar
            // Compute 2D polar coordinate function
            let val: f32 = (n * r.cos() + 5.0 * theta).sin();  // Results in a value between -1 and 1
            //float val = cos(r);                            // Another simple function
            //float val = sin(theta);                        // Another simple function
            // Map resulting vale to grayscale value
            pixels[(i as f32 + j as f32 * width()) as usize] = (color((val + 1.0) * 255.0 / 2.0)).to_argb();  // Scale to between 0 and 255
            y += dy;  // Increment y
        }
        x += dx;  // Increment x
    }
    update_pixels(&pixels);
}
