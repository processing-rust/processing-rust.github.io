/**
 * Simple Linear Gradient
 *
 * The lerpColor() function is useful for interpolating
 * between two colors.
 */

// Constants
const Y_AXIS: i32 = 1;
const X_AXIS: i32 = 2;
let mut b1: Color = color(0);
let mut b2: Color = color(0);
let mut c1: Color = color(0);
let mut c2: Color = color(0);

fn setup() {
    size(640, 360);

    // Define colors
    b1 = color(255);
    b2 = color(0);
    c1 = color(204, 102, 0);
    c2 = color(0, 102, 153);

    no_loop();
}

fn draw() {
    // Background
    set_gradient(0, 0, width() / 2.0, height(), b1, b2, X_AXIS);
    set_gradient((width() / 2.0) as i32, 0, width() / 2.0, height(), b2, b1, X_AXIS);
    // Foreground
    set_gradient(50, 90, 540.0, 80.0, c1, c2, Y_AXIS);
    set_gradient(50, 190, 540.0, 80.0, c2, c1, X_AXIS);
}

fn set_gradient(x: i32, y: i32, w: f32, h: f32, c1: Color, c2: Color, axis: i32) {
    no_fill();

    if axis == Y_AXIS {
        // Top to bottom gradient
        for i in y..=(y as f32 + h) as i32 {
            let inter: f32 = map(i as f32, y as f32, y as f32 + h, 0.0, 1.0);
            let c: Color = lerp_color(c1, c2, inter);
            stroke(c);
            line(x as f32, i as f32, x as f32 + w, i as f32);
        }
    } else if axis == X_AXIS {
        // Left to right gradient
        for i in x..=(x as f32 + w) as i32 {
            let inter: f32 = map(i as f32, x as f32, x as f32 + w, 0.0, 1.0);
            let c: Color = lerp_color(c1, c2, inter);
            stroke(c);
            line(i as f32, y as f32, i as f32, y as f32 + h);
        }
    }
}
