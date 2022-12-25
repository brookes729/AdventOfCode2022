use std::collections::HashMap;

use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day24.txt");

    let mut wind: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    let mut grid: Vec<(i32, i32)> = Vec::new();

    let mut row = 0;
    let mut column = 0;

    for line in file_contents {
        column = 0;
        for char in line.chars() {
            if char == '.' {
                grid.push((row, column));
            } else if char != '#' {
                grid.push((row, column));
                wind.insert((row, column), vec![char]);
            }
            column += 1;
        }
        row += 1;
    }

    let goal = (row - 1, column - 2);

    let mut options: HashMap<i32, Vec<(i32, i32, i32, i32)>> = HashMap::new();
    let mut minimum_option = 0;
    options.insert(minimum_option, vec![(0, 1, 0, 0)]);
    let mut visited: Vec<(i32, i32, i32, i32)> = vec![];

    let mut cached_wind: HashMap<i32, HashMap<(i32, i32), Vec<char>>> = HashMap::new();
    cached_wind.insert(0, wind.clone());
    let directions = vec![(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut part_one = 0;
    let mut part_one_half = 0;
    let mut part_two = 0;

    while let Some(option) = get_next_best_option(&mut options, &mut minimum_option) {
        if visited.contains(&option) {
            continue;
        }
        // println!("Checking option {:?}, {}", option, minimum_option);

        visited.push(option);
        if (option.0, option.1) == goal && option.3 == 0 {
            if part_one == 0 {
                part_one = option.2;

                println!("Part 1: {}", part_one - 1);
            }
            let option_score = get_option_score(option.0, option.1, option.2, 1, goal);
            options = HashMap::new();
            options.insert(
                option_score,
                vec![(option.0, option.1, option.2, option.3 + 1)],
            );
            if minimum_option > option_score {
                minimum_option = option_score;
            }
            continue;
        }
        if (option.0, option.1) == (0, 1) && option.3 == 1 {
            if part_one_half == 0 {
                part_one_half = option.2;

                println!("Part 1.5: {}", part_one_half - 1);
            }
            let option_score = get_option_score(option.0, option.1, option.2, 2, goal);
            options = HashMap::new();
            options.insert(option_score, vec![(option.0, option.1, option.2, 2)]);
            if minimum_option > option_score {
                minimum_option = option_score;
            }
            continue;
        }

        if (option.0, option.1) == goal && option.3 == 2 {
            part_two = option.2;
            break;
        }
        let new_wind = find_next_wind(
            option.2,
            wind.clone(),
            grid.clone(),
            row,
            column,
            &mut cached_wind,
        );

        for direction in directions.clone() {
            let new_position = (option.0 + direction.0, option.1 + direction.1);
            if grid.contains(&new_position) && !new_wind.contains_key(&new_position) {
                let option_score =
                    get_option_score(new_position.0, new_position.1, option.2, option.3, goal);
                options.entry(option_score).or_insert(Vec::new()).push((
                    new_position.0,
                    new_position.1,
                    option.2 + 1,
                    option.3,
                ));
                if minimum_option > option_score {
                    minimum_option = option_score;
                }
            }
        }
    }

    println!("Part 2: {}", part_two - 1);
}

fn get_option_score(x: i32, y: i32, time: i32, found: i32, goal: (i32, i32)) -> i32 {
    let mut option_score = 0;
    match found {
        0 => {
            option_score = time - x - y + 2 * (goal.0 + goal.1);
        }
        1 => {
            option_score = time + x + y;
        }
        2 => {
            option_score = time - x - y;
        }
        _ => (),
    }
    option_score
}

// fn draw_grid(
//     new_wind: HashMap<(i32, i32), Vec<char>>,
//     grid: Vec<(i32, i32)>,
//     row: i32,
//     column: i32,
// ) {
//     println!("\x1B[2J\x1B[1;1H");
//     for x in 0..row {
//         for y in 0..column {
//             let key = new_wind.contains_key(&((x, y)));
//             if key {
//                 if new_wind.get(&(x, y)).unwrap().len() > 1 {
//                     print!("{}", new_wind.get(&(x, y)).unwrap().len());
//                 } else {
//                     print!("{}", new_wind.get(&(x, y)).unwrap()[0]);
//                 }
//             } else {
//                 if grid.contains(&(x, y)) {
//                     print!(".");
//                 } else {
//                     print!("#");
//                 }
//             }
//         }
//         println!();
//     }
// }

fn get_next_best_option(
    options: &mut HashMap<i32, Vec<(i32, i32, i32, i32)>>,
    minimum_option: &mut i32,
) -> Option<(i32, i32, i32, i32)> {
    while !options.contains_key(minimum_option) || options.get(&minimum_option).unwrap().len() == 0
    {
        *minimum_option += 1;
        if minimum_option == &1000 {
            return None;
        }
    }

    options
        .get_mut(minimum_option)
        .unwrap()
        .sort_by(|a, b| a.2.cmp(&b.2));
    options.get_mut(minimum_option).unwrap().pop()
}

fn find_next_wind(
    round: i32,
    wind: HashMap<(i32, i32), Vec<char>>,
    grid: Vec<(i32, i32)>,
    row: i32,
    column: i32,
    cached_wind: &mut HashMap<i32, HashMap<(i32, i32), Vec<char>>>,
) -> HashMap<(i32, i32), Vec<char>> {
    if cached_wind.contains_key(&(round % ((row - 2) * (column - 2)))) {
        return cached_wind
            .get(&(round % ((row - 2) * (column - 2))))
            .unwrap()
            .clone();
    }
    let mut previous_wind = wind.clone();
    if cached_wind.contains_key(&(round - 1)) {
        previous_wind = cached_wind.get(&(round - 1)).unwrap().clone();
    }

    let mut new_wind: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    for blizzards in previous_wind {
        for blizz in blizzards.1 {
            let mut new_blizzard = blizzards.0.clone();
            match blizz {
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
                match blizz {
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
            new_wind
                .entry(new_blizzard)
                .or_insert(Vec::new())
                .push(blizz);
        }
    }

    cached_wind.insert(round, new_wind.clone());
    new_wind
}
