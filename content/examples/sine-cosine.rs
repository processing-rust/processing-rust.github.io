/**
 * Sine Cosine.
 *
 * Linear movement with sin() and cos().
 * Numbers between 0 and PI*2 (TWO_PI which angles roughly 6.28)
 * are put into these functions and numbers between -1 and 1 are
 * returned. These values are then scaled to produce larger movements.
 */
let mut x1: f32 = 0.0;
let mut x2: f32 = 0.0;
let mut y1: f32 = 0.0;
let mut y2: f32 = 0.0;
let mut angle1: f32 = 0.0;
let mut angle2: f32 = 0.0;
let scalar: f32 = 70.0;

fn setup() {
    size(640, 360);
    no_stroke();
    rect_mode(CENTER);
}

fn draw() {
    background(0);

    let ang1: f32 = radians(angle1);
    let ang2: f32 = radians(angle2);

    x1 = width() / 2.0 + (scalar * ang1.cos());
    x2 = width() / 2.0 + (scalar * ang2.cos());

    y1 = height() / 2.0 + (scalar * ang1.sin());
    y2 = height() / 2.0 + (scalar * ang2.sin());

    fill(255);
    rect(width() * 0.5, height() * 0.5, 140.0, 140.0);

    fill(0, 102, 153);
    ellipse(x1, height() * 0.5 - 120.0, scalar, scalar);
    ellipse(x2, height() * 0.5 + 120.0, scalar, scalar);

    fill(255, 204, 0);
    ellipse(width() * 0.5 - 120.0, y1, scalar, scalar);
    ellipse(width() * 0.5 + 120.0, y2, scalar, scalar);

    angle1 += 2.0;
    angle2 += 3.0;
}
