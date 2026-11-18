/**
 * Morph.
 *
 * Changing one shape into another by interpolating
 * vertices from one to another
 */

// Two ArrayLists to store the vertices for two shapes
// This example assumes that each shape will have the same
// number of vertices, i.e. the size of each ArrayList will be the same
let mut circle: Vec<PVector> = Vec::new();
let mut square: Vec<PVector> = Vec::new();

// An ArrayList for a third set of vertices, the ones we will be drawing
// in the window
let mut morph: Vec<PVector> = Vec::new();

// This boolean variable will control if we are morphing to a circle or square
let mut state: bool = false;

fn setup() {
    size(640, 360);

    // Create a circle using vectors pointing from center
    for angle in (0..360).step_by(9) {
        // Note we are not starting from 0 in order to match the
        // path of a circle.
        let mut v: PVector = PVector::from_angle(radians((angle - 135) as f32));
        v.mult(100.0);
        circle.push(v);
        // Let's fill out morph ArrayList with blank PVectors while we are at it
        morph.push(PVector::new(0.0, 0.0));
    }

    // A square is a bunch of vertices along straight lines
    // Top of square
    for x in (-50..50).step_by(10) {
        square.push(PVector::new(x as f32, -50.0));
    }
    // Right side
    for y in (-50..50).step_by(10) {
        square.push(PVector::new(50.0, y as f32));
    }
    // Bottom
    for x in (-50 + 1..=50).rev().step_by(10) {
        square.push(PVector::new(x as f32, 50.0));
    }
    // Left side
    for y in (-50 + 1..=50).rev().step_by(10) {
        square.push(PVector::new(-50.0, y as f32));
    }
}

fn draw() {
    background(51);

    // We will keep how far the vertices are from their target
    let mut totalDistance: f32 = 0.0;

    // Look at each vertex
    for i in 0..circle.len() {
        let mut v1: PVector;
        // Are we lerping to the circle or square?
        if state {
            v1 = circle[i];
        } else {
            v1 = square[i];
        }
        // Get the vertex we will draw
        let mut v2: PVector = morph[i];
        // Lerp to the target
        v2.lerp(&v1, 0.1);
        // Check how far we are from target
        totalDistance += v1.dist(&v2);
    }

    // If all the vertices are close, switch shape
    if totalDistance < 0.1 {
        state = !state;
    }

    // Draw relative to center
    translate(width() / 2.0, height() / 2.0);
    stroke_weight(4.0);
    // Draw a polygon that makes up all the vertices
    begin_shape();
    no_fill();
    stroke(255);
    for &v in morph.iter() {
        vertex(v.x, v.y);
    }
    end_shape(CLOSE);
}
