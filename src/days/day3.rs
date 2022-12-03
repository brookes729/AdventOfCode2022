use std::collections::HashSet;

use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day3.txt");

    let mut running_total_one = 0;
    let mut running_total_two = 0;
    let mut bag_one: HashSet<char> = HashSet::new();
    let mut bag_two: HashSet<char> = HashSet::new();
    let mut bag_three: HashSet<char> = HashSet::new();

    for line in file_contents {
        let letter_halves = line.split_at(line.len() / 2);
        for char in letter_halves.1.chars() {
            if letter_halves.0.contains(&char.to_string()) {
                increase_total(char, &mut running_total_one);
                break;
            }
        }

        if bag_one.is_empty() {
            bag_one = line.chars().collect();
        } else if bag_two.is_empty() {
            bag_two = line.chars().collect();
        } else if bag_three.is_empty() {
            bag_three = line.chars().collect();
            let shared_one_two: HashSet<&char> = bag_one.intersection(&bag_two).collect();
            let shared_one_three: HashSet<&char> = bag_one.intersection(&bag_three).collect();
            //let shared_two_three: HashSet<&char> = bag_two.intersection(&bag_three).collect();
            let shared_all: HashSet<&&char> =
                shared_one_two.intersection(&shared_one_three).collect();

            if !shared_all.is_empty() {
                let matched_char = ***shared_all.iter().next().unwrap();
                increase_total(matched_char, &mut running_total_two);
            }
            bag_one = HashSet::new();
            bag_two = HashSet::new();
            bag_three = HashSet::new();
        }
    }

    println!("Part 1 Score: {}", running_total_one);
    println!("Part 2 Score: {}", running_total_two);
}

fn increase_total(char: char, running_total: &mut i32) {
    let mut char_value = (char as i32) - 96;
    if char_value < 0 {
        char_value += 58;
    }
    *running_total += char_value;
}
