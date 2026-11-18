/**
 * Bezier.
 *
 * The first two parameters for the bezier() function specify the
 * first point in the curve and the last two parameters specify
 * the last point. The middle parameters set the control points
 * that define the shape of the curve.
 */
fn setup() {
    size(640, 360);
    stroke(255);
    no_fill();
}

fn draw() {
    background(0);
    for i in (0..200).step_by(20) {
        bezier(mouse_x() - (i as f32 / 2.0), (40 + i) as f32, 410.0, 20.0, 440.0, 300.0, 240.0 - (i as f32 / 16.0), 300.0 + (i as f32 / 8.0));
    }
}
