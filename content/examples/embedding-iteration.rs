/**
 * Embedding Iteration.
 *
 * Embedding "for" structures allows repetition in two dimensions.
 *
 */

fn setup() {
    size(640, 360);
    background(0);

    let gridSize: i32 = 40;

    {
        let mut x: i32 = gridSize;
        while (x as f32) <= width() - gridSize as f32 {
            {
                let mut y: i32 = gridSize;
                while (y as f32) <= height() - gridSize as f32 {
                    no_stroke();
                    fill(255);
                    rect((x - 1) as f32, (y - 1) as f32, 3.0, 3.0);
                    stroke(255, 100);
                    line(x as f32, y as f32, width() / 2.0, height() / 2.0);
                    y += gridSize;
                }
            }
            x += gridSize;
        }
    }
}
