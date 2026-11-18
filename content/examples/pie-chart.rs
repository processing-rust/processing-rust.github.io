/**
 * Pie Chart
 *
 * Uses the arc() function to generate a pie chart from the data
 * stored in an array.
 */
let mut angles: Vec<i32> = vec![30, 10, 45, 35, 60, 38, 75, 67];

fn setup() {
    size(640, 360);
    no_stroke();
    no_loop();  // Run once and stop
}

fn draw() {
    background(100);
    pie_chart(300.0, &angles);
}

fn pie_chart(diameter: f32, data: &[i32]) {
    let mut lastAngle: f32 = 0.0;
    for i in 0..data.len() {
        let gray: f32 = map(i as f32, 0.0, data.len() as f32, 0.0, 255.0);
        fill(gray);
        arc(width() / 2.0, height() / 2.0, diameter, diameter, lastAngle, lastAngle + radians(data[i] as f32));
        lastAngle += radians(data[i] as f32);
    }
}
