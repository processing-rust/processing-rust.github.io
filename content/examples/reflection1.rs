/**
 * Non-orthogonal Reflection
 * by Ira Greenberg.
 *
 * Based on the equation (R = 2N(N*L)-L) where R is the
 * reflection vector, N is the normal, and L is the incident
 * vector.
 */

// Position of left hand side of floor
let mut base1: PVector = PVector::new(0.0, 0.0);
// Position of right hand side of floor
let mut base2: PVector = PVector::new(0.0, 0.0);
// Length of floor
let mut baseLength: f32 = 0.0;

// An array of subpoints along the floor path
let mut coords: Vec<PVector> = Vec::new();

// Variables related to moving ball
let mut position: PVector = PVector::new(0.0, 0.0);
let mut velocity: PVector = PVector::new(0.0, 0.0);
let r: f32 = 6.0;
let speed: f32 = 3.5;

fn setup() {
    size(640, 360);

    fill(128);
    base1 = PVector::new(0.0, height() - 150.0);
    base2 = PVector::new(width(), height());
    create_ground(base1, base2, &mut baseLength, &mut coords);

    // start ellipse at middle top of screen
    position = PVector::new(width() / 2.0, 0.0);

    // calculate initial random velocity
    velocity = PVector::random_2d();
    velocity.mult(speed);
}

fn draw() {
    // draw background
    fill(0, 12);
    no_stroke();
    rect(0.0, 0.0, width(), height());

    // draw base
    fill(200);
    quad(base1.x, base1.y, base2.x, base2.y, base2.x, height(), 0.0, height());

    // calculate base top normal
    let mut baseDelta: PVector = (base2 - base1);
    baseDelta.normalize();
    let normal: PVector = PVector::new(-baseDelta.y, baseDelta.x);

    // draw ellipse
    no_stroke();
    fill(255);
    ellipse(position.x, position.y, r * 2.0, r * 2.0);

    // move elipse
    position.add(&velocity);

    // normalized incidence vector
    let mut incidence: PVector = (velocity * -1.0);
    incidence.normalize();

    // detect and handle collision
    for i in 0..coords.len() {
        // check distance between ellipse and base top coordinates
        if position.dist(&coords[i]) < r {
            // calculate dot product of incident vector and base top normal
            let dot: f32 = incidence.dot(&normal);

            // calculate reflection vector
            // assign reflection vector to direction vector
            velocity.set(2.0 * normal.x * dot - incidence.x, 2.0 * normal.y * dot - incidence.y, 0.0);
            velocity.mult(speed);

            // draw base top normal at collision point
            stroke(255, 128, 0);
            line(position.x, position.y, position.x - normal.x * 100.0, position.y - normal.y * 100.0);
        }
    }

    // detect boundary collision
    // right
    if position.x > width() - r {
        position.x = width() - r;
        velocity.x *= -1.0;
    }
    // left
    if position.x < r {
        position.x = r;
        velocity.x *= -1.0;
    }
    // top
    if position.y < r {
        position.y = r;
        velocity.y *= -1.0;
        // randomize base top
        base1.y = random(height() - 100.0, height());
        base2.y = random(height() - 100.0, height());
        create_ground(base1, base2, &mut baseLength, &mut coords);
    }
}

// Calculate variables for the ground
fn create_ground(base1: PVector, base2: PVector, baseLength: &mut f32, coords: &mut Vec<PVector>) {
    // calculate length of base top
    *baseLength = base1.dist(&base2);

    // fill base top coordinate array
    *coords = vec![PVector::new(0.0, 0.0); ((*baseLength).ceil() as i32) as usize];
    for i in 0..coords.len() {
        coords[i] = PVector::new(0.0, 0.0);
        coords[i].x = base1.x + ((base2.x - base1.x) / *baseLength) * i as f32;
        coords[i].y = base1.y + ((base2.y - base1.y) / *baseLength) * i as f32;
    }
}
