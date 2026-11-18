/**
 * Hue.
 *
 * Hue is the color reflected from or transmitted through an object
 * and is typically referred to as the name of the color such as
 * red, blue, or yellow. In this example, move the cursor vertically
 * over each bar to alter its hue.
 */
let barWidth: i32 = 20;
let mut lastBar: i32 = -1;

fn setup() {
    size(640, 360);
    color_mode(HSB, height(), height(), height());
    no_stroke();
    background(0);
}

fn draw() {
    let whichBar: i32 = (mouse_x() / barWidth as f32) as i32;
    if whichBar != lastBar {
        let barX: i32 = whichBar * barWidth;
        fill(mouse_y(), height(), height());
        rect(barX as f32, 0.0, barWidth as f32, height());
        lastBar = whichBar;
    }
}
