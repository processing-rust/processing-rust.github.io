/**
 * Bounce.
 *
 * When the shape hits the edge of the window, it reverses its direction.
 */
let rad: i32 = 60;  // Width of the shape
let mut xpos: f32 = 0.0;
let mut ypos: f32 = 0.0;  // Starting position of shape

let xspeed: f32 = 2.8;  // Speed of the shape
let yspeed: f32 = 2.2;  // Speed of the shape

let mut xdirection: i32 = 1;  // Left or Right
let mut ydirection: i32 = 1;  // Top to Bottom

fn setup() {
    size(640, 360);
    no_stroke();
    frame_rate(30.0);
    ellipse_mode(RADIUS);
    // Set the starting position of the shape
    xpos = width() / 2.0;
    ypos = height() / 2.0;
}

fn draw() {
    background(102);

    // Update the position of the shape
    xpos = xpos + (xspeed * xdirection as f32);
    ypos = ypos + (yspeed * ydirection as f32);

    // Test to see if the shape exceeds the boundaries of the screen
    // If it does, reverse its direction by multiplying by -1
    if xpos > width() - rad as f32 || xpos < rad as f32 {
        xdirection *= -1;
    }
    if ypos > height() - rad as f32 || ypos < rad as f32 {
        ydirection *= -1;
    }

    // Draw the shape
    ellipse(xpos, ypos, rad as f32, rad as f32);
}
