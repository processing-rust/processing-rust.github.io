/**
 * Radial Gradient.
 *
 * Draws a series of concentric circles to create a gradient
 * from one color to another.
 */
let mut dim: i32 = 0;

fn setup() {
    size(640, 360);
    dim = (width() / 2.0) as i32;
    background(0);
    color_mode(HSB, 360.0, 100.0, 100.0);
    no_stroke();
    ellipse_mode(RADIUS);
    frame_rate(1.0);
}

fn draw() {
    background(0);
    {
        let mut x: i32 = 0;
        while (x as f32) <= width() {
            draw_gradient(x as f32, height() / 2.0, dim);
            x += dim;
        }
    }
}

fn draw_gradient(x: f32, y: f32, dim: i32) {
    let radius: i32 = dim / 2;
    let mut h: f32 = random(0.0, 360.0);
    for r in (0 + 1..=radius).rev() {
        fill(h, 90, 90);
        ellipse(x, y, r as f32, r as f32);
        h = (h + 1.0) % 360.0;
    }
}
