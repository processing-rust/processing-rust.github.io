/**
 * Pulses.
 *
 * Software drawing instruments can follow a rhythm or abide by rules independent
 * of drawn gestures. This is a form of collaborative drawing in which the draftsperson
 * controls some aspects of the image and the software controls others.
 */
let mut angle: i32 = 0;

fn setup() {
    size(640, 360);
    background(102);
    no_stroke();
    fill(0, 102);
}

fn draw() {
    // Draw only when mouse is pressed
    if mouse_pressed_global() == true {
        angle += 5;
        let val: f32 = (radians(angle as f32)).cos() * 12.0;
        for a in (0..360).step_by(75) {
            let xoff: f32 = (radians(a as f32)).cos() * val;
            let yoff: f32 = (radians(a as f32)).sin() * val;
            fill(0);
            ellipse(mouse_x() + xoff, mouse_y() + yoff, val, val);
        }
        fill(255);
        ellipse(mouse_x(), mouse_y(), 2.0, 2.0);
    }
}
