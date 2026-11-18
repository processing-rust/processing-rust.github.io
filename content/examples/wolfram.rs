/**
 * Wolfram Cellular Automata
 * by Daniel Shiffman.
 *
 * Simple demonstration of a Wolfram's 1-dimensional
 * cellular automata. When the system reaches bottom
 * of the window, it restarts with a new ruleset.
 * Mouse click restarts as well.
 */
let mut ca: CA = CA::default();  // An instance object to the cellular automata

fn setup() {
    size(640, 360);
    let ruleset = vec![0, 1, 0, 1, 1, 0, 1, 0];  // An initial rule system
    ca = CA::new(&ruleset);  // Initialize CA
    background(0);
}

fn draw() {
    ca.render();  // Draw the CA
    ca.generate();  // Generate the next level

    // If we're done, clear the screen,
    // pick a new ruleset and restart
    if ca.finished() {
        background(0);
        ca.randomize();
        ca.restart();
    }
}

fn mouse_pressed() {
    background(0);
    ca.randomize();
    ca.restart();
}

#[derive(Clone, Default)]
struct CA {
    cells: Vec<i32>,
    generation: i32,
    scl: i32,
    rules: Vec<i32>,
}

impl CA {
    // An array of 0s and 1s
    // How many generations?
    // How many pixels wide/high is each cell?
    // Array to store the rules, for example {0,1,1,0,1,1,0,1}
    fn new(r: &[i32]) -> CA {
        let mut this = CA {
            cells: Vec::new(),
            generation: 0,
            scl: 1,
            rules: r.to_vec(),
        };
        this.cells = vec![0; (width() / this.scl as f32) as usize];
        this.restart();
        this
    }

    // Set the rules of the CA
    fn set_rules(&mut self, r: &[i32]) {
        self.rules = r.to_vec();
    }

    // Make a random ruleset
    fn randomize(&mut self) {
        for i in 0..8 {
            self.rules[i as usize] = random(2.0) as i32;
        }
    }

    // Reset to generation 0
    fn restart(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = 0;
        }
        // We arbitrarily start with just the middle
        // cell having a state of "1"
        let last = self.cells.len() / 2;
        self.cells[last] = 1;
        self.generation = 0;
    }

    // The process of creating the new generation
    fn generate(&mut self) {
        // First we create an empty array for the new values
        let mut nextgen = vec![0; self.cells.len()];
        // For every spot, determine new state by examing current
        // state, and neighbor states
        // Ignore edges that only have one neighor
        for i in 1..self.cells.len() - 1 {
            let left: i32 = self.cells[i - 1];  // Left neighbor state
            let me: i32 = self.cells[i];  // Current state
            let right: i32 = self.cells[i + 1];  // Right neighbor state
            // Compute next generation state based on ruleset
            nextgen[i] = self.execute_rules(left, me, right);
        }
        // Copy the array into current value
        for i in 1..self.cells.len() - 1 {
            self.cells[i] = nextgen[i];
        }
        //cells = (int[]) nextgen.clone();
        self.generation += 1;
    }

    // This is the easy part, just draw the cells,
    // fill 255 for '1', fill 0 for '0'
    fn render(&self) {
        for i in 0..self.cells.len() {
            if self.cells[i] == 1 {
                fill(255);
            } else {
                fill(0);
            }
            no_stroke();
            rect((i as i32 * self.scl) as f32, (self.generation * self.scl) as f32, self.scl as f32, self.scl as f32);
        }
    }

    // Implementing the Wolfram rules
    // Could be improved and made more concise,
    // but here we can explicitly see what is going on for each case
    fn execute_rules(&self, a: i32, b: i32, c: i32) -> i32 {
        if a == 1 && b == 1 && c == 1 {
            return self.rules[0];
        }
        if a == 1 && b == 1 && c == 0 {
            return self.rules[1];
        }
        if a == 1 && b == 0 && c == 1 {
            return self.rules[2];
        }
        if a == 1 && b == 0 && c == 0 {
            return self.rules[3];
        }
        if a == 0 && b == 1 && c == 1 {
            return self.rules[4];
        }
        if a == 0 && b == 1 && c == 0 {
            return self.rules[5];
        }
        if a == 0 && b == 0 && c == 1 {
            return self.rules[6];
        }
        if a == 0 && b == 0 && c == 0 {
            return self.rules[7];
        }
        return 0;
    }

    // The CA is done if it reaches the bottom of the screen
    fn finished(&self) -> bool {
        if (self.generation as f32) > height() / self.scl as f32 {
            return true;
        } else {
            return false;
        }
    }
}
