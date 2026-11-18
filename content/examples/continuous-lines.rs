/**
 * Continuous Lines.
 *
 * Click and drag the mouse to draw a line.
 */
fn setup() {
    size(640, 360);
    background(102);
}

fn draw() {
    stroke(255);
    if mouse_pressed_global() == true {
        line(mouse_x(), mouse_y(), pmouse_x(), pmouse_y());
    }
}
