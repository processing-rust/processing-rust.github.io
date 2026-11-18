/**
 * Saturation.
 *
 * Saturation is the strength or purity of the color and represents the
 * amount of gray in proportion to the hue. A "saturated" color is pure
 * and an "unsaturated" color has a large percentage of gray.
 * Move the cursor vertically over each bar to alter its saturation.
 */
let barWidth: i32 = 20;
let mut lastBar: i32 = -1;

fn setup() {
    size(640, 360);
    color_mode(HSB, width(), height(), 100.0);
    no_stroke();
}

fn draw() {
    let whichBar: i32 = (mouse_x() / barWidth as f32) as i32;
    if whichBar != lastBar {
        let barX: i32 = whichBar * barWidth;
        fill(barX, mouse_y(), 66);
        rect(barX as f32, 0.0, barWidth as f32, height());
        lastBar = whichBar;
    }
}
