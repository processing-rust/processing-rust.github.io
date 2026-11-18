/**
 * Inheritance
 *
 * A class can be defined using another class as a foundation. In object-oriented
 * programming terminology, one class can inherit fi elds and methods from another.
 * An object that inherits from another is called a subclass, and the object it
 * inherits from is called a superclass. A subclass extends the superclass.
 */
let mut spots: SpinSpots = SpinSpots::default();
let mut arm: SpinArm = SpinArm::default();

fn setup() {
    size(640, 360);
    arm = SpinArm::new(width() / 2.0, height() / 2.0, 0.01);
    spots = SpinSpots::new(width() / 2.0, height() / 2.0, -0.02, 90.0);
}

fn draw() {
    background(204);
    arm.spin.update();
    arm.display();
    spots.spin.update();
    spots.display();
}

#[derive(Clone, Default)]
struct Spin {
    x: f32,
    y: f32,
    speed: f32,
    angle: f32,
}

impl Spin {
    fn new(xpos: f32, ypos: f32, s: f32) -> Spin {
        Spin {
            x: xpos,
            y: ypos,
            speed: s,
            angle: 0.0,
        }
    }

    fn update(&mut self) {
        self.angle += self.speed;
    }
}

#[derive(Clone, Default)]
struct SpinArm {
    spin: Spin,  // what SpinArm builds on (Java: extends Spin)
}

impl SpinArm {
    fn new(x: f32, y: f32, s: f32) -> SpinArm {
        SpinArm {
            spin: Spin::new(x, y, s),
        }
    }

    fn display(&mut self) {
        stroke_weight(1.0);
        stroke(0);
        push_matrix();
        translate(self.spin.x, self.spin.y);
        self.spin.angle += self.spin.speed;
        rotate(self.spin.angle);
        line(0.0, 0.0, 165.0, 0.0);
        pop_matrix();
    }
}

#[derive(Clone, Default)]
struct SpinSpots {
    spin: Spin,  // what SpinSpots builds on (Java: extends Spin)
    dim: f32,
}

impl SpinSpots {
    fn new(x: f32, y: f32, s: f32, d: f32) -> SpinSpots {
        SpinSpots {
            spin: Spin::new(x, y, s),
            dim: d,
        }
    }

    fn display(&mut self) {
        no_stroke();
        push_matrix();
        translate(self.spin.x, self.spin.y);
        self.spin.angle += self.spin.speed;
        rotate(self.spin.angle);
        ellipse(-self.dim / 2.0, 0.0, self.dim, self.dim);
        ellipse(self.dim / 2.0, 0.0, self.dim, self.dim);
        pop_matrix();
    }
}
