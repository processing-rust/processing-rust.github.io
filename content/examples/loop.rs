/**
 * Loop.
 *
 * If noLoop() is run in setup(), the code in draw()
 * is only run once. In this example, click the mouse
 * to run the loop() function to cause the draw() the
 * run continuously.
 */
let mut y: f32 = 180.0;

// The statements in the setup() function
// run once when the program begins
fn setup() {
    size(640, 360);  // Size should be the first statement
    stroke(255);  // Set stroke color to white
    no_loop();
}

fn draw() {
    background(0);  // Set the background to black
    line(0.0, y, width(), y);
    y = y - 1.0;
    if y < 0.0 {
        y = height();
    }
}

fn mouse_pressed() {
    r#loop();
}
