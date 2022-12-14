use std::cmp;

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day14.txt");

    let mut rock_pattern: Vec<(i32, i32)> = Vec::new();
    let mut lowest_rock = 0;

    for line in file_contents {
        let coords = line.split(" -> ");
        let mut current_cords = (0, 0);
        for coord in coords {
            let coord_split: Vec<&str> = coord.split(",").collect();
            let new_coords = (
                string_to_number_i32(coord_split.get(0).unwrap().to_string()),
                string_to_number_i32(coord_split.get(1).unwrap().to_string()),
            );
            if current_cords.0 != 0 && current_cords.1 != 0 {
                // skip first time
                for x in cmp::min(current_cords.0, new_coords.0)
                    ..=cmp::max(current_cords.0, new_coords.0)
                {
                    for y in cmp::min(current_cords.1, new_coords.1)
                        ..=cmp::max(current_cords.1, new_coords.1)
                    {
                        if !rock_pattern.contains(&(x, y)) {
                            rock_pattern.push((x, y))
                        }
                    }
                }
            }
            if cmp::max(current_cords.1, new_coords.1) > lowest_rock {
                lowest_rock = cmp::max(current_cords.1, new_coords.1)
            }
            current_cords = new_coords
        }
    }

    let mut pieces_of_sand = 0;
    let sand_start = (500, 0);
    let mut sand_position: (i32, i32);
    let mut part_one_declared = false;

    loop {
        pieces_of_sand += 1;
        sand_position = sand_start.clone();

        loop {
            if sand_position.1 == lowest_rock + 1 {
                //part 2 hit the floor
                rock_pattern.push(sand_position);
                break;
            } else if !rock_pattern.contains(&(sand_position.0, sand_position.1 + 1)) {
                sand_position.1 += 1;
            } else if !rock_pattern.contains(&(sand_position.0 - 1, sand_position.1 + 1)) {
                sand_position.0 -= 1;
                sand_position.1 += 1;
            } else if !rock_pattern.contains(&(sand_position.0 + 1, sand_position.1 + 1)) {
                sand_position.0 += 1;
                sand_position.1 += 1;
            } else {
                // sand is stable
                rock_pattern.push(sand_position);
                break;
            }
            if sand_position.1 == lowest_rock && !part_one_declared {
                // last piece is falling our way!
                println!("Part 1 {}", pieces_of_sand - 1);
                part_one_declared = true;
            }
        }

        if rock_pattern.contains(&(500, 0)) {
            break;
        }
    }
    println!("Part 2 {}", pieces_of_sand);
}
