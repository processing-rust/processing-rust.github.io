/**
 * Rotate Push Pop.
 *
 * The push() and pop() functions allow for more control over transformations.
 * The push function saves the current coordinate system to the stack
 * and pop() restores the prior coordinate system.
 */
let mut a: f32 = 0.0;  // Angle of rotation
let offset: f32 = PI / 24.0;  // Angle offset between boxes
let num: i32 = 12;  // Number of boxes

fn setup() {
    size(640, 360, P3D);
    no_stroke();
}

fn draw() {
    lights();

    background(0, 0, 26);
    translate(width() / 2.0, height() / 2.0);

    for i in 0..num {
        let gray: f32 = map(i as f32, 0.0, (num - 1) as f32, 0.0, 255.0);
        push_matrix();
        fill(gray);
        rotate_y(a + offset * i as f32);
        rotate_x(a / 2.0 + offset * i as f32);
        box(200);
        pop_matrix();
    }

    a += 0.01;
}
