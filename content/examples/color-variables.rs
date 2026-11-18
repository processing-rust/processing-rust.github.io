/**
 * Color Variables (Homage to Albers).
 *
 * This example creates variables for colors that may be referred to
 * in the program by a name, rather than a number.
 */

fn setup() {
    size(640, 360);
    no_stroke();
    background(51, 0, 0);

    let inside: Color = color(204, 102, 0);
    let middle: Color = color(204, 153, 0);
    let outside: Color = color(153, 51, 0);

    // These statements are equivalent to the statements above.
    // Programmers may use the format they prefer.
    //color inside = #CC6600;
    //color middle = #CC9900;
    //color outside = #993300;

    push_matrix();
    translate(80.0, 80.0);
    fill(outside);
    rect(0.0, 0.0, 200.0, 200.0);
    fill(middle);
    rect(40.0, 60.0, 120.0, 120.0);
    fill(inside);
    rect(60.0, 90.0, 80.0, 80.0);
    pop_matrix();

    push_matrix();
    translate(360.0, 80.0);
    fill(inside);
    rect(0.0, 0.0, 200.0, 200.0);
    fill(outside);
    rect(40.0, 60.0, 120.0, 120.0);
    fill(middle);
    rect(60.0, 90.0, 80.0, 80.0);
    pop_matrix();
}
