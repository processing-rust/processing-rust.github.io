/**
 * Brightness Pixels
 * by Daniel Shiffman.
 *
 * This program adjusts the brightness of a part of the image by
 * calculating the distance of each pixel to the mouse.
 */

let mut pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let mut img_pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let mut img: u64 = 0;

fn setup() {
    size(640, 360);
    frame_rate(30.0);
    img = load_image("moon-wide.jpg");
    img_pixels = img.load_pixels();
    // Only need to load the pixels[] array once, because we're only
    // manipulating pixels[] inside draw(), not drawing shapes.
    pixels = load_pixels();
}

fn draw() {
    for x in 0..img.width() as i32 {
        for y in 0..img.height() as i32 {
            // Calculate the 1D location from a 2D grid
            let loc: i32 = (x as f32 + y as f32 * img.width()) as i32;
            // Get the R,G,B values from image
            let mut r: f32;
            let mut g: f32;
            let mut b: f32;
            r = red(Color::from_u32(img_pixels[loc as usize]));
            //g = green (img.pixels[loc]);
            //b = blue (img.pixels[loc]);
            // Calculate an amount to change brightness based on proximity to the mouse
            let maxdist: f32 = 50.0;  //dist(0,0,width,height);
            let d: f32 = dist(x as f32, y as f32, mouse_x(), mouse_y());
            let adjustbrightness: f32 = 255.0 * (maxdist - d) / maxdist;
            r += adjustbrightness;
            //g += adjustbrightness;
            //b += adjustbrightness;
            // Constrain RGB to make sure they are within 0-255 color range
            r = constrain(r, 0.0, 255.0);
            //g = constrain(g, 0, 255);
            //b = constrain(b, 0, 255);
            // Make a new color and set pixel in the window
            //color c = color(r, g, b);
            let c: Color = color(r);
            pixels[(y as f32 * width() + x as f32) as usize] = c.to_argb();
        }
    }
    update_pixels(&pixels);
}
