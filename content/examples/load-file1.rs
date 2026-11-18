/**
 * LoadFile 1
 *
 * Loads a text file that contains two numbers separated by a tab ('\t').
 * A new pair of numbers is loaded each frame and used to draw a point on the screen.
 */
let mut lines: Vec<String> = Vec::new();
let mut index: i32 = 0;

fn setup() {
    size(640, 360);
    background(0);
    stroke(255);
    frame_rate(12.0);
    lines = load_strings("positions.txt");
}

fn draw() {
    if index < lines.len() as i32 {
        let pieces = split(&lines[index as usize], "\t");
        if pieces.len() == 2 {
            // Scale the coordinates to match the size of the sketch window
            let x: f32 = map(pieces[0].trim().parse::<f32>().unwrap_or(0.0), 0.0, 100.0, 0.0, width());
            let y: f32 = map(pieces[1].trim().parse::<f32>().unwrap_or(0.0), 0.0, 100.0, 0.0, height());
            point(x, y);
        }
        // Go to the next line for the next run through draw()
        index = index + 1;
    }
}
