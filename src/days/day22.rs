use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let mut file_contents = read_file_to_string_vector("day22.txt").into_iter();

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut starting_point = (0, 0);
    let mut starting_point_two = (0, 0);

    let mut row = 0;
    let mut column_maxes: Vec<i32> = Vec::new();
    while let Some(line) = file_contents.next() {
        if line.is_empty() {
            break;
        }
        let mut column = 0;
        for char in line.chars() {
            if char != ' ' {
                if starting_point == (0, 0) {
                    starting_point = (row, column);
                    starting_point_two = (row, column);
                }
                grid.insert((row, column), char);
            }
            column += 1;
        }
        column_maxes.push(column);
        row += 1;
    }

    let instructions = file_contents.next().unwrap();

    let mut direction = (0, 1);

    map_grid(
        &mut starting_point,
        &mut direction,
        instructions.clone(),
        &mut grid.clone(),
        column_maxes.clone(),
        false,
    );

    let mut part_one = 1000 * (starting_point.0 + 1) + 4 * (starting_point.1 + 1);
    if direction.0 == 1 {
        part_one += 1
    } else if direction.1 == -1 {
        part_one += 2
    } else if direction.0 == -1 {
        part_one += 3
    }

    println!();
    println!("Part 1: {}", part_one);

    map_grid(
        &mut starting_point_two,
        &mut direction,
        instructions.clone(),
        &mut grid.clone(),
        column_maxes.clone(),
        true,
    );

    let mut part_two = 1000 * (starting_point_two.0 + 1) + 4 * (starting_point_two.1 + 1);
    if direction.0 == 1 {
        part_two += 1
    } else if direction.1 == -1 {
        part_two += 2
    } else if direction.0 == -1 {
        part_two += 3
    }
    println!("Part 2: {}", part_two);
}

fn map_grid(
    grid_point: &mut (i32, i32),
    direction: &mut (i32, i32),
    instructions: String,
    grid: &mut HashMap<(i32, i32), char>,
    column_maxes: Vec<i32>,
    cube: bool,
) {
    let steps_regex = Regex::new(r"[\d]+").unwrap();
    let mut steps_regex_iter = steps_regex.captures_iter(&instructions);

    let directions_regex = Regex::new(r"[RL]+").unwrap();
    let mut directions_regex_iter = directions_regex.captures_iter(&instructions);

    loop {
        let steps_option = steps_regex_iter.next();

        match steps_option {
            Some(_) => (),
            None => break,
        }

        let steps =
            string_to_number_i32(steps_option.unwrap().get(0).unwrap().as_str().to_string());

        // print!(
        //     "Move {} steps {:?}, from {:?}",
        //     steps, direction, grid_point
        // );
        for _ in 1..=steps {
            let mut next_spot = (grid_point.0 + direction.0, grid_point.1 + direction.1);
            let mut next_direction = direction.clone();
            if !grid.contains_key(&next_spot) || (cube && grid.get(&next_spot).unwrap() == &' ') {
                if cube {
                    cube_loop_around(&mut next_spot, &mut next_direction);
                } else {
                    loop_around(
                        &mut next_spot,
                        direction,
                        grid.clone(),
                        column_maxes.clone(),
                    );
                }
            }
            if grid.get(&next_spot).unwrap() == &'#' {
                break;
            }
            grid_point.0 = next_spot.0;
            grid_point.1 = next_spot.1;
            direction.0 = next_direction.0;
            direction.1 = next_direction.1;
            *grid.get_mut(&grid_point).unwrap() = '@';
        }
        // println!("to {:?}", grid_point);
        // draw_grid(grid.clone(), column_maxes.clone());

        let direction_option = directions_regex_iter.next();

        if let Some(value) = direction_option {
            rotate(direction, value)
        }
    }
}

