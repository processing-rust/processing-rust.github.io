/**
 * Scale
 * by Denis Grutze.
 *
 * Paramenters for the scale() function are values specified
 * as decimal percentages. For example, the method call scale(2.0)
 * will increase the dimension of the shape by 200 percent.
 * Objects always scale from the origin.
 */
let mut a: f32 = 0.0;
let mut s: f32 = 0.0;

fn setup() {
    size(640, 360);
    no_stroke();
    rect_mode(CENTER);
    frame_rate(30.0);
}

fn draw() {
    background(102);

    a = a + 0.04;
    s = a.cos() * 2.0;

    translate(width() / 2.0, height() / 2.0);
    scale(s);
    fill(51);
    rect(0.0, 0.0, 50.0, 50.0);

    translate(75.0, 0.0);
    fill(255);
    scale(s);
    rect(0.0, 0.0, 50.0, 50.0);
}
