/**
 * Create Graphics.
 *
 * The createGraphics() function creates an object from
 * the PGraphics class. PGraphics is the main graphics and
 * rendering context for Processing. The beginDraw() method
 * is necessary to prepare for drawing and endDraw() is
 * necessary to finish. Use this class if you need to draw
 * into an off-screen graphics buffer or to maintain two
 * drawing surfaces with different properties.
 */
let mut pg: u64 = 0;

fn setup() {
    size(640, 360);
    pg = create_graphics(400, 200);
}

fn draw() {
    fill(0, 12);
    rect(0.0, 0.0, width(), height());
    fill(255);
    no_stroke();
    ellipse(mouse_x(), mouse_y(), 60.0, 60.0);

    begin_graphics(pg);
    background(51);
    no_fill();
    stroke(255);
    ellipse(mouse_x() - 120.0, mouse_y() - 60.0, 60.0, 60.0);
    end_graphics();

    // Draw the offscreen buffer to the screen with image()
    image(pg, 120.0, 60.0);
}
