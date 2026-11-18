/**
 * Create Image.
 *
 * The createImage() function provides a fresh buffer of pixels to play with.
 * This example creates an image gradient.
 */

let mut img_pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let mut img: u64 = 0;

fn setup() {
    size(640, 360);
    img = create_image(230, 230, ARGB);
    for i in 0..img_pixels.len() {
        let a: f32 = map(i as f32, 0.0, img_pixels.len() as f32, 255.0, 0.0);
        img_pixels[i] = (color(0, 153, 204, a)).to_argb();
    }
}

fn draw() {
    background(0);
    image(img, 90.0, 80.0);
    image(img, mouse_x() - img.width() / 2.0, mouse_y() - img.height() / 2.0);
}
