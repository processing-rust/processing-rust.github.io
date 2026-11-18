/**
 * Relativity.
 *
 * Each color is perceived in relation to other colors. The top and bottom
 * bars each contain the same component colors, but a different display order
 * causes individual colors to appear differently.
 */
let mut a: Color = color(0);
let mut b: Color = color(0);
let mut c: Color = color(0);
let mut d: Color = color(0);
let mut e: Color = color(0);

fn setup() {
    size(640, 360);
    no_stroke();
    a = color(165, 167, 20);
    b = color(77, 86, 59);
    c = color(42, 106, 105);
    d = color(165, 89, 20);
    e = color(146, 150, 127);
    no_loop();  // Draw only one time
}

fn draw() {
    draw_band(a, b, c, d, e, 0, (width() / 128.0) as i32);
    draw_band(c, a, d, b, e, (height() / 2.0) as i32, (width() / 128.0) as i32);
}

fn draw_band(v: Color, w: Color, x: Color, y: Color, z: Color, ypos: i32, barWidth: i32) {
    let num: i32 = 5;
    let colorOrder = vec![v, w, x, y, z];
    {
        let mut i: i32 = 0;
        while (i as f32) < width() {
            for j in 0..num {
                fill(colorOrder[j as usize]);
                rect((i + j * barWidth) as f32, ypos as f32, barWidth as f32, height() / 2.0);
            }
            i += barWidth * num;
        }
    }
}
