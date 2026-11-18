/**
 * Iteration.
 *
 * Iteration with a "for" structure to construct repetitive forms.
 */

fn setup() {
    let mut y: i32;
    let num: i32 = 14;

    size(640, 360);
    background(102);
    no_stroke();

    // White bars
    fill(255);
    y = 60;
    for i in 0..num / 3 {
        rect(50.0, y as f32, 475.0, 10.0);
        y += 20;
    }

    // Gray bars
    fill(51);
    y = 40;
    for i in 0..num {
        rect(405.0, y as f32, 30.0, 10.0);
        y += 20;
    }
    y = 50;
    for i in 0..num {
        rect(425.0, y as f32, 30.0, 10.0);
        y += 20;
    }

    // Thin lines
    y = 45;
    fill(0);
    for i in 0..num - 1 {
        rect(120.0, y as f32, 40.0, 1.0);
        y += 20;
    }
}
