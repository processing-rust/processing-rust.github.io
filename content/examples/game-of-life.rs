/**
 * Game of Life
 * by Joan Soler-Adillon.
 *
 * Press SPACE BAR to pause and change the cell's values
 * with the mouse. On pause, click to activate/deactivate
 * cells. Press 'R' to randomly reset the cells' grid.
 * Press 'C' to clear the cells' grid. The original Game
 * of Life was created by John Conway in 1970.
 */

// Size of cells
const CELLSIZE: i32 = 5;

// How likely for a cell to be alive at start (in percentage)
let probabilityOfAliveAtStart: f32 = 15.0;

// Variables for timer
let interval: i32 = 100;
let mut lastRecordedTime: i32 = 0;

// Colors for active/inactive cells
let alive: Color = color(0, 200, 0);
let dead: Color = color(0);

// Array of cells
let mut cells: Vec<Vec<i32>> = Vec::new();
// Buffer to record the state of the cells and use this
// while changing the others in the interations
let mut cellsBuffer: Vec<Vec<i32>> = Vec::new();

// Pause
let mut pause: bool = false;

fn setup() {
    size(640, 360);

    // Instantiate arrays
    cells = vec![vec![0; (height() / CELLSIZE as f32) as usize]; (width() / CELLSIZE as f32) as usize];
    cellsBuffer = vec![vec![0; (height() / CELLSIZE as f32) as usize]; (width() / CELLSIZE as f32) as usize];

    // This stroke will draw the background grid
    stroke(48);

    no_smooth();

    // Initialization of cells
    for x in 0..(width() / CELLSIZE as f32) as i32 {
        for y in 0..(height() / CELLSIZE as f32) as i32 {
            let mut state: f32 = random(100.0);
            if state > probabilityOfAliveAtStart {
                state = 0.0;
            } else {
                state = 1.0;
            }
            cells[x as usize][y as usize] = state as i32;  // Save state of each cell
        }
    }
    // Fill in black in case cells don't cover all the windows
    background(0);
}

fn draw() {
    //Draw grid
    for x in 0..(width() / CELLSIZE as f32) as i32 {
        for y in 0..(height() / CELLSIZE as f32) as i32 {
            if cells[x as usize][y as usize] == 1 {
                fill(alive);  // If alive
            } else {
                fill(dead);  // If dead
            }
            rect((x * CELLSIZE) as f32, (y * CELLSIZE) as f32, CELLSIZE as f32, CELLSIZE as f32);
        }
    }
    // Iterate if timer ticks
    if millis() as i32 - lastRecordedTime > interval {
        if !pause {
            iteration(&mut cells, &mut cellsBuffer);
            lastRecordedTime = millis() as i32;
        }
    }

    // Create  new cells manually on pause
    if pause && mouse_pressed_global() {
        // Map and avoid out of bound errors
        let mut xCellOver: i32 = (map(mouse_x(), 0.0, width(), 0.0, width() / CELLSIZE as f32)) as i32;
        xCellOver = (constrain(xCellOver as f32, 0.0, width() / CELLSIZE as f32 - 1.0)) as i32;
        let mut yCellOver: i32 = (map(mouse_y(), 0.0, height(), 0.0, height() / CELLSIZE as f32)) as i32;
        yCellOver = (constrain(yCellOver as f32, 0.0, height() / CELLSIZE as f32 - 1.0)) as i32;

        // Check against cells in buffer
        if cellsBuffer[xCellOver as usize][yCellOver as usize] == 1 {
            // Cell is alive
            cells[xCellOver as usize][yCellOver as usize] = 0;  // Kill
            fill(dead);  // Fill with kill color
        } else {
            // Cell is dead
            cells[xCellOver as usize][yCellOver as usize] = 1;  // Make alive
            fill(alive);  // Fill alive color
        }
    } else if pause && !mouse_pressed_global() {
        // And then save to buffer once mouse goes up
        // Save cells to buffer (so we opeate with one array keeping the other intact)
        for x in 0..(width() / CELLSIZE as f32) as i32 {
            for y in 0..(height() / CELLSIZE as f32) as i32 {
                cellsBuffer[x as usize][y as usize] = cells[x as usize][y as usize];
            }
        }
    }
}

fn iteration(cells: &mut Vec<Vec<i32>>, cellsBuffer: &mut Vec<Vec<i32>>) {
    // When the clock ticks
    // Save cells to buffer (so we opeate with one array keeping the other intact)
    for x in 0..(width() / CELLSIZE as f32) as i32 {
        for y in 0..(height() / CELLSIZE as f32) as i32 {
            cellsBuffer[x as usize][y as usize] = cells[x as usize][y as usize];
        }
    }

    // Visit each cell:
    for x in 0..(width() / CELLSIZE as f32) as i32 {
        for y in 0..(height() / CELLSIZE as f32) as i32 {
            // And visit all the neighbours of each cell
            let mut neighbours: i32 = 0;  // We'll count the neighbours
            for xx in x - 1..=x + 1 {
                for yy in y - 1..=y + 1 {
                    if ((xx >= 0) && ((xx as f32) < width() / CELLSIZE as f32)) && ((yy >= 0) && ((yy as f32) < height() / CELLSIZE as f32)) {
                        // Make sure you are not out of bounds
                        if !((xx == x) && (yy == y)) {
                            // Make sure to to check against self
                            if cellsBuffer[xx as usize][yy as usize] == 1 {
                                neighbours += 1;  // Check alive neighbours and count them
                            }
                        }  // End of if
                    }  // End of if
                }  // End of yy loop
            }  //End of xx loop
            // We've checked the neigbours: apply rules!
            if cellsBuffer[x as usize][y as usize] == 1 {
                // The cell is alive: kill it if necessary
                if neighbours < 2 || neighbours > 3 {
                    cells[x as usize][y as usize] = 0;  // Die unless it has 2 or 3 neighbours
                }
            } else {
                // The cell is dead: make it live if necessary
                if neighbours == 3 {
                    cells[x as usize][y as usize] = 1;  // Only if it has 3 neighbours
                }
            }  // End of if
        }  // End of y loop
    }  // End of x loop
}  // End of function

fn key_pressed() {
    if key() == 'r' || key() == 'R' {
        // Restart: reinitialization of cells
        for x in 0..(width() / CELLSIZE as f32) as i32 {
            for y in 0..(height() / CELLSIZE as f32) as i32 {
                let mut state: f32 = random(100.0);
                if state > probabilityOfAliveAtStart {
                    state = 0.0;
                } else {
                    state = 1.0;
                }
                cells[x as usize][y as usize] = state as i32;  // Save state of each cell
            }
        }
    }
    if key() == ' ' {
        // On/off of pause
        pause = !pause;
    }
    if key() == 'c' || key() == 'C' {
        // Clear all
        for x in 0..(width() / CELLSIZE as f32) as i32 {
            for y in 0..(height() / CELLSIZE as f32) as i32 {
                cells[x as usize][y as usize] = 0;  // Save all to zero
            }
        }
    }
}
