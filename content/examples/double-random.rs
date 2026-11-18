/**
 * Double Random
 * by Ira Greenberg.
 *
 * Using two random() calls and the point() function
 * to create an irregular sawtooth line.
 */
let totalPts: i32 = 300;
let steps: f32 = (totalPts + 1) as f32;

fn setup() {
    size(640, 360);
    stroke(255);
    frame_rate(1.0);
}

fn draw() {
    background(0);
    let mut rand: f32 = 0.0;
    for i in 1..steps as i32 {
        point((width() / steps) * i as f32, (height() / 2.0) + random(-rand, rand));
        rand += random(-5.0, 5.0);
    }
}
