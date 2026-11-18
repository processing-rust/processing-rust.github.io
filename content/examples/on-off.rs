/**
 * On/Off.
 *
 * Uses the default lights to show a simple box. The lights() function
 * is used to turn on the default lighting. Click the mouse to turn the
 * lights off.
 */
let mut spin: f32 = 0.0;

fn setup() {
    size(640, 360, P3D);
    no_stroke();
}

fn draw() {
    background(51);

    if !mouse_pressed_global() {
        lights();
    }

    spin += 0.01;

    push_matrix();
    translate(width() / 2.0, height() / 2.0, 0.0);
    rotate_x(PI / 9.0);
    rotate_y(PI / 5.0 + spin);
    box(150);
    pop_matrix();
}
