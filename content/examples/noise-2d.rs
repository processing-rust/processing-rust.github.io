/**
 * Noise2D
 * by Daniel Shiffman.
 *
 * Using 2D noise to create simple texture.
 */

let mut pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let increment: f32 = 0.02;

fn setup() {
    size(640, 360);
}

fn draw() {
    pixels = load_pixels();

    let mut xoff: f32 = 0.0;  // Start xoff at 0
    let detail: f32 = map(mouse_x(), 0.0, width(), 0.1, 0.6);
    noise_detail(8, detail);

    // For every x,y coordinate in a 2D space, calculate a noise value and produce a brightness value
    for x in 0..width() as i32 {
        xoff += increment;  // Increment xoff
        let mut yoff: f32 = 0.0;  // For every xoff, start yoff at 0
        for y in 0..height() as i32 {
            yoff += increment;  // Increment yoff

            // Calculate noise and scale by 255
            let bright: f32 = noise(xoff, yoff) * 255.0;

            // Try using this line instead
            //float bright = random(0,255);

            // Set each pixel onscreen to a grayscale value
            pixels[(x as f32 + y as f32 * width()) as usize] = color(bright).to_argb();
        }
    }

    update_pixels(&pixels);
}
