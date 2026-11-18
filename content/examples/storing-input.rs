/**
 * Storing Input.
 *
 * Move the mouse across the screen to change the position
 * of the circles. The positions of the mouse are recorded
 * into an array and played back every frame. Between each
 * frame, the newest value are added to the end of each array
 * and the oldest value is deleted.
 */
let num: i32 = 60;
let mut mx: Vec<f32> = vec![0.0; num as usize];
let mut my: Vec<f32> = vec![0.0; num as usize];

fn setup() {
    size(640, 360);
    no_stroke();
    fill(255, 153);
}

fn draw() {
    background(51);

    // Cycle through the array, using a different entry on each frame.
    // Using modulo (%) like this is faster than moving all the values over.
    let which: i32 = frame_count() as i32 % num;
    mx[which as usize] = mouse_x();
    my[which as usize] = mouse_y();

    for i in 0..num {
        // which+1 is the smallest (the oldest in the array)
        let index: i32 = (which + 1 + i) % num;
        ellipse(mx[index as usize], my[index as usize], i as f32, i as f32);
    }
}
