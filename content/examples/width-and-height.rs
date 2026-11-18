/**
 * Width and Height.
 *
 * The 'width' and 'height' variables contain the width and height
 * of the display window as defined in the size() function.
 */
fn setup() {
    size(640, 360);
}

fn draw() {
    background(127);
    no_stroke();
    for i in (0..height() as i32).step_by(20) {
        fill(129, 206, 15);
        rect(0.0, i as f32, width(), 10.0);
        fill(255);
        rect(i as f32, 0.0, 10.0, height());
    }
}
