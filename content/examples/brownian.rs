/**
 * Brownian motion.
 *
 * Recording random movement as a continuous line.
 */
let num: i32 = 2000;
let range: i32 = 6;

let mut ax: Vec<f32> = vec![0.0; num as usize];
let mut ay: Vec<f32> = vec![0.0; num as usize];

fn setup() {
    size(640, 360);
    for i in 0..num {
        ax[i as usize] = width() / 2.0;
        ay[i as usize] = height() / 2.0;
    }
    frame_rate(30.0);
}

fn draw() {
    background(51);

    // Shift all elements 1 place to the left
    for i in 1..num {
        ax[(i - 1) as usize] = ax[i as usize];
        ay[(i - 1) as usize] = ay[i as usize];
    }

    // Put a new value at the end of the array
    ax[(num - 1) as usize] += random((-range) as f32, range as f32);
    ay[(num - 1) as usize] += random((-range) as f32, range as f32);

    // Constrain all points to the screen
    ax[(num - 1) as usize] = constrain(ax[(num - 1) as usize], 0.0, width());
    ay[(num - 1) as usize] = constrain(ay[(num - 1) as usize], 0.0, height());

    // Draw a line connecting the points
    for i in 1..num {
        let val: f32 = i as f32 / num as f32 * 204.0 + 51.0;
        stroke(val);
        line(ax[(i - 1) as usize], ay[(i - 1) as usize], ax[i as usize], ay[i as usize]);
    }
}
