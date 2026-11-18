/**
 * Milliseconds.
 *
 * A millisecond is 1/1000 of a second.
 * Processing keeps track of the number of milliseconds a program has run.
 * By modifying this number with the modulo(%) operator,
 * different patterns in time are created.
 */
let mut scale: f32 = 0.0;

fn setup() {
    size(640, 360);
    no_stroke();
    scale = width() / 20.0;
}

fn draw() {
    for i in 0..scale as i32 {
        color_mode(RGB, ((i + 1)) as f32 * scale * 10.0);
        fill(millis() as f32 % (((i + 1)) as f32 * scale * 10.0));
        rect(i as f32 * scale, 0.0, scale, height());
    }
}
