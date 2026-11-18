/**
 * Array Objects.
 *
 * Demonstrates the syntax for creating an array of custom objects.
 */
let mut unit: i32 = 40;
let mut count: i32 = 0;
let mut mods: Vec<Module> = Vec::new();

fn setup() {
    size(640, 360);
    no_stroke();
    let wideCount: i32 = (width() / unit as f32) as i32;
    let highCount: i32 = (height() / unit as f32) as i32;
    count = wideCount * highCount;
    mods = vec![Module::default(); count as usize];

    let mut index: i32 = 0;
    for y in 0..highCount {
        for x in 0..wideCount {
            mods[index as usize] = Module::new(x * unit, y * unit, unit / 2, unit / 2, random(0.05, 0.8), unit);
            index += 1;
        }
    }
}

fn draw() {
    background(0);
    for mod_ in mods.iter_mut() {
        mod_.update();
        mod_.display();
    }
}

#[derive(Clone, Default)]
struct Module {
    xOffset: i32,
    yOffset: i32,
    x: f32,
    y: f32,
    unit: i32,
    xDirection: i32,
    yDirection: i32,
    speed: f32,
}

impl Module {
    // Contructor
    fn new(xOffsetTemp: i32, yOffsetTemp: i32, xTemp: i32, yTemp: i32, speedTemp: f32, tempUnit: i32) -> Module {
        Module {
            xOffset: xOffsetTemp,
            yOffset: yOffsetTemp,
            x: xTemp as f32,
            y: yTemp as f32,
            unit: tempUnit,
            xDirection: 1,
            yDirection: 1,
            speed: speedTemp,
        }
    }

    // Custom method for updating the variables
    fn update(&mut self) {
        self.x = self.x + (self.speed * self.xDirection as f32);
        if self.x >= self.unit as f32 || self.x <= 0.0 {
            self.xDirection *= -1;
            self.x = self.x + ((1 * self.xDirection)) as f32;
            self.y = self.y + ((1 * self.yDirection)) as f32;
        }
        if self.y >= self.unit as f32 || self.y <= 0.0 {
            self.yDirection *= -1;
            self.y = self.y + ((1 * self.yDirection)) as f32;
        }
    }

    // Custom method for drawing the object
    fn display(&self) {
        fill(255);
        ellipse(self.xOffset as f32 + self.x, self.yOffset as f32 + self.y, 6.0, 6.0);
    }
}
