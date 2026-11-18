/**
 * Polar to Cartesian
 * by Daniel Shiffman.
 *
 * Convert a polar coordinate (r,theta) to cartesian (x,y).
 * The calculations are x=r*cos(theta) and y=r*sin(theta).
 */
let mut r: f32 = 0.0;

// Angle and angular velocity, accleration
let mut theta: f32 = 0.0;
let mut theta_vel: f32 = 0.0;
let mut theta_acc: f32 = 0.0;

fn setup() {
    size(640, 360);

    // Initialize all values
    r = height() * 0.45;
    theta = 0.0;
    theta_vel = 0.0;
    theta_acc = 0.0001;
}

fn draw() {
    background(0);

    // Translate the origin point to the center of the screen
    translate(width() / 2.0, height() / 2.0);

    // Convert polar to cartesian
    let x: f32 = r * theta.cos();
    let y: f32 = r * theta.sin();

    // Draw the ellipse at the cartesian coordinate
    ellipse_mode(CENTER);
    no_stroke();
    fill(200);
    ellipse(x, y, 32.0, 32.0);

    // Apply acceleration and velocity to angle
    theta_vel += theta_acc;
    theta += theta_vel;
}
