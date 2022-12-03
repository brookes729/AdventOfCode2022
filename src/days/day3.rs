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
        let first_half_set: HashSet<char> = letter_halves.0.chars().collect();
        let second_half_set: HashSet<char> = letter_halves.1.chars().collect();

        let shared_char: &char = first_half_set
            .iter()
            .filter(|k| second_half_set.contains(k))
            .next()
            .unwrap();

        increase_total(*shared_char, &mut running_total_one);

        if bag_one.is_empty() {
            bag_one = line.chars().collect();
        } else if bag_two.is_empty() {
            bag_two = line.chars().collect();
        } else if bag_three.is_empty() {
            bag_three = line.chars().collect();

            let shared_char: &char = bag_one
                .iter()
                .filter(|k| bag_two.contains(k))
                .filter(|k| bag_three.contains(k))
                .next()
                .unwrap();

            increase_total(*shared_char, &mut running_total_two);

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
