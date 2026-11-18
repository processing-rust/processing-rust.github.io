/**
 * Perspective vs. Ortho
 *
 * Move the mouse left to right to change the "far"
 * parameter for the perspective() and ortho() functions.
 * This parameter sets the maximum distance from the
 * origin away from the viewer and will clip the geometry.
 * Click a mouse button to switch between the perspective and
 * orthographic projections.
 */
let mut showPerspective: bool = false;

fn setup() {
    size(600, 360, P3D);
    no_fill();
    fill(255);
    no_stroke();
}

fn draw() {
    lights();
    background(0);
    let far: f32 = map(mouse_x(), 0.0, width(), 120.0, 400.0);
    if showPerspective == true {
        perspective(PI / 3.0, width() as f32 / height() as f32, 10, far);
    } else {
        ortho(-width() / 2.0, width() / 2.0, -height() / 2.0, height() / 2.0, 10, far);
    }
    translate(width() / 2.0, height() / 2.0, 0.0);
    rotate_x(-PI / 6.0);
    rotate_y(PI / 3.0);
    box(180);
}

fn mouse_pressed() {
    showPerspective = !showPerspective;
}
