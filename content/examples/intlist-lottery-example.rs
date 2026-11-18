/**
 * IntList Lottery example
 * by Daniel Shiffman.
 *
 * This example demonstrates an IntList can be used to store a list of numbers.
 * While an array of integers serves a similar purpose it is of fixed size.  The
 * An IntList can easily have values added or deleted and it can also be
 * shuffled and sorted.  For lists of floats or Strings, you can use FloatList
 * and StringList.  For lists of objects, use ArrayList.
 *
 * In this example, three lists of integers are created.  One is a pool of numbers
 * that is shuffled and picked randomly from.  One is the list of "picked" numbers.
 * And one is a lottery "ticket" which includes 5 numbers that are trying to be matched.
 */

// Three lists of integers
let mut lottery: IntList = Default::default();
let mut results: IntList = Default::default();
let mut ticket: IntList = Default::default();

fn setup() {
    size(640, 360);
    frame_rate(30.0);
    // Create empy lists
    lottery = IntList::new();
    results = IntList::new();
    ticket = IntList::new();

    // Add 20 integers in order to the lottery list
    for i in 0..20 {
        lottery.append(i);
    }

    // Pick five numbers from the lottery list to go into the Ticket list
    for i in 0..5 {
        let index: i32 = (random(lottery.size() as f32)) as i32;
        ticket.append(lottery.get(index as usize));
    }
}

fn draw() {
    background(51);

    // The shuffle() method randomly shuffles the order of the values in the list
    lottery.shuffle();

    // Call a method that will display the integers in the list at an x,y location
    show_list(&lottery, 16.0, 48.0);
    show_list(&results, 16.0, 100.0);
    show_list(&ticket, 16.0, 140.0);

    // This loop checks if the picked numbers (results)
    // match the ticket numbers
    for i in 0..results.size() {
        // Are the integers equal?
        if results.get(i) == ticket.get(i) {
            fill(0, 255, 0, 100);  // if so green
        } else {
            fill(255, 0, 0, 100);  // if not red
        }
        ellipse((16 + i * 32) as f32, 140.0, 24.0, 24.0);
    }

    // One every 30 frames we pick a new lottery number to go in results
    if frame_count() % 30 == 0 {
        if results.size() < 5 {
            // Get the first value in the lottery list and remove it
            let val: i32 = lottery.remove(0);
            // Put it in the results
            results.append(val);
        } else {
            // Ok we picked five numbers, let's reset
            for i in 0..results.size() {
                // Put the picked results back into the lottery
                lottery.append(results.get(i));
            }
            // Clear the results and start over
            results.clear();
        }
    }
}

// Draw a list of numbers starting at an x,y location
fn show_list(list: &IntList, x: f32, y: f32) {
    for i in 0..list.size() {
        // Use get() to pull a value from the list at the specified index
        let val: i32 = list.get(i);
        stroke(255);
        no_fill();
        ellipse(x + (i * 32) as f32, y, 24.0, 24.0);
        text_align(CENTER);
        fill(255);
        text(&val.to_string(), x + (i * 32) as f32, y + 6.0);
    }
}
