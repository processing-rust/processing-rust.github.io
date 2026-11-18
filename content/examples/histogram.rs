/**
 * Histogram.
 *
 * Calculates the histogram of an image.
 * A histogram is the frequency distribution
 * of the gray levels with the number of pure black values
 * displayed on the left and number of pure white values on the right.
 *
 * Note that this sketch will behave differently on Android,
 * since most images will no longer be full 24-bit color.
 */

fn setup() {
    size(640, 360);

    // Load an image from the data directory
    // Load a different image by modifying the comments
    let img: u64 = load_image("frontier.jpg");
    image(img, 0.0, 0.0);
    let mut hist = vec![0; 256];

    // Calculate the histogram
    for i in 0..img.width() as i32 {
        for j in 0..img.height() as i32 {
            let bright: i32 = (brightness(get(i, j))) as i32;
            hist[bright as usize] += 1;
        }
    }

    // Find the largest value in the histogram
    let histMax: i32 = (*hist.iter().max().unwrap()) as i32;

    stroke(255);
    // Draw half of the histogram (skip every second value)
    for i in (0..img.width() as i32).step_by(2) {
        // Map i (from 0..img.width) to a location in the histogram (0..255)
        let which: i32 = (map(i as f32, 0.0, img.width(), 0.0, 255.0)) as i32;
        // Convert the histogram value to a location between
        // the bottom and the top of the picture
        let y: i32 = (map((hist[which as usize]) as f32, 0.0, histMax as f32, img.height(), 0.0)) as i32;
        line(i as f32, img.height(), i as f32, y as f32);
    }
}
