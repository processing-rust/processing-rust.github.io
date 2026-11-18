/**
 * Move Eye.
 * by Simon Greenwold.
 *
 * The camera lifts up (controlled by mouseY) while looking at the same point.
 */
fn setup() {
    size(640, 360, P3D);
    fill(204);
}

fn draw() {
    lights();
    background(0);

    // Change height of the camera with mouseY
    camera(30.0, mouse_y(), 220.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);  // eyeX, eyeY, eyeZ, centerX, centerY, centerZ  // upX, upY, upZ

    no_stroke();
    box(90);
    stroke(255);
    line(-100, 0, 0, 100, 0, 0);
    line(0, -100, 0, 0, 100, 0);
    line(0, 0, -100, 0, 0, 100);
}
