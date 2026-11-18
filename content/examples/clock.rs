/**
 * Clock.
 *
 * The current time can be read with the second(), minute(),
 * and hour() functions. In this example, sin() and cos() values
 * are used to set the position of the hands.
 */
let mut cx: i32 = 0;
let mut cy: i32 = 0;
let mut secondsRadius: f32 = 0.0;
let mut minutesRadius: f32 = 0.0;
let mut hoursRadius: f32 = 0.0;
let mut clockDiameter: f32 = 0.0;

fn setup() {
    size(640, 360);
    stroke(255);

    let radius: i32 = (min(width(), height()) / 2.0) as i32;
    secondsRadius = radius as f32 * 0.72;
    minutesRadius = radius as f32 * 0.60;
    hoursRadius = radius as f32 * 0.50;
    clockDiameter = radius as f32 * 1.8;

    cx = (width() / 2.0) as i32;
    cy = (height() / 2.0) as i32;
}

fn draw() {
    background(0);

    // Draw the clock background
    fill(80);
    no_stroke();
    ellipse(cx as f32, cy as f32, clockDiameter, clockDiameter);

    // Angles for sin() and cos() start at 3 o'clock;
    // subtract HALF_PI to make them start at the top
    let s: f32 = map(second() as f32, 0.0, 60.0, 0.0, TWO_PI) - HALF_PI;
    let m: f32 = map(minute() as f32 + norm(second() as f32, 0.0, 60.0), 0.0, 60.0, 0.0, TWO_PI) - HALF_PI;
    let h: f32 = map(hour() as f32 + norm(minute() as f32, 0.0, 60.0), 0.0, 24.0, 0.0, TWO_PI * 2.0) - HALF_PI;

    // Draw the hands of the clock
    stroke(255);
    stroke_weight(1.0);
    line(cx as f32, cy as f32, cx as f32 + s.cos() * secondsRadius, cy as f32 + s.sin() * secondsRadius);
    stroke_weight(2.0);
    line(cx as f32, cy as f32, cx as f32 + m.cos() * minutesRadius, cy as f32 + m.sin() * minutesRadius);
    stroke_weight(4.0);
    line(cx as f32, cy as f32, cx as f32 + h.cos() * hoursRadius, cy as f32 + h.sin() * hoursRadius);

    // Draw the minute ticks
    stroke_weight(2.0);
    begin_shape(POINTS);
    for a in (0..360).step_by(6) {
        let angle: f32 = radians(a as f32);
        let x: f32 = cx as f32 + angle.cos() * secondsRadius;
        let y: f32 = cy as f32 + angle.sin() * secondsRadius;
        vertex(x, y);
    }
    end_shape();
}
