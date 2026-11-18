/**
 * Shape Primitives.
 *
 * The basic shape primitive functions are triangle(),
 * rect(), quad(), ellipse(), and arc(). Squares are made
 * with rect() and circles are made with ellipse(). Each
 * of these functions requires a number of parameters to
 * determine the shape's position and size.
 */

fn setup() {
    size(640, 360);
    background(0);
    no_stroke();

    fill(204);
    triangle(18.0, 18.0, 18.0, 360.0, 81.0, 360.0);

    fill(102);
    rect(81.0, 81.0, 63.0, 63.0);

    fill(204);
    quad(189.0, 18.0, 216.0, 18.0, 216.0, 360.0, 144.0, 360.0);

    fill(255);
    ellipse(252.0, 144.0, 72.0, 72.0);

    fill(204);
    triangle(288.0, 18.0, 351.0, 360.0, 288.0, 360.0);

    fill(255);
    arc(479.0, 300.0, 280.0, 280.0, PI, TWO_PI);
}
