/**
 * Reach 3
 * based on code from Keith Peters.
 *
 * The arm follows the position of the ball by
 * calculating the angles with atan2().
 */
let numSegments: i32 = 8;
let mut x: Vec<f32> = vec![0.0; numSegments as usize];
let mut y: Vec<f32> = vec![0.0; numSegments as usize];
let mut angle: Vec<f32> = vec![0.0; numSegments as usize];
const SEGLENGTH: f32 = 26.0;
let mut targetX: f32 = 0.0;
let mut targetY: f32 = 0.0;

let mut ballX: f32 = 50.0;
let mut ballY: f32 = 50.0;
let mut ballXDirection: i32 = 1;
let mut ballYDirection: i32 = -1;

fn setup() {
    size(640, 360);
    stroke_weight(20.0);
    stroke(255, 100);
    no_fill();
    let last = x.len() - 1;
    x[last] = width() / 2.0;  // Set base x-coordinate
    y[x.len() - 1] = height();  // Set base y-coordinate
}

fn draw() {
    background(0);

    stroke_weight(20.0);
    ballX = ballX + 1.0 * ballXDirection as f32;
    ballY = ballY + 0.8 * ballYDirection as f32;
    if ballX > width() - 25.0 || ballX < 25.0 {
        ballXDirection *= -1;
    }
    if ballY > height() - 25.0 || ballY < 25.0 {
        ballYDirection *= -1;
    }
    ellipse(ballX, ballY, 30.0, 30.0);

    reach_segment(0, ballX, ballY, &mut angle, &mut targetX, &mut targetY, &x, &y);
    for i in 1..numSegments {
        reach_segment(i, targetX, targetY, &mut angle, &mut targetX, &mut targetY, &x, &y);
    }
    for i in (1..x.len()).rev() {
        position_segment(i as i32, (i - 1) as i32, &angle, &mut x, &mut y);
    }
    for i in 0..x.len() {
        segment(x[i], y[i], angle[i], ((i + 1) * 2) as f32);
    }
}

fn position_segment(a: i32, b: i32, angle: &Vec<f32>, x: &mut Vec<f32>, y: &mut Vec<f32>) {
    x[b as usize] = x[a as usize] + (angle[a as usize]).cos() * SEGLENGTH;
    y[b as usize] = y[a as usize] + (angle[a as usize]).sin() * SEGLENGTH;
}

fn reach_segment(i: i32, xin: f32, yin: f32, angle: &mut Vec<f32>, targetX: &mut f32, targetY: &mut f32, x: &Vec<f32>, y: &Vec<f32>) {
    let dx: f32 = xin - x[i as usize];
    let dy: f32 = yin - y[i as usize];
    angle[i as usize] = dy.atan2(dx);
    *targetX = xin - (angle[i as usize]).cos() * SEGLENGTH;
    *targetY = yin - (angle[i as usize]).sin() * SEGLENGTH;
}

fn segment(x: f32, y: f32, a: f32, sw: f32) {
    stroke_weight(sw);
    push_matrix();
    translate(x, y);
    rotate(a);
    line(0.0, 0.0, SEGLENGTH, 0.0);
    pop_matrix();
}
