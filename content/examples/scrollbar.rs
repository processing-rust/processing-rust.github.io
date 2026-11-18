/**
 * Scrollbar.
 *
 * Move the scrollbars left and right to change the positions of the images.
 */

//True if a mouse button was pressed while no other button was.
let mut firstMousePress: bool = false;
let mut hs1: HScrollbar = HScrollbar::default();
let mut hs2: HScrollbar = HScrollbar::default();  // Two scrollbars
let mut img1: u64 = 0;
let mut img2: u64 = 0;  // Two images to load

fn setup() {
    size(640, 360);
    no_stroke();

    hs1 = HScrollbar::new(0.0, height() / 2.0 - 8.0, width() as i32, 16, 16);
    hs2 = HScrollbar::new(0.0, height() / 2.0 + 8.0, width() as i32, 16, 16);

    // Load images
    img1 = load_image("seedTop.jpg");
    img2 = load_image("seedBottom.jpg");
}

fn draw() {
    background(255);

    // Get the position of the img1 scrollbar
    // and convert to a value to display the img1 image
    let img1Pos: f32 = hs1.get_pos() - width() / 2.0;
    fill(255);
    image(img1, width() / 2.0 - img1.width() / 2.0 + img1Pos * 1.5, 0.0);

    // Get the position of the img2 scrollbar
    // and convert to a value to display the img2 image
    let img2Pos: f32 = hs2.get_pos() - width() / 2.0;
    fill(255);
    image(img2, width() / 2.0 - img2.width() / 2.0 + img2Pos * 1.5, height() / 2.0);

    hs1.update(firstMousePress);
    hs2.update(firstMousePress);
    hs1.display();
    hs2.display();

    stroke(0);
    line(0.0, height() / 2.0, width(), height() / 2.0);

    //After it has been used in the sketch, set it back to false
    if firstMousePress {
        firstMousePress = false;
    }
}

fn mouse_pressed() {
    if !firstMousePress {
        firstMousePress = true;
    }
}

#[derive(Clone, Default)]
struct HScrollbar {
    swidth: i32,
    sheight: i32,
    xpos: f32,
    ypos: f32,
    spos: f32,
    newspos: f32,
    sposMin: f32,
    sposMax: f32,
    loose: i32,
    over: bool,
    locked: bool,
    ratio: f32,
}

impl HScrollbar {
    // width and height of bar
    // x and y position of bar
    // x position of slider
    // max and min values of slider
    // how loose/heavy
    // is the mouse over the slider?
    fn new(xp: f32, yp: f32, sw: i32, sh: i32, l: i32) -> HScrollbar {
        let mut this = HScrollbar {
            swidth: sw,
            sheight: sh,
            xpos: 0.0,
            ypos: 0.0,
            spos: 0.0,
            newspos: 0.0,
            sposMin: 0.0,
            sposMax: 0.0,
            loose: 0,
            over: false,
            locked: false,
            ratio: 0.0,
        };
        let widthtoheight: i32 = sw - sh;
        this.ratio = sw as f32 / widthtoheight as f32;
        this.xpos = xp;
        this.ypos = yp - (this.sheight / 2) as f32;
        this.spos = this.xpos + (this.swidth / 2) as f32 - (this.sheight / 2) as f32;
        this.newspos = this.spos;
        this.sposMin = this.xpos;
        this.sposMax = this.xpos + this.swidth as f32 - this.sheight as f32;
        this.loose = l;
        this
    }

    fn update(&mut self, firstMousePress: bool) {
        if self.over_event() {
            self.over = true;
        } else {
            self.over = false;
        }
        if firstMousePress && self.over {
            self.locked = true;
        }
        if !mouse_pressed_global() {
            self.locked = false;
        }
        if self.locked {
            self.newspos = self.constrain(mouse_x() - (self.sheight / 2) as f32, self.sposMin, self.sposMax);
        }
        if abs(self.newspos - self.spos) > 1.0 {
            self.spos = self.spos + (self.newspos - self.spos) / self.loose as f32;
        }
    }

    fn constrain(&self, val: f32, minv: f32, maxv: f32) -> f32 {
        return min(max(val, minv), maxv);
    }

    fn over_event(&self) -> bool {
        if mouse_x() > self.xpos && mouse_x() < self.xpos + self.swidth as f32 && mouse_y() > self.ypos && mouse_y() < self.ypos + self.sheight as f32 {
            return true;
        } else {
            return false;
        }
    }

    fn display(&self) {
        no_stroke();
        fill(204);
        rect(self.xpos, self.ypos, self.swidth as f32, self.sheight as f32);
        if self.over || self.locked {
            fill(0, 0, 0);
        } else {
            fill(102, 102, 102);
        }
        rect(self.spos, self.ypos, self.sheight as f32, self.sheight as f32);
    }

    fn get_pos(&self) -> f32 {
        // Convert spos to be values between
        // 0 and the total width of the scrollbar
        return self.spos * self.ratio;
    }
}
