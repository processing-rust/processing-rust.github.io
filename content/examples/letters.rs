/**
 * Letters.
 *
 * Draws letters to the screen. This requires loading a font,
 * setting the font, and then drawing the letters.
 */
let mut f: u64 = 0;

fn setup() {
    size(640, 360);
    background(0);

    // Create the font
    print_array(&PFont::list());
    f = create_font("SourceCodePro-Regular.ttf", 24.0);
    text_font(f);
    text_align(CENTER, CENTER);
}

fn draw() {
    background(0);

    // Set the left and top margin
    let margin: i32 = 10;
    translate((margin * 4) as f32, (margin * 4) as f32);

    let gap: i32 = 46;
    let mut counter: i32 = 35;

    {
        let mut y: i32 = 0;
        while (y as f32) < height() - gap as f32 {
            {
                let mut x: i32 = 0;
                while (x as f32) < width() - gap as f32 {
                    let letter: char = char::from(counter as u8);

                    if letter == 'A' || letter == 'E' || letter == 'I' || letter == 'O' || letter == 'U' {
                        fill(255, 204, 0);
                    } else {
                        fill(255);
                    }

                    // Draw the letter to the screen
                    text(&letter.to_string(), x as f32, y as f32);

                    // Increment the counter
                    counter += 1;
                    x += gap;
                }
            }
            y += gap;
        }
    }
}
