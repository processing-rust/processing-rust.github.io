/**
 * Triangle Strip
 * by Ira Greenberg.
 *
 * Generate a closed ring using the vertex() function and
 * beginShape(TRIANGLE_STRIP) mode. The outsideRadius and insideRadius
 * variables control ring's radii respectively.
 */
let mut x: i32 = 0;
let mut y: i32 = 0;
let outsideRadius: f32 = 150.0;
let insideRadius: f32 = 100.0;

fn setup() {
    size(640, 360);
    background(204);
    x = (width() / 2.0) as i32;
    y = (height() / 2.0) as i32;
}

fn draw() {
    background(204);

    let numPoints: i32 = (map(mouse_x(), 0.0, width(), 6.0, 60.0)) as i32;
    let mut angle: f32 = 0.0;
    let angleStep: f32 = 180.0 / numPoints as f32;

    begin_shape(TRIANGLE_STRIP);
    for i in 0..=numPoints {
        let mut px: f32 = x as f32 + radians(angle).cos() * outsideRadius;
        let mut py: f32 = y as f32 + radians(angle).sin() * outsideRadius;
        angle += angleStep;
        vertex(px, py);
        px = x as f32 + radians(angle).cos() * insideRadius;
        py = y as f32 + radians(angle).sin() * insideRadius;
        vertex(px, py);
        angle += angleStep;
    }
    end_shape();
}
