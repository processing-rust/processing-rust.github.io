/**
 * Arctangent.
 *
 * Move the mouse to change the direction of the eyes.
 * The atan2() function computes the angle from each eye
 * to the cursor.
 */
let mut e1: Eye = Eye::default();
let mut e2: Eye = Eye::default();
let mut e3: Eye = Eye::default();

fn setup() {
    size(640, 360);
    no_stroke();
    e1 = Eye::new(250, 16, 120);
    e2 = Eye::new(164, 185, 80);
    e3 = Eye::new(420, 230, 220);
}

fn draw() {
    background(102);

    e1.update(mouse_x() as i32, mouse_y() as i32);
    e2.update(mouse_x() as i32, mouse_y() as i32);
    e3.update(mouse_x() as i32, mouse_y() as i32);

    e1.display();
    e2.display();
    e3.display();
}

#[derive(Clone, Default)]
struct Eye {
    x: i32,
    y: i32,
    size: i32,
    angle: f32,
}

impl Eye {
    fn new(tx: i32, ty: i32, ts: i32) -> Eye {
        Eye {
            x: tx,
            y: ty,
            size: ts,
            angle: 0.0,
        }
    }

    fn update(&mut self, mx: i32, my: i32) {
        self.angle = ((my - self.y) as f32).atan2((mx - self.x) as f32);
    }

    fn display(&self) {
        push_matrix();
        translate(self.x as f32, self.y as f32);
        fill(255);
        ellipse(0.0, 0.0, self.size as f32, self.size as f32);
        rotate(self.angle);
        fill(153, 204, 0);
        ellipse((self.size / 4) as f32, 0.0, (self.size / 2) as f32, (self.size / 2) as f32);
        pop_matrix();
    }
}
