/**
 * Distance 1D.
 *
 * Move the mouse left and right to control the
 * speed and direction of the moving shapes.
 */
let mut xpos1: f32 = 0.0;
let mut xpos2: f32 = 0.0;
let mut xpos3: f32 = 0.0;
let mut xpos4: f32 = 0.0;
let thin: i32 = 8;
let thick: i32 = 36;

fn setup() {
    size(640, 360);
    no_stroke();
    xpos1 = width() / 2.0;
    xpos2 = width() / 2.0;
    xpos3 = width() / 2.0;
    xpos4 = width() / 2.0;
}

fn draw() {
    background(0);

    let mx: f32 = mouse_x() * 0.4 - width() / 5.0;

    fill(102);
    rect(xpos2, 0.0, thick as f32, height() / 2.0);
    fill(204);
    rect(xpos1, 0.0, thin as f32, height() / 2.0);
    fill(102);
    rect(xpos4, height() / 2.0, thick as f32, height() / 2.0);
    fill(204);
    rect(xpos3, height() / 2.0, thin as f32, height() / 2.0);

    xpos1 += mx / 16.0;
    xpos2 += mx / 64.0;
    xpos3 -= mx / 16.0;
    xpos4 -= mx / 64.0;

    if xpos1 < (-thin) as f32 {
        xpos1 = width();
    }
    if xpos1 > width() {
        xpos1 = (-thin) as f32;
    }
    if xpos2 < (-thick) as f32 {
        xpos2 = width();
    }
    if xpos2 > width() {
        xpos2 = (-thick) as f32;
    }
    if xpos3 < (-thin) as f32 {
        xpos3 = width();
    }
    if xpos3 > width() {
        xpos3 = (-thin) as f32;
    }
    if xpos4 < (-thick) as f32 {
        xpos4 = width();
    }
    if xpos4 > width() {
        xpos4 = (-thick) as f32;
    }
}
