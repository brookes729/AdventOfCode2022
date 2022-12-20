use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day20.txt")
        .into_iter()
        .enumerate()
        .map(|s| (s.0, string_to_number_i32(s.1) as isize))
        .collect::<Vec<_>>();

    let starting_order = file_contents.clone();

    let current_state = mix(starting_order.clone(), 1);

    let part_one = get_grove_coords(current_state);

    println!("Part 1 Score: {}", part_one);

    let part_two_order = mix(
        starting_order
            .clone()
            .into_iter()
            .map(|x| (x.0, x.1 * 811589153))
            .collect::<Vec<_>>(),
        10,
    );

    let part_two = get_grove_coords(part_two_order);

    println!("Part 2 Score: {}", part_two);
}

fn get_grove_coords(current_state: Vec<(usize, isize)>) -> isize {
    let zero_index = current_state
        .clone()
        .into_iter()
        .position(|x| x.1 == 0)
        .unwrap();
    let part_one_index = vec![
        (zero_index + 1000) % current_state.len(),
        (zero_index + 2000) % current_state.len(),
        (zero_index + 3000) % current_state.len(),
    ];
    let part_one = current_state[part_one_index[0]].1
        + current_state[part_one_index[1]].1
        + current_state[part_one_index[2]].1;
    part_one
}

fn mix(starting_order: Vec<(usize, isize)>, times_to_mix: i32) -> Vec<(usize, isize)> {
    let mut current_state = starting_order.clone();

    for _ in 1..=times_to_mix {
        for i in 0..starting_order.len() {
            let current_index = current_state
                .clone()
                .into_iter()
                .position(|x| x.0 == i)
                .unwrap();

            let state = current_state.clone();
            let (index, value) = state.get(current_index).unwrap();

            let new_index = (current_index as isize + value)
                .rem_euclid(current_state.len() as isize - 1) as usize;

            let mut new_state = vec![];
            if current_index > 0 {
                new_state.extend_from_slice(&current_state[..current_index].to_vec());
            }
            if current_index < current_state.len() {
                new_state.extend_from_slice(&current_state[(current_index + 1)..].to_vec());
            }

            current_state = vec![];
            if new_index > 0 {
                current_state.extend_from_slice(&new_state[0..new_index]);
            }
            current_state.push((index.clone(), value.clone()));
            if new_index < new_state.len() {
                current_state.extend_from_slice(&new_state[new_index..]);
            }
        }
    }
    current_state
}
