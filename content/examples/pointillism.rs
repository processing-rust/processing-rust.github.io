/**
 * Pointillism
 * by Daniel Shiffman.
 *
 * Mouse horizontal location controls size of dots.
 * Creates a simple pointillist effect using ellipses colored
 * according to pixels in an image.
 */
let mut img: u64 = 0;
let mut smallPoint: i32 = 0;
let mut largePoint: i32 = 0;

fn setup() {
    size(640, 360);
    img = load_image("moonwalk.jpg");
    smallPoint = 4;
    largePoint = 40;
    image_mode(CENTER);
    no_stroke();
    background(255);
}

fn draw() {
    let pointillize: f32 = map(mouse_x(), 0.0, width(), smallPoint as f32, largePoint as f32);
    let x: i32 = random(img.width()) as i32;
    let y: i32 = random(img.height()) as i32;
    let pix: Color = img.get(x, y);
    fill(pix, 128);
    ellipse(x as f32, y as f32, pointillize, pointillize);
}
