/**
 * Koch Curve
 * by Daniel Shiffman.
 *
 * Renders a simple fractal, the Koch snowflake.
 * Each recursive level is drawn in sequence.
 */
let mut k: KochFractal = KochFractal::default();

fn setup() {
    size(640, 360);
    frame_rate(1.0);  // Animate slowly
    k = KochFractal::new();
}

fn draw() {
    background(0);
    // Draws the snowflake!
    k.render();
    // Iterate
    k.next_level();
    // Let's not do it more than 5 times. . .
    if k.get_count() > 5 {
        k.restart();
    }
}

// Koch Curve
// A class to manage the list of line segments in the snowflake pattern

#[derive(Clone, Default)]
struct KochFractal {
    start: PVector,
    end: PVector,
    lines: Vec<KochLine>,
    count: i32,
}

impl KochFractal {
    // A PVector for the start
    // A PVector for the end
    // A list to keep track of all the lines
    fn new() -> KochFractal {
        let mut this = KochFractal {
            start: PVector::new(0.0, height() - 20.0),
            end: PVector::new(width(), height() - 20.0),
            lines: Vec::new(),
            count: 0,
        };
        this.restart();
        this
    }

    fn next_level(&mut self) {
        // For every line that is in the arraylist
        // create 4 more lines in a new arraylist
        self.lines = self.iterate(&self.lines);
        self.count += 1;
    }

    fn restart(&mut self) {
        self.count = 0;  // Reset count
        self.lines.clear();  // Empty the array list
        self.lines.push(KochLine::new(self.start, self.end));  // Add the initial line (from one end PVector to the other)
    }

    fn get_count(&self) -> i32 {
        return self.count;
    }

    // This is easy, just draw all the lines
    fn render(&mut self) {
        for l in self.lines.iter() {
            l.display();
        }
    }

    // This is where the **MAGIC** happens
    // Step 1: Create an empty arraylist
    // Step 2: For every line currently in the arraylist
    //   - calculate 4 line segments based on Koch algorithm
    //   - add all 4 line segments into the new arraylist
    // Step 3: Return the new arraylist and it becomes the list of line segments for the structure
    // As we do this over and over again, each line gets broken into 4 lines, which gets broken into 4 lines, and so on. . .
    fn iterate(&self, before: &[KochLine]) -> Vec<KochLine> {
        let mut now = Vec::new();  // Create emtpy list
        for l in before.iter() {
            // Calculate 5 koch PVectors (done for us by the line object)
            let a: PVector = l.start();
            let b: PVector = l.kochleft();
            let c: PVector = l.kochmiddle();
            let d: PVector = l.kochright();
            let e: PVector = l.end();
            // Make line segments between all the PVectors and add them
            now.push(KochLine::new(a, b));
            now.push(KochLine::new(b, c));
            now.push(KochLine::new(c, d));
            now.push(KochLine::new(d, e));
        }
        return now;
    }
}

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com

// Koch Curve
// A class to describe one line segment in the fractal
// Includes methods to calculate midPVectors along the line according to the Koch algorithm

#[derive(Clone, Default)]
struct KochLine {
    a: PVector,
    b: PVector,
}

impl KochLine {
    // Two PVectors,
    // a is the "left" PVector and
    // b is the "right PVector
    fn new(start: PVector, end: PVector) -> KochLine {
        KochLine {
            a: start.copy(),
            b: end.copy(),
        }
    }

    fn display(&self) {
        stroke(255);
        line(self.a.x, self.a.y, self.b.x, self.b.y);
    }

    fn start(&self) -> PVector {
        return self.a.copy();
    }

    fn end(&self) -> PVector {
        return self.b.copy();
    }

    // This is easy, just 1/3 of the way
    fn kochleft(&self) -> PVector {
        let mut v: PVector = (self.b - self.a);
        v.div(3.0);
        v.add(&self.a);
        return v;
    }

    // More complicated, have to use a little trig to figure out where this PVector is!
    fn kochmiddle(&self) -> PVector {
        let mut v: PVector = (self.b - self.a);
        v.div(3.0);

        let mut p: PVector = self.a.copy();
        p.add(&v);

        v.rotate(-radians(60.0));
        p.add(&v);

        return p;
    }

    // Easy, just 2/3 of the way
    fn kochright(&self) -> PVector {
        let mut v: PVector = (self.a - self.b);
        v.div(3.0);
        v.add(&self.b);
        return v;
    }
}
