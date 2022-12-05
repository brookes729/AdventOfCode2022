use std::collections::{hash_map::Entry, HashMap, VecDeque};

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number};

pub fn run_day() {
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();

    get_starting_stack(&mut stacks);
    perform_instructions(&mut stacks, true);
    print!("Part One: ");
    for stack_number in 1..=stacks.len() {
        print!(
            "{}",
            stacks.get_mut(&stack_number).unwrap().pop_back().unwrap()
        )
    }

    println!();
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    get_starting_stack(&mut stacks);
    perform_instructions(&mut stacks, false);

    print!("Part Two: ");
    for stack_number in 1..=stacks.len() {
        print!(
            "{}",
            stacks.get_mut(&stack_number).unwrap().pop_back().unwrap()
        )
    }
}

fn perform_instructions(stacks: &mut HashMap<usize, VecDeque<char>>, move_single: bool) {
    let file_contents = read_file_to_string_vector("day5.txt");
    let mut instructions_found = false;
    for line in file_contents {
        if !instructions_found {
            if line.is_empty() {
                instructions_found = true;
            }
            continue;
        }
        // example: move 1 from 2 to 1
        let instruction: Vec<&str> = line.split(' ').collect();
        let number_of_crates = string_to_number(instruction[1].to_string());
        let start = string_to_number(instruction[3].to_string()) as usize;
        let finish = string_to_number(instruction[5].to_string()) as usize;
        if move_single {
            for _ in 1..=number_of_crates {
                let crate_to_move = stacks.get_mut(&start).unwrap().pop_back().unwrap();
                stacks.get_mut(&finish).unwrap().push_back(crate_to_move);
            }
        } else {
            let mut crates_being_moved: Vec<char> = Vec::new();

            for _ in 1..=number_of_crates {
                crates_being_moved.push(stacks.get_mut(&start).unwrap().pop_back().unwrap());
            }
            for _ in 1..=number_of_crates {
                stacks
                    .get_mut(&finish)
                    .unwrap()
                    .push_back(crates_being_moved.pop().unwrap());
            }
        }
    }
}

fn get_starting_stack(stacks: &mut HashMap<usize, VecDeque<char>>) {
    let file_contents = read_file_to_string_vector("day5.txt");
    for line in file_contents {
        if line.starts_with(" 1") {
            break;
        }
        let incoming_stack_line_length = line.len();
        let mut cursor = 1;

        while (cursor as usize) < incoming_stack_line_length {
            let stack_number = ((cursor as usize) + 3) / 4;
            let current_crate = line.chars().nth(cursor).unwrap();
            // println!(
            //     "{} cursor {} stack number {} char {} stack number",
            //     line, cursor, stack_number, current_crate
            // );

            if current_crate == ' ' {
                cursor += 4;
                continue;
            }

            match stacks.entry(stack_number) {
                Entry::Vacant(e) => {
                    e.insert(VecDeque::from([current_crate]) as VecDeque<char>);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push_front(current_crate);
                }
            }

            cursor += 4;
        }
    }
}
