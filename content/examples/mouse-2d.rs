/**
 * Mouse 2D.
 *
 * Moving the mouse changes the position and size of each box.
 */
fn setup() {
    size(640, 360);
    no_stroke();
    rect_mode(CENTER);
}

fn draw() {
    background(51);
    fill(255, 204);
    rect(mouse_x(), height() / 2.0, mouse_y() / 2.0 + 10.0, mouse_y() / 2.0 + 10.0);
    fill(255, 204);
    let inverseX: i32 = (width() - mouse_x()) as i32;
    let inverseY: i32 = (height() - mouse_y()) as i32;
    rect(inverseX as f32, height() / 2.0, ((inverseY / 2) + 10) as f32, ((inverseY / 2) + 10) as f32);
}
