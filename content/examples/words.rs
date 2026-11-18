/**
 * Words.
 *
 * The text() function is used for writing words to the screen.
 * The letters can be aligned left, center, or right with the
 * textAlign() function.
 */
let mut f: u64 = 0;

fn setup() {
    size(640, 360);

    // Create the font
    print_array(&PFont::list());
    f = create_font("SpaceMono-Regular.ttf", 18.0);
    text_font(f);
}

fn draw() {
    background(102);
    text_align(RIGHT);
    draw_type(width() * 0.25);
    text_align(CENTER);
    draw_type(width() * 0.5);
    text_align(LEFT);
    draw_type(width() * 0.75);
}

fn draw_type(x: f32) {
    line(x, 0.0, x, 65.0);
    line(x, 220.0, x, height());
    fill(0);
    text("ichi", x, 95.0);
    fill(51);
    text("ni", x, 130.0);
    fill(204);
    text("san", x, 165.0);
    fill(255);
    text("shi", x, 210.0);
}
