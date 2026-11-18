/**
 * Reach 1
 * based on code from Keith Peters.
 *
 * The arm follows the position of the mouse by
 * calculating the angles with atan2().
 */
const SEGLENGTH: f32 = 80.0;
let mut x: f32 = 0.0;
let mut y: f32 = 0.0;
let mut x2: f32 = 0.0;
let mut y2: f32 = 0.0;

fn setup() {
    size(640, 360);
    stroke_weight(20.0);
    stroke(255, 100);

    x = width() / 2.0;
    y = height() / 2.0;
    x2 = x;
    y2 = y;
}

fn draw() {
    background(0);

    let mut dx: f32 = mouse_x() - x;
    let mut dy: f32 = mouse_y() - y;
    let angle1: f32 = dy.atan2(dx);

    let tx: f32 = mouse_x() - angle1.cos() * SEGLENGTH;
    let ty: f32 = mouse_y() - angle1.sin() * SEGLENGTH;
    dx = tx - x2;
    dy = ty - y2;
    let angle2: f32 = dy.atan2(dx);
    x = x2 + angle2.cos() * SEGLENGTH;
    y = y2 + angle2.sin() * SEGLENGTH;

    segment(x, y, angle1);
    segment(x2, y2, angle2);
}

fn segment(x: f32, y: f32, a: f32) {
    push_matrix();
    translate(x, y);
    rotate(a);
    line(0.0, 0.0, SEGLENGTH, 0.0);
    pop_matrix();
}
