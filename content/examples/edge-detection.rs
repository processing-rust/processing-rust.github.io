/**
 * Edge Detection.
 *
 * This program analyzes every pixel in an image and compares it with thee
 * neighboring pixels to identify edges.
 *
 * This is an example of an "image convolution" using a kernel (small matrix)
 * to analyze and transform a pixel based on the values of its neighbors.
 *
 * This kernel describes a "Laplacian Edge Detector".  It is effective,
 * but sensitive to noise.  One common enhancement is to add a Gaussian
 * blur to the source image first, as in
 *   grayImg.filter(BLUR);
 * to reduce impact of noise on the output.  The combination is often called
 * "Laplace of Gaussian", or "LoG" for short.
 *
 * For weaker detection effect, try this kernel:     [  0  -1   0 ]
 *                                                   [ -1   4  -1 ]
 *                                                   [  0  -1   0 ]
 */
let mut kernel: Vec<Vec<f32>> = vec![vec![-1.0, -1.0, -1.0], vec![-1.0, 8.0, -1.0], vec![-1.0, -1.0, -1.0]];

let mut img: u64 = 0;

fn setup() {
    size(640, 360);
    img = load_image("moon.jpg");  // Load the original image
    no_loop();
}

fn draw() {
    image(img, 0.0, 0.0);  // Displays the image from point (0,0)
    let mut img_pixels = img.load_pixels();

    // Edge detection should be done on a grayscale image.
    //  Create a copy of the source image, and convert to gray.
    let mut grayImg: u64 = img.copy();
    let mut grayImg_pixels = grayImg.load_pixels();
    grayImg.filter(GRAY, 0.0);
    // grayImg.filter(BLUR);

    // Create an opaque image of the same size as the original
    let mut edgeImg: u64 = create_image(grayImg.width() as u32, grayImg.height() as u32, RGB);
    let mut edgeImg_pixels = edgeImg.load_pixels();

    // Loop through every pixel in the image
    for y in 1..(grayImg.height() - 1.0) as i32 {
        // Skip top and bottom edges
        for x in 1..(grayImg.width() - 1.0) as i32 {
            // Skip left and right edges
            // Output of this filter is shown as offset from 50% gray.
            // This preserves transitions from low (dark) to high (light) value.
            // Starting from zero will show only high edges on black instead.
            let mut sum: f32 = 128.0;
            for ky in -1..=1 {
                for kx in -1..=1 {
                    // Calculate the adjacent pixel for this kernel point
                    let pos: i32 = (((y + ky)) as f32 * grayImg.width() + ((x + kx)) as f32) as i32;

                    // Image is grayscale, red/green/blue are identical
                    let val: f32 = blue(Color::from_u32(grayImg_pixels[pos as usize]));
                    // Multiply adjacent pixels based on the kernel values
                    sum += kernel[(ky + 1) as usize][(kx + 1) as usize] * val;
                }
            }
            // For this pixel in the new image, set the output value
            // based on the sum from the kernel
            edgeImg_pixels[(y as f32 * edgeImg.width() + x as f32) as usize] = color(sum).to_argb();
        }
    }
    // State that there are changes to edgeImg.pixels[]
    edgeImg.update_pixels(&edgeImg_pixels);

    image(edgeImg, width() / 2.0, 0.0);  // Draw the new image
}
