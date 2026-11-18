/**
 * Loading Tabular Data
 * by Daniel Shiffman.
 *
 * This example demonstrates how to use loadTable()
 * to retrieve data from a CSV file and make objects
 * from that data.
 *
 * Here is what the CSV looks like:
 *
x,y,diameter,name
160,103,43.19838,Happy
372,137,52.42526,Sad
273,235,61.14072,Joyous
121,179,44.758068,Melancholy
 */

// An Array of Bubble objects
let mut bubbles: Vec<Bubble> = Vec::new();
// A Table object
let mut table: Table = Default::default();

fn setup() {
    size(640, 360);
    load_data(&mut bubbles, &mut table);
}

fn draw() {
    background(255);
    // Display all bubbles
    for b in bubbles.iter_mut() {
        b.display();
        b.rollover(mouse_x(), mouse_y());
    }

    text_align(LEFT);
    fill(0);
    text("Click to add bubbles.", 10.0, height() - 10.0);
}

fn load_data(bubbles: &mut Vec<Bubble>, table: &mut Table) {
    // Load CSV file into a Table object
    // "header" option indicates the file has a header row
    *table = load_table("data.csv", "header");

    // The size of the array of Bubble objects is determined by the total number of rows in the CSV
    *bubbles = vec![Bubble::default(); table.get_row_count()];

    // You can access iterate over all the rows in a table
    let mut rowCount: i32 = 0;
    for row in table.rows() {
        // You can access the fields via their column name (or index)
        let x: f32 = row.get_float("x");
        let y: f32 = row.get_float("y");
        let d: f32 = row.get_float("diameter");
        let n: String = row.get_string("name");
        // Make a Bubble object out of the data read
        bubbles[rowCount as usize] = Bubble::new(x, y, d, &n);
        rowCount += 1;
    }
}

fn mouse_pressed() {
    // Create a new row
    let mut row = table.add_row_handle();
    // Set the values of that row
    row.set_float("x", mouse_x());
    row.set_float("y", mouse_y());
    row.set_float("diameter", random(40.0, 80.0));
    row.set_string("name", "Blah");

    // If the table has more than 10 rows
    if table.get_row_count() > 10 {
        // Delete the oldest row
        table.remove_row(0);
    }

    // Writing the CSV back to the same file
    save_table(&table, "data/data.csv");
    // And reloading it
    load_data(&mut bubbles, &mut table);
}

// A Bubble class

#[derive(Clone, Default)]
struct Bubble {
    x: f32,
    y: f32,
    diameter: f32,
    name: String,
    over: bool,
}

impl Bubble {
    // Create  the Bubble
    fn new(x_: f32, y_: f32, diameter_: f32, s: &str) -> Bubble {
        Bubble {
            x: x_,
            y: y_,
            diameter: diameter_,
            name: s.to_string(),
            over: false,
        }
    }

    // CHecking if mouse is over the Bubble
    fn rollover(&mut self, px: f32, py: f32) {
        let d: f32 = dist(px, py, self.x, self.y);
        if d < self.diameter / 2.0 {
            self.over = true;
        } else {
            self.over = false;
        }
    }

    // Display the Bubble
    fn display(&self) {
        stroke(0);
        stroke_weight(2.0);
        no_fill();
        ellipse(self.x, self.y, self.diameter, self.diameter);
        if self.over {
            fill(0);
            text_align(CENTER);
            text(&self.name, self.x, self.y + self.diameter / 2.0 + 20.0);
        }
    }
}
