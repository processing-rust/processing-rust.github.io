/**
 * The Mandelbrot Set
 * by Daniel Shiffman.
 * (slight modification by l8l)
 *
 * Simple rendering of the Mandelbrot set.
 */

fn setup() {
    size(640, 360);
    no_loop();
    background(255);

    // Establish a range of values on the complex plane
    // A different range will allow us to "zoom" in or out on the fractal

    // It all starts with the width, try higher or lower values
    let w: f32 = 4.0;
    let h: f32 = (w * height()) / width();

    // Start at negative half the width and height
    let xmin: f32 = -w / 2.0;
    let ymin: f32 = -h / 2.0;

    // Make sure we can write to the pixels[] array.
    // Only need to do this once since we don't do any other drawing.
    let mut pixels = load_pixels();

    // Maximum number of iterations for each point on the complex plane
    let maxiterations: i32 = 100;

    // x goes from xmin to xmax
    let xmax: f32 = xmin + w;
    // y goes from ymin to ymax
    let ymax: f32 = ymin + h;

    // Calculate amount we increment x,y for each pixel
    let dx: f32 = (xmax - xmin) / (width());
    let dy: f32 = (ymax - ymin) / (height());

    // Start y
    let mut y: f32 = ymin;
    for j in 0..height() as i32 {
        // Start x
        let mut x: f32 = xmin;
        for i in 0..width() as i32 {
            // Now we test, as we iterate z = z^2 + c does z tend towards infinity?
            let mut a: f32 = x;
            let mut b: f32 = y;
            let mut n: i32 = 0;
            let max: f32 = 4.0;  // Infinity in our finite world is simple, let's just consider it 4
            let mut absOld: f32 = 0.0;
            let mut convergeNumber: f32 = maxiterations as f32;  // this will change if the while loop breaks due to non-convergence
            while n < maxiterations {
                // We suppose z = a+ib
                let aa: f32 = a * a;
                let bb: f32 = b * b;
                let abs: f32 = sqrt(aa + bb);
                if abs > max {
                    // |z| = sqrt(a^2+b^2)
                    // Now measure how much we exceeded the maximum:
                    let diffToLast: f32 = ((abs - absOld)) as f32;
                    let diffToMax: f32 = ((max - absOld)) as f32;
                    convergeNumber = n as f32 + diffToMax / diffToLast;
                    break;  // Bail
                }
                let twoab: f32 = 2.0 * a * b;
                a = aa - bb + x;  // this operation corresponds to z -> z^2+c where z=a+ib c=(x,y)
                b = twoab + y;
                n += 1;
                absOld = abs;
            }

            // We color each pixel based on how long it takes to get to infinity
            // If we never got there, let's pick the color black
            if n == maxiterations {
                pixels[(i as f32 + j as f32 * width()) as usize] = color(0).to_argb();
            } else {
                // Gosh, we could make fancy colors here if we wanted
                let norm: f32 = map(convergeNumber, 0.0, maxiterations as f32, 0.0, 1.0);
                pixels[(i as f32 + j as f32 * width()) as usize] = (color(map(sqrt(norm), 0.0, 1.0, 0.0, 255.0))).to_argb();
            }
            x += dx;
        }
        y += dy;
    }
    update_pixels(&pixels);
}
