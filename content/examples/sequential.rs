/**
 * Sequential
 * by James Paterson.
 *
 * Displaying a sequence of images creates the illusion of motion.
 * Twelve images are loaded and each is displayed individually in a loop.
 */
let numFrames: i32 = 12;  // The number of frames in the animation
let mut currentFrame: i32 = 0;
let mut images: Vec<u64> = vec![0; numFrames as usize];

fn setup() {
    size(640, 360);
    frame_rate(24.0);

    images[0] = load_image("PT_anim0000.gif");
    images[1] = load_image("PT_anim0001.gif");
    images[2] = load_image("PT_anim0002.gif");
    images[3] = load_image("PT_anim0003.gif");
    images[4] = load_image("PT_anim0004.gif");
    images[5] = load_image("PT_anim0005.gif");
    images[6] = load_image("PT_anim0006.gif");
    images[7] = load_image("PT_anim0007.gif");
    images[8] = load_image("PT_anim0008.gif");
    images[9] = load_image("PT_anim0009.gif");
    images[10] = load_image("PT_anim0010.gif");
    images[11] = load_image("PT_anim0011.gif");

    // If you don't want to load each image separately
    // and you know how many frames you have, you
    // can create the filenames as the program runs.
    // The nf() command does number formatting, which will
    // ensure that the number is (in this case) 4 digits.
    //for (int i = 0; i < numFrames; i++) {
    //  String imageName = "PT_anim" + nf(i, 4) + ".gif";
    //  images[i] = loadImage(imageName);
    //}
}

fn draw() {
    background(0);
    currentFrame = (currentFrame + 1) % numFrames;  // Use % to cycle through frames
    let mut offset: i32 = 0;
    {
        let mut x: i32 = -100;
        while (x as f32) < width() {
            image(images[((currentFrame + offset) % numFrames) as usize], x as f32, -20.0);
            offset += 2;
            image(images[((currentFrame + offset) % numFrames) as usize], x as f32, height() / 2.0);
            offset += 2;
            x += images[0].width() as i32;
        }
    }
}
