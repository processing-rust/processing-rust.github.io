/**
 * Mouse Functions.
 *
 * Click on the box and drag it across the screen.
 */
let mut bx: f32 = 0.0;
let mut by: f32 = 0.0;
let boxSize: i32 = 75;
let mut overBox: bool = false;
let mut locked: bool = false;
let mut xOffset: f32 = 0.0;
let mut yOffset: f32 = 0.0;

fn setup() {
    size(640, 360);
    bx = width() / 2.0;
    by = height() / 2.0;
    rect_mode(RADIUS);
}

fn draw() {
    background(0);

    // Test if the cursor is over the box
    if mouse_x() > bx - boxSize as f32 && mouse_x() < bx + boxSize as f32 && mouse_y() > by - boxSize as f32 && mouse_y() < by + boxSize as f32 {
        overBox = true;
        if !locked {
            stroke(255);
            fill(153);
        }
    } else {
        stroke(153);
        fill(153);
        overBox = false;
    }

    // Draw the box
    rect(bx, by, boxSize as f32, boxSize as f32);
}

fn mouse_pressed() {
    if overBox {
        locked = true;
        fill(255, 255, 255);
    } else {
        locked = false;
    }
    xOffset = mouse_x() - bx;
    yOffset = mouse_y() - by;
}

fn mouse_dragged() {
    if locked {
        bx = mouse_x() - xOffset;
        by = mouse_y() - yOffset;
    }
}

fn mouse_released() {
    locked = false;
}
