/**
 * Additive Wave
 * by Daniel Shiffman.
 *
 * Create a more complex wave by adding two waves together.
 */
const XSPACING: i32 = 8;  // How far apart should each horizontal location be spaced
let mut w: i32 = 0;  // Width of entire wave
const MAXWAVES: i32 = 4;  // total # of waves to add together

let mut theta: f32 = 0.0;
let mut amplitude: Vec<f32> = vec![0.0; MAXWAVES as usize];  // Height of wave
let mut dx: Vec<f32> = vec![0.0; MAXWAVES as usize];  // Value for incrementing X, to be calculated as a function of period and xspacing
let mut yvalues: Vec<f32> = Vec::new();  // Using an array to store height values for the wave (not entirely necessary)

fn setup() {
    size(640, 360);
    frame_rate(30.0);
    color_mode(RGB, 255.0, 255.0, 255.0, 100.0);
    w = (width() + 16.0) as i32;

    for i in 0..MAXWAVES {
        amplitude[i as usize] = random(10.0, 30.0);
        let period: f32 = random(100.0, 300.0);  // How many pixels before the wave repeats
        dx[i as usize] = (TWO_PI / period) * XSPACING as f32;
    }

    yvalues = vec![0.0; (w / XSPACING) as usize];
}

fn draw() {
    background(0);
    calc_wave(&amplitude, &dx, &mut theta, &mut yvalues);
    render_wave(&yvalues);
}

fn calc_wave(amplitude: &Vec<f32>, dx: &Vec<f32>, theta: &mut f32, yvalues: &mut Vec<f32>) {
    // Increment theta (try different values for 'angular velocity' here
    *theta += 0.02;

    // Set all height values to zero
    for i in 0..yvalues.len() {
        yvalues[i] = 0.0;
    }

    // Accumulate wave height values
    for j in 0..MAXWAVES {
        let mut x: f32 = *theta;
        for i in 0..yvalues.len() {
            // Every other wave is cosine instead of sine
            if j % 2 == 0 {
                yvalues[i] += x.sin() * amplitude[j as usize];
            } else {
                yvalues[i] += x.cos() * amplitude[j as usize];
            }
            x += dx[j as usize];
        }
    }
}

fn render_wave(yvalues: &Vec<f32>) {
    // A simple way to draw the wave with an ellipse at each location
    no_stroke();
    fill(255, 50);
    ellipse_mode(CENTER);
    for x in 0..yvalues.len() {
        ellipse((x as i32 * XSPACING) as f32, height() / 2.0 + yvalues[x], 16.0, 16.0);
    }
}
