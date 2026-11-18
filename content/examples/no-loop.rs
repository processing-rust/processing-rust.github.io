/**
 * No Loop.
 *
 * The noLoop() function causes draw() to only run once.
 * Without calling noLoop(), the code inside draw() is
 * run continually.
 */
let mut y: f32 = 180.0;

// The statements in the setup() block
// run once when the program begins
fn setup() {
    size(640, 360);  // Size should be the first statement
    stroke(255);  // Set line drawing color to white
    no_loop();
}

// In this example, the code in the draw() block
// runs only once because of the noLoop() in setup()
fn draw() {
    background(0);  // Set the background to black
    line(0.0, y, width(), y);
    y = y - 1.0;
    if y < 0.0 {
        y = height();
    }
}
