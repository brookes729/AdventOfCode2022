use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day1.txt");

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut running_calories = 0;

    for line in file_contents {
        if line.is_empty() {
            elf_calories.push(running_calories);
            running_calories = 0;
            continue;
        }
        let line_calories = line.parse::<i32>().unwrap();
        running_calories += line_calories;
    }

    let mut sorted_elves = elf_calories.clone();
    sorted_elves.sort_by(|a, b| b.cmp(a));

    println!(
        "Highest Three: {}, {}, {}, total: {}",
        sorted_elves[0],
        sorted_elves[1],
        sorted_elves[2],
        sorted_elves[0] + sorted_elves[1] + sorted_elves[2]
    )
}
