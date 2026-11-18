/**
 * Variables.
 *
 * Variables are used for storing values. In this example, change
 * the values of variables to affect the composition.
 */

fn setup() {
    size(640, 360);
    background(0);
    stroke(153);
    stroke_weight(4.0);
    stroke_cap(SQUARE);

    let mut a: i32 = 50;
    let mut b: i32 = 120;
    let c: i32 = 180;

    line(a as f32, b as f32, (a + c) as f32, b as f32);
    line(a as f32, (b + 10) as f32, (a + c) as f32, (b + 10) as f32);
    line(a as f32, (b + 20) as f32, (a + c) as f32, (b + 20) as f32);
    line(a as f32, (b + 30) as f32, (a + c) as f32, (b + 30) as f32);

    a = a + c;
    b = (height() - b as f32) as i32;

    line(a as f32, b as f32, (a + c) as f32, b as f32);
    line(a as f32, (b + 10) as f32, (a + c) as f32, (b + 10) as f32);
    line(a as f32, (b + 20) as f32, (a + c) as f32, (b + 20) as f32);
    line(a as f32, (b + 30) as f32, (a + c) as f32, (b + 30) as f32);

    a = a + c;
    b = (height() - b as f32) as i32;

    line(a as f32, b as f32, (a + c) as f32, b as f32);
    line(a as f32, (b + 10) as f32, (a + c) as f32, (b + 10) as f32);
    line(a as f32, (b + 20) as f32, (a + c) as f32, (b + 20) as f32);
    line(a as f32, (b + 30) as f32, (a + c) as f32, (b + 30) as f32);
}
