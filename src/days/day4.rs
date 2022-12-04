use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number};
use regex::Regex;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day4.txt");

    let mut running_total_one = 0;
    let mut running_total_two = 0;

    for line in file_contents {
        let seperator = Regex::new(r"([,-]+)").expect("Invalid Regex");
        let line_sections: Vec<u32> = seperator
            .split(&line)
            .into_iter()
            .map(|s| string_to_number(s.to_string()))
            .collect();
        let range_one = Vec::from_iter(line_sections[0]..(line_sections[1] + 1));
        let range_two = Vec::from_iter(line_sections[2]..(line_sections[3] + 1));

        if range_one.iter().all(|item| range_two.contains(item))
            || range_two.iter().all(|item| range_one.contains(item))
        {
            running_total_one += 1;
        }
        if range_one.iter().any(|item| range_two.contains(item))
            || range_two.iter().any(|item| range_one.contains(item))
        {
            running_total_two += 1;
        }
    }

    println!("Part 1 Score: {}", running_total_one);
    println!("Part 2 Score: {}", running_total_two);
}
