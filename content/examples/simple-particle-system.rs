/**
 * Simple Particle System
 * by Daniel Shiffman.
 *
 * Particles are generated each cycle through draw(),
 * fall with gravity, and fade out over time.
 * A ParticleSystem object manages a variable size (ArrayList)
 * list of particles.
 */
let mut ps: ParticleSystem = ParticleSystem::default();

fn setup() {
    size(640, 360);
    ps = ParticleSystem::new(PVector::new(width() / 2.0, 50.0));
}

fn draw() {
    background(0);
    ps.add_particle();
    ps.run();
}

// A simple Particle class

#[derive(Clone, Default)]
struct Particle {
    position: PVector,
    velocity: PVector,
    acceleration: PVector,
    lifespan: f32,
}

impl Particle {
    fn new(l: PVector) -> Particle {
        Particle {
            position: l.copy(),
            velocity: PVector::new(random(-1.0, 1.0), random(-2.0, 0.0)),
            acceleration: PVector::new(0.0, 0.05),
            lifespan: 255.0,
        }
    }

    fn run(&mut self) {
        self.update();
        self.display();
    }

    // Method to update position
    fn update(&mut self) {
        self.velocity.add(&self.acceleration);
        self.position.add(&self.velocity);
        self.lifespan -= 1.0;
    }

    // Method to display
    fn display(&self) {
        stroke(255, self.lifespan);
        fill(255, self.lifespan);
        ellipse(self.position.x, self.position.y, 8.0, 8.0);
    }

    // Is the particle still useful?
    fn is_dead(&self) -> bool {
        if self.lifespan < 0.0 {
            return true;
        } else {
            return false;
        }
    }
}

// A class to describe a group of Particles
// An ArrayList is used to manage the list of Particles

#[derive(Clone, Default)]
struct ParticleSystem {
    particles: Vec<Particle>,
    origin: PVector,
}

impl ParticleSystem {
    fn new(position: PVector) -> ParticleSystem {
        ParticleSystem {
            particles: Vec::new(),
            origin: position.copy(),
        }
    }

    fn add_particle(&mut self) {
        self.particles.push(Particle::new(self.origin));
    }

    fn run(&mut self) {
        for i in (0..self.particles.len()).rev() {
            let p = &mut self.particles[i];
            p.run();
            if p.is_dead() {
                self.particles.remove(i);
            }
        }
    }
}
