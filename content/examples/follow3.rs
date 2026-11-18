/**
 * Follow 3
 * based on code from Keith Peters.
 *
 * A segmented line follows the mouse. The relative angle from
 * each segment to the next is calculated with atan2() and the
 * position of the next is calculated with sin() and cos().
 */
let mut x: Vec<f32> = vec![0.0; 20];
let mut y: Vec<f32> = vec![0.0; 20];
const SEGLENGTH: f32 = 18.0;

fn setup() {
    size(640, 360);
    stroke_weight(9.0);
    stroke(255, 100);
}

fn draw() {
    background(0);
    drag_segment(0, mouse_x(), mouse_y(), &mut x, &mut y);
    for i in 0..x.len() - 1 {
        drag_segment((i + 1) as i32, x[i], y[i], &mut x, &mut y);
    }
}

fn drag_segment(i: i32, xin: f32, yin: f32, x: &mut Vec<f32>, y: &mut Vec<f32>) {
    let dx: f32 = xin - x[i as usize];
    let dy: f32 = yin - y[i as usize];
    let angle: f32 = dy.atan2(dx);
    x[i as usize] = xin - angle.cos() * SEGLENGTH;
    y[i as usize] = yin - angle.sin() * SEGLENGTH;
    segment(x[i as usize], y[i as usize], angle);
}

fn segment(x: f32, y: f32, a: f32) {
    push_matrix();
    translate(x, y);
    rotate(a);
    line(0.0, 0.0, SEGLENGTH, 0.0);
    pop_matrix();
}
