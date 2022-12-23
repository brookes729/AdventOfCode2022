use std::collections::HashMap;

use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day23.txt");

    let mut grid: Vec<(i32, i32)> = Vec::new();
    let mut box_coord = ((9999, 0), (9999, 0));
    let mut row = 0;
    for line in file_contents {
        let mut column = 0;
        for char in line.chars() {
            if char == '#' {
                grid.push((row, column));
                update_bounding_box(row, column, &mut box_coord);
            }
            column += 1;
        }
        row += 1;
    }

    let directions = "NSWE";
    let mut round = 0;
    loop {
        let mut new_grid: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for elf in grid.clone() {
            let new_elf = decide_direction(elf, directions.clone(), round, grid.clone());
            new_grid.insert(new_elf.0, new_elf.1);
        }
        stop_duplicate_moves(&mut new_grid);
        grid = Vec::new();
        let mut moved = false;
        for elf in new_grid {
            grid.push(elf.1);
            update_bounding_box(elf.1 .0, elf.1 .1, &mut box_coord);
            if !moved && elf.0 != elf.1 {
                moved = true;
            }
        }
        //draw_and_count_grid(grid.clone(), box_coord, true);
        if !moved {
            break;
        }
        round += 1;
        if round == 10 {
            let part_one = draw_and_count_grid(grid.clone(), box_coord, false);
            println!("Part 1: {}", part_one);
        }
    }

    println!("Part 2: {}", round + 1);
}

fn stop_duplicate_moves(new_grid: &mut HashMap<(i32, i32), (i32, i32)>) {
    let mut destinations: HashMap<(i32, i32), i32> = HashMap::new();
    for elf in new_grid.clone() {
        if destinations.contains_key(&elf.1) {
            *destinations.get_mut(&elf.1).unwrap() += 1
        } else {
            destinations.insert(elf.1, 1);
        }
    }
    for elf in new_grid.clone() {
        if destinations.get(&elf.1).unwrap() > &1 {
            *new_grid.get_mut(&elf.0).unwrap() = elf.0;
        }
    }
}

fn decide_direction(
    elf: (i32, i32),
    directions: &str,
    round: i32,
    grid: Vec<(i32, i32)>,
) -> ((i32, i32), (i32, i32)) {
    let mut new_elf = (elf.clone(), elf.clone());

    if should_elf_move_direction(elf, ' ', grid.clone()) {
        for i in 0..=3 {
            let direction = directions.chars().nth(((round + i) % 4) as usize).unwrap();
            if should_elf_move_direction(elf, direction, grid.clone()) {
                match direction {
                    'N' => {
                        new_elf.1 .0 -= 1;
                    }
                    'S' => {
                        new_elf.1 .0 += 1;
                    }
                    'W' => {
                        new_elf.1 .1 -= 1;
                    }
                    'E' => {
                        new_elf.1 .1 += 1;
                    }
                    _ => (),
                }
                break;
            }
        }
    }

    new_elf
}

fn should_elf_move_direction(elf: (i32, i32), direction: char, grid: Vec<(i32, i32)>) -> bool {
    let mut will_move = true;
    match direction {
        'N' => {
            for y in -1..=1 {
                if grid.contains(&(elf.0 - 1, elf.1 + y)) {
                    return false;
                }
            }
        }
        'S' => {
            for y in -1..=1 {
                if grid.contains(&(elf.0 + 1, elf.1 + y)) {
                    return false;
                }
            }
        }
        'W' => {
            for y in -1..=1 {
                if grid.contains(&(elf.0 + y, elf.1 - 1)) {
                    return false;
                }
            }
        }
        'E' => {
            for y in -1..=1 {
                if grid.contains(&(elf.0 + y, elf.1 + 1)) {
                    return false;
                }
            }
        }
        _ => {
            let mut available_directions = 4;
            for y in -1..=1 {
                if grid.contains(&(elf.0 - 1, elf.1 + y)) {
                    available_directions -= 1; // can't move
                    break;
                }
            }
            for y in -1..=1 {
                if grid.contains(&(elf.0 + 1, elf.1 + y)) {
                    available_directions -= 1; // can't move
                    break;
                }
            }
            for y in -1..=1 {
                if grid.contains(&(elf.0 + y, elf.1 - 1)) {
                    available_directions -= 1; // can't move
                    break;
                }
            }
            for y in -1..=1 {
                if grid.contains(&(elf.0 + y, elf.1 + 1)) {
                    available_directions -= 1; // can't move
                    break;
                }
            }
            will_move = available_directions > 0 && available_directions < 4
        }
    }

    will_move
}

fn update_bounding_box(row: i32, column: i32, box_coord: &mut ((i32, i32), (i32, i32))) {
    if row < box_coord.0 .0 {
        box_coord.0 .0 = row
    }
    if row > box_coord.0 .1 {
        box_coord.0 .1 = row
    }
    if column < box_coord.1 .0 {
        box_coord.1 .0 = column
    }
    if column > box_coord.1 .1 {
        box_coord.1 .1 = column
    }
}

fn draw_and_count_grid(
    grid: Vec<(i32, i32)>,
    box_coords: ((i32, i32), (i32, i32)),
    draw: bool,
) -> i32 {
    if draw {
        println!("\x1B[2J\x1B[1;1H");
    } // clear console

    let mut count = 0;
    for x in (box_coords.0).0..=(box_coords.0).1 {
        for y in (box_coords.1).0..=(box_coords.1).1 {
            let key = grid.contains(&((x, y)));
            if key {
                if draw {
                    print!("#");
                }
            } else {
                if draw {
                    print!(".");
                }
                count += 1;
            }
        }
        if draw {
            println!();
        }
    }

    count
}
