/**
 * Sine Wave
 * by Daniel Shiffman.
 *
 * Render a simple sine wave.
 */
const XSPACING: i32 = 16;  // How far apart should each horizontal location be spaced
let mut w: i32 = 0;  // Width of entire wave

let mut theta: f32 = 0.0;  // Start angle at 0
const AMPLITUDE: f32 = 75.0;  // Height of wave
let period: f32 = 500.0;  // How many pixels before the wave repeats
let mut dx: f32 = 0.0;  // Value for incrementing X, a function of period and xspacing
let mut yvalues: Vec<f32> = Vec::new();  // Using an array to store height values for the wave

fn setup() {
    size(640, 360);
    w = (width() + 16.0) as i32;
    dx = (TWO_PI / period) * XSPACING as f32;
    yvalues = vec![0.0; (w / XSPACING) as usize];
}

fn draw() {
    background(0);
    calc_wave(dx, &mut theta, &mut yvalues);
    render_wave(&yvalues);
}

fn calc_wave(dx: f32, theta: &mut f32, yvalues: &mut Vec<f32>) {
    // Increment theta (try different values for 'angular velocity' here
    *theta += 0.02;

    // For every x value, calculate a y value with sine function
    let mut x: f32 = *theta;
    for i in 0..yvalues.len() {
        yvalues[i] = x.sin() * AMPLITUDE;
        x += dx;
    }
}

fn render_wave(yvalues: &Vec<f32>) {
    no_stroke();
    fill(255);
    // A simple way to draw the wave with an ellipse at each location
    for x in 0..yvalues.len() {
        ellipse((x as i32 * XSPACING) as f32, height() / 2.0 + yvalues[x], 16.0, 16.0);
    }
}
