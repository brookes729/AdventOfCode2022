use std::{collections::HashMap, ops::ControlFlow, str::Chars};

use crate::helpers::file_helper::read_file_to_string;

pub fn run_day() {
    let file_contents = read_file_to_string("day17.txt");

    let rocks = vec![
        (vec![(0, 0), (0, 1), (0, 2), (0, 3)], 4),
        (vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)], 3),
        (vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)], 3),
        (vec![(0, 0), (1, 0), (2, 0), (3, 0)], 1),
        (vec![(0, 0), (0, 1), (1, 0), (1, 1)], 2),
    ];
    let mut next_rock = 0;

    let mut filled_spaces: Vec<(i64, i64)> = Vec::new();
    let ground = 0;
    let mut highest_rock: i64 = 0;

    let jets = file_contents.chars();
    let mut next_jet = 0;

    let part_one = 0;

    let mut known_patterns: HashMap<(Vec<u8>, i64, i64), (i64, i64)> = HashMap::default();

    let mut rock_count = 1;
    let rock_counter_limit = 1000000000000 as i64;
    while rock_count < rock_counter_limit {
        let mut rock_position = (highest_rock + 3, 2 as i64);
        let rock = get_next_rock(&rocks, &mut next_rock);
        //draw_room(rock, rock_position, filled_spaces.clone());

        loop {
            let jet = get_next_jet(&mut jets.clone(), &mut next_jet);
            let movement = if jet == '<' { -1 } else { 1 };
            //print!("{}", movement);
            move_rock_sideways(movement, &mut rock_position, rock, &filled_spaces);

            if let ControlFlow::Break(_) = move_rock_down(
                rock,
                &mut filled_spaces,
                &mut rock_position,
                ground,
                &mut highest_rock,
            ) {
                break;
            }
        }

        let mut row_binary: HashMap<i64, u8> = HashMap::new();

        for space in filled_spaces.clone() {
            if !row_binary.contains_key(&(highest_rock - space.0)) {
                row_binary.insert(highest_rock - space.0, 1 << space.1);
            } else {
                *row_binary.get_mut(&(highest_rock - space.0)).unwrap() |= 1 << space.1;
            }
        }

        let binaries: Vec<u8> = get_top_shape(row_binary, highest_rock);

        let key: (Vec<u8>, i64, i64) = (binaries, next_rock, next_jet);
        if known_patterns.contains_key(&key) {
            let previous_point = known_patterns.get(&key).unwrap();
            let repeat_start = previous_point.0;
            let repeat_start_height = previous_point.1;
            let repeat_length = rock_count - previous_point.0;
            let repeat_height = highest_rock - previous_point.1;

            let repeats = rock_counter_limit / repeat_length;

            if repeats * repeat_length + repeat_start == rock_counter_limit {
                let mut new_filled_spaces: Vec<(i64, i64)> = Vec::new();
                for space in filled_spaces.clone() {
                    new_filled_spaces.push((
                        (repeats * repeat_height + repeat_start_height) - highest_rock + space.0,
                        space.1,
                    ));
                }

                filled_spaces = new_filled_spaces;
                highest_rock = repeats * repeat_height + repeat_start_height;
                rock_count = repeats * repeat_length + repeat_start;
            }
        }
        known_patterns.insert(key, (rock_count, highest_rock));

        rock_count += 1;
        println!("\x1B[2J\x1B[1;1H {} - {}", part_one, rock_count);
    }

    println!("Part Two: {}", highest_rock);
}

fn get_top_shape(row_binary: HashMap<i64, u8>, highest_rock: i64) -> Vec<u8> {
    let mut covered: u8 = 0;
    let mut top_shape: Vec<u8> = Vec::new();
    for row in 0..=highest_rock {
        let slice_option = row_binary.get(&(highest_rock - row));
        if let Some(slice) = slice_option {
            covered |= slice;
            top_shape.push(slice.clone());
            if covered == !(1 << 7) {
                break;
            }
        }
    }
    top_shape
}

fn move_rock_sideways(
    movement: i32,
    rock_position: &mut (i64, i64),
    rock: &(Vec<(i64, i64)>, i64),
    filled_spaces: &Vec<(i64, i64)>,
) {
    let mut rock_can_move_to_side = true;
    if (movement == 1 && rock_position.1 + rock.1 < 7) || (movement == -1 && rock_position.1 > 0) {
        for rock_coord in rock.clone().0 {
            if filled_spaces.contains(&(
                rock_position.0 + rock_coord.0,
                rock_position.1 + movement as i64 + rock_coord.1,
            )) {
                rock_can_move_to_side = false;
            }
        }
        if rock_can_move_to_side {
            rock_position.1 += movement as i64;
        }
    }
}

fn move_rock_down(
    rock: &(Vec<(i64, i64)>, i64),
    filled_spaces: &mut Vec<(i64, i64)>,
    rock_position: &mut (i64, i64),
    ground: i64,
    highest_rock: &mut i64,
) -> ControlFlow<()> {
    let mut rock_can_move_down = true;
    if rock_position.0 <= *highest_rock {
        for rock_coord in rock.clone().0 {
            if filled_spaces.contains(&(
                rock_position.0 - 1 + rock_coord.0,
                rock_position.1 + rock_coord.1,
            )) {
                rock_can_move_down = false;
            }
        }
    }
    if rock_position.0 > ground && rock_can_move_down {
        rock_position.0 -= 1;
    } else {
        for rock_coord in rock.clone().0 {
            if rock_position.0 + rock_coord.0 + 1 > *highest_rock {
                *highest_rock = rock_position.0 + rock_coord.0 + 1;
            }
            filled_spaces.push((
                rock_position.0 + rock_coord.0,
                rock_position.1 + rock_coord.1,
            ));
        }
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn get_next_rock<'a>(
    rocks: &'a Vec<(Vec<(i64, i64)>, i64)>,
    next_rock: &'a mut i64,
) -> &'a (Vec<(i64, i64)>, i64) {
    let rock = rocks.get(*next_rock as usize).unwrap();
    *next_rock = if (*next_rock + 1) as usize == rocks.len() {
        0
    } else {
        *next_rock + 1
    };
    rock
}
fn get_next_jet(jets: &mut Chars, next_jet: &mut i64) -> char {
    let jet = jets.clone().nth(*next_jet as usize).unwrap();
    *next_jet = if (*next_jet + 1) as usize == jets.count() {
        0
    } else {
        *next_jet + 1
    };
    jet
}
