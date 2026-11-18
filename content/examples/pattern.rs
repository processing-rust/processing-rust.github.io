/**
 * Patterns.
 *
 * Move the cursor over the image to draw with a software tool
 * which responds to the speed of the mouse.
 */
fn setup() {
    size(640, 360);
    background(102);
}

fn draw() {
    // Call the variableEllipse() method and send it the
    // parameters for the current mouse position
    // and the previous mouse position
    variable_ellipse(mouse_x() as i32, mouse_y() as i32, pmouse_x() as i32, pmouse_y() as i32);
}

// The simple method variableEllipse() was created specifically
// for this program. It calculates the speed of the mouse
// and draws a small ellipse if the mouse is moving slowly
// and draws a large ellipse if the mouse is moving quickly

fn variable_ellipse(x: i32, y: i32, px: i32, py: i32) {
    let speed: f32 = ((x - px).abs() + (y - py).abs()) as f32;
    stroke(speed);
    ellipse(x as f32, y as f32, speed, speed);
}
