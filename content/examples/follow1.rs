/**
 * Follow 1
 * based on code from Keith Peters.
 *
 * A line segment is pushed and pulled by the cursor.
 */
let mut x: f32 = 100.0;
let mut y: f32 = 100.0;
let mut angle1: f32 = 0.0;
const SEGLENGTH: f32 = 50.0;

fn setup() {
    size(640, 360);
    stroke_weight(20.0);
    stroke(255, 100);
}

fn draw() {
    background(0);

    let dx: f32 = mouse_x() - x;
    let dy: f32 = mouse_y() - y;
    angle1 = dy.atan2(dx);
    x = mouse_x() - (angle1.cos() * SEGLENGTH);
    y = mouse_y() - (angle1.sin() * SEGLENGTH);

    segment(x, y, angle1);
    ellipse(x, y, 20.0, 20.0);
}

fn segment(x: f32, y: f32, a: f32) {
    push_matrix();
    translate(x, y);
    rotate(a);
    line(0.0, 0.0, SEGLENGTH, 0.0);
    pop_matrix();
}
