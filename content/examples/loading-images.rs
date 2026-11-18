/**
 * Loading Images.
 *
 * Processing applications can load images from the network.
 */
let mut img: u64 = 0;

fn setup() {
    size(640, 360);
    img = load_image("https://processing.org/img/processing-web.png");
    no_loop();
}

fn draw() {
    background(0);
    if img != 0 {
        for i in 0..5 {
            image(img, 0.0, img.height() * i as f32);
        }
    }
}
