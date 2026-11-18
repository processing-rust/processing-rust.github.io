/**
 * Smoke Particle System
 * by Daniel Shiffman.
 *
 * A basic smoke effect using a particle system. Each particle
 * is rendered as an alpha masked image.
 */
let mut ps: ParticleSystem = ParticleSystem::default();

fn setup() {
    size(640, 360);
    let img: u64 = load_image("texture.png");
    ps = ParticleSystem::new(0, PVector::new(width() / 2.0, height() - 60.0), img);
}

fn draw() {
    background(0);

    // Calculate a "wind" force based on mouse horizontal position
    let dx: f32 = map(mouse_x(), 0.0, width(), -0.2, 0.2);
    let wind: PVector = PVector::new(dx, 0.0);
    ps.apply_force(wind);
    ps.run();
    for i in 0..2 {
        ps.add_particle();
    }

    // Draw an arrow representing the wind force
    draw_vector(wind, PVector::new3(width() / 2.0, 50.0, 0.0), 500.0);
}

// Renders a vector object 'v' as an arrow and a position 'loc'
fn draw_vector(v: PVector, loc: PVector, scayl: f32) {
    push_matrix();
    let arrowsize: f32 = 4.0;
    // Translate to position to render vector
    translate(loc.x, loc.y);
    stroke(255);
    // Call vector heading function to get direction (note that pointing up is a heading of 0) and rotate
    rotate(v.heading());
    // Calculate length of vector & scale it to be bigger or smaller if necessary
    let len: f32 = v.mag() * scayl;
    // Draw three lines to make an arrow (draw pointing up since we've rotate to the proper direction)
    line(0.0, 0.0, len, 0.0);
    line(len, 0.0, len - arrowsize, arrowsize / 2.0);
    line(len, 0.0, len - arrowsize, -arrowsize / 2.0);
    pop_matrix();
}

// A simple Particle class, renders the particle as an image

#[derive(Clone, Default)]
struct Particle {
    loc: PVector,
    vel: PVector,
    acc: PVector,
    lifespan: f32,
    img: u64,
}

impl Particle {
    fn new(l: PVector, img_: u64) -> Particle {
        let mut this = Particle {
            loc: PVector::new(0.0, 0.0),
            vel: PVector::new(0.0, 0.0),
            acc: PVector::new(0.0, 0.0),
            lifespan: 0.0,
            img: 0,
        };
        let vx: f32 = random_gaussian() * 0.3;
        let vy: f32 = random_gaussian() * 0.3 - 1.0;
        this.vel = PVector::new(vx, vy);
        this.loc = l.copy();
        this.lifespan = 100.0;
        this.img = img_;
        this
    }

    fn run(&mut self) {
        self.update();
        self.render();
    }

    // Method to apply a force vector to the Particle object
    // Note we are ignoring "mass" here
    fn apply_force(&mut self, f: PVector) {
        self.acc.add(&f);
    }

    // Method to update position
    fn update(&mut self) {
        self.vel.add(&self.acc);
        self.loc.add(&self.vel);
        self.lifespan -= 2.5;
        self.acc.mult(0.0);  // clear Acceleration
    }

    // Method to display
    fn render(&self) {
        image_mode(CENTER);
        tint(255.0, self.lifespan);
        image(self.img, self.loc.x, self.loc.y);
        // Drawing a circle instead
        // fill(255,lifespan);
        // noStroke();
        // ellipse(loc.x,loc.y,img.width,img.height);
    }

    // Is the particle still useful?
    fn is_dead(&self) -> bool {
        if self.lifespan <= 0.0 {
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
    img: u64,
}

impl ParticleSystem {
    // An arraylist for all the particles
    // An origin point for where particles are birthed
    fn new(num: i32, v: PVector, img_: u64) -> ParticleSystem {
        let mut this = ParticleSystem {
            particles: Vec::new(),
            origin: PVector::new(0.0, 0.0),
            img: 0,
        };
        // Initialize the arraylist
        this.origin = v.copy();  // Store the origin point
        this.img = img_;
        for i in 0..num {
            this.particles.push(Particle::new(this.origin, this.img));  // Add "num" amount of particles to the arraylist
        }
        this
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

    // Method to add a force vector to all particles currently in the system
    fn apply_force(&mut self, dir: PVector) {
        // Enhanced loop!!!
        for p in self.particles.iter_mut() {
            p.apply_force(dir);
        }
    }

    fn add_particle(&mut self) {
        self.particles.push(Particle::new(self.origin, self.img));
    }
}
