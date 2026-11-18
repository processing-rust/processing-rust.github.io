/**
 * Load and Display an OBJ Shape.
 *
 * The loadShape() command is used to read simple SVG (Scalable Vector Graphics)
 * files and OBJ (Object) files into a Processing sketch. This example loads an
 * OBJ file of a rocket and displays it to the screen.
 */
let mut rocket: u64 = 0;

let mut ry: f32 = 0.0;

fn setup() {
    size(640, 360, P3D);

    rocket = load_shape("rocket.obj");
}

fn draw() {
    background(0);
    lights();

    translate(width() / 2.0, height() / 2.0 + 100.0, -200.0);
    rotate_z(PI);
    rotate_y(ry);
    shape(rocket);

    ry += 0.02;
}
