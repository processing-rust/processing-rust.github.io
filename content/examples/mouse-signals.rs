/**
 * Mouse Signals
 *
 * Move and click the mouse to generate signals.
 * The top row is the signal from "mouseX",
 * the middle row is the signal from "mouseY",
 * and the bottom row is the signal from "mousePressed".
 */
let mut xvals: Vec<i32> = Vec::new();
let mut yvals: Vec<i32> = Vec::new();
let mut bvals: Vec<i32> = Vec::new();

fn setup() {
    size(640, 360);
    no_smooth();
    xvals = vec![0; width() as usize];
    yvals = vec![0; width() as usize];
    bvals = vec![0; width() as usize];
}

fn draw() {
    background(102);

    for i in 1..width() as i32 {
        xvals[(i - 1) as usize] = xvals[i as usize];
        yvals[(i - 1) as usize] = yvals[i as usize];
        bvals[(i - 1) as usize] = bvals[i as usize];
    }
    // Add the new values to the end of the array
    xvals[(width() - 1.0) as usize] = mouse_x() as i32;
    yvals[(width() - 1.0) as usize] = mouse_y() as i32;

    if mouse_pressed_global() == true {
        bvals[(width() - 1.0) as usize] = 0;
    } else {
        bvals[(width() - 1.0) as usize] = (height() / 3.0) as i32;
    }

    fill(255);
    no_stroke();
    rect(0.0, height() / 3.0, width(), height() / 3.0 + 1.0);

    for i in 1..width() as i32 {
        // Draw the x-values
        stroke(255);
        point(i as f32, map((xvals[i as usize]) as f32, 0.0, width(), 0.0, height() / 3.0 - 1.0));

        // Draw the y-values
        stroke(0);
        point(i as f32, height() / 3.0 + (yvals[i as usize] / 3) as f32);

        // Draw the mouse presses
        stroke(255);
        line(i as f32, (2.0 * height() / 3.0) + (bvals[i as usize]) as f32, i as f32, (2.0 * height() / 3.0) + (bvals[(i - 1) as usize]) as f32);
    }
}
