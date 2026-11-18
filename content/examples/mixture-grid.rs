/**
 * Mixture Grid
 * modified from an example by Simon Greenwold.
 *
 * Display a 2D grid of boxes with three different kinds of lights.
 */
fn setup() {
    size(640, 360, P3D);
    no_stroke();
}

fn draw() {
    define_lights();
    background(0);

    for x in (0..=width() as i32).step_by(60) {
        for y in (0..=height() as i32).step_by(60) {
            push_matrix();
            translate(x as f32, y as f32);
            rotate_y(map(mouse_x(), 0.0, width(), 0.0, PI));
            rotate_x(map(mouse_y(), 0.0, height(), 0.0, PI));
            box(90);
            pop_matrix();
        }
    }
}

fn define_lights() {
    // Orange point light on the right
    point_light(150, 100, 0, 200, -150, 0);  // Color  // Position

    // Blue directional light from the left
    directional_light(0, 102, 255, 1, 0, 0);  // Color  // The x-, y-, z-axis direction

    // Yellow spotlight from the front
    spot_light(255, 255, 109, 0, 40, 200, 0, -0.5, -0.5, PI / 2.0, 2);  // Color, Position, Direction  // Angle, concentration
}
