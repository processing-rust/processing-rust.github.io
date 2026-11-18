/**
 * Loading JSON Data
 * by Daniel Shiffman.
 *
 * This example demonstrates how to use loadJSON()
 * to retrieve data from a JSON file and make objects
 * from that data.
 *
 * Here is what the JSON looks like (partial):
 *
{
"bubbles": [
{
"position": {
"x": 160,
"y": 103
},
"diameter": 43.19838,
"label": "Happy"
},
{
"position": {
"x": 372,
"y": 137
},
"diameter": 52.42526,
"label": "Sad"
}
]
}
 */

// An Array of Bubble objects
let mut bubbles: Vec<Bubble> = Vec::new();
// A JSON object
let mut json: Json = Default::default();

fn setup() {
    size(640, 360);
    load_data(&mut bubbles, &mut json);
}

fn draw() {
    background(255);
    // Display all bubbles
    for b in bubbles.iter_mut() {
        b.display();
        b.rollover(mouse_x(), mouse_y());
    }
    //
    text_align(LEFT);
    fill(0);
    text("Click to add bubbles.", 10.0, height() - 10.0);
}
fn load_data(bubbles: &mut Vec<Bubble>, json: &mut Json) {
    // Load JSON file
    // Temporary full path until path problem resolved.
    *json = load_json_object("data.json");

    let bubbleData = json.get_json_array("bubbles");

    // The size of the array of Bubble objects is determined by the total XML elements named "bubble"
    *bubbles = vec![Bubble::default(); bubbleData.size()];

    for i in 0..bubbleData.size() {
        // Get each object in the array
        let bubble = bubbleData.get_json_object(i);
        // Get a position object
        let position = bubble.get_json_object("position");
        // Get x,y from position
        let x: i32 = position.get_int("x");
        let y: i32 = position.get_int("y");

        // Get diamter and label
        let diameter: f32 = bubble.get_float("diameter");
        let label: String = bubble.get_string("label");

        // Put object in array
        bubbles[i as usize] = Bubble::new(x as f32, y as f32, diameter, &label);
    }
}

fn mouse_pressed() {
    // Create a new JSON bubble object
    let mut newBubble = Json::object();

    // Create a new JSON position object
    let mut position = Json::object();
    position.set_int("x", mouse_x() as i32);
    position.set_int("y", mouse_y() as i32);

    // Add position to bubble
    newBubble.set_json_object("position", position);

    // Add diamater and label to bubble
    newBubble.set_float("diameter", random(40.0, 80.0));
    newBubble.set_string("label", "New label");

    // Append the new JSON bubble object to the array
    // get_mut: the array inside json, to change it in place
    let bubbleData = json.get_mut("bubbles");
    bubbleData.append(newBubble);

    if bubbleData.size() > 10 {
        bubbleData.remove_at(0);
    }

    // Save new data
    save_json_object(&json, "data/data.json");
    load_data(&mut bubbles, &mut json);
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
