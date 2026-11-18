/**
 * Sine.
 *
 * Smoothly scaling size with the sin() function.
 */
let mut diameter: f32 = 0.0;
let mut angle: f32 = 0.0;

fn setup() {
    size(640, 360);
    diameter = height() - 10.0;
    no_stroke();
    fill(255, 204, 0);
}

fn draw() {
    background(0);

    let d1: f32 = 10.0 + (angle.sin() * diameter / 2.0) + diameter / 2.0;
    let d2: f32 = 10.0 + ((angle + PI / 2.0).sin() * diameter / 2.0) + diameter / 2.0;
    let d3: f32 = 10.0 + ((angle + PI).sin() * diameter / 2.0) + diameter / 2.0;

    ellipse(0.0, height() / 2.0, d1, d1);
    ellipse(width() / 2.0, height() / 2.0, d2, d2);
    ellipse(width(), height() / 2.0, d3, d3);

    angle += 0.02;
}
