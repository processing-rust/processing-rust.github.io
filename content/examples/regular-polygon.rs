/**
 * Regular Polygon
 *
 * What is your favorite? Pentagon? Hexagon? Heptagon?
 * No? What about the icosagon? The polygon() function
 * created for this example is capable of drawing any
 * regular polygon. Try placing different numbers into the
 * polygon() function calls within draw() to explore.
 */
fn setup() {
    size(640, 360);
}

fn draw() {
    background(102);

    push_matrix();
    translate(width() * 0.2, height() * 0.5);
    rotate(frame_count() as f32 / 200.0);
    polygon(0.0, 0.0, 82.0, 3);  // Triangle
    pop_matrix();

    push_matrix();
    translate(width() * 0.5, height() * 0.5);
    rotate(frame_count() as f32 / 50.0);
    polygon(0.0, 0.0, 80.0, 20);  // Icosagon
    pop_matrix();

    push_matrix();
    translate(width() * 0.8, height() * 0.5);
    rotate(frame_count() as f32 / -100.0);
    polygon(0.0, 0.0, 70.0, 7);  // Heptagon
    pop_matrix();
}

fn polygon(x: f32, y: f32, radius: f32, npoints: i32) {
    let angle: f32 = TWO_PI / npoints as f32;
    begin_shape();
    {
        let mut a: f32 = 0.0;
        while a < TWO_PI {
            let sx: f32 = x + a.cos() * radius;
            let sy: f32 = y + a.sin() * radius;
            vertex(sx, sy);
            a += angle;
        }
    }
    end_shape(CLOSE);
}
