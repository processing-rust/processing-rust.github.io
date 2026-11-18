/**
 * Flocking
 * by Daniel Shiffman.
 *
 * An implementation of Craig Reynold's Boids program to simulate
 * the flocking behavior of birds. Each boid steers itself based on
 * rules of avoidance, alignment, and coherence.
 *
 * Click the mouse to add a new boid.
 */
let mut flock: Flock = Flock::default();

fn setup() {
    size(640, 360);
    flock = Flock::new();
    // Add an initial set of boids into the system
    for i in 0..150 {
        flock.add_boid(Boid::new(width() / 2.0, height() / 2.0));
    }
}

fn draw() {
    background(50);
    flock.run();
}

// Add a new boid into the System
fn mouse_pressed() {
    flock.add_boid(Boid::new(mouse_x(), mouse_y()));
}

// The Boid class

#[derive(Clone, Default)]
struct Boid {
    position: PVector,
    velocity: PVector,
    acceleration: PVector,
    r: f32,
    maxforce: f32,
    maxspeed: f32,
}

impl Boid {
    // Maximum steering force
    // Maximum speed
    fn new(x: f32, y: f32) -> Boid {
        let mut this = Boid {
            position: PVector::new(0.0, 0.0),
            velocity: PVector::new(0.0, 0.0),
            acceleration: PVector::new(0.0, 0.0),
            r: 0.0,
            maxforce: 0.0,
            maxspeed: 0.0,
        };
        // This is a new PVector method not yet implemented in JS
        // velocity = PVector.random2D();

        // Leaving the code temporarily this way so that this example runs in JS
        let angle: f32 = random(TWO_PI);
        this.velocity = PVector::new(angle.cos(), angle.sin());

        this.position = PVector::new(x, y);
        this.r = 2.0;
        this.maxspeed = 2.0;
        this.maxforce = 0.03;
        this
    }

    fn run(&mut self, boids: &[Boid]) {
        self.flock(boids);
        self.update();
        self.borders();
        self.render();
    }

    fn apply_force(&mut self, force: PVector) {
        // We could add mass here if we want A = F / M
        self.acceleration.add(&force);
    }

    // We accumulate a new acceleration each time based on three rules
    fn flock(&mut self, boids: &[Boid]) {
        let mut sep: PVector = self.separate(boids);  // Separation
        let mut ali: PVector = self.align(boids);  // Alignment
        let mut coh: PVector = self.cohesion(boids);  // Cohesion
        // Arbitrarily weight these forces
        sep.mult(1.5);
        ali.mult(1.0);
        coh.mult(1.0);
        // Add the force vectors to acceleration
        self.apply_force(sep);
        self.apply_force(ali);
        self.apply_force(coh);
    }

    // Method to update position
    fn update(&mut self) {
        // Update velocity
        self.velocity.add(&self.acceleration);
        // Limit speed
        self.velocity.limit(self.maxspeed);
        self.position.add(&self.velocity);
        // Reset accelertion to 0 each cycle
        self.acceleration.mult(0.0);
    }

    // A method that calculates and applies a steering force towards a target
    // STEER = DESIRED MINUS VELOCITY
    fn seek(&self, target: PVector) -> PVector {
        let mut desired: PVector = (target - self.position);  // A vector pointing from the position to the target
        // Scale to maximum speed
        desired.normalize();
        desired.mult(self.maxspeed);

        // Above two lines of code below could be condensed with new PVector setMag() method
        // Not using this method until Processing.js catches up
        // desired.setMag(maxspeed);

        // Steering = Desired minus Velocity
        let mut steer: PVector = (desired - self.velocity);
        steer.limit(self.maxforce);  // Limit to maximum steering force
        return steer;
    }

    fn render(&self) {
        // Draw a triangle rotated in the direction of velocity
        let theta: f32 = self.velocity.heading() + radians(90.0);
        // heading2D() above is now heading() but leaving old syntax until Processing.js catches up

        fill(200, 100);
        stroke(255);
        push_matrix();
        translate(self.position.x, self.position.y);
        rotate(theta);
        begin_shape(TRIANGLES);
        vertex(0.0, -self.r * 2.0);
        vertex(-self.r, self.r * 2.0);
        vertex(self.r, self.r * 2.0);
        end_shape();
        pop_matrix();
    }

