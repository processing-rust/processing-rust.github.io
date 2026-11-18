/**
 * Reflection
 * by Simon Greenwold.
 *
 * Vary the specular reflection component of a material
 * with the horizontal position of the mouse.
 */
fn setup() {
    size(640, 360, P3D);
    no_stroke();
    color_mode(RGB, 1.0);
    fill(0.4);
}

fn draw() {
    background(0);
    translate(width() / 2.0, height() / 2.0);
    // Set the specular color of lights that follow
    light_specular(1, 1, 1);
    directional_light(0.8, 0.8, 0.8, 0, 0, -1);
    let s: f32 = mouse_x() / width() as f32;
    specular(s, s, s);
    sphere(120);
}
