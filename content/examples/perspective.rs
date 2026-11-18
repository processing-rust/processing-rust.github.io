/**
 * Perspective.
 *
 * Move the mouse left and right to change the field of view (fov).
 * Click to modify the aspect ratio. The perspective() function
 * sets a perspective projection applying foreshortening, making
 * distant objects appear smaller than closer ones. The parameters
 * define a viewing volume with the shape of truncated pyramid.
 * Objects near to the front of the volume appear their actual size,
 * while farther objects appear smaller. This projection simulates
 * the perspective of the world more accurately than orthographic projection.
 * The version of perspective without parameters sets the default
 * perspective and the version with four parameters allows the programmer
 * to set the area precisely.
 */
fn setup() {
    size(640, 360, P3D);
    no_stroke();
}

fn draw() {
    lights();
    background(0);
    let cameraY: f32 = height() / 2.0;
    let fov: f32 = mouse_x() / width() as f32 * PI / 2.0;
    let cameraZ: f32 = cameraY / (fov / 2.0).tan();
    let mut aspect: f32 = width() as f32 / height() as f32;
    if mouse_pressed_global() {
        aspect = aspect / 2.0;
    }
    perspective(fov, aspect, cameraZ / 10.0, cameraZ * 10.0);

    translate(width() / 2.0 + 30.0, height() / 2.0, 0.0);
    rotate_x(-PI / 6.0);
    rotate_y(PI / 3.0 + mouse_y() / height() as f32 * PI);
    box(45);
    translate(0.0, 0.0, -50.0);
    box(30);
}
