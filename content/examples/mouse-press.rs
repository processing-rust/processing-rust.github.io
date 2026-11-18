/**
 * Mouse Press.
 *
 * Move the mouse to position the shape.
 * Press the mouse button to invert the color.
 */
fn setup() {
    size(640, 360);
    no_smooth();
    fill(126);
    background(102);
}

fn draw() {
    if mouse_pressed_global() {
        stroke(255);
    } else {
        stroke(0);
    }
    line(mouse_x() - 66.0, mouse_y(), mouse_x() + 66.0, mouse_y());
    line(mouse_x(), mouse_y() - 66.0, mouse_x(), mouse_y() + 66.0);
}
