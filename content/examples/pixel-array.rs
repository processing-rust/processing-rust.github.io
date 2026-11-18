/**
 * Pixel Array.
 *
 * Click and drag the mouse up and down to control the signal and
 * press and hold any key to see the current pixel being read.
 * This program sequentially reads the color of every pixel of an image
 * and displays this color to fill the window.
 */
let mut img: u64 = 0;
let mut direction: i32 = 1;
let mut signal: f32 = 0.0;

fn setup() {
    size(640, 360);
    no_fill();
    stroke(255);
    frame_rate(30.0);
    img = load_image("sea.jpg");
}

fn draw() {
    if signal > img.width() * img.height() - 1.0 || signal < 0.0 {
        direction = direction * -1;
    }

    if mouse_pressed_global() {
        let mx: i32 = (constrain(mouse_x(), 0.0, img.width() - 1.0)) as i32;
        let my: i32 = (constrain(mouse_y(), 0.0, img.height() - 1.0)) as i32;
        signal = my as f32 * img.width() + mx as f32;
    } else {
        signal += 0.33 * direction as f32;
    }

    let sx: i32 = ((signal as i32) as f32 % img.width()) as i32;
    let sy: i32 = ((signal as i32) as f32 / img.width()) as i32;

    if key_pressed_global() {
        set(0, 0, img);  // fast way to draw an image
        point(sx as f32, sy as f32);
        rect((sx - 5) as f32, (sy - 5) as f32, 10.0, 10.0);
    } else {
        let c: Color = img.get(sx, sy);
        background(c);
    }
}
