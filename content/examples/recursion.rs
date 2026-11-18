/**
 * Recursion.
 *
 * A demonstration of recursion, which means functions call themselves.
 * Notice how the drawCircle() function calls itself at the end of its block.
 * It continues to do this until the variable "level" is equal to 1.
 */
fn setup() {
    size(640, 360);
    no_stroke();
    no_loop();
}

fn draw() {
    draw_circle((width() / 2.0) as i32, 280, 6);
}

fn draw_circle(x: i32, radius: i32, mut level: i32) {
    let tt: f32 = (126 * level) as f32 / 4.0;
    fill(tt);
    ellipse(x as f32, height() / 2.0, (radius * 2) as f32, (radius * 2) as f32);
    if level > 1 {
        level = level - 1;
        draw_circle(x - radius / 2, radius / 2, level);
        draw_circle(x + radius / 2, radius / 2, level);
    }
}
