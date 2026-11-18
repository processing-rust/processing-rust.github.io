/**
 * Composite Objects
 *
 * An object can include several other objects. Creating such composite objects
 * is a good way to use the principles of modularity and build higher levels of
 * abstraction within a program.
 */
let mut er1: EggRing = EggRing::default();
let mut er2: EggRing = EggRing::default();

fn setup() {
    size(640, 360);
    er1 = EggRing::new(width() * 0.45, height() * 0.5, 2.0, 120.0);
    er2 = EggRing::new(width() * 0.65, height() * 0.8, 10.0, 180.0);
}

fn draw() {
    background(0);
    er1.transmit();
    er2.transmit();
}

#[derive(Clone, Default)]
struct Egg {
    x: f32,
    y: f32,
    tilt: f32,
    angle: f32,
    scalar: f32,
    range: f32,
}

impl Egg {
    // X-coordinate, y-coordinate
    // Left and right angle offset
    // Used to define the tilt
    // Height of the egg
    // Constructor
    fn new(xpos: f32, ypos: f32, r: f32, s: f32) -> Egg {
        Egg {
            x: xpos,
            y: ypos,
            tilt: 0.0,
            angle: 0.0,
            scalar: s / 100.0,
            range: r,
        }
    }

    fn wobble(&mut self) {
        self.tilt = self.angle.cos() / self.range;
        self.angle += 0.1;
        //print (angle + "-");
    }

    fn display(&self) {
        no_stroke();
        fill(255);
        push_matrix();
        translate(self.x, self.y);
        rotate(self.tilt);
        scale(self.scalar);
        begin_shape();
        vertex(0.0, -100.0);
        bezier_vertex(25.0, -100.0, 40.0, -65.0, 40.0, -40.0);
        bezier_vertex(40.0, -15.0, 25.0, 0.0, 0.0, 0.0);
        bezier_vertex(-25.0, 0.0, -40.0, -15.0, -40.0, -40.0);
        bezier_vertex(-40.0, -65.0, -25.0, -100.0, 0.0, -100.0);
        end_shape();
        pop_matrix();
    }
}

#[derive(Clone, Default)]
struct EggRing {
    ovoid: Egg,
    circle: Ring,
}

impl EggRing {
    fn new(x: f32, y: f32, t: f32, sp: f32) -> EggRing {
        let mut this = EggRing {
            ovoid: Egg::new(x, y, t, sp),
            circle: Ring::new(),
        };
        this.circle.start(x, y - sp / 2.0);
        this
    }

    fn transmit(&mut self) {
        self.ovoid.wobble();
        self.ovoid.display();
        self.circle.grow();
        self.circle.display();
        if self.circle.on == false {
            self.circle.on = true;
        }
    }
}

#[derive(Clone, Default)]
struct Ring {
    x: f32,
    y: f32,
    diameter: f32,
    on: bool,
}

impl Ring {
    // X-coordinate, y-coordinate
    // Diameter of the ring
    // Turns the display on and off
    fn start(&mut self, xpos: f32, ypos: f32) {
        self.x = xpos;
        self.y = ypos;
        self.on = true;
        self.diameter = 1.0;
    }

    fn grow(&mut self) {
        if self.on == true {
            self.diameter += 0.5;
            if self.diameter > width() * 2.0 {
                self.diameter = 0.0;
            }
        }
    }

    fn display(&self) {
        if self.on == true {
            no_fill();
            stroke_weight(4.0);
            stroke(155, 153);
            ellipse(self.x, self.y, self.diameter, self.diameter);
        }
    }

    fn new() -> Ring {
        Ring {
            x: 0.0,
            y: 0.0,
            diameter: 0.0,
            on: false,
        }
    }
}
