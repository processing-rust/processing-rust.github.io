/**
 * Shape Vertices.
 *
 * How to iterate over the vertices of a shape.
 * When loading an obj or SVG, getVertexCount()
 * will typically return 0 since all the vertices
 * are in the child shapes.
 *
 * You should iterate through the children and then
 * iterate through their vertices.
 */

// The shape
let mut uk: u64 = 0;

fn setup() {
    size(640, 360);
    // Load the shape
    uk = load_shape("uk.svg");
}

fn draw() {
    background(51);
    // Center where we will draw all the vertices
    translate(width() / 2.0 - shape_width(uk) / 2.0, height() / 2.0 - shape_height(uk) / 2.0);

    // Iterate over the children
    let children: i32 = uk.get_child_count() as i32;
    for i in 0..children {
        let child: u64 = uk.get_child(i);
        let total: i32 = child.get_vertex_count() as i32;

        // Now we can actually get the vertices from each child
        for j in 0..total {
            let v: PVector = child.get_vertex(j);
            // Cycling brightness for each vertex
            stroke((frame_count() as i32 + (i + 1) * j) % 255);
            // Just a dot for each one
            point(v.x, v.y);
        }
    }
}
