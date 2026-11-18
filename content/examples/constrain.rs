/**
 * Constrain.
 *
 * Move the mouse across the screen to move the circle.
 * The program constrains the circle to its box.
 */
let mut mx: f32 = 0.0;
let mut my: f32 = 0.0;
let easing: f32 = 0.05;
let radius: i32 = 24;
let edge: i32 = 100;
let inner: i32 = edge + radius;

fn setup() {
    size(640, 360);
    no_stroke();
    ellipse_mode(RADIUS);
    rect_mode(CORNERS);
}

fn draw() {
    background(51);

    if abs(mouse_x() - mx) > 0.1 {
        mx = mx + (mouse_x() - mx) * easing;
    }
    if abs(mouse_y() - my) > 0.1 {
        my = my + (mouse_y() - my) * easing;
    }

    mx = constrain(mx, inner as f32, width() - inner as f32);
    my = constrain(my, inner as f32, height() - inner as f32);
    fill(76);
    rect(edge as f32, edge as f32, width() - edge as f32, height() - edge as f32);
    fill(255);
    ellipse(mx, my, radius as f32, radius as f32);
}
