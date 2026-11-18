/**
 * Integers Floats.
 *
 * Integers and floats are two different kinds of numerical data.
 * An integer (more commonly called an int) is a number without
 * a decimal point. A float is a floating-point number, which means
 * it is a number that has a decimal place. Floats are used when
 * more precision is needed.
 */
let mut a: i32 = 0;  // Create a variable "a" of the datatype "int"
let mut b: f32 = 0.0;  // Create a variable "b" of the datatype "float"

fn setup() {
    size(640, 360);
    stroke(255);
}

fn draw() {
    background(0);

    a = a + 1;
    b = b + 0.2;
    line(a as f32, 0.0, a as f32, height() / 2.0);
    line(b, height() / 2.0, b, height());

    if (a as f32) > width() {
        a = 0;
    }
    if b > width() {
        b = 0.0;
    }
}
