/**
 * Variable Scope.
 *
 * Variables have a global or local "scope".
 * For example, variables declared within either the
 * setup() or draw() functions may be only used in these
 * functions. Global variables, variables declared outside
 * of setup() and draw(), may be used anywhere within the program.
 * If a local variable is declared with the same name as a
 * global variable, the program will use the local variable to make
 * its calculations within the current scope. Variables are localized
 * within each block, the space between a { and }.
 */
let mut a: i32 = 80;  // Create a global variable "a"

fn setup() {
    size(640, 360);
    background(0);
    stroke(255);
    no_loop();
}

fn draw() {
    // Draw a line using the global variable "a"
    line(a as f32, 0.0, a as f32, height());

    // Create a new variable "a" local to the for() statement
    for a in (120..200).step_by(2) {
        line(a as f32, 0.0, a as f32, height());
    }

    // Create a new variable "a" local to the draw() function
    let mut a: i32 = 300;
    // Draw a line using the new local variable "a"
    line(a as f32, 0.0, a as f32, height());

    // Make a call to the custom function drawAnotherLine()
    draw_another_line();

    // Make a call to the custom function setYetAnotherLine()
    draw_yet_another_line(a);
}

fn draw_another_line() {
    // Create a new variable "a" local to this method
    let a: i32 = 320;
    // Draw a line using the local variable "a"
    line(a as f32, 0.0, a as f32, height());
}

fn draw_yet_another_line(a: i32) {
    // Because no new local variable "a" is set,
    // this line draws using the original global
    // variable "a", which is set to the value 80.
    line((a + 2) as f32, 0.0, (a + 2) as f32, height());
}
