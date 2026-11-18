/**
 * Conditionals 1.
 *
 * Conditions are like questions.
 * They allow a program to decide to take one action if
 * the answer to a question is "true" or to do another action
 * if the answer to the question is "false."
 * The questions asked within a program are always logical
 * or relational statements. For example, if the variable 'i' is
 * equal to zero then draw a line.
 */

fn setup() {
    size(640, 360);
    background(0);

    for i in (10..width() as i32).step_by(10) {
        // If 'i' divides by 20 with no remainder draw
        // the first line, else draw the second line
        if (i % 20) == 0 {
            stroke(255);
            line(i as f32, 80.0, i as f32, height() / 2.0);
        } else {
            stroke(153);
            line(i as f32, 20.0, i as f32, 180.0);
        }
    }
}
