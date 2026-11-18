/**
 * Noise Wave
 * by Daniel Shiffman.
 *
 * Using Perlin Noise to generate a wave-like pattern.
 */
let mut yoff: f32 = 0.0;  // 2nd dimension of perlin noise

fn setup() {
    size(640, 360);
}

fn draw() {
    background(51);

    fill(255);
    // We are going to draw a polygon out of the wave points
    begin_shape();

    let mut xoff: f32 = 0.0;  // Option #1: 2D Noise
    // float xoff = yoff; // Option #2: 1D Noise

    // Iterate over horizontal pixels
    {
        let mut x: f32 = 0.0;
        while x <= width() {
            // Calculate a y value according to noise, map to
            let y: f32 = map(noise(xoff, yoff), 0.0, 1.0, 200.0, 300.0);  // Option #1: 2D Noise
            // float y = map(noise(xoff), 0, 1, 200,300);    // Option #2: 1D Noise

            // Set the vertex
            vertex(x, y);
            // Increment x dimension for noise
            xoff += 0.05;
            x += 10.0;
        }
    }
    // increment y dimension for noise
    yoff += 0.01;
    vertex(width(), height());
    vertex(0.0, height());
    end_shape(CLOSE);
}
