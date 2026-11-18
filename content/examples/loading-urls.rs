/**
 * Loading URLs.
 *
 * Click on the button to open a URL in a browser.
 */
let mut overButton: bool = false;

fn setup() {
    size(640, 360);
}

fn draw() {
    background(204);

    if overButton == true {
        fill(255);
    } else {
        no_fill();
    }
    rect(105.0, 60.0, 75.0, 75.0);
    line(135.0, 105.0, 155.0, 85.0);
    line(140.0, 85.0, 155.0, 85.0);
    line(155.0, 85.0, 155.0, 100.0);
}

fn mouse_pressed() {
    if overButton {
        link("http://www.processing.org");
    }
}

fn mouse_moved() {
    check_buttons(&mut overButton);
}

fn mouse_dragged() {
    check_buttons(&mut overButton);
}

fn check_buttons(overButton: &mut bool) {
    if mouse_x() > 105.0 && mouse_x() < 180.0 && mouse_y() > 60.0 && mouse_y() < 135.0 {
        *overButton = true;
    } else {
        *overButton = false;
    }
}
