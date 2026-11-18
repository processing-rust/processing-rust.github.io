/**
 * Random Gaussian.
 *
 * This sketch draws ellipses with x and y locations tied to a gaussian distribution of random numbers.
 */
fn setup() {
    size(640, 360);
    background(0);
}

fn draw() {
    // Get a gaussian random number w/ mean of 0 and standard deviation of 1.0
    let val: f32 = random_gaussian();

    let sd: f32 = 60.0;  // Define a standard deviation
    let mean: f32 = width() / 2.0;  // Define a mean value (middle of the screen along the x-axis)
    let x: f32 = (val * sd) + mean;  // Scale the gaussian random number by standard deviation and mean

    no_stroke();
    fill(255, 10);
    ellipse(x, height() / 2.0, 32.0, 32.0);  // Draw an ellipse at our "normal" random location
}
