/**
 * Array 2D.
 *
 * Demonstrates the syntax for creating a two-dimensional (2D) array.
 * Values in a 2D array are accessed through two index values.
 * 2D arrays are useful for storing images. In this example, each dot
 * is colored in relation to its distance from the center of the image.
 */
let mut distances: Vec<Vec<f32>> = Vec::new();
let mut maxDistance: f32 = 0.0;
let mut spacer: i32 = 0;

fn setup() {
    size(640, 360);
    maxDistance = dist(width() / 2.0, height() / 2.0, width(), height());
    distances = vec![vec![0.0; height() as usize]; width() as usize];
    for y in 0..height() as i32 {
        for x in 0..width() as i32 {
            let distance: f32 = dist(width() / 2.0, height() / 2.0, x as f32, y as f32);
            distances[x as usize][y as usize] = distance / maxDistance * 255.0;
        }
    }
    spacer = 10;
    stroke_weight(6.0);
    no_loop();  // Run once and stop
}

fn draw() {
    background(0);
    // This embedded loop skips over values in the arrays based on
    // the spacer variable, so there are more values in the array
    // than are drawn here. Change the value of the spacer variable
    // to change the density of the points
    {
        let mut y: i32 = 0;
        while (y as f32) < height() {
            {
                let mut x: i32 = 0;
                while (x as f32) < width() {
                    stroke(distances[x as usize][y as usize]);
                    point((x + spacer / 2) as f32, (y + spacer / 2) as f32);
                    x += spacer;
                }
            }
            y += spacer;
        }
    }
}
