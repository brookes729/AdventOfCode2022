use std::collections::HashMap;

use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day12.txt");

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut goal = (0, 0);
    let mut mapping_pos = (0, 0);
    let mut options: Vec<(i32, i32, i32)> = Vec::new();
    let mut a_options: Vec<(i32, i32, i32)> = Vec::new();
    let mut visited: Vec<(i32, i32)> = Vec::new();

    for line in file_contents {
        for cell in line.chars() {
            if cell == 'S' {
                options.push((mapping_pos.0, mapping_pos.1, 0));
                a_options.push((mapping_pos.0, mapping_pos.1, 0));
                map.insert(mapping_pos.clone(), 1);
            } else if cell == 'E' {
                goal = mapping_pos.clone();
                map.insert(mapping_pos.clone(), 26);
            } else {
                let char_value = (cell as i32) - 96;
                map.insert(mapping_pos.clone(), char_value);
                if char_value == 1 {
                    a_options.push((mapping_pos.0, mapping_pos.1, 0));
                }
            }
            mapping_pos.1 += 1;
        }
        mapping_pos.0 += 1;
        mapping_pos.1 = 0;
    }

    println!("Part 1");
    find_the_top(options, goal, &mut visited, &map);

    println!("Part 2");
    let mut visited: Vec<(i32, i32)> = Vec::new();
    find_the_top(a_options, goal, &mut visited, &map);
}

fn find_the_top(
    mut options: Vec<(i32, i32, i32)>,
    goal: (i32, i32),
    visited: &mut Vec<(i32, i32)>,
    map: &HashMap<(i32, i32), i32>,
) {
    while options.len() > 0 {
        // take the next option (sorted at end of loop)
        let (option_x, option_y, option_effort) = options.pop().unwrap();

        if goal == (option_x, option_y) {
            println!("Shortest route: {}", option_effort)
        }

        visited.push((option_x, option_y));

        let curren_height = map.get(&(option_x, option_y)).unwrap().clone();

        if can_move(option_x + 1, option_y, &curren_height, &visited, &map) {
            if !options.contains(&(option_x + 1, option_y, option_effort + 1)) {
                options.push((option_x + 1, option_y, option_effort + 1))
            }
        }
        if can_move(option_x, option_y + 1, &curren_height, &visited, &map) {
            if !options.contains(&(option_x, option_y + 1, option_effort + 1)) {
                options.push((option_x, option_y + 1, option_effort + 1))
            }
        }
        if can_move(option_x - 1, option_y, &curren_height, &visited, &map) {
            if !options.contains(&(option_x - 1, option_y, option_effort + 1)) {
                options.push((option_x - 1, option_y, option_effort + 1))
            }
        }
        if can_move(option_x, option_y - 1, &curren_height, &visited, &map) {
            if !options.contains(&(option_x, option_y - 1, option_effort + 1)) {
                options.push((option_x, option_y - 1, option_effort + 1))
            }
        }

        options.sort_by(|a, b| b.2.cmp(&a.2));
    }
}

fn can_move(
    x: i32,
    y: i32,
    curren_height: &i32,
    visited: &Vec<(i32, i32)>,
    map: &HashMap<(i32, i32), i32>,
) -> bool {
    if !map.contains_key(&(x, y)) {
        false
    } else if visited.contains(&(x, y)) {
        false
    } else {
        let to_be_height = map.get(&(x, y)).unwrap();
        if to_be_height <= &(curren_height + 1) {
            true
        } else {
            false
        }
    }
}
