/**
 * Mixture
 * by Simon Greenwold.
 *
 * Display a box with three different kinds of lights.
 */
fn setup() {
    size(640, 360, P3D);
    no_stroke();
}

fn draw() {
    background(0);
    translate(width() / 2.0, height() / 2.0);

    // Orange point light on the right
    point_light(150, 100, 0, 200, -150, 0);  // Color  // Position

    // Blue directional light from the left
    directional_light(0, 102, 255, 1, 0, 0);  // Color  // The x-, y-, z-axis direction

    // Yellow spotlight from the front
    spot_light(255, 255, 109, 0, 40, 200, 0, -0.5, -0.5, PI / 2.0, 2);  // Color, Position, Direction  // Angle, concentration

    rotate_y(map(mouse_x(), 0.0, width(), 0.0, PI));
    rotate_x(map(mouse_y(), 0.0, height(), 0.0, PI));
    box(150);
}
