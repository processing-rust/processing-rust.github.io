/**
 * Transparency.
 *
 * Move the pointer left and right across the image to change
 * its position. This program overlays one image over another
 * by modifying the alpha value of the image with the tint() function.
 */
let mut img: u64 = 0;
let mut offset: f32 = 0.0;
let easing: f32 = 0.05;

fn setup() {
    size(640, 360);
    img = load_image("moonwalk.jpg");  // Load an image into the program
}

fn draw() {
    image(img, 0.0, 0.0);  // Display at full opacity
    let dx: f32 = (mouse_x() - img.width() / 2.0) - offset;
    offset += dx * easing;
    tint(255.0, 127.0);  // Display at half opacity
    image(img, offset, 0.0);
}
