/**
 * Distance 2D.
 *
 * Move the mouse across the image to obscure and reveal the matrix.
 * Measures the distance from the mouse to each square and sets the
 * size proportionally.
 */
let mut max_distance: f32 = 0.0;

fn setup() {
    size(640, 360);
    no_stroke();
    max_distance = dist(0.0, 0.0, width(), height());
}

fn draw() {
    background(0);

    for i in (0..=width() as i32).step_by(20) {
        for j in (0..=height() as i32).step_by(20) {
            let mut size: f32 = dist(mouse_x(), mouse_y(), i as f32, j as f32);
            size = size / max_distance * 66.0;
            ellipse(i as f32, j as f32, size, size);
        }
    }
}
