/**
 * Functions.
 *
 * The drawTarget() function makes it easy to draw many distinct targets.
 * Each call to drawTarget() specifies the position, size, and number of
 * rings for each target.
 */
fn setup() {
    size(640, 360);
    background(51);
    no_stroke();
    no_loop();
}

fn draw() {
    draw_target(width() * 0.25, height() * 0.4, 200, 4);
    draw_target(width() * 0.5, height() * 0.5, 300, 10);
    draw_target(width() * 0.75, height() * 0.3, 120, 6);
}

fn draw_target(xloc: f32, yloc: f32, size: i32, num: i32) {
    let grayvalues: f32 = (255 / num) as f32;
    let steps: f32 = (size / num) as f32;
    for i in 0..num {
        fill(i as f32 * grayvalues);
        ellipse(xloc, yloc, size as f32 - i as f32 * steps, size as f32 - i as f32 * steps);
    }
}
