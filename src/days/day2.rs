use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day2.txt");

    let score_one: i32 = file_contents
        .into_iter()
        .map(|line| match line.as_str() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("Missed one somehow!"),
        })
        .sum();

    let file_contents = read_file_to_string_vector("day2.txt");

    let score_two: i32 = file_contents
        .into_iter()
        .map(|line| match line.as_str() {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("Missed one somehow!"),
        })
        .sum();

    println!("Part 1 Score: {}", score_one);
    println!("Part 2 Score: {}", score_two);
}