    // Wraparound
    fn borders(&mut self) {
        if self.position.x < -self.r {
            self.position.x = width() + self.r;
        }
        if self.position.y < -self.r {
            self.position.y = height() + self.r;
        }
        if self.position.x > width() + self.r {
            self.position.x = -self.r;
        }
        if self.position.y > height() + self.r {
            self.position.y = -self.r;
        }
    }

    // Separation
    // Method checks for nearby boids and steers away
    fn separate(&self, boids: &[Boid]) -> PVector {
        let desiredseparation: f32 = 25.0;
        let mut steer: PVector = PVector::new3(0.0, 0.0, 0.0);
        let mut count: i32 = 0;
        // For every boid in the system, check if it's too close
        for other in boids.iter() {
            let d: f32 = self.position.dist(&other.position);
            // If the distance is greater than 0 and less than an arbitrary amount (0 when you are yourself)
            if (d > 0.0) && (d < desiredseparation) {
                // Calculate vector pointing away from neighbor
                let mut diff: PVector = (self.position - other.position);
                diff.normalize();
                diff.div(d);  // Weight by distance
                steer.add(&diff);
                count += 1;  // Keep track of how many
            }
        }
        // Average -- divide by how many
        if count > 0 {
            steer.div(count as f32);
        }

        // As long as the vector is greater than 0
        if steer.mag() > 0.0 {
            // First two lines of code below could be condensed with new PVector setMag() method
            // Not using this method until Processing.js catches up
            // steer.setMag(maxspeed);

            // Implement Reynolds: Steering = Desired - Velocity
            steer.normalize();
            steer.mult(self.maxspeed);
            steer.sub(&self.velocity);
            steer.limit(self.maxforce);
        }
        return steer;
    }

    // Alignment
    // For every nearby boid in the system, calculate the average velocity
    fn align(&self, boids: &[Boid]) -> PVector {
        let neighbordist: f32 = 50.0;
        let mut sum: PVector = PVector::new(0.0, 0.0);
        let mut count: i32 = 0;
        for other in boids.iter() {
            let d: f32 = self.position.dist(&other.position);
            if (d > 0.0) && (d < neighbordist) {
                sum.add(&other.velocity);
                count += 1;
            }
        }
        if count > 0 {
            sum.div(count as f32);
            // First two lines of code below could be condensed with new PVector setMag() method
            // Not using this method until Processing.js catches up
            // sum.setMag(maxspeed);

            // Implement Reynolds: Steering = Desired - Velocity
            sum.normalize();
            sum.mult(self.maxspeed);
            let mut steer: PVector = (sum - self.velocity);
            steer.limit(self.maxforce);
            return steer;
        } else {
            return PVector::new(0.0, 0.0);
        }
    }

    // Cohesion
    // For the average position (i.e. center) of all nearby boids, calculate steering vector towards that position
    fn cohesion(&self, boids: &[Boid]) -> PVector {
        let neighbordist: f32 = 50.0;
        let mut sum: PVector = PVector::new(0.0, 0.0);  // Start with empty vector to accumulate all positions
        let mut count: i32 = 0;
        for other in boids.iter() {
            let d: f32 = self.position.dist(&other.position);
            if (d > 0.0) && (d < neighbordist) {
                sum.add(&other.position);  // Add position
                count += 1;
            }
        }
        if count > 0 {
            sum.div(count as f32);
            return self.seek(sum);  // Steer towards the position
        } else {
            return PVector::new(0.0, 0.0);
        }
    }
}

// The Flock (a list of Boid objects)

#[derive(Clone, Default)]
struct Flock {
    boids: Vec<Boid>,
}

impl Flock {
    // An ArrayList for all the boids
    fn new() -> Flock {
        let mut this = Flock {
            boids: Vec::new(),
        };
        // Initialize the ArrayList
        this
    }

    fn run(&mut self) {
        let others = self.boids.clone();
        for b in self.boids.iter_mut() {
            b.run(&others);  // Passing the entire list of boids to each boid individually
        }
    }

    fn add_boid(&mut self, b: Boid) {
        self.boids.push(b);
    }
}
