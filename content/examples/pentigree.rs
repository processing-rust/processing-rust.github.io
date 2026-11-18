/**
 * Pentigree L-System
 * by Geraldine Sarmiento.
 *
 * This example was based on Patrick Dwyer's L-System class.
 */
let mut ps: PentigreeLSystem = PentigreeLSystem::default();

fn setup() {
    size(640, 360);
    ps = PentigreeLSystem::new();
    ps.l_system.simulate(3);
}

fn draw() {
    background(0);
    ps.render();
}

#[derive(Clone, Default)]
struct LSystem {
    steps: i32,
    axiom: String,
    rule: String,
    production: String,
    startLength: f32,
    drawLength: f32,
    theta: f32,
    generations: i32,
}

impl LSystem {
    fn new() -> LSystem {
        let mut this = LSystem {
            steps: 0,
            axiom: "F".to_string(),
            rule: "F+F-F".to_string(),
            production: String::new(),
            startLength: 90.0,
            drawLength: 0.0,
            theta: radians(120.0),
            generations: 0,
        };
        this.reset();
        this
    }

    fn reset(&mut self) {
        self.production = self.axiom.clone();
        self.drawLength = self.startLength;
        self.generations = 0;
    }

    fn get_age(&self) -> i32 {
        return self.generations;
    }

    fn render(&mut self) {
        translate(width() / 2.0, height() / 2.0);
        self.steps += 5;
        if self.steps > self.production.len() as i32 {
            self.steps = self.production.len() as i32;
        }
        for i in 0..self.steps {
            let step: char = self.production.chars().nth(i as usize).unwrap();
            if step == 'F' {
                rect(0.0, 0.0, -self.drawLength, -self.drawLength);
                no_fill();
                translate(0.0, -self.drawLength);
            } else if step == '+' {
                rotate(self.theta);
            } else if step == '-' {
                rotate(-self.theta);
            } else if step == '[' {
                push_matrix();
            } else if step == ']' {
                pop_matrix();
            }
        }
    }

    fn simulate(&mut self, gen_: i32) {
        while self.get_age() < gen_ {
            self.production = self.iterate(&self.production.clone(), &self.rule.clone());
        }
    }

    fn iterate(&mut self, prod_: &str, rule_: &str) -> String {
        self.drawLength = self.drawLength * 0.6;
        self.generations += 1;
        let mut newProduction: String = prod_.to_string();
        newProduction = newProduction.replace("F", &rule_);
        return newProduction;
    }
}

#[derive(Clone, Default)]
struct PentigreeLSystem {
    l_system: LSystem,  // what PentigreeLSystem builds on (Java: extends LSystem)
    steps: i32,
    somestep: f32,
    xoff: f32,
}

impl PentigreeLSystem {
    fn new() -> PentigreeLSystem {
        let mut this = PentigreeLSystem {
            l_system: LSystem::default(),
            steps: 0,
            somestep: 0.1,
            xoff: 0.01,
        };
        this.l_system.axiom = "F-F-F-F-F".to_string();
        this.l_system.rule = "F-F++F+F-F-F".to_string();
        this.l_system.startLength = 60.0;
        this.l_system.theta = radians(72.0);
        this.reset();
        this
    }

    fn use_rule(&mut self, r_: &str) {
        self.l_system.rule = r_.to_string();
    }

    fn use_axiom(&mut self, a_: &str) {
        self.l_system.axiom = a_.to_string();
    }

    fn use_length(&mut self, l_: f32) {
        self.l_system.startLength = l_;
    }

    fn use_theta(&mut self, t_: f32) {
        self.l_system.theta = radians(t_);
    }

    fn reset(&mut self) {
        self.l_system.production = self.l_system.axiom.clone();
        self.l_system.drawLength = self.l_system.startLength;
        self.l_system.generations = 0;
    }

    fn get_age(&self) -> i32 {
        return self.l_system.generations;
    }

    fn render(&mut self) {
        translate(width() / 4.0, height() / 2.0);
        self.steps += 3;
        if self.steps > self.l_system.production.len() as i32 {
            self.steps = self.l_system.production.len() as i32;
        }

        for i in 0..self.steps {
            let step: char = self.l_system.production.chars().nth(i as usize).unwrap();
            if step == 'F' {
                no_fill();
                stroke(255);
                line(0.0, 0.0, 0.0, -self.l_system.drawLength);
                translate(0.0, -self.l_system.drawLength);
            } else if step == '+' {
                rotate(self.l_system.theta);
            } else if step == '-' {
                rotate(-self.l_system.theta);
            } else if step == '[' {
                push_matrix();
            } else if step == ']' {
                pop_matrix();
            }
        }
    }
}
