/**
 * Convolution
 * by Daniel Shiffman.
 *
 * Applies a convolution matrix to a portion of an image. Move mouse to
 * apply filter to different parts of the image. Click mouse to cycle
 * through different effects (kernels).
 */

let mut pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let mut img_pixels: Vec<u32> = Vec::new();  // filled by load_pixels()
let mut img: u64 = 0;
let mut effect: i32 = 0;
let w: i32 = 120;

// It's possible to convolve the image with many different
// matrices to produce different effects.  Here are some
// example kernels to try.
let mut identity: Vec<Vec<f32>> = vec![vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0]];

let mut darken: Vec<Vec<f32>> = vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.5, 0.0], vec![0.0, 0.0, 0.0]];

let mut lighten: Vec<Vec<f32>> = vec![vec![0.0, 0.0, 0.0], vec![0.0, 2.0, 0.0], vec![0.0, 0.0, 0.0]];

let mut sharpen: Vec<Vec<f32>> = vec![vec![0.0, -1.0, 0.0], vec![-1.0, 5.0, -1.0], vec![0.0, -1.0, 0.0]];

let mut sharpen2: Vec<Vec<f32>> = vec![vec![-1.0, -1.0, -1.0], vec![-1.0, 9.0, -1.0], vec![-1.0, -1.0, -1.0]];

let mut box_blur: Vec<Vec<f32>> = vec![vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0], vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0], vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0]];

let mut edge_det: Vec<Vec<f32>> = vec![vec![0.0, 1.0, 0.0], vec![1.0, -4.0, 1.0], vec![0.0, 1.0, 0.0]];

let mut emboss: Vec<Vec<f32>> = vec![vec![-2.0, -1.0, 0.0], vec![-1.0, 1.0, 1.0], vec![0.0, 1.0, 2.0]];

// collect the kernels and names into arrays for our program
let mut kernels: Vec<Vec<Vec<f32>>> = vec![identity.clone(), darken.clone(), lighten.clone(), sharpen.clone(), sharpen2.clone(), box_blur.clone(), edge_det.clone(), emboss.clone()];

let mut effect_names: Vec<String> = vec!["Identity (no change)".to_string(), "Darken".to_string(), "Lighten".to_string(), "Sharpen".to_string(), "Sharpen More".to_string(), "Box Blur".to_string(), "Edge Detect".to_string(), "Emboss".to_string()];

//
fn setup() {
    size(640, 360);
    img = load_image("moon-wide.jpg");

    no_loop();
}

// Clicking the mouse advances to the next effect
fn mouse_pressed() {
    effect += 1;
    if effect >= effect_names.len() as i32 {
        effect = 0;
    }

    redraw();
}

// Moving the mouse triggers a screen redraw
fn mouse_moved() {
    redraw();
}
fn mouse_dragged() {
    redraw();
}

fn draw() {
    // We're only going to process a portion of the image
    // so let's set the whole image as the background first
    image(img, 0.0, 0.0);

    // Calculate the small rectangle we will process
    let xstart: i32 = (constrain(mouse_x() - (w / 2) as f32, 0.0, img.width())) as i32;
    let ystart: i32 = (constrain(mouse_y() - (w / 2) as f32, 0.0, img.height())) as i32;
    let xend: i32 = (constrain(mouse_x() + (w / 2) as f32, 0.0, img.width())) as i32;
    let yend: i32 = (constrain(mouse_y() + (w / 2) as f32, 0.0, img.height())) as i32;
    let matrixsize: i32 = 3;
    pixels = load_pixels();
    // Begin our loop for every pixel in the smaller image
    for x in xstart..xend {
        for y in ystart..yend {
            let c: Color = convolution(x, y, &kernels[effect as usize], matrixsize, img);
            let loc: i32 = (x as f32 + y as f32 * img.width()) as i32;
            pixels[loc as usize] = c.to_argb();
        }
    }
    update_pixels(&pixels);

    text_size(24.0);
    text(&effect_names[effect as usize], 4.0, 24.0);
}

fn convolution(x: i32, y: i32, matrix: &[Vec<f32>], matrixsize: i32, img: u64) -> Color {
    let img_pixels = img.load_pixels();
    let mut rtotal: f32 = 0.0;
    let mut gtotal: f32 = 0.0;
    let mut btotal: f32 = 0.0;
    let offset: i32 = matrixsize / 2;
    for i in 0..matrixsize {
        for j in 0..matrixsize {
            // What pixel are we testing
            let xloc: i32 = x + i - offset;
            let yloc: i32 = y + j - offset;
            let mut loc: i32 = (xloc as f32 + img.width() * yloc as f32) as i32;
            // Make sure we haven't walked off our image, we could do better here
            loc = (constrain(loc as f32, 0.0, (img_pixels.len() - 1) as f32)) as i32;
            // Calculate the convolution
            rtotal += (red(Color::from_u32(img_pixels[loc as usize])) * matrix[i as usize][j as usize]);
            gtotal += (green(Color::from_u32(img_pixels[loc as usize])) * matrix[i as usize][j as usize]);
            btotal += (blue(Color::from_u32(img_pixels[loc as usize])) * matrix[i as usize][j as usize]);
        }
    }
    // Make sure RGB is within range
    rtotal = constrain(rtotal, 0.0, 255.0);
    gtotal = constrain(gtotal, 0.0, 255.0);
    btotal = constrain(btotal, 0.0, 255.0);
    // Return the resulting color
    return color(rtotal, gtotal, btotal);
}
