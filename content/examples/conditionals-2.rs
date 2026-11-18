/**
 * Conditionals 2.
 *
 * We extend the language of conditionals from the previous
 * example by adding the keyword "else". This allows conditionals
 * to ask two or more sequential questions, each with a different
 * action.
 */

fn setup() {
    size(640, 360);
    background(0);

    for i in (2..(width() - 2.0) as i32).step_by(2) {
        // If 'i' divides by 20 with no remainder
        if (i % 20) == 0 {
            stroke(255);
            line(i as f32, 80.0, i as f32, height() / 2.0);
            // If 'i' divides by 10 with no remainder
        } else if (i % 10) == 0 {
            stroke(153);
            line(i as f32, 20.0, i as f32, 180.0);
            // If neither of the above two conditions are met
            // then draw this line
        } else {
            stroke(102);
            line(i as f32, height() / 2.0, i as f32, height() - 20.0);
        }
    }
}
