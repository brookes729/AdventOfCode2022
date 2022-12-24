use std::collections::HashMap;

use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day24.txt");

    let mut wind: HashMap<(i32, i32), char> = HashMap::new();
    let mut grid: Vec<(i32, i32)> = Vec::new();

    let mut row = 0;
    let mut column = 0;

    for line in file_contents {
        column = 0;
        for char in line.chars() {
            if char == '.' {
                grid.push((row, column));
            } else if char != '#' {
                wind.insert((row, column), char);
            }
            column += 1;
        }
        row += 1;
    }

    for _ in 1..=10 {
        let new_wind = find_next_wind(wind, grid.clone(), row, column);
        wind = new_wind.clone();
    }

    println!("Part 1: {}", 1);
    println!("Part 2: {}", 2);
}

fn find_next_wind(
    wind: HashMap<(i32, i32), char>,
    grid: Vec<(i32, i32)>,
    row: i32,
    column: i32,
) -> HashMap<(i32, i32), char> {
    let mut new_wind: HashMap<(i32, i32), char> = HashMap::new();
    for blizzard in wind {
        let mut new_blizzard = blizzard.0.clone();
        match blizzard.1 {
            'v' => {
                new_blizzard.0 += 1;
            }
            '^' => {
                new_blizzard.0 -= 1;
            }
            '<' => {
                new_blizzard.1 -= 1;
            }
            '>' => {
                new_blizzard.1 += 1;
            }
            _ => (),
        }
        if !grid.contains(&new_blizzard) {
            match blizzard.1 {
                'v' => {
                    new_blizzard.0 = 1;
                }
                '^' => {
                    new_blizzard.0 = row - 2;
                }
                '<' => {
                    new_blizzard.1 = column - 2;
                }
                '>' => {
                    new_blizzard.1 = 1;
                }
                _ => (),
            }
        }

        new_wind.insert(new_blizzard, blizzard.1);
    }
    new_wind
}
