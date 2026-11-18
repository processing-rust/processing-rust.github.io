/**
 * Keyboard Functions
 * by Martin Gomez
 *
 * Click on the window to give it focus and press the letter keys to type colors.
 * The keyboard function keyPressed() is called whenever
 * a key is pressed. keyReleased() is another keyboard
 * function that is called when a key is released.
 *
 * Original 'Color Typewriter' concept by John Maeda.
 */
let maxHeight: i32 = 40;
let minHeight: i32 = 20;
let mut letterHeight: i32 = maxHeight;  // Height of the letters
let letterWidth: i32 = 20;  // Width of the letter

let mut x: i32 = -letterWidth;  // X position of the letters
let mut y: i32 = 0;  // Y position of the letters

let mut newletter: bool = false;

let numChars: i32 = 26;  // There are 26 characters in the alphabet
let mut colors: Vec<Color> = vec![color(0); numChars as usize];

fn setup() {
    size(640, 360);
    no_stroke();
    color_mode(HSB, numChars as f32);
    background(numChars / 2);
    // Set a hue value for each key
    for i in 0..numChars {
        colors[i as usize] = color(i, numChars, numChars);
    }
}

fn draw() {
    if newletter == true {
        // Draw the "letter"
        let mut y_pos: i32;
        if letterHeight == maxHeight {
            y_pos = y;
            rect(x as f32, y_pos as f32, letterWidth as f32, letterHeight as f32);
        } else {
            y_pos = y + minHeight;
            rect(x as f32, y_pos as f32, letterWidth as f32, letterHeight as f32);
            fill(numChars / 2);
            rect(x as f32, (y_pos - minHeight) as f32, letterWidth as f32, letterHeight as f32);
        }
        newletter = false;
    }
}

fn key_pressed() {
    // If the key is between 'A'(65) to 'Z' and 'a' to 'z'(122)
    if (key() >= 'A' && key() <= 'Z') || (key() >= 'a' && key() <= 'z') {
        let mut keyIndex: i32;
        if key() <= 'Z' {
            keyIndex = ((key() as i32) - ('A' as i32) as i32);
            letterHeight = maxHeight;
            fill(colors[keyIndex as usize]);
        } else {
            keyIndex = ((key() as i32) - ('a' as i32) as i32);
            letterHeight = minHeight;
            fill(colors[keyIndex as usize]);
        }
    } else {
        fill(0);
        letterHeight = 10;
    }

    newletter = true;

    // Update the "letter" position
    x = (x + letterWidth);

    // Wrap horizontally
    if (x as f32) > width() - letterWidth as f32 {
        x = 0;
        y += maxHeight;
    }

    // Wrap vertically
    if (y as f32) > height() - letterHeight as f32 {
        y = 0;  // reset y to 0
    }
}
