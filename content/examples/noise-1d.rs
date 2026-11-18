/**
 * Noise1D.
 *
 * Using 1D Perlin Noise to assign location.
 */
let mut xoff: f32 = 0.0;
let xincrement: f32 = 0.01;

fn setup() {
    size(640, 360);
    background(0);
    no_stroke();
}

fn draw() {
    // Create an alpha blended background
    fill(0, 10);
    rect(0.0, 0.0, width(), height());

    //float n = random(0,width);  // Try this line instead of noise

    // Get a noise value based on xoff and scale it according to the window's width
    let n: f32 = noise(xoff) * width();

    // With each cycle, increment xoff
    xoff += xincrement;

    // Draw the ellipse at the value produced by perlin noise
    fill(200);
    ellipse(n, height() / 2.0, 64.0, 64.0);
}
