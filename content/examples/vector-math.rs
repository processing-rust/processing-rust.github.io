/**
 * Vector
 * by Daniel Shiffman.
 *
 * Demonstration of some basic vector math: subtraction,
 * normalization, scaling. Normalizing a vector sets
 * its length to 1.
 */
fn setup() {
    size(640, 360);
}

fn draw() {
    background(0);

    // A vector that points to the mouse location
    let mut mouse: PVector = PVector::new(mouse_x(), mouse_y());

    // A vector that points to the center of the window
    let center: PVector = PVector::new(width() / 2.0, height() / 2.0);

    // Subtract center from mouse which results in a
    // vector that points from center to mouse
    mouse.sub(&center);

    // Normalize the vector
    mouse.normalize();

    // Multiply its length by 150 (Scaling its length)
    mouse.mult(150.0);

    translate(width() / 2.0, height() / 2.0);
    // Draw the resulting vector
    stroke(255);
    stroke_weight(4.0);
    line(0.0, 0.0, mouse.x, mouse.y);
}
