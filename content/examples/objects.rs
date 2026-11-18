/**
 * Objects
 * by hbarragan.
 *
 * Move the cursor across the image to change the speed and positions
 * of the geometry. The class MRect defines a group of lines.
 */
let mut r1: MRect = MRect::default();
let mut r2: MRect = MRect::default();
let mut r3: MRect = MRect::default();
let mut r4: MRect = MRect::default();

fn setup() {
    size(640, 360);
    fill(255, 204);
    no_stroke();
    r1 = MRect::new(1, 134.0, 0.532, 0.1 * height(), 10.0, 60.0);
    r2 = MRect::new(2, 44.0, 0.166, 0.3 * height(), 5.0, 50.0);
    r3 = MRect::new(2, 58.0, 0.332, 0.4 * height(), 10.0, 35.0);
    r4 = MRect::new(1, 120.0, 0.0498, 0.9 * height(), 15.0, 60.0);
}

fn draw() {
    background(0);

    r1.display();
    r2.display();
    r3.display();
    r4.display();

    r1.move_(mouse_x() - (width() / 2.0), mouse_y() + (height() * 0.1), 30.0);
    r2.move_((mouse_x() + (width() * 0.05)) % width(), mouse_y() + (height() * 0.025), 20.0);
    r3.move_(mouse_x() / 4.0, mouse_y() - (height() * 0.025), 40.0);
    r4.move_(mouse_x() - (width() / 2.0), (height() - mouse_y()), 50.0);
}

#[derive(Clone, Default)]
struct MRect {
    w: i32,
    xpos: f32,
    h: f32,
    ypos: f32,
    d: f32,
    t: f32,
}

impl MRect {
    // single bar width
    // rect xposition
    // rect height
    // rect yposition
    // single bar distance
    // number of bars
    fn new(iw: i32, ixp: f32, ih: f32, iyp: f32, id: f32, it: f32) -> MRect {
        MRect {
            w: iw,
            xpos: ixp,
            h: ih,
            ypos: iyp,
            d: id,
            t: it,
        }
    }

    fn move_(&mut self, posX: f32, posY: f32, damping: f32) {
        let mut dif: f32 = self.ypos - posY;
        if abs(dif) > 1.0 {
            self.ypos -= dif / damping;
        }
        dif = self.xpos - posX;
        if abs(dif) > 1.0 {
            self.xpos -= dif / damping;
        }
    }

    fn display(&self) {
        for i in 0..self.t as i32 {
            rect(self.xpos + (i as f32 * (self.d + self.w as f32)), self.ypos, self.w as f32, height() * self.h);
        }
    }
}
