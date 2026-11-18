/**
 * Operator Precedence
 *
 * If you don't direction state the order in which an
 * expression is evaluated, it is decided by the operator
 * precedence. For example, in the expression 4+2*8, the
 * 2 will first be multiplied by 8 and then the result will
 * be added to 4. This is because multiplication has a higher
 * precedence than addition. To avoid ambiguity in reading
 * the program, it is recommended to write the expression as
 * 4+(2*8). The order of evaluation can be controlled through
 * adding parenthesis in the code.
 */

// The highest precedence is at the top of the list and
// the lowest is at the bottom.
// Multiplicative: * / %
// Additive: + -
// Relational: < > <= >=
// Equality: == !=
// Logical AND: &&
// Logical OR: ||
// Assignment: = += -= *= /= %=

fn setup() {
    size(640, 360);
    background(51);
    no_fill();
    stroke(51);

    stroke(204);
    for i in (0..(width() - 20.0) as i32).step_by(4) {
        // The 30 is added to 70 and then evaluated
        // if it is greater than the current value of "i"
        // For clarity, write as "if (i > (30 + 70)) {"
        if i > 30 + 70 {
            line(i as f32, 0.0, i as f32, 50.0);
        }
    }

    stroke(255);
    // The 2 is multiplied by the 8 and the result is added to the 4
    // For clarity, write as "rect(5 + (2 * 8), 0, 90, 20);"
    rect(4.0 + 2.0 * 8.0, 52.0, 290.0, 48.0);
    rect((4.0 + 2.0) * 8.0, 100.0, 290.0, 49.0);

    stroke(153);
    for i in (0..width() as i32).step_by(2) {
        // The relational statements are evaluated
        // first, and then the logical AND statements and
        // finally the logical OR. For clarity, write as:
        // "if(((i > 20) && (i < 50)) || ((i > 100) && (i < width-20))) {"
        if i > 20 && i < 50 || i > 100 && (i as f32) < width() - 20.0 {
            line(i as f32, 151.0, i as f32, height() - 1.0);
        }
    }
}
