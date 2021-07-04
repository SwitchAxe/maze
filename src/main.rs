use std::{thread, time};
use rand::Rng;

const ROWS: usize = 50;
const COLS: usize = 50;

#[derive(Clone)]
#[derive(Copy)]
struct Cell {
    old_state: bool,
    new_state: bool,
}

fn main() {
    let interval = time::Duration::from_millis(10);
    let mut maze = [[Cell {old_state: false, new_state: false, }; ROWS];COLS];
    //randomly assigning cells of the maze either true or false as first values:
//    for i in 1..ROWS - 1 {
//        for j in 1..COLS - 1 {
//            maze[i][j].old_state = match rand::thread_rng().gen_range(1..3) {
//                1 => true,
//                2 => false,
//                _ => false,
//            };
//        }
//    }


     maze[ROWS/2][COLS/2].old_state = true;
     maze[ROWS/2 - 1][COLS/2 - 1].old_state = true;
     maze[ROWS/2 + 1][COLS/2 - 1].old_state = true;
     maze[ROWS/2 - 1][COLS/2 + 1].old_state = true;
     maze[ROWS/2 + 1][COLS/2 + 1].old_state = true;
    let mut generations = 0;
    print!("\x1b[2J");
    loop {
        print!("\x1b[0;0H");
        print_maze(&maze);
        update_maze(&mut maze);
        update_states(&mut maze);
        println!("generations: {}", generations);
        generations = generations + 1;
        thread::sleep(interval);
    }
}

fn print_maze(maze: &[[Cell;ROWS];COLS]) {
    for i in 1..ROWS {
        for j in 1..COLS {
            print!("{}", if maze[i][j].old_state == true &&
                            maze[i+1][j].old_state == true {
                "█"
            } else if maze[i][j].old_state == true &&
                      maze[i+1][j].old_state == false {
                "■"
            } else {
                " "
            });
        }
        print!("\n");
    }
}

fn update_maze(maze: &mut [[Cell;ROWS];COLS]) {
    for i in 1..ROWS - 1 {
        for j in 1..COLS - 1 {
            match maze[i][j].old_state {
                true => {
                    if check_neighbourhood(&maze, i, j) >= 1 &&
                       check_neighbourhood(&maze, i, j) < 5 {
                        maze[i][j].new_state = true;
                    } else {
                        maze[i][j].new_state = false;
                    }
                },
                false => {
                    if check_neighbourhood(&maze, i, j) == 3 {
                        maze[i][j].new_state = true;
                    } else {
                        maze[i][j].new_state = false;
                    }
                },
            }
        }
    }
}

fn update_states(maze: &mut [[Cell; ROWS]; COLS]) {
    for i in 1..ROWS - 1 {
        for j in 1..COLS - 1 {
            maze[i][j].old_state = maze[i][j].new_state;
        }
    }
}

fn check_neighbourhood(maze: &[[Cell; ROWS]; COLS], x: usize, y: usize) -> usize {
    let mut neighbour_counter: usize = 0;
    if maze[x][y-1].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x-1][y-1].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x-1][y].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x-1][y+1].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x][y+1].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x+1][y+1].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x+1][y].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x+1][y-1].old_state == true {
        neighbour_counter += 1;
    }
    if maze[x][y].old_state == true {
        neighbour_counter += 1;
    }
    neighbour_counter
}
