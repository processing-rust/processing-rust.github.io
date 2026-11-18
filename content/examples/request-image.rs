/**
 * Request Image
 * by Ira Greenberg
 *
 * Shows how to use the requestImage() function with preloader animation.
 * The requestImage() function loads images on a separate thread so that
 * the sketch does not freeze while they load. It's useful when you are
 * loading large images. These images are small for a quick download, but
 * try it with your own huge images to get the full effect.
 */
let imgCount: i32 = 12;
let mut imgs: Vec<u64> = vec![0; imgCount as usize];
let mut imgW: f32 = 0.0;

// Keeps track of loaded images (true or false)
let mut loadStates: Vec<bool> = vec![false; imgCount as usize];

// For loading animation
let mut loaderX: f32 = 0.0;
let mut loaderY: f32 = 0.0;
let mut theta: f32 = 0.0;

fn setup() {
    size(640, 360);
    imgW = width() / imgCount as f32;

    // Load images asynchronously
    for i in 0..imgCount {
        imgs[i as usize] = request_image(&format!("PT_anim{}.gif", nf(i, 4)));
    }
}

fn draw() {
    background(0);

    // Start loading animation
    run_loader_ani(&imgs, &loadStates, &mut loaderX, &mut loaderY, &mut theta);

    for i in 0..imgs.len() {
        // Check if individual images are fully loaded
        if (imgs[i].width() != 0.0) && (imgs[i].width() != -1.0) {
            // As images are loaded set true in boolean array
            loadStates[i] = true;
        }
    }
    // When all images are loaded draw them to the screen
    if check_load_states(&imgs, &loadStates) {
        draw_images(&imgs);
    }
}

fn draw_images(imgs: &Vec<u64>) {
    let y: i32 = ((height() - imgs[0].height()) / 2.0) as i32;
    for i in 0..imgs.len() {
        image(imgs[i], width() / imgs.len() as f32 * i as f32, y as f32, imgs[i].height(), imgs[i].height());
    }
}

// Loading animation
fn run_loader_ani(imgs: &Vec<u64>, loadStates: &Vec<bool>, loaderX: &mut f32, loaderY: &mut f32, theta: &mut f32) {
    // Only run when images are loading
    if !check_load_states(imgs, loadStates) {
        ellipse(*loaderX, *loaderY, 10.0, 10.0);
        *loaderX += 2.0;
        *loaderY = height() / 2.0 + (*theta).sin() * (height() / 8.0);
        *theta += PI / 22.0;
        // Reposition ellipse if it goes off the screen
        if *loaderX > width() + 5.0 {
            *loaderX = -5.0;
        }
    }
}

// Return true when all images are loaded - no false values left in array
fn check_load_states(imgs: &Vec<u64>, loadStates: &Vec<bool>) -> bool {
    for i in 0..imgs.len() {
        if loadStates[i] == false {
            return false;
        }
    }
    return true;
}