fn cube_loop_around(next_spot: &mut (i32, i32), direction: &mut (i32, i32)) {
    let sample = false;
    if sample {
        if next_spot.0 == -1 && (next_spot.1 >= 8 && next_spot.1 < 12) {
            // A
            direction.0 = 1;
            direction.1 = 0;
            next_spot.0 = 4;
            next_spot.1 = 11 - next_spot.1;
        } else if next_spot.0 == 3 && (next_spot.1 >= 0 && next_spot.1 < 4) {
            // B
            direction.0 = 1;
            direction.1 = 0;
            next_spot.1 = 11 - next_spot.1;
            next_spot.0 = 0;
        } else if next_spot.0 == 3 && (next_spot.1 >= 4 && next_spot.1 < 8) {
            // C
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = next_spot.1 - 4;
            next_spot.1 = 8;
        } else if next_spot.1 == 7 && (next_spot.0 >= 0 && next_spot.0 < 4) {
            // D
            direction.0 = 1;
            direction.1 = 0;
            next_spot.1 = 4 + next_spot.0;
            next_spot.0 = 4;
        } else if next_spot.1 == -1 && (next_spot.0 >= 4 && next_spot.0 < 8) {
            // E
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = 19 - next_spot.0;
            next_spot.0 = 11;
        } else if next_spot.0 == 8 && (next_spot.1 >= 0 && next_spot.1 < 4) {
            // F
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = 11 - next_spot.1;
            next_spot.0 = 11;
        } else if next_spot.0 == 8 && (next_spot.0 >= 4 && next_spot.0 < 8) {
            // G
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = 15 - next_spot.1;
            next_spot.1 = 8;
        } else if next_spot.1 == 7 && (next_spot.0 >= 8 && next_spot.0 < 12) {
            // H
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = 15 - next_spot.0;
            next_spot.0 = 7;
        } else if next_spot.0 == 12 && (next_spot.1 >= 8 && next_spot.1 < 12) {
            // I
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = 11 - next_spot.1;
            next_spot.0 = 7;
        } else if next_spot.0 == 12 && (next_spot.1 >= 12 && next_spot.1 < 16) {
            // J
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = 19 - next_spot.1;
            next_spot.1 = 0;
        } else if next_spot.1 == 16 && (next_spot.0 >= 8 && next_spot.0 < 12) {
            // K
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = 11 - next_spot.0;
            next_spot.1 = 11;
        } else if next_spot.0 == 7 && (next_spot.1 >= 12 && next_spot.1 < 16) {
            // L
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = 19 - next_spot.1;
            next_spot.1 = 11;
        } else if next_spot.1 == 12 && (next_spot.0 >= 4 && next_spot.0 < 8) {
            // M
            direction.0 = 1;
            direction.1 = 0;
            next_spot.1 = 19 - next_spot.0;
            next_spot.0 = 8;
        } else if next_spot.1 == 12 && (next_spot.0 >= 0 && next_spot.0 < 4) {
            // N
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = 8 + next_spot.0;
            next_spot.1 = 15;
        }
    } else {
        if next_spot.0 == -1 && (next_spot.1 >= 50 && next_spot.1 < 100) {
            // A
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = 100 + next_spot.1;
            next_spot.1 = 0;
        } else if next_spot.1 == 49 && (next_spot.0 >= 0 && next_spot.0 < 50) {
            // B
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = 149 - next_spot.0;
            next_spot.1 = 0;
        } else if next_spot.1 == 49 && (next_spot.0 >= 50 && next_spot.0 < 100) {
            // C
            direction.0 = 1;
            direction.1 = 0;
            next_spot.1 = next_spot.0 - 50;
            next_spot.0 = 100;
        } else if next_spot.0 == 99 && (next_spot.1 >= 0 && next_spot.1 < 50) {
            // D
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = 50 + next_spot.0;
            next_spot.1 = 50;
        } else if next_spot.1 == -1 && (next_spot.0 >= 100 && next_spot.0 < 150) {
            // E
            direction.0 = 0;
            direction.1 = 1;
            next_spot.0 = 149 - next_spot.0;
            next_spot.1 = 50;
        } else if next_spot.1 == -1 && (next_spot.0 >= 150 && next_spot.0 < 200) {
            // F
            direction.0 = 1;
            direction.1 = 0;
            next_spot.1 = next_spot.0 - 100;
            next_spot.0 = 0;
        } else if next_spot.0 == 200 && (next_spot.1 >= 0 && next_spot.1 < 50) {
            // G
            direction.0 = 1;
            direction.1 = 0;
            next_spot.1 = 100 + next_spot.1;
            next_spot.0 = 0;
        } else if next_spot.1 == 50 && (next_spot.0 >= 150 && next_spot.0 < 200) {
            // H
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = next_spot.0 - 100;
            next_spot.0 = 149;
        } else if next_spot.0 == 150 && (next_spot.1 >= 50 && next_spot.1 < 100) {
            // I
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = 100 + next_spot.1;
            next_spot.1 = 49;
        } else if next_spot.1 == 100 && (next_spot.0 >= 100 && next_spot.0 < 150) {
            // J
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = 149 - next_spot.0;
            next_spot.1 = 149;
        } else if next_spot.1 == 100 && (next_spot.0 >= 50 && next_spot.0 < 100) {
            // K
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = 50 + next_spot.0;
            next_spot.0 = 49;
        } else if next_spot.0 == 50 && (next_spot.1 >= 100 && next_spot.1 < 150) {
            // L
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = next_spot.1 - 50;
            next_spot.1 = 99;
        } else if next_spot.1 == 150 && (next_spot.0 >= 0 && next_spot.0 < 50) {
            // M
            direction.0 = 0;
            direction.1 = -1;
            next_spot.0 = 149 - next_spot.0;
            next_spot.1 = 99;
        } else if next_spot.0 == -1 && (next_spot.1 >= 100 && next_spot.1 < 150) {
            // N
            direction.0 = -1;
            direction.1 = 0;
            next_spot.1 = next_spot.1 - 100;
            next_spot.0 = 199;
        } else {
            println!("Thats not right!");
        }
    }
}

