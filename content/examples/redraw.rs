/**
 * Redraw.
 *
 * The redraw() function makes draw() execute once.
 * In this example, draw() is executed once every time
 * the mouse is clicked.
 */
let mut y: f32 = 0.0;

// The statements in the setup() function
// execute once when the program begins
fn setup() {
    size(640, 360);  // Size should be the first statement
    stroke(255);  // Set line drawing color to white
    no_loop();
    y = height() * 0.5;
}

// The code in draw() is run until the program
// is stopped. Each statement is executed in
// sequence and after the last line is read,
// the first line is run again.
fn draw() {
    background(0);  // Set the background to black
    y = y - 4.0;
    if y < 0.0 {
        y = height();
    }
    line(0.0, y, width(), y);
}

fn mouse_pressed() {
    redraw();
}
