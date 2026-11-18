/**
 * Animated Sprite (Shifty + Teddy)
 * by James Paterson.
 *
 * Press the mouse button to change animations.
 * Demonstrates loading, displaying, and animating GIF images.
 * It would be easy to write a program to display
 * animated GIFs, but would not allow as much control over
 * the display sequence and rate of display.
 */
let mut animation1: Animation = Animation::default();
let mut animation2: Animation = Animation::default();

let mut xpos: f32 = 0.0;
let mut ypos: f32 = 0.0;
let drag: f32 = 30.0;

fn setup() {
    size(640, 360);
    background(255, 204, 0);
    frame_rate(24.0);
    animation1 = Animation::new("PT_Shifty_", 38);
    animation2 = Animation::new("PT_Teddy_", 60);
    ypos = height() * 0.25;
}

fn draw() {
    let dx: f32 = mouse_x() - xpos;
    xpos = xpos + dx / drag;

    // Display the sprite at the position xpos, ypos
    if mouse_pressed_global() {
        background(153, 153, 0);
        animation1.display(xpos - (animation1.get_width() / 2) as f32, ypos);
    } else {
        background(255, 204, 0);
        animation2.display(xpos - (animation1.get_width() / 2) as f32, ypos);
    }
}

// Class for animating a sequence of GIFs

#[derive(Clone, Default)]
struct Animation {
    images: Vec<u64>,
    imageCount: i32,
    frame: i32,
}

impl Animation {
    fn new(imagePrefix: &str, count: i32) -> Animation {
        let mut this = Animation {
            images: Vec::new(),
            imageCount: count,
            frame: 0,
        };
        this.images = vec![0; this.imageCount as usize];

        for i in 0..this.imageCount {
            // Use nf() to number format 'i' into four digits
            let filename: String = format!("{}{}.gif", imagePrefix, nf(i, 4));
            this.images[i as usize] = load_image(&filename);
        }
        this
    }

    fn display(&mut self, xpos: f32, ypos: f32) {
        self.frame = (self.frame + 1) % self.imageCount;
        image(self.images[self.frame as usize], xpos, ypos);
    }

    fn get_width(&self) -> i32 {
        return self.images[0].width() as i32;
    }
}
