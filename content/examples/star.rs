/**
 * Star
 *
 * The star() function created for this example is capable of drawing a
 * wide range of different forms. Try placing different numbers into the
 * star() function calls within draw() to explore.
 */
fn setup() {
    size(640, 360);
}

fn draw() {
    background(102);

    push_matrix();
    translate(width() * 0.2, height() * 0.5);
    rotate(frame_count() as f32 / 200.0);
    star(0.0, 0.0, 5.0, 70.0, 3);
    pop_matrix();

    push_matrix();
    translate(width() * 0.5, height() * 0.5);
    rotate(frame_count() as f32 / 400.0);
    star(0.0, 0.0, 80.0, 100.0, 40);
    pop_matrix();

    push_matrix();
    translate(width() * 0.8, height() * 0.5);
    rotate(frame_count() as f32 / -100.0);
    star(0.0, 0.0, 30.0, 70.0, 5);
    pop_matrix();
}

fn star(x: f32, y: f32, radius1: f32, radius2: f32, npoints: i32) {
    let angle: f32 = TWO_PI / npoints as f32;
    let halfAngle: f32 = angle / 2.0;
    begin_shape();
    {
        let mut a: f32 = 0.0;
        while a < TWO_PI {
            let mut sx: f32 = x + a.cos() * radius2;
            let mut sy: f32 = y + a.sin() * radius2;
            vertex(sx, sy);
            sx = x + (a + halfAngle).cos() * radius1;
            sy = y + (a + halfAngle).sin() * radius1;
            vertex(sx, sy);
            a += angle;
        }
    }
    end_shape(CLOSE);
}
