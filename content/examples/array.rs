/**
 * Array.
 *
 * An array is a list of data. Each piece of data in an array
 * is identified by an index number representing its position in
 * the array. Arrays are zero based, which means that the first
 * element in the array is [0], the second element is [1], and so on.
 * In this example, an array named "coswave" is created and
 * filled with the cosine values. This data is displayed three
 * separate ways on the screen.
 */
let mut coswave: Vec<f32> = Vec::new();

fn setup() {
    size(640, 360);
    coswave = vec![0.0; width() as usize];
    for i in 0..width() as i32 {
        let amount: f32 = map(i as f32, 0.0, width(), 0.0, PI);
        coswave[i as usize] = abs(amount.cos());
    }
    background(255);
    no_loop();
}

fn draw() {
    let mut y1: i32 = 0;
    let mut y2: i32 = (height() / 3.0) as i32;
    for i in 0..width() as i32 {
        stroke(coswave[i as usize] * 255.0);
        line(i as f32, y1 as f32, i as f32, y2 as f32);
    }

    y1 = y2;
    y2 = y1 + y1;
    for i in 0..width() as i32 {
        stroke(coswave[i as usize] * 255.0 / 4.0);
        line(i as f32, y1 as f32, i as f32, y2 as f32);
    }

    y1 = y2;
    y2 = height() as i32;
    for i in 0..width() as i32 {
        stroke(255.0 - coswave[i as usize] * 255.0);
        line(i as f32, y1 as f32, i as f32, y2 as f32);
    }
}
