/**
 * Handles.
 *
 * Click and drag the white boxes to change their position.
 */
let mut handles: Vec<Handle> = Vec::new();

//True if a mouse button has just been pressed while no other button was.
let mut firstMousePress: bool = false;

fn setup() {
    size(640, 360);
    let num: i32 = (height() / 15.0) as i32;
    handles = vec![Handle::default(); num as usize];
    let hsize: i32 = 10;
    for i in 0..handles.len() {
        handles[i] = Handle::new((width() / 2.0) as i32, (10 + i * 15) as i32, 50 - hsize / 2, 10, &handles);
    }
}

fn draw() {
    background(153);

    for i in 0..handles.len() {
        handles[i].update(firstMousePress);
        handles[i].display();
    }

    fill(0);
    rect(0.0, 0.0, width() / 2.0, height());

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

fn mouse_released() {
    for i in 0..handles.len() {
        handles[i].release_event();
    }
}

#[derive(Clone, Default)]
struct Handle {
    x: i32,
    y: i32,
    boxx: i32,
    boxy: i32,
    stretch: i32,
    size: i32,
    over: bool,
    press: bool,
    locked: bool,
    otherslocked: bool,
    others: Vec<Handle>,
}

impl Handle {
    fn new(ix: i32, iy: i32, il: i32, is: i32, o: &[Handle]) -> Handle {
        let mut this = Handle {
            x: ix,
            y: iy,
            boxx: 0,
            boxy: 0,
            stretch: il,
            size: is,
            over: false,
            press: false,
            locked: false,
            otherslocked: false,
            others: Vec::new(),
        };
        this.boxx = this.x + this.stretch - this.size / 2;
        this.boxy = this.y - this.size / 2;
        this.others = o.to_vec();
        this
    }

    fn update(&mut self, firstMousePress: bool) {
        self.boxx = self.x + self.stretch;
        self.boxy = self.y - self.size / 2;

        for i in 0..self.others.len() {
            if self.others[i].locked == true {
                self.otherslocked = true;
                break;
            } else {
                self.otherslocked = false;
            }
        }

        if self.otherslocked == false {
            self.over_event();
            self.press_event(firstMousePress);
        }

        if self.press {
            self.stretch = lock((mouse_x() - width() / 2.0 - (self.size / 2) as f32) as i32, 0, (width() / 2.0 - self.size as f32 - 1.0) as i32);
        }
    }

    fn over_event(&mut self) {
        if over_rect(self.boxx, self.boxy, self.size, self.size) {
            self.over = true;
        } else {
            self.over = false;
        }
    }

    fn press_event(&mut self, firstMousePress: bool) {
        if self.over && firstMousePress || self.locked {
            self.press = true;
            self.locked = true;
        } else {
            self.press = false;
        }
    }

    fn release_event(&mut self) {
        self.locked = false;
    }

    fn display(&self) {
        line(self.x as f32, self.y as f32, (self.x + self.stretch) as f32, self.y as f32);
        fill(255);
        stroke(0);
        rect(self.boxx as f32, self.boxy as f32, self.size as f32, self.size as f32);
        if self.over || self.press {
            line(self.boxx as f32, self.boxy as f32, (self.boxx + self.size) as f32, (self.boxy + self.size) as f32);
            line(self.boxx as f32, (self.boxy + self.size) as f32, (self.boxx + self.size) as f32, self.boxy as f32);
        }
    }
}

fn over_rect(x: i32, y: i32, width: i32, height: i32) -> bool {
    if mouse_x() >= x as f32 && mouse_x() <= (x + width) as f32 && mouse_y() >= y as f32 && mouse_y() <= (y + height) as f32 {
        return true;
    } else {
        return false;
    }
}

fn lock(val: i32, minv: i32, maxv: i32) -> i32 {
    return val.max(minv).min(maxv);
}
