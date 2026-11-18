/**
 * Increment Decrement.
 *
 * Writing "a++" is equivalent to "a = a + 1".
 * Writing "a--" is equivalent to "a = a - 1".
 */
let mut a: i32 = 0;
let mut b: i32 = 0;
let mut direction: bool = false;

fn setup() {
    size(640, 360);
    color_mode(RGB, width());
    a = 0;
    b = width() as i32;
    direction = true;
    frame_rate(30.0);
}

fn draw() {
    a += 1;
    if (a as f32) > width() {
        a = 0;
        direction = !direction;
    }
    if direction == true {
        stroke(a);
    } else {
        stroke(width() - a as f32);
    }
    line(a as f32, 0.0, a as f32, height() / 2.0);

    b -= 1;
    if b < 0 {
        b = width() as i32;
    }
    if direction == true {
        stroke(width() - b as f32);
    } else {
        stroke(b);
    }
    line(b as f32, height() / 2.0 + 1.0, b as f32, height());
}
