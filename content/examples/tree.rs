/**
 * Recursive Tree
 * by Daniel Shiffman.
 *
 * Renders a simple tree-like structure via recursion.
 * The branching angle is calculated as a function of
 * the horizontal mouse location. Move the mouse left
 * and right to change the angle.
 */
let mut theta: f32 = 0.0;

fn setup() {
    size(640, 360);
}

fn draw() {
    background(0);
    frame_rate(30.0);
    stroke(255);
    // Let's pick an angle 0 to 90 degrees based on the mouse position
    let a: f32 = (mouse_x() / width() as f32) * 90.0;
    // Convert it to radians
    theta = radians(a);
    // Start the tree from the bottom of the screen
    translate(width() / 2.0, height());
    // Draw a line 120 pixels
    line(0.0, 0.0, 0.0, -120.0);
    // Move to the end of that line
    translate(0.0, -120.0);
    // Start the recursive branching!
    branch(120.0, theta);
}

fn branch(mut h: f32, theta: f32) {
    // Each branch will be 2/3rds the size of the previous one
    h *= 0.66;

    // All recursive functions must have an exit condition!!!!
    // Here, ours is when the length of the branch is 2 pixels or less
    if h > 2.0 {
        push_matrix();  // Save the current state of transformation (i.e. where are we now)
        rotate(theta);  // Rotate by theta
        line(0.0, 0.0, 0.0, -h);  // Draw the branch
        translate(0.0, -h);  // Move to the end of the branch
        branch(h, theta);  // Ok, now call myself to draw two new branches!!
        pop_matrix();  // Whenever we get back here, we "pop" in order to restore the previous matrix state

        // Repeat the same thing, only branch off to the "left" this time!
        push_matrix();
        rotate(-theta);
        line(0.0, 0.0, 0.0, -h);
        translate(0.0, -h);
        branch(h, theta);
        pop_matrix();
    }
}
