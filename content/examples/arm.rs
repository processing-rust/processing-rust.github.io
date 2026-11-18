/**
 * Arm.
 *
 * The angle of each segment is controlled with the mouseX and
 * mouseY position. The transformations applied to the first segment
 * are also applied to the second segment because they are inside
 * the same pushMatrix() and popMatrix() group.
 */
let mut x: f32 = 0.0;
let mut y: f32 = 0.0;
let mut angle1: f32 = 0.0;
let mut angle2: f32 = 0.0;
const SEGLENGTH: f32 = 100.0;

fn setup() {
    size(640, 360);
    stroke_weight(30.0);
    stroke(255, 160);

    x = width() * 0.3;
    y = height() * 0.5;
}

fn draw() {
    background(0);

    angle1 = (mouse_x() / width() as f32 - 0.5) * -PI;
    angle2 = (mouse_y() / height() as f32 - 0.5) * PI;

    push_matrix();
    segment(x, y, angle1);
    segment(SEGLENGTH, 0.0, angle2);
    pop_matrix();
}

fn segment(x: f32, y: f32, a: f32) {
    translate(x, y);
    rotate(a);
    line(0.0, 0.0, SEGLENGTH, 0.0);
}
