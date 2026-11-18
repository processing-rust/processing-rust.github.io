/**
 * Penrose Tile L-System
 * by Geraldine Sarmiento.
 *
 * This example was based on Patrick Dwyer's L-System class.
 */
let mut ds: PenroseLSystem = PenroseLSystem::default();

fn setup() {
    size(640, 360);
    ds = PenroseLSystem::new();
    ds.l_system.simulate(4);
}

fn draw() {
    background(0);
    ds.render();
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
            startLength: 190.0,
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
struct PenroseLSystem {
    l_system: LSystem,  // what PenroseLSystem builds on (Java: extends LSystem)
    steps: i32,
    somestep: f32,
    ruleW: String,
    ruleX: String,
    ruleY: String,
    ruleZ: String,
}

impl PenroseLSystem {
    fn new() -> PenroseLSystem {
        let mut this = PenroseLSystem {
            l_system: LSystem::default(),
            steps: 0,
            somestep: 0.1,
            ruleW: String::new(),
            ruleX: String::new(),
            ruleY: String::new(),
            ruleZ: String::new(),
        };
        this.l_system.axiom = "[X]++[X]++[X]++[X]++[X]".to_string();
        this.ruleW = "YF++ZF4-XF[-YF4-WF]++".to_string();
        this.ruleX = "+YF--ZF[3-WF--XF]+".to_string();
        this.ruleY = "-WF++XF[+++YF++ZF]-".to_string();
        this.ruleZ = "--YF++++WF[+ZF++++XF]--XF".to_string();
        this.l_system.startLength = 460.0;
        this.l_system.theta = radians(36.0);
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
        translate(width() / 2.0, height() / 2.0);
        let mut pushes: i32 = 0;
        let mut repeats: i32 = 1;
        self.steps += 12;
        if self.steps > self.l_system.production.len() as i32 {
            self.steps = self.l_system.production.len() as i32;
        }

        for i in 0..self.steps {
            let step: char = self.l_system.production.chars().nth(i as usize).unwrap();
            if step == 'F' {
                stroke(255, 60);
                for j in 0..repeats {
                    line(0.0, 0.0, 0.0, -self.l_system.drawLength);
                    no_fill();
                    translate(0.0, -self.l_system.drawLength);
                }
                repeats = 1;
            } else if step == '+' {
                for j in 0..repeats {
                    rotate(self.l_system.theta);
                }
                repeats = 1;
            } else if step == '-' {
                for j in 0..repeats {
                    rotate(-self.l_system.theta);
                }
                repeats = 1;
            } else if step == '[' {
                pushes += 1;
                push_matrix();
            } else if step == ']' {
                pop_matrix();
                pushes -= 1;
            } else if step.is_ascii_digit() {
                repeats = step.to_digit(10).unwrap() as i32;
            }
        }

        // Unpush if we need too
        while pushes > 0 {
            pop_matrix();
            pushes -= 1;
        }
    }

    fn iterate(&mut self, prod_: &str, rule_: &str) -> String {
        let mut newProduction: String = "".to_string();
        for i in 0..prod_.len() {
            let step: char = self.l_system.production.chars().nth(i).unwrap();
            if step == 'W' {
                newProduction = format!("{}{}", newProduction, self.ruleW);
            } else if step == 'X' {
                newProduction = format!("{}{}", newProduction, self.ruleX);
            } else if step == 'Y' {
                newProduction = format!("{}{}", newProduction, self.ruleY);
            } else if step == 'Z' {
                newProduction = format!("{}{}", newProduction, self.ruleZ);
            } else {
                if step != 'F' {
                    newProduction = format!("{}{}", newProduction, step);
                }
            }
        }

        self.l_system.drawLength = self.l_system.drawLength * 0.5;
        self.l_system.generations += 1;
        return newProduction;
    }
}
