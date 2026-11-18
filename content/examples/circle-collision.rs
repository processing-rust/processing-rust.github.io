/**
 * Circle Collision with Swapping Velocities
 * by Ira Greenberg.
 *
 * Based on Keith Peter's Solution in
 * Foundation Actionscript Animation: Making Things Move!
 */
let mut balls: Vec<Ball> = vec![Ball::new(100.0, 400.0, 20.0), Ball::new(700.0, 400.0, 80.0)];

fn setup() {
    size(640, 360);
}

fn draw() {
    background(51);

    for b in balls.iter_mut() {
        b.update();
        b.display();
        b.check_boundary_collision();
    }

    // two balls of the same list, one changed through the other
    let (first, rest) = balls.split_at_mut(1);
    first[0].check_collision(&mut rest[0]);
}

#[derive(Clone, Default)]
struct Ball {
    position: PVector,
    velocity: PVector,
    radius: f32,
    m: f32,
}

impl Ball {
    fn new(x: f32, y: f32, r_: f32) -> Ball {
        let mut this = Ball {
            position: PVector::new(x, y),
            velocity: PVector::random_2d(),
            radius: 0.0,
            m: 0.0,
        };
        this.velocity.mult(3.0);
        this.radius = r_;
        this.m = this.radius * 0.1;
        this
    }

    fn update(&mut self) {
        self.position.add(&self.velocity);
    }

    fn check_boundary_collision(&mut self) {
        if self.position.x > width() - self.radius {
            self.position.x = width() - self.radius;
            self.velocity.x *= -1.0;
        } else if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        } else if self.position.y > height() - self.radius {
            self.position.y = height() - self.radius;
            self.velocity.y *= -1.0;
        } else if self.position.y < self.radius {
            self.position.y = self.radius;
            self.velocity.y *= -1.0;
        }
    }

    fn check_collision(&mut self, other: &mut Ball) {
        // Get distances between the balls components
        let distanceVect: PVector = (other.position - self.position);

        // Calculate magnitude of the vector separating the balls
        let distanceVectMag: f32 = distanceVect.mag();

        // Minimum distance before they are touching
        let minDistance: f32 = self.radius + other.radius;

        if distanceVectMag < minDistance {
            let distanceCorrection: f32 = (minDistance - distanceVectMag) / 2.0;
            let mut d: PVector = distanceVect.copy();
            let correctionVector: PVector = *d.normalize().mult(distanceCorrection);
            other.position.add(&correctionVector);
            self.position.sub(&correctionVector);

            // get angle of distanceVect
            let theta: f32 = distanceVect.heading();
            // precalculate trig values
            let sine: f32 = theta.sin();
            let cosine: f32 = theta.cos();

            /* bTemp will hold rotated ball positions. You
            just need to worry about bTemp[1] position*/
            let mut bTemp = vec![PVector::new(0.0, 0.0), PVector::new(0.0, 0.0)];

            /* this ball's position is relative to the other
            so you can use the vector between them (bVect) as the
            reference point in the rotation expressions.
            bTemp[0].position.x and bTemp[0].position.y will initialize
            automatically to 0.0, which is what you want
            since b[1] will rotate around b[0] */
            bTemp[1].x = cosine * distanceVect.x + sine * distanceVect.y;
            bTemp[1].y = cosine * distanceVect.y - sine * distanceVect.x;

            // rotate Temporary velocities
            let mut vTemp = vec![PVector::new(0.0, 0.0), PVector::new(0.0, 0.0)];

            vTemp[0].x = cosine * self.velocity.x + sine * self.velocity.y;
            vTemp[0].y = cosine * self.velocity.y - sine * self.velocity.x;
            vTemp[1].x = cosine * other.velocity.x + sine * other.velocity.y;
            vTemp[1].y = cosine * other.velocity.y - sine * other.velocity.x;

            /* Now that velocities are rotated, you can use 1D
            conservation of momentum equations to calculate
            the final velocity along the x-axis. */
            let mut vFinal = vec![PVector::new(0.0, 0.0), PVector::new(0.0, 0.0)];

            // final rotated velocity for b[0]
            vFinal[0].x = ((self.m - other.m) * vTemp[0].x + 2.0 * other.m * vTemp[1].x) / (self.m + other.m);
            vFinal[0].y = vTemp[0].y;

            // final rotated velocity for b[0]
            vFinal[1].x = ((other.m - self.m) * vTemp[1].x + 2.0 * self.m * vTemp[0].x) / (self.m + other.m);
            vFinal[1].y = vTemp[1].y;

            // hack to avoid clumping
            bTemp[0].x += vFinal[0].x;
            bTemp[1].x += vFinal[1].x;

            /* Rotate ball positions and velocities back
            Reverse signs in trig expressions to rotate
            in the opposite direction */
            // rotate balls
            let mut bFinal = vec![PVector::new(0.0, 0.0), PVector::new(0.0, 0.0)];

            bFinal[0].x = cosine * bTemp[0].x - sine * bTemp[0].y;
            bFinal[0].y = cosine * bTemp[0].y + sine * bTemp[0].x;
            bFinal[1].x = cosine * bTemp[1].x - sine * bTemp[1].y;
            bFinal[1].y = cosine * bTemp[1].y + sine * bTemp[1].x;

            // update balls to screen position
            other.position.x = self.position.x + bFinal[1].x;
            other.position.y = self.position.y + bFinal[1].y;

            self.position.add(&bFinal[0]);

            // update velocities
            self.velocity.x = cosine * vFinal[0].x - sine * vFinal[0].y;
            self.velocity.y = cosine * vFinal[0].y + sine * vFinal[0].x;
            other.velocity.x = cosine * vFinal[1].x - sine * vFinal[1].y;
            other.velocity.y = cosine * vFinal[1].y + sine * vFinal[1].x;
        }
    }

    fn display(&self) {
        no_stroke();
        fill(204);
        ellipse(self.position.x, self.position.y, self.radius * 2.0, self.radius * 2.0);
    }
}
