/**
 * Blur.
 *
 * This program analyzes every pixel in an image and blends it with the
 * neighboring pixels to blur the image.
 *
 * This is an example of an "image convolution" using a kernel (small matrix)
 * to analyze and transform a pixel based on the values of its neighbors.
 *
 * Image blur is also called a "low-pass filter".  Pixels of low frequency
 * change (similar brightness as neighbors) are left mostly unchanged, while
 * those with high frequency change (sharply different values) are smoothed
 * out.
 *
 * The kernel here is a Box Blur, in which all components are equally valued.
 * Another common blur is "Gaussian Blur", in which pixels nearer the center
 * of the kernel have more weight than those further away.
 *
 * An example 3x3 Gaussian kernel might be:          [ 1  2  1 ]
 *                                            1/16 * [ 2  4  2 ]
 *                                                   [ 1  2  1 ]
 *
 * An example 5x5 kernel, which creates a greater blur effect:
 *                                                   [ 1   4   6   4  1 ]
 *                                                   [ 4  16  24  16  4 ]
 *                                           1/256 * [ 6  24  36  24  6 ]
 *                                                   [ 4  16  24  16  4 ]
 *                                                   [ 1   4   6   4  1 ]
 */

let mut img_pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let v: f32 = 1.0 / 9.0;
let mut kernel: Vec<Vec<f32>> = vec![vec![v, v, v], vec![v, v, v], vec![v, v, v]];

let mut img: u64 = 0;

fn setup() {
    size(640, 360);
    img = load_image("moon.jpg");  // Load the original image
    no_loop();
}

fn draw() {
    image(img, 0.0, 0.0);  // Displays the image from point (0,0)
    img_pixels = img.load_pixels();

    // Create an opaque image of the same size as the original
    let mut blurImg: u64 = create_image(img.width() as u32, img.height() as u32, RGB);
    let mut blurImg_pixels = blurImg.load_pixels();

    // Loop through every pixel in the image
    for y in 1..(img.height() - 1.0) as i32 {
        // Skip top and bottom edges
        for x in 1..(img.width() - 1.0) as i32 {
            // Skip left and right edges
            let mut sumRed: f32 = 0.0;  // Kernel sums for this pixel
            let mut sumGreen: f32 = 0.0;
            let mut sumBlue: f32 = 0.0;
            for ky in -1..=1 {
                for kx in -1..=1 {
                    // Calculate the adjacent pixel for this kernel point
                    let pos: i32 = (((y + ky)) as f32 * img.width() + ((x + kx)) as f32) as i32;

                    // Process each channel separately, Red first.
                    let valRed: f32 = red(Color::from_u32(img_pixels[pos as usize]));
                    // Multiply adjacent pixels based on the kernel values
                    sumRed += kernel[(ky + 1) as usize][(kx + 1) as usize] * valRed;

                    // Green
                    let valGreen: f32 = green(Color::from_u32(img_pixels[pos as usize]));
                    sumGreen += kernel[(ky + 1) as usize][(kx + 1) as usize] * valGreen;

                    // Blue
                    let valBlue: f32 = blue(Color::from_u32(img_pixels[pos as usize]));
                    sumBlue += kernel[(ky + 1) as usize][(kx + 1) as usize] * valBlue;
                }
            }
            // For this pixel in the new image, set the output value
            // based on the sum from the kernel
            blurImg_pixels[(y as f32 * blurImg.width() + x as f32) as usize] = (color(sumRed, sumGreen, sumBlue)).to_argb();
        }
    }
    // State that there are changes to blurImg.pixels[]
    blurImg.update_pixels(&blurImg_pixels);

    image(blurImg, width() / 2.0, 0.0);  // Draw the new image
}
