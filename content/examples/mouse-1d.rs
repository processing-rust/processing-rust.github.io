/**
 * Mouse 1D.
 *
 * Move the mouse left and right to shift the balance.
 * The "mouseX" variable is used to control both the
 * size and color of the rectangles.
 */
fn setup() {
    size(640, 360);
    no_stroke();
    color_mode(RGB, height(), height(), height());
    rect_mode(CENTER);
}

fn draw() {
    background(0.0);

    let r1: f32 = map(mouse_x(), 0.0, width(), 0.0, height());
    let r2: f32 = height() - r1;

    fill(r1);
    rect(width() / 2.0 + r1 / 2.0, height() / 2.0, r1, r1);

    fill(r2);
    rect(width() / 2.0 - r2 / 2.0, height() / 2.0, r2, r2);
}
