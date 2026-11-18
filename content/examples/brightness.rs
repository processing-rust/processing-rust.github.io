/**
 * Brightness
 * by Rusty Robison.
 *
 * Brightness is the relative lightness or darkness of a color.
 * Move the cursor vertically over each bar to alter its brightness.
 */
let barWidth: i32 = 20;
let mut lastBar: i32 = -1;

fn setup() {
    size(640, 360);
    color_mode(HSB, width(), 100.0, height());
    no_stroke();
    background(0);
}

fn draw() {
    let whichBar: i32 = (mouse_x() / barWidth as f32) as i32;
    if whichBar != lastBar {
        let barX: i32 = whichBar * barWidth;
        fill(barX, 100, mouse_y());
        rect(barX as f32, 0.0, barWidth as f32, height());
        lastBar = whichBar;
    }
}
