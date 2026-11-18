/**
 * Datatype Conversion.
 *
 * It is sometimes beneficial to convert a value from one type of
 * data to another. Each of the conversion functions converts its parameter
 * to an equivalent representation within its datatype.
 * The conversion functions include int(), float(), char(), byte(), and others.
 */

fn setup() {
    size(640, 360);
    background(0);
    no_stroke();

    text_font(create_font("SourceCodePro-Regular.ttf", 24.0));

    let mut c: char;  // Chars are used for storing alphanumeric symbols
    let mut f: f32;  // Floats are decimal numbers
    let mut i: i32;  // Integers are values between 2,147,483,647 and -2147483648
    let mut b: i8;  // Bytes are values between -128 and 127

    c = 'A';
    f = c as u32 as f32;  // Sets f = 65.0
    i = (f * 1.4) as i32;  // Sets i to 91
    b = ((c as i32) / 2) as i8;  // Sets b to 32

    //println(f);
    //println(i);
    //println(b);

    text(&format!("The value of variable c is {}", c), 50.0, 100.0);
    text(&format!("The value of variable f is {}", f), 50.0, 150.0);
    text(&format!("The value of variable i is {}", i), 50.0, 200.0);
    text(&format!("The value of variable b is {}", b), 50.0, 250.0);
}
