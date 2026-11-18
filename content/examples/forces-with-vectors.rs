/**
 * Forces (Gravity and Fluid Resistence) with Vectors
 * by Daniel Shiffman.
 *
 * Demonstration of multiple forces acting on bodies.
 * Bodies experience gravity continuously and fluid
 * resistance when in "water".
 */

// Five moving bodies
let mut movers: Vec<Mover> = vec![Mover::default(); 10];

// Liquid
let mut liquid: Liquid = Liquid::default();

fn setup() {
    size(640, 360);
    reset(&mut movers);
    // Create liquid object
    liquid = Liquid::new(0.0, height() / 2.0, width(), height() / 2.0, 0.1);
}

fn draw() {
    background(0);

    // Draw water
    liquid.display();

    for mover in movers.iter_mut() {
        // Is the Mover in the liquid?
        if liquid.contains(&mover) {
            // Calculate drag force
            let drag: PVector = liquid.drag(&mover);
            // Apply drag force to Mover
            mover.apply_force(drag);
        }

        // Gravity is scaled by mass here!
        let gravity: PVector = PVector::new(0.0, 0.1 * mover.mass);
        // Apply gravity
        mover.apply_force(gravity);

        // Update and display
        mover.update();
        mover.display();
        mover.check_edges();
    }

    fill(255);
    text("click mouse to reset", 10.0, 30.0);
}

fn mouse_pressed() {
    reset(&mut movers);
}

// Restart all the Mover objects randomly
fn reset(movers: &mut Vec<Mover>) {
    for i in 0..movers.len() {
        movers[i] = Mover::new(random(0.5, 3.0), (40 + i * 70) as f32, 0.0);
    }
}

/**
 * Forces (Gravity and Fluid Resistence) with Vectors
 * by Daniel Shiffman.
 *
 * Demonstration of multiple force acting on bodies (Mover class)
 * Bodies experience gravity continuously
 * Bodies experience fluid resistance when in "water"
 */

#[derive(Clone, Default)]
struct Liquid {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    c: f32,
}

impl Liquid {
    // Liquid is a rectangle
    // Coefficient of drag
    fn new(x_: f32, y_: f32, w_: f32, h_: f32, c_: f32) -> Liquid {
        Liquid {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
            c: c_,
        }
    }

    // Is the Mover in the Liquid?
    fn contains(&self, m: &Mover) -> bool {
        let l: PVector = m.position;
        if l.x > self.x && l.x < self.x + self.w && l.y > self.y && l.y < self.y + self.h {
            return true;
        } else {
            return false;
        }
    }

    // Calculate drag force
    fn drag(&self, m: &Mover) -> PVector {
        // Magnitude is coefficient * speed squared
        let speed: f32 = m.velocity.mag();
        let dragMagnitude: f32 = self.c * speed * speed;

        // Direction is inverse of velocity
        let mut drag: PVector = m.velocity.copy();
        drag.mult(-1.0);

        // Scale according to magnitude
        drag.set_mag(dragMagnitude);
        return drag;
    }

    fn display(&self) {
        no_stroke();
        fill(127);
        rect(self.x, self.y, self.w, self.h);
    }
}

/**
 * Forces (Gravity and Fluid Resistence) with Vectors
 * by Daniel Shiffman.
 *
 * Demonstration of multiple force acting on bodies (Mover class)
 * Bodies experience gravity continuously
 * Bodies experience fluid resistance when in "water"
 */

#[derive(Clone, Default)]
struct Mover {
    position: PVector,
    velocity: PVector,
    acceleration: PVector,
    mass: f32,
}

impl Mover {
    // position, velocity, and acceleration
    // Mass is tied to size
    fn new(m: f32, x: f32, y: f32) -> Mover {
        Mover {
            position: PVector::new(x, y),
            velocity: PVector::new(0.0, 0.0),
            acceleration: PVector::new(0.0, 0.0),
            mass: m,
        }
    }

    // Newton's 2nd law: F = M * A
    // or A = F / M
    fn apply_force(&mut self, force: PVector) {
        // Divide by mass
        let f: PVector = (force / self.mass);
        // Accumulate all forces in acceleration
        self.acceleration.add(&f);
    }

    fn update(&mut self) {
        // Velocity changes according to acceleration
        self.velocity.add(&self.acceleration);
        // position changes by velocity
        self.position.add(&self.velocity);
        // We must clear acceleration each frame
        self.acceleration.mult(0.0);
    }

    // Draw Mover
    fn display(&self) {
        stroke(255);
        stroke_weight(2.0);
        fill(255, 200);
        ellipse(self.position.x, self.position.y, self.mass * 16.0, self.mass * 16.0);
    }

    // Bounce off bottom of window
    fn check_edges(&mut self) {
        if self.position.y > height() {
            self.velocity.y *= -0.9;  // A little dampening when hitting the bottom
            self.position.y = height();
        }
    }
}
