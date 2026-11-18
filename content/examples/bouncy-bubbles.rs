/**
 * Bouncy Bubbles
 * based on code from Keith Peters.
 *
 * Multiple-object collision.
 */
const NUMBALLS: i32 = 12;
const SPRING: f32 = 0.05;
const GRAVITY: f32 = 0.03;
const FRICTION: f32 = -0.9;
let mut balls: Vec<Ball> = vec![Ball::default(); NUMBALLS as usize];

fn setup() {
    size(640, 360);
    for i in 0..NUMBALLS {
        balls[i as usize] = Ball::new(random(width()), random(height()), random(30.0, 70.0), i, &balls);
    }
    no_stroke();
    fill(255, 204);
}

fn draw() {
    background(0);
    for ball in balls.iter_mut() {
        ball.collide();
        ball.move_();
        ball.display();
    }
}

#[derive(Clone, Default)]
struct Ball {
    x: f32,
    y: f32,
    diameter: f32,
    vx: f32,
    vy: f32,
    id: i32,
    others: Vec<Ball>,
}

impl Ball {
    fn new(xin: f32, yin: f32, din: f32, idin: i32, oin: &[Ball]) -> Ball {
        Ball {
            x: xin,
            y: yin,
            diameter: din,
            vx: 0.0,
            vy: 0.0,
            id: idin,
            others: oin.to_vec(),
        }
    }

    fn collide(&mut self) {
        for i in self.id + 1..NUMBALLS {
            let dx: f32 = self.others[i as usize].x - self.x;
            let dy: f32 = self.others[i as usize].y - self.y;
            let distance: f32 = sqrt(dx * dx + dy * dy);
            let minDist: f32 = self.others[i as usize].diameter / 2.0 + self.diameter / 2.0;
            if distance < minDist {
                let angle: f32 = dy.atan2(dx);
                let targetX: f32 = self.x + angle.cos() * minDist;
                let targetY: f32 = self.y + angle.sin() * minDist;
                let ax: f32 = (targetX - self.others[i as usize].x) * SPRING;
                let ay: f32 = (targetY - self.others[i as usize].y) * SPRING;
                self.vx -= ax;
                self.vy -= ay;
                self.others[i as usize].vx += ax;
                self.others[i as usize].vy += ay;
            }
        }
    }

    fn move_(&mut self) {
        self.vy += GRAVITY;
        self.x += self.vx;
        self.y += self.vy;
        if self.x + self.diameter / 2.0 > width() {
            self.x = width() - self.diameter / 2.0;
            self.vx *= FRICTION;
        } else if self.x - self.diameter / 2.0 < 0.0 {
            self.x = self.diameter / 2.0;
            self.vx *= FRICTION;
        }
        if self.y + self.diameter / 2.0 > height() {
            self.y = height() - self.diameter / 2.0;
            self.vy *= FRICTION;
        } else if self.y - self.diameter / 2.0 < 0.0 {
            self.y = self.diameter / 2.0;
            self.vy *= FRICTION;
        }
    }

    fn display(&self) {
        ellipse(self.x, self.y, self.diameter, self.diameter);
    }
}
