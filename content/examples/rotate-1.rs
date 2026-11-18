/**
 * Rotate 1.
 *
 * Rotating simultaneously in the X and Y axis.
 * Transformation functions such as rotate() are additive.
 * Successively calling rotate(1.0) and rotate(2.0)
 * is equivalent to calling rotate(3.0).
 */
let mut a: f32 = 0.0;
let mut rSize: f32 = 0.0;  // rectangle size

fn setup() {
    size(640, 360, P3D);
    rSize = width() / 6.0;
    no_stroke();
    fill(204, 204);
}

fn draw() {
    background(126);

    a += 0.005;
    if a > TWO_PI {
        a = 0.0;
    }

    translate(width() / 2.0, height() / 2.0);

    rotate_x(a);
    rotate_y(a * 2.0);
    fill(255);
    rect(-rSize, -rSize, rSize * 2.0, rSize * 2.0);

    rotate_x(a * 1.001);
    rotate_y(a * 2.002);
    fill(0);
    rect(-rSize, -rSize, rSize * 2.0, rSize * 2.0);
}
