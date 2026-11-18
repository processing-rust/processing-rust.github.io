/**
 * Noise3D.
 *
 * Using 3D noise to create simple animated texture.
 * Here, the third dimension ('z') is treated as time.
 */

let mut pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let increment: f32 = 0.01;
// The noise function's 3rd argument, a global variable that increments once per cycle
let mut zoff: f32 = 0.0;
// We will increment zoff differently than xoff and yoff
let zincrement: f32 = 0.02;

fn setup() {
    size(640, 360);
    frame_rate(30.0);
}

fn draw() {
    // Optional: adjust noise detail here
    // noiseDetail(8,0.65f);

    pixels = load_pixels();

    let mut xoff: f32 = 0.0;  // Start xoff at 0

    // For every x,y coordinate in a 2D space, calculate a noise value and produce a brightness value
    for x in 0..width() as i32 {
        xoff += increment;  // Increment xoff
        let mut yoff: f32 = 0.0;  // For every xoff, start yoff at 0
        for y in 0..height() as i32 {
            yoff += increment;  // Increment yoff

            // Calculate noise and scale by 255
            let bright: f32 = noise(xoff, yoff, zoff) * 255.0;

            // Try using this line instead
            //float bright = random(0,255);

            // Set each pixel onscreen to a grayscale value
            pixels[(x as f32 + y as f32 * width()) as usize] = (color(bright, bright, bright)).to_argb();
        }
    }
    update_pixels(&pixels);

    zoff += zincrement;  // Increment zoff
}
