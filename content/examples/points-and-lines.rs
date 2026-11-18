/**
 * Points and Lines.
 *
 * Points and lines can be used to draw basic geometry.
 * Change the value of the variable 'd' to scale the form.
 * The four variables set the positions based on the value of 'd'.
 */

fn setup() {
    let d: i32 = 70;
    let p1: i32 = d;
    let p2: i32 = p1 + d;
    let p3: i32 = p2 + d;
    let p4: i32 = p3 + d;

    size(640, 360);
    no_smooth();
    background(0);
    translate(140.0, 0.0);

    // Draw gray box
    stroke(153);
    line(p3 as f32, p3 as f32, p2 as f32, p3 as f32);
    line(p2 as f32, p3 as f32, p2 as f32, p2 as f32);
    line(p2 as f32, p2 as f32, p3 as f32, p2 as f32);
    line(p3 as f32, p2 as f32, p3 as f32, p3 as f32);

    // Draw white points
    stroke(255);
    point(p1 as f32, p1 as f32);
    point(p1 as f32, p3 as f32);
    point(p2 as f32, p4 as f32);
    point(p3 as f32, p1 as f32);
    point(p4 as f32, p2 as f32);
    point(p4 as f32, p4 as f32);
}
