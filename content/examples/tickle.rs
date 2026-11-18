/**
 * Tickle.
 *
 * The word "tickle" jitters when the cursor hovers over.
 * Sometimes, it can be tickled off the screen.
 */
let message: String = "tickle".to_string();
let mut x: f32 = 0.0;
let mut y: f32 = 0.0;  // X and Y coordinates of text
let mut hr: f32 = 0.0;
let mut vr: f32 = 0.0;  // horizontal and vertical radius of the text

fn setup() {
    size(640, 360);

    // Create the font
    text_font(create_font("SourceCodePro-Regular.ttf", 36.0));
    text_align(CENTER, CENTER);

    hr = text_width(&message) / 2.0;
    vr = (text_ascent() + text_descent()) / 2.0;
    no_stroke();
    x = width() / 2.0;
    y = height() / 2.0;
}

fn draw() {
    // Instead of clearing the background, fade it by drawing
    // a semi-transparent rectangle on top
    fill(204, 120);
    rect(0.0, 0.0, width(), height());

    // If the cursor is over the text, change the position
    if abs(mouse_x() - x) < hr && abs(mouse_y() - y) < vr {
        x += random(-5.0, 5.0);
        y += random(-5.0, 5.0);
    }
    fill(0);
    text("tickle", x, y);
}
