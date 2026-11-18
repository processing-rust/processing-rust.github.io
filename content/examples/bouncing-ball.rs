/**
 * Bouncing Ball with Vectors
 * by Daniel Shiffman.
 *
 * Demonstration of using vectors to control motion
 * of a body. This example is not object-oriented
 * See AccelerationWithVectors for an example of how
 * to simulate motion using vectors in an object.
 */
let mut location: PVector = PVector::new(0.0, 0.0);  // Location of shape
let mut velocity: PVector = PVector::new(0.0, 0.0);  // Velocity of shape
let mut gravity: PVector = PVector::new(0.0, 0.0);  // Gravity acts at the shape's acceleration

fn setup() {
    size(640, 360);
    location = PVector::new(100.0, 100.0);
    velocity = PVector::new(1.5, 2.1);
    gravity = PVector::new(0.0, 0.2);
}

fn draw() {
    background(0);

    // Add velocity to the location.
    location.add(&velocity);
    // Add gravity to velocity
    velocity.add(&gravity);

    // Bounce off edges
    if (location.x > width()) || (location.x < 0.0) {
        velocity.x = velocity.x * -1.0;
    }
    if location.y > height() {
        // We're reducing velocity ever so slightly
        // when it hits the bottom of the window
        velocity.y = velocity.y * -0.95;
        location.y = height();
    }

    // Display circle at location vector
    stroke(255);
    stroke_weight(2.0);
    fill(127);
    ellipse(location.x, location.y, 48.0, 48.0);
}
