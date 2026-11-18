/**
 * True/False.
 *
 * A Boolean variable has only two possible values: true or false.
 * It is common to use Booleans with control statements to
 * determine the flow of a program. In this example, when the
 * boolean value "x" is true, vertical black lines are drawn and when
 * the boolean value "x" is false, horizontal gray lines are drawn.
 */

fn setup() {
    let mut b: bool = false;

    size(640, 360);
    background(0);
    stroke(255);

    let d: i32 = 20;
    let middle: i32 = (width() / 2.0) as i32;

    {
        let mut i: i32 = d;
        while (i as f32) <= width() {
            if i < middle {
                b = true;
            } else {
                b = false;
            }

            if b == true {
                // Vertical line
                line(i as f32, d as f32, i as f32, height() - d as f32);
            }

            if b == false {
                // Horizontal line
                line(middle as f32, (i - middle + d) as f32, width() - d as f32, (i - middle + d) as f32);
            }
            i += d;
        }
    }
}
