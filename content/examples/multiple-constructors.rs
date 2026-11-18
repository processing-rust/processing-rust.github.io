/**
 * Multiple constructors
 *
 * A class can have multiple constructors that assign the fields in different ways.
 * Sometimes it's beneficial to specify every aspect of an object's data by assigning
 * parameters to the fields, but other times it might be appropriate to define only
 * one or a few.
 */
let mut sp1: Spot = Spot::default();
let mut sp2: Spot = Spot::default();

fn setup() {
    size(640, 360);
    background(204);
    no_loop();
    // Run the constructor without parameters
    sp1 = Spot::new();
    // Run the constructor with three parameters
    sp2 = Spot::with(width() * 0.5, height() * 0.5, 120.0);
}

fn draw() {
    sp1.display();
    sp2.display();
}

#[derive(Clone, Default)]
struct Spot {
    x: f32,
    y: f32,
    radius: f32,
}

impl Spot {
    // First version of the Spot constructor;
    // the fields are assigned default values
    fn new() -> Spot {
        Spot {
            x: width() * 0.25,
            y: height() * 0.5,
            radius: 40.0,
        }
    }

    // Second version of the Spot constructor;
    // the fields are assigned with parameters.
    // Rust has no overloading, so this one gets its own name.
    fn with(xpos: f32, ypos: f32, r: f32) -> Spot {
        Spot {
            x: xpos,
            y: ypos,
            radius: r,
        }
    }

    fn display(&self) {
        ellipse(self.x, self.y, self.radius * 2.0, self.radius * 2.0);
    }
}
