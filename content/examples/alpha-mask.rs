/**
 * Alpha Mask.
 *
 * Loads a "mask" for an image to specify the transparency
 * in different parts of the image. The two images are blended
 * together using the mask() method of PImage.
 */
let mut img: u64 = 0;
let mut imgMask: u64 = 0;

fn setup() {
    size(640, 360);
    img = load_image("moonwalk.jpg");
    imgMask = load_image("mask.jpg");
    img.mask(imgMask);
    image_mode(CENTER);
}

fn draw() {
    background(0, 102, 153);
    image(img, width() / 2.0, height() / 2.0);
    image(img, mouse_x(), mouse_y());
}
