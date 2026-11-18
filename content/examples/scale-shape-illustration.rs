/**
 * Scale Shape.
 * Illustration by George Brower.
 *
 * Move the mouse left and right to zoom the SVG file.
 * This shows how, unlike an imported image, the lines
 * remain smooth at any size.
 */
let mut bot: u64 = 0;

fn setup() {
    size(640, 360);
    // The file "bot1.svg" must be in the data folder
    // of the current sketch to load successfully
    bot = load_shape("bot1.svg");
}

fn draw() {
    background(102);
    translate(width() / 2.0, height() / 2.0);
    let zoom: f32 = map(mouse_x(), 0.0, width(), 0.1, 4.5);
    scale(zoom);
    shape(bot, -140, -140);
}
