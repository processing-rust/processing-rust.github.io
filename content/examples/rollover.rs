/**
 * Rollover.
 *
 * Roll over the colored squares in the center of the image
 * to change the color of the outside rectangle.
 */
let mut rectX: i32 = 0;
let mut rectY: i32 = 0;  // Position of square button
let mut circleX: i32 = 0;
let mut circleY: i32 = 0;  // Position of circle button
const RECTSIZE: i32 = 90;  // Diameter of rect
const CIRCLESIZE: i32 = 93;  // Diameter of circle

let mut rectColor: Color = color(0);
let mut circleColor: Color = color(0);
let mut baseColor: Color = color(0);

let mut rectOver: bool = false;
let mut circleOver: bool = false;

fn setup() {
    size(640, 360);
    rectColor = color(0);
    circleColor = color(255);
    baseColor = color(102);
    circleX = (width() / 2.0 + (CIRCLESIZE / 2) as f32 + 10.0) as i32;
    circleY = (height() / 2.0) as i32;
    rectX = (width() / 2.0 - RECTSIZE as f32 - 10.0) as i32;
    rectY = (height() / 2.0 - (RECTSIZE / 2) as f32) as i32;
    ellipse_mode(CENTER);
}

fn draw() {
    update(mouse_x() as i32, mouse_y() as i32, &mut circleOver, circleX, circleY, &mut rectOver, rectX, rectY);

    no_stroke();
    if rectOver {
        background(rectColor);
    } else if circleOver {
        background(circleColor);
    } else {
        background(baseColor);
    }

    stroke(255);
    fill(rectColor);
    rect(rectX as f32, rectY as f32, RECTSIZE as f32, RECTSIZE as f32);
    stroke(0);
    fill(circleColor);
    ellipse(circleX as f32, circleY as f32, CIRCLESIZE as f32, CIRCLESIZE as f32);
}

fn update(x: i32, y: i32, circleOver: &mut bool, circleX: i32, circleY: i32, rectOver: &mut bool, rectX: i32, rectY: i32) {
    if over_circle(circleX, circleY, CIRCLESIZE) {
        *circleOver = true;
        *rectOver = false;
    } else if over_rect(rectX, rectY, RECTSIZE, RECTSIZE) {
        *rectOver = true;
        *circleOver = false;
    } else {
        *rectOver = false;
        *circleOver = *rectOver;
    }
}

fn over_rect(x: i32, y: i32, width: i32, height: i32) -> bool {
    if mouse_x() >= x as f32 && mouse_x() <= (x + width) as f32 && mouse_y() >= y as f32 && mouse_y() <= (y + height) as f32 {
        return true;
    } else {
        return false;
    }
}

fn over_circle(x: i32, y: i32, diameter: i32) -> bool {
    let disX: f32 = x as f32 - mouse_x();
    let disY: f32 = y as f32 - mouse_y();
    if sqrt(disX * disX + disY * disY) < (diameter / 2) as f32 {
        return true;
    } else {
        return false;
    }
}
