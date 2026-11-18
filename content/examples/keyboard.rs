/**
 * Keyboard.
 *
 * Click on the image to give it focus and press the letter keys
 * to create forms in time and space. Each key has a unique identifying
 * number. These numbers can be used to position shapes in space.
 */
let mut rectWidth: i32 = 0;

fn setup() {
    size(640, 360);
    no_stroke();
    background(0);
    rectWidth = (width() / 4.0) as i32;
}

fn draw() {
    // keep draw() here to continue looping while waiting for keys
}

fn key_pressed() {
    let mut keyIndex: i32 = -1;
    if key() >= 'A' && key() <= 'Z' {
        keyIndex = ((key() as i32) - ('A' as i32) as i32);
    } else if key() >= 'a' && key() <= 'z' {
        keyIndex = ((key() as i32) - ('a' as i32) as i32);
    }
    if keyIndex == -1 {
        // If it's not a letter key, clear the screen
        background(0);
    } else {
        // It's a letter key, fill a rectangle
        fill(millis() % 255);
        let x: f32 = map(keyIndex as f32, 0.0, 25.0, 0.0, width() - rectWidth as f32);
        rect(x, 0.0, rectWidth as f32, height());
    }
}
