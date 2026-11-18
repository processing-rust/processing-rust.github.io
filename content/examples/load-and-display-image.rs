/**
 * Load and Display
 *
 * Images can be loaded and displayed to the screen at their actual size
 * or any other size.
 */
let mut img: u64 = 0;  // Declare variable "a" of type PImage

fn setup() {
    size(640, 360);
    // The image file must be in the data folder of the current sketch
    // to load successfully
    img = load_image("moonwalk.jpg");  // Load the image into the program
}

fn draw() {
    // Displays the image at its actual size at point (0,0)
    image(img, 0.0, 0.0);
    // Displays the image at point (0, height/2) at half of its size
    image(img, 0.0, height() / 2.0, img.width() / 2.0, img.height() / 2.0);
}
