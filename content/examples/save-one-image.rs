/**
 * Save One Image
 *
 * The save() function allows you to save an image from the
 * display window. In this example, save() is run when a mouse
 * button is pressed. The image "line.tif" is saved to the
 * same folder as the sketch's program file.
 */
fn setup() {
    size(640, 360);
}

fn draw() {
    background(204);
    line(0.0, 0.0, mouse_x(), height());
    line(width(), 0.0, 0.0, mouse_y());
}

fn mouse_pressed() {
    save("line.tif");
}
