/**
 * Logical Operators.
 *
 * The logical operators for AND (&&) and OR (||) are used to
 * combine simple relational statements into more complex expressions.
 * The NOT (!) operator is used to negate a boolean statement.
 */

fn setup() {
    size(640, 360);
    background(126);

    let mut test: bool = false;

    for i in (5..=height() as i32).step_by(5) {
        // Logical AND
        stroke(0);
        if (i > 35) && (i < 100) {
            line(width() / 4.0, i as f32, width() / 2.0, i as f32);
            test = false;
        }

        // Logical OR
        stroke(76);
        if (i <= 35) || (i >= 100) {
            line(width() / 2.0, i as f32, width(), i as f32);
            test = true;
        }

        // Testing if a boolean value is "true"
        // The expression "if(test)" is equivalent to "if(test == true)"
        if test {
            stroke(0);
            point(width() / 3.0, i as f32);
        }

        // Testing if a boolean value is "false"
        // The expression "if(!test)" is equivalent to "if(test == false)"
        if !test {
            stroke(255);
            point(width() / 4.0, i as f32);
        }
    }
}
