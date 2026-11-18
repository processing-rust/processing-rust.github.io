/**
 * Acceleration with Vectors
 * by Daniel Shiffman.
 *
 * Demonstration of the basics of motion with vector.
 * A "Mover" object stores location, velocity, and
 * acceleration as vectors. The motion is controlled by
 * affecting the acceleration (in this case towards the mouse).
 */

// A Mover object
let mut mover: Mover = Mover::default();

fn setup() {
    size(640, 360);
    mover = Mover::new();
}

fn draw() {
    background(0);

    // Update the location
    mover.update();
    // Display the Mover
    mover.display();
}

/**
 * Acceleration with Vectors
 * by Daniel Shiffman.
 *
 * Demonstration of the basics of motion with vector.
 * A "Mover" object stores location, velocity, and acceleration as vectors
 * The motion is controlled by affecting the acceleration (in this case towards the mouse)
 */

#[derive(Clone, Default)]
struct Mover {
    location: PVector,
    velocity: PVector,
    acceleration: PVector,
    topspeed: f32,
}

impl Mover {
    // The Mover tracks location, velocity, and acceleration
    // The Mover's maximum speed
    fn new() -> Mover {
        let mut this = Mover {
            location: PVector::new(0.0, 0.0),
            velocity: PVector::new(0.0, 0.0),
            acceleration: PVector::new(0.0, 0.0),
            topspeed: 0.0,
        };
        // Start in the center
        this.location = PVector::new(width() / 2.0, height() / 2.0);
        this.velocity = PVector::new(0.0, 0.0);
        this.topspeed = 5.0;
        this
    }

    fn update(&mut self) {
        // Compute a vector that points from location to mouse
        let mouse: PVector = PVector::new(mouse_x(), mouse_y());
        let mut acceleration: PVector = (mouse - self.location);
        // Set magnitude of acceleration
        acceleration.set_mag(0.2);

        // Velocity changes according to acceleration
        self.velocity.add(&acceleration);
        // Limit the velocity by topspeed
        self.velocity.limit(self.topspeed);
        // Location changes by velocity
        self.location.add(&self.velocity);
    }

    fn display(&self) {
        stroke(255);
        stroke_weight(2.0);
        fill(127);
        ellipse(self.location.x, self.location.y, 48.0, 48.0);
    }
}
