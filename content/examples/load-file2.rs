/**
 * LoadFile 2
 *
 * This example loads a data file about cars. Each element is separated
 * with a tab and corresponds to a different aspect of each car. The file stores
 * the miles per gallon, cylinders, displacement, etc., for more than 400 different
 * makes and models. Press a mouse button to advance to the next group of entries.
 */
let mut records: Vec<Record> = Vec::new();
let mut lines: Vec<String> = Vec::new();
let mut recordCount: i32 = 0;
let mut body: u64 = 0;
let num: i32 = 9;  // Display this many entries on each screen.
let mut startingEntry: i32 = 0;  // Display from this entry number

fn setup() {
    size(640, 360);
    fill(255);
    no_loop();

    body = load_font("TheSans-Plain-12.vlw");
    text_font(body);
    text_size(20.0);

    lines = load_strings("cars2.tsv");
    records = vec![Record::default(); lines.len()];
    for i in 0..lines.len() {
        let pieces = split(&lines[i], &TAB.to_string());  // Load data into array
        if pieces.len() == 9 {
            records[recordCount as usize] = Record::new(&pieces);
            recordCount += 1;
        }
    }
    if recordCount != records.len() as i32 {
        records = records[..recordCount as usize].to_vec();
    }
}

fn draw() {
    background(0);
    for i in 0..num {
        let thisEntry: i32 = startingEntry + i;
        if thisEntry < recordCount {
            text(&format!("{} > {}", thisEntry, records[thisEntry as usize].name), 20.0, (20 + i * 20) as f32);
        }
    }
}

fn mouse_pressed() {
    startingEntry += num;
    if startingEntry > records.len() as i32 {
        startingEntry = 0;  // go back to the beginning
    }
    redraw();
}

#[derive(Clone, Default)]
struct Record {
    name: String,
    mpg: f32,
    cylinders: i32,
    displacement: f32,
    horsepower: f32,
    weight: f32,
    acceleration: f32,
    year: i32,
    origin: f32,
}

impl Record {
    fn new(pieces: &[String]) -> Record {
        Record {
            name: pieces[0].clone(),
            mpg: pieces[1].trim().parse::<f32>().unwrap_or(0.0),
            cylinders: pieces[2].trim().parse::<i32>().unwrap_or(0),
            displacement: pieces[3].trim().parse::<f32>().unwrap_or(0.0),
            horsepower: pieces[4].trim().parse::<f32>().unwrap_or(0.0),
            weight: pieces[5].trim().parse::<f32>().unwrap_or(0.0),
            acceleration: pieces[6].trim().parse::<f32>().unwrap_or(0.0),
            year: pieces[7].trim().parse::<i32>().unwrap_or(0),
            origin: pieces[8].trim().parse::<f32>().unwrap_or(0.0),
        }
    }
}