// fn draw_grid(grid: HashMap<(i32, i32), char>, column_maxes: Vec<i32>) {
//     print!("\x1B[2J\x1B[1;1H"); // clear console
//     let mut x = 0;
//     for row in column_maxes {
//         println!();
//         for y in 0..row {
//             print!("{}", grid.get(&(x, y)).unwrap_or(&' '));
//         }
//         x += 1;
//     }
//     println!();
// }

fn rotate(direction: &mut (i32, i32), direction_option: Captures) {
    let turning = direction_option.get(0).unwrap().as_str();
    if turning == "R" {
        // 0 1 -> 1 0 -> 0 -1 -> -1 0 -> 0 1
        let temp = direction.0;
        direction.0 = direction.1;
        direction.1 = -1 * temp;
    } else {
        // 0 1 -> -1 0 -> 0 -1 -> 1 0 -> 0 1
        let temp = direction.0;
        direction.0 = -1 * direction.1;
        direction.1 = temp;
    }
}

fn loop_around(
    next_spot: &mut (i32, i32),
    direction: &mut (i32, i32),
    grid: HashMap<(i32, i32), char>,
    column_maxes: Vec<i32>,
) {
    if direction.0 == 1 {
        next_spot.0 = 0;
    }
    if direction.0 == -1 {
        next_spot.0 = column_maxes.len() as i32;
    }
    if direction.1 == 1 {
        next_spot.1 = 0;
    }
    if direction.1 == -1 {
        next_spot.1 = *column_maxes.get(next_spot.0.clone() as usize).unwrap();
    }
    while !grid.contains_key(&next_spot) {
        next_spot.0 += direction.0;
        next_spot.1 += direction.1;
    }
}
