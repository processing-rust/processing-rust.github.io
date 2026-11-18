/**
 * Random.
 *
 * Random numbers create the basis of this image.
 * Each time the program is loaded the result is different.
 */
fn setup() {
    size(640, 360);
    background(0);
    stroke_weight(20.0);
    frame_rate(2.0);
}

fn draw() {
    for i in 0..width() as i32 {
        let r: f32 = random(255.0);
        stroke(r);
        line(i as f32, 0.0, i as f32, height());
    }
}
