/**
 * Moving On Curves.
 *
 * In this example, the circles moves along the curve y = x^4.
 * Click the mouse to have it move to a new position.
 */
let mut beginX: f32 = 20.0;  // Initial x-coordinate
let mut beginY: f32 = 10.0;  // Initial y-coordinate
let mut endX: f32 = 570.0;  // Final x-coordinate
let mut endY: f32 = 320.0;  // Final y-coordinate
let mut distX: f32 = 0.0;  // X-axis distance to move
let mut distY: f32 = 0.0;  // Y-axis distance to move
let exponent: f32 = 4.0;  // Determines the curve
let mut x: f32 = 0.0;  // Current x-coordinate
let mut y: f32 = 0.0;  // Current y-coordinate
let step: f32 = 0.01;  // Size of each step along the path
let mut pct: f32 = 0.0;  // Percentage traveled (0.0 to 1.0)

fn setup() {
    size(640, 360);
    no_stroke();
    distX = endX - beginX;
    distY = endY - beginY;
}

fn draw() {
    fill(0, 2);
    rect(0.0, 0.0, width(), height());
    pct += step;
    if pct < 1.0 {
        x = beginX + (pct * distX);
        y = beginY + (pow(pct, exponent) * distY);
    }
    fill(255);
    ellipse(x, y, 20.0, 20.0);
}

fn mouse_pressed() {
    pct = 0.0;
    beginX = x;
    beginY = y;
    endX = mouse_x();
    endY = mouse_y();
    distX = endX - beginX;
    distY = endY - beginY;
}
