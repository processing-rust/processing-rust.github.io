/**
 * Non-orthogonal Collision with Multiple Ground Segments
 * by Ira Greenberg.
 *
 * Based on Keith Peter's Solution in
 * Foundation Actionscript Animation: Making Things Move!
 */
let mut orb: Orb = Orb::default();

let mut gravity: PVector = PVector::new(0.0, 0.05);
// The ground is an array of "Ground" objects
let segments: i32 = 40;
let mut ground: Vec<Ground> = vec![Ground::default(); segments as usize];

fn setup() {
    size(640, 360);
    // An orb object that will fall and bounce around
    orb = Orb::new(50.0, 50.0, 3.0);

    // Calculate ground peak heights
    let mut peakHeights = vec![0.0; (segments + 1) as usize];
    for i in 0..peakHeights.len() {
        peakHeights[i] = random(height() - 40.0, height() - 30.0);
    }

    /* Float value required for segment width (segs)
    calculations so the ground spans the entire
    display window, regardless of segment number. */
    let segs: f32 = segments as f32;
    for i in 0..segments {
        ground[i as usize] = Ground::new(width() / segs * i as f32, peakHeights[i as usize], width() / segs * ((i + 1)) as f32, peakHeights[(i + 1) as usize]);
    }
}

fn draw() {
    // Background
    no_stroke();
    fill(0, 15);
    rect(0.0, 0.0, width(), height());

    // Move and display the orb
    orb.move_(gravity);
    orb.display();
    // Check walls
    orb.check_wall_collision();

    // Check against all the ground segments
    for i in 0..segments {
        orb.check_ground_collision(&ground[i as usize]);
    }

    // Draw ground
    fill(127);
    begin_shape();
    for i in 0..segments {
        vertex(ground[i as usize].x1, ground[i as usize].y1);
        vertex(ground[i as usize].x2, ground[i as usize].y2);
    }
    vertex(ground[(segments - 1) as usize].x2, height());
    vertex(ground[0].x1, height());
    end_shape(CLOSE);
}

#[derive(Clone, Default)]
struct Ground {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x: f32,
    y: f32,
    len: f32,
    rot: f32,
}

impl Ground {
    // Constructor
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Ground {
        let mut this = Ground {
            x1,
            y1,
            x2,
            y2,
            x: 0.0,
            y: 0.0,
            len: 0.0,
            rot: 0.0,
        };
        this.x = (x1 + x2) / 2.0;
        this.y = (y1 + y2) / 2.0;
        this.len = dist(x1, y1, x2, y2);
        this.rot = ((y2 - y1)).atan2((x2 - x1));
        this
    }
}

#[derive(Clone, Default)]
struct Orb {
    position: PVector,
    velocity: PVector,
    r: f32,
    damping: f32,
}

impl Orb {
    // Orb has positio and velocity
    // A damping of 80% slows it down when it hits the ground
    fn new(x: f32, y: f32, r_: f32) -> Orb {
        Orb {
            position: PVector::new(x, y),
            velocity: PVector::new(0.5, 0.0),
            r: r_,
            damping: 0.8,
        }
    }

    fn move_(&mut self, gravity: PVector) {
        // Move orb
        self.velocity.add(&gravity);
        self.position.add(&self.velocity);
    }

    fn display(&self) {
        // Draw orb
        no_stroke();
        fill(200);
        ellipse(self.position.x, self.position.y, self.r * 2.0, self.r * 2.0);
    }

    // Check boundaries of window
    fn check_wall_collision(&mut self) {
        if self.position.x > width() - self.r {
            self.position.x = width() - self.r;
            self.velocity.x *= -self.damping;
        } else if self.position.x < self.r {
            self.position.x = self.r;
            self.velocity.x *= -self.damping;
        }
    }

    fn check_ground_collision(&mut self, groundSegment: &Ground) {
        // Get difference between orb and ground
        let mut deltaX: f32 = self.position.x - groundSegment.x;
        let mut deltaY: f32 = self.position.y - groundSegment.y;

        // Precalculate trig values
        let cosine: f32 = groundSegment.rot.cos();
        let sine: f32 = groundSegment.rot.sin();

        /* Rotate ground and velocity to allow
        orthogonal collision calculations */
        let groundXTemp: f32 = cosine * deltaX + sine * deltaY;
        let mut groundYTemp: f32 = cosine * deltaY - sine * deltaX;
        let velocityXTemp: f32 = cosine * self.velocity.x + sine * self.velocity.y;
        let mut velocityYTemp: f32 = cosine * self.velocity.y - sine * self.velocity.x;

        /* Ground collision - check for surface
        collision and also that orb is within
        left/rights bounds of ground segment */
        if groundYTemp > -self.r && self.position.x > groundSegment.x1 && self.position.x < groundSegment.x2 {
            // keep orb from going into ground
            groundYTemp = -self.r;
            // bounce and slow down orb
            velocityYTemp *= -1.0;
            velocityYTemp *= self.damping;
        }

        // Reset ground, velocity and orb
        deltaX = cosine * groundXTemp - sine * groundYTemp;
        deltaY = cosine * groundYTemp + sine * groundXTemp;
        self.velocity.x = cosine * velocityXTemp - sine * velocityYTemp;
        self.velocity.y = cosine * velocityYTemp + sine * velocityXTemp;
        self.position.x = groundSegment.x + deltaX;
        self.position.y = groundSegment.y + deltaY;
    }
}
