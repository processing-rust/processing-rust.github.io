/**
 * ArrayList of objects
 * by Daniel Shiffman.
 *
 * This example demonstrates how to use a Java ArrayList to store
 * a variable number of objects.  Items can be added and removed
 * from the ArrayList.
 *
 * Click the mouse to add bouncing balls.
 */
let mut balls: Vec<Ball> = Vec::new();
let ballWidth: i32 = 48;

fn setup() {
    size(640, 360);
    no_stroke();

    // Create an empty ArrayList (will store Ball objects)
    balls = Vec::new();

    // Start by adding one element
    balls.push(Ball::new(width() / 2.0, 0.0, ballWidth as f32));
}

fn draw() {
    background(255);

    // With an array, we say balls.length, with an ArrayList, we say balls.size()
    // The length of an ArrayList is dynamic
    // Notice how we are looping through the ArrayList backwards
    // This is because we are deleting elements from the list
    for i in (0..balls.len()).rev() {
        // An ArrayList doesn't know what it is storing so we have to cast the object coming out
        let ball = &mut balls[i];
        ball.move_();
        ball.display();
        if ball.finished() {
            // Items can be deleted with remove()
            balls.remove(i);
        }
    }
}

fn mouse_pressed() {
    // A new ball object is added to the ArrayList (by default to the end)
    balls.push(Ball::new(mouse_x(), mouse_y(), ballWidth as f32));
}

// Simple bouncing ball class

#[derive(Clone, Default)]
struct Ball {
    x: f32,
    y: f32,
    speed: f32,
    gravity: f32,
    w: f32,
    life: f32,
}

impl Ball {
    fn new(tempX: f32, tempY: f32, tempW: f32) -> Ball {
        Ball {
            x: tempX,
            y: tempY,
            speed: 0.0,
            gravity: 0.1,
            w: tempW,
            life: 255.0,
        }
    }

    fn move_(&mut self) {
        // Add gravity to speed
        self.speed = self.speed + self.gravity;
        // Add speed to y location
        self.y = self.y + self.speed;
        // If square reaches the bottom
        // Reverse speed
        if self.y > height() {
            // Dampening
            self.speed = self.speed * -0.8;
            self.y = height();
        }
    }

    fn finished(&mut self) -> bool {
        // Balls fade out
        self.life -= 1.0;
        if self.life < 0.0 {
            return true;
        } else {
            return false;
        }
    }

    fn display(&self) {
        // Display the circle
        fill(0, self.life);
        //stroke(0,life);
        ellipse(self.x, self.y, self.w, self.w);
    }
}
