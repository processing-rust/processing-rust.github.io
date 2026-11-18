/**
 * Directional.
 *
 * Move the mouse the change the direction of the light.
 * Directional light comes from one direction and is stronger
 * when hitting a surface squarely and weaker if it hits at a
 * a gentle angle. After hitting a surface, a directional lights
 * scatters in all directions.
 */
fn setup() {
    size(640, 360, P3D);
    no_stroke();
    fill(204);
}

fn draw() {
    no_stroke();
    background(0);
    let dirY: f32 = (mouse_y() / height() as f32 - 0.5) * 2.0;
    let dirX: f32 = (mouse_x() / width() as f32 - 0.5) * 2.0;
    directional_light(204, 204, 204, -dirX, -dirY, -1);
    translate(width() / 2.0 - 100.0, height() / 2.0, 0.0);
    sphere(80);
    translate(200.0, 0.0, 0.0);
    sphere(80);
}
