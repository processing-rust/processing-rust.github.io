/**
 * Text Rotation.
 *
 * Draws letters to the screen and rotates them at different angles.
 */
let mut f: u64 = 0;
let mut angleRotate: f32 = 0.0;

fn setup() {
    size(640, 360);
    background(0);

    // Create the font from the .ttf file in the data folder
    f = create_font("SourceCodePro-Regular.ttf", 18.0);
    text_font(f);
}

fn draw() {
    background(0);

    stroke_weight(1.0);
    stroke(153);

    push_matrix();
    let angle1: f32 = radians(45.0);
    translate(100.0, 180.0);
    rotate(angle1);
    text("45 DEGREES", 0.0, 0.0);
    line(0.0, 0.0, 150.0, 0.0);
    pop_matrix();

    push_matrix();
    let angle2: f32 = radians(270.0);
    translate(200.0, 180.0);
    rotate(angle2);
    text("270 DEGREES", 0.0, 0.0);
    line(0.0, 0.0, 150.0, 0.0);
    pop_matrix();

    push_matrix();
    translate(440.0, 180.0);
    rotate(radians(angleRotate));
    text(&format!("{} DEGREES", angleRotate as i32 % 360), 0.0, 0.0);
    line(0.0, 0.0, 150.0, 0.0);
    pop_matrix();

    angleRotate += 0.25;

    stroke(255, 0, 0);
    stroke_weight(4.0);
    point(100.0, 180.0);
    point(200.0, 180.0);
    point(440.0, 180.0);
}
