/**
 * Rotate.
 *
 * Rotating a square around the Z axis. To get the results
 * you expect, send the rotate function angle parameters that are
 * values between 0 and PI*2 (TWO_PI which is roughly 6.28). If you prefer to
 * think about angles as degrees (0-360), you can use the radians()
 * method to convert your values. For example: scale(radians(90))
 * is identical to the statement scale(PI/2).
 */
let mut angle: f32 = 0.0;
let mut jitter: f32 = 0.0;

fn setup() {
    size(640, 360);
    no_stroke();
    fill(255);
    rect_mode(CENTER);
}

fn draw() {
    background(51);

    // during even-numbered seconds (0, 2, 4, 6...)
    if second() % 2 == 0 {
        jitter = random(-0.1, 0.1);
    }
    angle = angle + jitter;
    let c: f32 = angle.cos();
    translate(width() / 2.0, height() / 2.0);
    rotate(c);
    rect(0.0, 0.0, 180.0, 180.0);
}
